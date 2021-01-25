mod join;

use futures::executor::block_on;
use std::marker::PhantomPinned;
use std::pin::Pin;
use crate::join::join_call;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _pin: PhantomPinned,
}

impl Test {

    fn new(s: String) -> Self {
        Self {
            a: s,
            b: std::ptr::null(),
            _pin: PhantomPinned,
        }
    }

    fn init(self: Pin<&mut Self>) {
        let tem_b: *const String  = &self.a;
        let data = unsafe {self.get_unchecked_mut()};
        println!("{:?}", tem_b);
        data.b = tem_b;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe {&*(self.b)}  
    }
}

async fn lifetimes(x: &u8) -> u8 { *x }

async fn foo(x: &u8) -> u8 {
    lifetimes(x).await
}

fn main() {
    //let x = 90;
    let mf = async {
        let x = 9;
        let x:u8 = foo(&x).await;
        println!("{}", x);
    };
    //block_on(mf);
    //println!("{:?}", mf);
    println!("Hello, world!");
    block_on(mf);

    //// Pin ////
    let mut test1 = Test::new(String::from("test1"));
    //test1.init();
    let mut test2 = Test::new(String::from("test2"));
    //test2.init();
    //println!("test1 {:#?} test2 {:#?}", test1, test2);
    // unsatablity
    //std::mem::swap(&mut test1, &mut test2);
    //println!("test1 {:#?} test2 {:#?}", test1, test2);
    //
    ///// With Pin ///
    let mut test1 = unsafe {Pin::new_unchecked(&mut test1)};
    let mut test2 = unsafe {Pin::new_unchecked(&mut test2)};
    Test::init(test1.as_mut());
    Test::init(test2.as_mut());
    std::mem::swap(&mut test1, &mut test2);
    println!("test1 {:#?} test2 {:#?}", test1, test2);
    join_call();

}
