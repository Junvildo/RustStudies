use std::{fs::{File, OpenOptions, remove_file}, io::{Write, Read}};
fn main() {
    //let mut file=File::create("src/example.txt").expect("created failed");
    //file.write_all("Hello World!!!\n".as_bytes()).expect("failed");
    //let mut file = OpenOptions::new().append(true)
    //    .open("src/example.txt")
    //    .expect("cannot open file");
    //file.write_all("Adding content to the file.\n".as_bytes()).expect("write failed");
    

    let mut file = File::open("src/example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    remove_file("src/example.txt").expect("failed");
}
