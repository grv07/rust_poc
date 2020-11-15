
use std::rc::Rc;


struct Inner<T> {
    data: T,
}
struct P<T> {
    inner: Box<Rc<Inner<T>>>,
}

impl<T>  P<T> {
    fn new(value: T) -> Self {
        let inner = Inner {data: value};
        P {
            inner: Box::new(Rc::new(inner)),
        }
    }
}

fn main() {
    let mut p = P::new(34);
    let m_t = Rc::get_mut(&mut p.inner).unwrap();
    m_t.data = 90;
    println!("Hello, world!");
}
