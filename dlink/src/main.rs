// use std::mem::take;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

type ListNode<T> = Box<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T>
where
    T: Debug,
{
    value: T,
    next: Option<ListNode<T>>,
    prev: Option<ListNode<T>>,
}

impl<T> Node<T>
where
    T: Debug,
{
    pub fn new(value: T, next: Option<ListNode<T>>, prev: Option<ListNode<T>>) -> Self {
        Node {
            value: value,
            next: next,
            prev: prev,
        }
    }

    pub fn into_option_box(self) -> ListNode<T> {
        Box::new(Rc::new(RefCell::new(self)))
    }
}

#[derive(Debug)]
struct Tail<T>
where
    T: Debug,
{
    head: Option<ListNode<T>>,
}

impl<T> Tail<T>
where
    T: Debug,
{
    pub fn new(head: Option<ListNode<T>>) -> Self {
        Tail { head: head }
    }

    pub fn push(&mut self, mut new_node: Node<T>) {
        let head = self.head.take();
        if let Some(old_head) = head {
            new_node.next = Some(old_head.clone());
            let new_node = new_node.into_option_box();
            self.head = Some(new_node.clone());
            let mut old_head = old_head.borrow_mut();
            old_head.prev = Some(new_node.clone());
        }
    }

    pub fn traverse(&self, next: &Option<ListNode<T>>) {
        if let Some(next) = next {
            let next = next.borrow();
            println!("Node value: {:?}", next.value);
            if let Some(prev) = &next.prev {
                let prev = prev.borrow();
                println!("Prev value {:?}", prev.value);
            } else {
                println!("Prev value None");
            }
            &self.traverse(&next.next);
        }
    }
}

fn main() {}

#[test]
fn make_it() {
    let n0 = Node::new(0, None, None);
    let mut tail = Tail::new(Some(n0.into_option_box()));
    let n1 = Node::new(1, None, None);
    let n2 = Node::new(2, None, None);
    let n3 = Node::new(3, None, None);
    let n4 = Node::new(4, None, None);
    let n5 = Node::new(5, None, None);

    tail.push(n1);
    tail.push(n2);
    tail.push(n3);
    tail.push(n4);
    tail.push(n5);

    dbg!("{:?}", tail.traverse(&tail.head));
}
