use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pencakes;

fn main() {
    println!("Hello, world!");
    Pencakes::hello_macro();
}
