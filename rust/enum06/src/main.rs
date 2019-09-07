#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}


fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let localhost = IpAddrKind::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", localhost);
}
