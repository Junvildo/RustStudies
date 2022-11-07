fn main() {
    println!("Hello, world!");
    // Formatting
    println!("My name is {}, and I'm {} years old", "Tien Loc", 20);
    // Expressions
    println!("a + b = {}", 3 + 9);
    // Positional Arguments
    println!("{0} has a {2} and {1}", "Tien Loc", "cat", "dog");
    // Named Arguments
    println!("{name} {surname}", surname="Tien", name="Loc");
    // Printting traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 50, 50, 50);
    // Debug
    println!("Array {:?}", [1, 2, 3]);
}
