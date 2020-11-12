use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

struct Sender<T> {
    share: Share<T>,
}

impl<T> Sender<T> {
    fn new(share: Share<T>) -> Self {
        Sender { share: share }
    }

    fn send(&self, value: T) {
        let mut data = self.share.queue.lock().unwrap();
        data.push_back(value);
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
        let mut queue = self.share.queue.lock().unwrap();
        println!("{:?}", queue);
        queue.pop_front()
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

struct Share<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
}

fn channel<T>() -> (Sender<T>, Reciver<T>) {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let tx_share = Share {
        queue: Arc::clone(&queue),
    };
    let rx_share = Share { queue: queue };

    (Sender::new(tx_share), Reciver::new(rx_share))
}

fn main() {}

#[test]
fn pub_sub() {
    let (tx, rx) = channel();
    tx.send(1);
    assert_eq!(Some(1), rx.rcv());
}
