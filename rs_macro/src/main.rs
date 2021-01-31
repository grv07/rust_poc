macro_rules! hey {
    ($($name:expr),*) => {{
        let mut vec = Vec::new();
        $( vec.push($name); )*
        println!("{:?}", vec);
        vec
    }};
}

macro_rules! data {
    () => {
        println!("data");
    };
}

macro_rules! avec {
    () => {
        Vec::new()
    };

    ($($item:expr),+ $(,)?) => {{
        let mut vc = Vec::new();
        $(vc.push($item);)*
        vc
    }};

    ($($item:expr),+; $($item2:ident),*) => {{
        let mut vc = Vec::new();
        for _ 0..$item2 {
            vc.push($item2);
        }
        vc
    }};

    ($item:expr, $item2:expr) => {{
        let mut vc = Vec::new();
        vc.push($item);
        vc.push($item);
        vc
    }};
}

fn main() {
    println!("Hello, world!");
    hey!["my name", "veta"];
    data![];
}

#[test]
fn empty_vec() {
    let d: Vec<i32> = avec!();
    assert!(d.is_empty());
}

#[test]
fn single_test() {
    let d: Vec<i32> = avec!(75);
    assert!(!d.is_empty());
}

#[test]
fn double_test() {
    let d: Vec<i32> = avec!(7, 5,);
    assert_eq!(2, d.len());
}
