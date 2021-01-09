fn main() {
    demo();
}

fn demo() {
    let j = |x: i32|  x;
    let i = || 1;
    println!("{}, {:?}", i(), j(78));
}

fn capturing() {
    let color = String::from("blue");
    let print_color = || println!("{}", color);
    
    // Below 3 line compiles because its just using ref 
    print_color();
    let _re_borrow_color = &color;
    print_color();
    
    let mut count = 1;

    let mut inc = || {
        count += 1;
        println!("{}", count);
    };
    
   // let _re_borrow_count = count;
    inc();
    // You cant take more then 1 mut ref.
    //let _p = &mut count;
    inc();

    let moveable = Box::new(2);
    let consume = || {
        println!("moveable = {:?}", moveable);
        std::mem::drop(moveable);
    }; 
    consume();
    consume();
}
