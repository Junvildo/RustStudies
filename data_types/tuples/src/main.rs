#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

fn main() {
    let mut person: (&str, i64, bool) = ("John", 27, true);
    println!("{:?}", person);

    println!("Name: {}", person.0);

    person.0 = "Mike";
    println!("Name: {}", person.0);
    let (name, age, employment) = person;
    println!("Name: {}, age: {}, employment: {}", name, age, employment);
}
