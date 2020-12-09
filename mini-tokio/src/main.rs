use std::time::{Duration, Instant};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;
use std::collections::VecDeque;
use futures::task;

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
            assert_eq!("done", out);
        });
    mini_tokio.spawn(
        async {
            let when = Instant::now() + Duration::from_millis(1900);
            let future = Delay { when };
            let out = future.await;
            assert_eq!("done", out);
        });
    mini_tokio.run();
}

impl MiniTokio {
    fn new() -> Self {
        MiniTokio {
            tasks: VecDeque::new()
        }
    }

    fn spawn<F>(&mut self, future: F) where F: Future<Output=()> + Send + 'static {
        self.tasks.push_front(Box::pin(future));
    }

    fn run(&mut self) {
       let waker = task::noop_waker();
       let mut ctx = Context::from_waker(&waker);
       while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut ctx).is_pending() {
                self.tasks.push_back(task);
            }
       } 
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
            println!("Done ...");
            Poll::Ready("done")
        } else {
            ctx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
