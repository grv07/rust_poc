// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

//use derive_builder::Builder;
//
//#[derive(Builder)]
//pub struct Command {
//    executable: String,
//    #[builder(each = "arg")]
//    args: Vec<String>,
//    #[builder(each = "env")]
//    env: Vec<String>,
//    current_dir: Option<String>,
//}
//
//fn main() {
//    let command = Command::builder()
//        .executable("cargo".to_owned())
//        .arg("build".to_owned())
//        .arg("--release".to_owned())
//        .build();
//    println!("{:?}", command.is_ok());
//
//    command.unwrap();
//
//    assert_eq!(command.executable, "cargo");
//    assert_eq!(command.args, vec!["build", "--release"]);
//    }

use sorted::sorted;

use std::fmt::{self, Display};
use std::io;

#[sorted]
pub enum Error {
    Fmt(fmt::Error),
    Io(io::Error),
}

impl Display for Error {
    #[sorted::check]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        //#[sorted]
        match self {
            Io(e) => write!(f, "{}", e),
            Fmt(e) => write!(f, "{}", e),
        }
    }
}

fn main() {}
