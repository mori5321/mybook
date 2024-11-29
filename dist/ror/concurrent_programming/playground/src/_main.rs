use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{fence, AtomicBool, AtomicUsize, Ordering};


// スレッドの最大数
pub const NUM_LOCK: usize = 8;
const MASK: usize = NUM_LOCK - 1; // NUM_LOCKの剰余を求めるためのビットマスク

pub struct FairLock<T> {
    waiting: Vec<AtomicBool>, // n番目のスレッドがロック獲得試行中ならば waiting == true
    lock: AtomicBool, // スピンロック
    turn: AtomicUsize,
    data: UnsafeCell<T>,
}

// ロックの開放と保護対象データへのアクセスを行うための型
pub struct FairLockGuard<'a, T> {
    fair_lock: &'a FairLock<T>,
    idx: usize,
}

impl<T> FairLock<T> {
    pub fn new(v: T) -> Self {
        let mut vec = Vec::new();
        for _ in 0..NUM_LOCK {
            vec.push(AtomicBool::new(false));
        }
    
        FairLock {
            waiting: vec,
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(v),
            turn: AtomicUsize::new(0),
        }
    }

    pub fn lock(&self, idx: usize) -> FairLockGuard<T> {
        assert!(idx < NUM_LOCK); // idxが最大数未満であるか検査

        // 自分を待機中にする set true
        // この値は他スレッドからfalseに設定される場合があり、この場合は実行権限が委譲されたことを示す。この委譲のタイミングで公平性を決める
        self.waiting[idx].store(true, Ordering::Relaxed);
        loop {
            // 他のスレッドがfalseを設定した場合にロック獲得
            // = 自分の値がfalse(not waiting)かどうか?
            // = 実行権限が委譲されたかどうか
            if self.waiting[idx].load(Ordering::Relaxed) {
                break; // falseならばロック獲得
            }

            if !self.lock.load(Ordering::Relaxed) {
                // waiting = trueのままなら、TTASを実行しロックを獲得できた場合にループを抜ける
                if let Ok(_) = self.lock.compare_exchange_weak(
                    false,
                    true, 
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    break;
                }
            }
        }
        fence(Ordering::Acquire);

        FairLockGuard {
            fair_lock: self,
            idx,
        }
    }
}

impl<'a, T>Drop for FairLockGuard<'a, T> {
    fn drop(&mut self) {
        let fl = self.fair_lock;

        fl.waiting[self.idx].store(false, Ordering::Relaxed);
        
        // 現在のロック獲得優先スレッドが自分なら、次のスレッドに設定する
        let turn = fl.turn.load(Ordering::Relaxed);
        let next = if turn == self.idx {
            (turn + 1) & MASK
        } else {
            turn 
        };

        if fl.waiting[next].load(Ordering::Relaxed) {
            // 次のロック獲得優先スレッドがロック獲得中ならば、それスレッドにロックを渡す(委譲する,waiting true -> false)
            fl.turn.store(next, Ordering::Relaxed);
            fl.waiting[next].store(false, Ordering::Release);
        } else {
            // 次のロック獲得優先スレッドがロック獲得中でない場合、その次のスレッドにロックを渡す
            fl.turn.store((next + 1) & MASK, Ordering::Relaxed);
            fl.lock.store(false, Ordering::Release)
        }
    }
}

#![allow(unused)]
fn main() {
    println!("Hello, world!");
}
