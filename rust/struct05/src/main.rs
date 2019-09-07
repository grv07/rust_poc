#[derive(Debug)]
struct User {
    username: String,
    email: String,
    is_active: bool,
}

impl User {
    fn get_username(&self) -> &String {
        &self.username
    }
}

//Tuple struct
#[derive(Debug)]
struct Point(i32, i32);
#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    let user = User {
        username: String::from("help"),
        email: String::from("help@gmail.com"),
        is_active: true,
    };
    println!("{:?}", user.email);
    println!("username > {:?}", user.get_username());
    let origin = Point(0, 0);
    let black = Color(0, 0, 0);
    println!("{:?}", origin.1);
    println!("{} >> {:#?}", black.2, black);
}
