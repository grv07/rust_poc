use std::collections::VecDeque;
// use std::ops::Drop;
use std::sync::{Arc, Condvar, Mutex};

struct Share<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    is_available: Condvar,
}

struct Sender<T> {
    share: Share<T>,
    count: usize,
}

impl<T> Sender<T> {
    fn new(share: Share<T>) -> Self {
        Sender {
            share: share,
            count: 1,
        }
    }

    fn send(&self, value: T) {
        let mut data = self.share.queue.lock().unwrap();
        data.push_back(value);
        self.share.is_available.notify_one();
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        dbg!("Sender is droping ...");
        self.count -= 1;
        self.share.is_available.notify_one();
    }
}

struct Reciver<T> {
    share: Share<T>,
}

impl<T> Reciver<T> {
    fn new(share: Share<T>) -> Self {
        Reciver { share: share }
    }

    fn rcv(&self) -> Option<T>
    where
        T: std::fmt::Debug,
    {
        loop {
            let mut queue = self.share.queue.lock().unwrap();
            if let Some(t) = queue.pop_front() {
                return Some(t);
            } else {
                self.share.is_available.wait(queue);
            }
        }
    }
}

impl<T> Iterator for Reciver<T>
where
    T: std::fmt::Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.rcv();
        println!("rcv > {:?}", r);
        r
    }
}

fn channel<T>() -> (Sender<T>, Reciver<T>) {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let tx_share = Share {
        queue: Arc::clone(&queue),
        is_available: Condvar::new(),
    };
    let rx_share = Share {
        queue: queue,
        is_available: Condvar::new(),
    };

    (Sender::new(tx_share), Reciver::new(rx_share))
}

fn main() {}

#[test]
fn pub_sub() {
    let (tx, rx) = channel();
    tx.send(1);
    drop(tx);
    assert_eq!(Some(1), rx.rcv());
}
