use std::fmt::Debug;

#[derive(Debug)]
struct Node<T>
where
    T: Debug,
{
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Debug,
{
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Node {
            value: value,
            next: next,
        }
    }
}

#[derive(Debug)]
struct Tail<T>
where
    T: Debug,
{
    head: Option<Box<Node<T>>>,
}

impl<T> Tail<T>
where
    T: Debug,
{
    pub fn new(head: Option<Box<Node<T>>>) -> Self {
        if let Some(head) = head {
            return Tail { head: Some(head) };
        }
        Tail { head: None }
    }
    
    pub fn push(&mut self, mut node: Box<Node<T>>) {
        let old_head = self.head.take();
        if let Some(_) = old_head {
            node.next = old_head;
            self.head = Some(node);
        } else {
            self.head = Some(node);
        }
    }
}

impl<T> Iterator for Tail<T> where T: Debug {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(old_head) = self.head.take() {
            self.head = old_head.next;
            return Some(old_head.value);
        }
        None
    }
}

fn main() {
    let mut tail: Tail<u64> = Tail::new(None);

    let n1 = Box::new(Node::new(1, None));
    let n2 = Box::new(Node::new(2, None));
    let n3 = Box::new(Node::new(3, None));
    let n4 = Box::new(Node::new(4, None));
    tail.push(n1);
    tail.push(n2);
    tail.push(n3);
    tail.push(n4);

    for i in tail.into_iter() {
        dbg!(i);
    }
}
