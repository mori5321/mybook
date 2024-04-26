// Box::<dyn Fn<T> -> A>
// はヒープ上に確保された関数とデータへのポインタを持つクロージャへのスマートポインタである
// fn mul_x(x: u64) -> Box::<dyn Fn(u64) -> u64> {
//     // xは自由変数であるため、
//     // xを借用するか 所有権を移す必要がある
//     Box::new(move |y| x * y)
// }

// 自由変数 = 関数の外で定義される変数
// 束縛変数 = 関数内で定義される変数
//
// C言語は関数内で関数を定義できないため
// 自由変数 = グローバル変数
// 束縛変数 = ローカル変数
// が成り立つ
//
// しかしRustでは 関数内で関数を定義できるため
// グローバル変数 = 自由変数
// ローカル変数 = 自由変数 or 束縛変数 のどちらにもなりうる


// fn main() {
//     let f = mul_x(3);
//     println!("{}", f(4));
//
//     let f2 = |a| move |b| a * b;
//     println!("{}", f2(3)(4));
// }


// struct Foo {
//     val: u32
// }

// ライフタイムも方の一種である
// ライフタイムを明示できるのは参照のみである

// fn add<'a>(x: &'a Foo, y: &'a Foo) -> u32 {
//     x.val + y.val
// }
//
// fn main() {
//     let x = Foo { val: 3 };
//
//     {
//         let y = Foo { val: 4 };
//         let z = add(&x, &y);
//         println!("{}", z);
//     }
// }

// Shared Nothing を 時間単位での排他性で実現するのがRust
//
//


// unwrapのつかいどころ => 呼んでもpanicが発生しないと思われるとき
// 基本的にはエラーハンドリングを行うべき
//

// use std::thread::spawn;
//
// // fn hello() {
// //     println!("Hello World");
// // }
// //
// // fn main() {
// //     let _ = spawn(hello).join(); // デタッチスレッドであるためjoinする必要はないが、join関数でスレッドの終了を待つこともできる
// //
// //     let h = || println!("Hello World");
// //     let _ = spawn(h).join();
// // }
// //
//
// fn func() {
//     let v = 10;
//     let f = move || v * 2;
//
//     let result = spawn(f).join();
//
//     println!("{}", result.unwrap());
//
//     // スレッドがpanicしたらEither Errがもらえる
//     match spawn(|| panic!("I'm panicked")).join() {
//         Ok(_) => println!("Success"),
//         Err(e) => println!("Error: {:?}", e),
//     }
// }
//
// fn main() {
//     func();
// }
//

// use std::sync::{Arc, Mutex};
// use std::thread;
//
// fn some_func(lock: Arc<Mutex<u64>>) {
//     loop {
//         let mut val = lock.lock().unwrap();
//         *val += 1;
//         println!("{}", *val);
//     }
// }
//
//
// fn main() {
//     let lock0 = Arc::new(Mutex::new(0));
//     let lock1 =lock0.clone();
//
//     let th0 = thread::spawn(move || {
//         some_func(lock0);
//     });
//
//     let th1 = thread::spawn(move || {
//         some_func(lock1);
//     });
//
//     th0.join().unwrap();
//     th1.join().unwrap();
// }


