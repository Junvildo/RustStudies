use std::{fs::File, io::Read};
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("src/username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let a = read_username_from_file();
    println!("{:?}", a);
}
