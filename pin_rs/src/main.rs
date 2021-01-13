use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _pin: PhantomPinned,
}

impl Test {
    fn new(a: String) -> Self {
        Test {
            a: a,
            b: std::ptr::null(),
            _pin: PhantomPinned,
        }
    }

    fn init(&mut self) {
        let data: *const String = &self.a;
        self.b = data;
    }

    fn init_pin(self: Pin<&mut Self>) {
        let this = unsafe {self.get_unchecked_mut()};     
        let data: *const String = &this.a;
        this.b = data; 
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn a_pin(self: Pin<&Self>) -> &str {
        ""
    }

    fn b(&self) -> &str {
        unsafe {&*self.b}
    }
    fn b_pin(self: Pin<&Self>) -> &str {
        unsafe {&*self.b}
    }
}

fn main() {
    let mut t1 = Test::new(String::from("test1")); 
    Test::init(&mut t1); 
    let mut t2 = Test::new(String::from("test2")); 
    Test::init(&mut t2); 
    /// When type was not pinned to mem.
    println!("T1 {:#?} {:#?}", t1.a(), t1.b());
    println!("T2 {:#?} {:#?}", t2.a(), t2.b());
    std::mem::swap(&mut t1, &mut t2);
    println!("T1 {:#?} {:#?}", t1.a(), t1.b());
    println!("T2 {:#?} {:#?}", t2.a(), t2.b());
    
    /// When type was pin
    let t1 = unsafe {Pin::new_unchecked(&mut t1) };
    Test::init_pin(t1);
}

