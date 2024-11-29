use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

// ロックフリースタック

// スタックのノード。リスト構造で管理する。
struct Node<T> {
    next: AtomicPtr<Node<T>>,
    data: T,
}

// スタックの先頭
pub struct StackBad<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> StackBad<T> {
    pub fn new() -> Self {
        StackBad {
            head: AtomicPtr::new(null_mut())
        }
    }

    pub fn push(&self, v: T) {
        let node = Box::new(Node {
            next: AtomicPtr::new(null_mut()),
            data: v,
        });

        // Boxからpointer を取り出す
        let ptr = Box::into_raw(node);
        
        unsafe {
            loop {
                // 現在のheadをとってくる
                let head = self.head.load(Ordering::Relaxed);

                // 新しいノードのnextにheadを設定
                (*ptr).next.store(head, Ordering::Relaxed);

                // CAS操作 で atomic に head を更新する
                // CAS = Compare And Swap
                // head が
                // 取得したタイミングから変わっていないかを確認して、変わっていなければself.headを更新する
                if let Ok(_) = self.head.compare_exchange_weak(head, ptr, Ordering::Release, Ordering::Relaxed) { // Order の指定の仕方がよくわからん
                    break;
                }
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        unsafe {
            loop {
                let head = self.head.load(Ordering::Relaxed);
                if head == null_mut() {
                    return None;
                }

                // 次のheadになるやつ
                let next = (*head).next.load(Ordering::Relaxed);

                // head の値が取得時から更新されていないか確認する。
                // 更新されていないならば、head.next を self.head にする。
                if let Ok(_) = self.head.compare_exchange_weak(head, next, Ordering::Acquire, Ordering::Relaxed) {
                    let h = Box::from_raw(head);
                    return Some((*h).data);
                }
            }
        }
    }
}

