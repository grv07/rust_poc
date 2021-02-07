use std::sync::Arc;
use std::thread;

struct Test {
    data: Arc<i32>,
}

fn main() {
    let mut test = Test {data: Arc::new(67)};
    let pi = Arc::clone(&test.data);

    let handle = thread::spawn(move || {
       test.data = Arc::new(788); 
       println!("{}", test.data);
    });
    handle.join();
    println!("{}", pi);
    println!("{}", pi);
}
