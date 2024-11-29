use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

struct Task {
    // 実行するコルーチン
    future: Mutex<BoxFuture<'static, ()>>,

    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 自信をスケジューリングする
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}


struct Executor {
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}

impl Executor {
    fn new() -> Self {
        // キューの最大サイズは1024
        let (sender, receiver) = sync_channel(1024);
        Executor {
            sender: sender.clone(),
            receiver
        }
    }

    // 新たにTaskを生成して実行キューにキューイングするためのオブジェクトをリターンする
    //
    fn get_spawner(&self) -> Spawner {
        Spawner {
            sender: self.sender.clone()
        }
    }

    // タスクをチャンネルから受信して実行する
    fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            let mut future = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);

            let _ = future.as_mut().poll(&mut ctx);
        }
    }
}

struct Spawner {
    sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    // タスクの生成とキューへの追加
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });

        self.sender.send(task).unwrap();
    }
}

enum StateHello {
    HELLO,
    WORLD,
    END
}

struct Hello {
    state: StateHello
}

impl Hello {
    fn new() -> Hello {
        Hello {
            state: StateHello::HELLO,
        }
    }
}

impl Future for Hello {
    type Output = (); 

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                println!("Hello");
                (*self).state = StateHello::WORLD;
                cx.waker().wake_by_ref();
                return Poll::Pending
            }
            StateHello::WORLD => {
                println!("World");
                (*self).state = StateHello::END;
                cx.waker().wake_by_ref(); // 自信をenqueue
                return Poll::Pending
            }
            StateHello::END => {
                println!("End");
                return Poll::Ready(());
            }
        }
    }
}


// fn main() {
//     let executor = Executor::new();
//     executor.get_spawner().spawn(Hello::new());
//     executor.run();
// }


fn main() {
    let executor = Executor::new();
    executor.get_spawner().spawn(async {
        // async で 囲まれた部分がFutureトレイとを実装した型の値に変換され
        // await で Futureトレイとのpollを’呼び出す。
        // つまり async { コード }
        // と書かれた場合、Futureトレイとを実装した型がコンパイラによって新たに定義され、
        // async { コード } 部分には、その型のnew関数に相当する呼び出しが行われる。
        // またそのpoll関数には、asyncのコード部分が実装されている
        // 
        // h.await の意味は以下の省略と捉えてよい
        // match h.poll(cx) {
        //    Poll::Pending => return Poll::Pending
        //    Poll::Result(x) +> x,
        // }
        //
        // こうすることでasync, つまりFutureトレイトのpoll関数がネストして呼び出された場合でも
        // 関数の中断と値返しを適切に行うことができる。
        // つまりpoll関数の呼び出しでPendingがリターンされた場合は、ExecutorまでPendingであることがさかのぼって伝達されていく。
        let h = Hello::new();
        h.await;
    });

    executor.run();
}
