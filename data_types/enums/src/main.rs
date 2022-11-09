use crate::Color::Red;
use crate::Person::Name;
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
fn main() {
    let my_color = Color::Red;
    println!("{:?}", my_color);
    let my_color = Red;

    let person = Name(String::from("Alex"));
    println!("{:?}", person);
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue
}

#[derive(Debug)]
enum Person{
    Name(String),
    Surname(String),
    Age(u32)
}