// use std::sync::{Arc, Mutex, Condvar};
// use std::thread;
//
// // なぜ Mutex と Condvar をペアにするのか？
// fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
//     let &(ref lock, ref cvar) = &*p;
//
//     // parentのlockが外れたらとれる OR parentがロックする前に取れる
//     let mut started = lock.lock().unwrap();
//     while !*started { // parentが先によばれた場合の対策  OR 擬似覚醒の対策 
//        
//         // cvar.waitは lockを一時的に解放して待機する。通知を受けるとlockを再取得する。
//         // > This function will atomically unlock the mutex specified (represented by guard) and block the current thread. 
//         started = cvar.wait(started).unwrap(); // 通知があるまで待機
//     }
//
//     // lockの解放はどこで行ってるの?
//     // スコープを抜けたら解放されるっぽいぞ。賢い。
//     // 
//     // Rustでは保護対象データがスコープを外れたときに自動的にロックを解放するようになっている。
//        
//     println!("Child {} started", id);
// }
//
// fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
//     let &(ref lock, ref cvar) = &*p;
//
//     let mut started = lock.lock().unwrap();
//     *started = true;
//     cvar.notify_all();
//     println!("parent");
// }
//
// fn main() {
//     let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
//     let pair1 = pair0.clone();
//     let pair2 = pair0.clone();
//
//     let c0 = thread::spawn(move || { child(0, pair0) });
//     let c1 = thread::spawn(move || { child(1, pair1) });
//     let p = thread::spawn(move || { parent(pair2) });
//
//
//     c0.join().unwrap();
//     c1.join().unwrap();
//     p.join().unwrap();
// }

// use std::sync::{Arc, Barrier};
// use std::thread;
//
// fn main() {
//     let mut v = Vec::new();
//     let barrier = Arc::new(Barrier::new(10)); // 10スレッドがバリアに到達するまで待機
//
//     for _  in 0..10 {
//         let b = barrier.clone();
//         let th = thread::spawn(move || {
//             b.wait();
//             println!("Finish Barrier");
//         });
//         v.push(th);
//     }
//
//     for th in v {
//         th.join().unwrap();
//     }
// }

// mod semaphore;
//
// use semaphore::Semaphore;
// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::sync::Arc;
//
// const NUM_LOOP: usize = 100000;
// const NUM_THREADS: usize = 10;
// const SEM_NUM: isize = 4;
//
// static mut CNT: AtomicUsize = AtomicUsize::new(0);
//
// fn main() {
//     let mut v = Vec::new();
//     let sem = Arc::new(Semaphore::new(SEM_NUM));
//
//     for i in 0..NUM_THREADS {
//         let s = sem.clone();
//         let t = std::thread::spawn(move  || {
//             for _ in 0..NUM_LOOP {
//                 s.wait();
//
//                 unsafe { CNT.fetch_add(1, Ordering::SeqCst) };
//                 let n = unsafe { CNT.load(Ordering::SeqCst) };
//                 println!("Thread {}: {}", i, n);
//                 assert!(n <= SEM_NUM as usize);
//                 unsafe { CNT.fetch_sub(1, Ordering::SeqCst) };
//
//                 s.post();
//             }
//         });
//
//         v.push(t);
//     }
//
//     for t in v {
//         t.join().unwrap();
//     }
// }

