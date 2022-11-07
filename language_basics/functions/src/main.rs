#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {                 
    let mut name = "John";
    say_hello(name);
    say_hello2(&mut name);
    let greeting = say_hello3(&mut name);
    println!("{}", greeting);
}       
fn say_hello(name: &str) {     
    println!("Hello {}", name);
}

fn say_hello2(name: &mut &str) {
    println!("Hello {}",name);
    *name = "Alex";
}

fn say_hello3(name: &mut &str) -> String {
    let greeting = format!("Hello {}", name);
    return greeting;
}
