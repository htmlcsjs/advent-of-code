use std::{io, num::ParseIntError};

use error_macro::ErrorWrapper;

fn main() {
    let sus = Test::from(io::Error::new(io::ErrorKind::Other, "amogus"));
    if let Test::IoError(e) = sus {
        println!("{:?}", e);
    }
}

#[derive(ErrorWrapper)]
enum Test {
    IoError(io::Error),
    ParseIntError(ParseIntError),
}
