use std::time::{Duration, Instant};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;
use std::collections::VecDeque;

//use furure::task;

struct MiniTokio {
    tasks: VecDeque<Task>
}

fn main() {
    let mut mini_tokio = MiniTokio::new();
    mini_tokio.spawn(
        async {
            let when = Instant::now() + Duration::from_millis(100);
            let future = Delay { when };
            let out = future.await;
            assert_eq!("Done", out);
        });
    mini_tokio.run();
}

impl MiniTokio {
    fn new() -> Self {
        MiniTokio {
            tasks: VecDeque::new()
        }
    }

    fn spawn<F>(&self, future: F) where F: Future<Output=()> + Send + 'static {
        self.tasks.push_front(Box::pin(future));
    } 
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;
    
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready("done")
        } else {
            ctx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
