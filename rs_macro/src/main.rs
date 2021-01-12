macro_rules! hey {

    ($($name:expr),*) => {{
        let mut vec = Vec::new();
        $( vec.push($name); )*
        println!("{:?}", vec);
        vec
    }};
}

fn main() {
    println!("Hello, world!");
    hey!("my name", "veta");
}