// mod semaphore;
//
// use semaphore::Semaphore;
// use std::collections::LinkedList;
// use std::sync::{Arc, Condvar, Mutex};
//
// #[derive(Clone)]
// pub struct Sender<T> {
//     sem: Arc<Semaphore>, // セマフォ
//     buf: Arc<Mutex<LinkedList<T>>>, // キュー
//     cond: Arc<Condvar> // 読み込み側の条件変数
// }
//
// impl<T: Send> Sender<T> {
//     pub fn send(&self, data: T) {
//         self.sem.wait(); // キューの最大値ならば待機。
//         let mut buf = self.buf.lock().unwrap();
//         buf.push_back(data); // エンキュー
//         self.cond.notify_one(); // 読み込み側に通知 
//                                 // notify_oneってなに? 
//                                 // 待機しているスレッドのうち1つだけを起こす
//     }
// }
//
// pub struct Receiver<T> {
//     sem: Arc<Semaphore>,
//     buf: Arc<Mutex<LinkedList<T>>>,
//     cond: Arc<Condvar>,
// }
//
// impl <T> Receiver<T> {
//     pub fn recv(&self) -> T {
//         let mut buf = self.buf.lock().unwrap();
//         loop {
//             if let Some(data) = buf.pop_front() {
//                 self.sem.post();
//                 return data;
//             }
//             // bufが空ならば条件変数を待機
//             buf = self.cond.wait(buf).unwrap();
//         }
//     }
// }
//
// pub fn channel<T>(max: isize) -> (Sender<T>, Receiver<T>) {
//     assert!(max > 0);
//     let sem = Arc::new(Semaphore::new(max));
//     let buf = Arc::new(Mutex::new(LinkedList::new()));
//     let cond = Arc::new(Condvar::new());
//     let tx = Sender {
//         sem: sem.clone(),
//         buf: buf.clone(),
//         cond: cond.clone(),
//     };
//     let rx = Receiver { sem, buf, cond };
//     (tx, rx)
// }
//
// const NUM_LOOP: usize = 100000;
// const NUM_THREADS: usize = 8;
//
// fn main() {
//     let (tx, rx) = channel(4);
//     let mut v = Vec::new();
//
//     let t = std::thread::spawn(move || {
//         let mut cnt = 0;
//         while cnt < NUM_THREADS * NUM_LOOP {
//             let n = rx.recv();
//             println!("recv: n = {:?}", n);
//             cnt += 1;
//         }
//     });
//
//     v.push(t);
//
//     for i in 0..NUM_THREADS {
//         let tx0 = tx.clone();
//         let t = std::thread::spawn(move || {
//             for j in 0..NUM_LOOP {
//                 tx0.send((i, j));
//             }
//         });
//
//         v.push(t);
//     }
//
//     for t in v {
//         t.join().unwrap();
//     }
// }


// アトミック命令を利用しない同期手法 -- パン屋のアルゴリズム
use std::ptr::{read_volatile, write_volatile};
use std::sync::atomic::{fence, Ordering}; // メモリバリア用の関数
use std::thread;

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100000;

macro_rules! read_mem { // コンパイラによる最適化を抑制してメモリに書き込む
    ($addr: expr) => { unsafe { read_volatile($addr)} }
}

macro_rules! write_mem {
    ($addr: expr, $val: expr) => { unsafe { write_volatile($addr, $val)} }
}

struct BakeryLock {
    entering: [bool; NUM_THREADS],
    tickets: [Option<u64>;  NUM_THREADS],
}

struct LockGuard { idx: usize }

impl BakeryLock {
    fn lock(&mut self, idx: usize) -> LockGuard {
        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], true);
        fence(Ordering::SeqCst);

        let mut max = 0;
        for i in 0..NUM_THREADS {
            if let Some(t) = read_mem!(&self.tickets[i]) {
                max = max.max(t);
            }
        }

        let ticket = max + 1;
        write_mem!(&mut self.tickets[idx], Some(ticket));

        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], false);
        fence(Ordering::SeqCst);
        
        for i in 0..NUM_THREADS {
            if i == idx {
                continue;
            }

            while read_mem!(&self.entering[i]) {}

            loop {
                match read_mem!(&self.tickets[i]) {
                    Some(t) => {
                        if ticket < t || (ticket == t && idx < i) {
                            break;
                        } 
                    },
                    None => {
                        break;
                    }
                }
            }
        }

        // fence(Ordering::SeqCst) is 何
        fence(Ordering::SeqCst);
        LockGuard { idx }
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.idx], None);
    }
}
    
static mut LOCK: BakeryLock = BakeryLock {
    entering: [false; NUM_THREADS],
    tickets: [None; NUM_THREADS],
};

static mut COUNT: u64 = 0;

fn main() {
    let mut v = Vec::new();

    for i in 0..NUM_THREADS {
        let th = thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let _lock = unsafe { LOCK.lock(i) };
                unsafe {
                    let c = read_volatile(&COUNT);
                    write_volatile(&mut COUNT, c + 1);
                }
            }
        });

        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {})"    ,
        unsafe { COUNT },
        NUM_LOOP * NUM_THREADS,
    )
}
