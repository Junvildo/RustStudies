struct Dog {}
struct Cat {}



trait Animal {
    fn make_noise(&self) -> &'static str;
}

fn get_animal(rand_number: f64) -> Animal {
    if rand_number < 1.0 {
        Dog {}
    } else {
        Cat {}
    }
}


fn main() {
    println!("Hello, world!");
}
