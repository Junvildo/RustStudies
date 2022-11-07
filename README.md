# Cargo

**Cargo is the package manager for Rust**

`cargo new <project_name>` to build a new project

`cargo run` to run project, need to be in the project folder

`cargo build` to compile the project to machine code, but not run the project, need to be in the project folder

`cargo clean` clean everything beside the source codes

`cargo check` check the project to make sure there are no errors

`cargo doc` create documentation for project. The documentation file is the `index.html` file in `project_name/target/doc/<project_name>/`

# Basic
## Comments
`//` : **line comment**. Ex: `// This is a line comment`

`/* */`: **multi line comments** are allowed but rarely used.

Ex:
`/* This is not
very common */`

`///`: **doc comments**. This is mainly used to document functionality.

Ex: `/// This is mainly used to document functionality.`
## Markdown
### Heading
`//! # Main heading`
### Code

## Language Basic
### Variables
`let name = "John"`

`let age = 20`
Rust is a strongly typed language

Variable type is optional if it can be inferred

Type can be specified explicitly

`let amount:i64 = 14817283416`

`let amount = 591849080923` // error
#### Naming rule
* Names can contain letters, numbers and underscores
* Must start with a letter or underscore
* Follow snake_case naming convention
* Immutable by default

    ```
    let length = 34;
    length = 35;    //error
    ```
* Can be declared mutable

    ```
    let mut length = 34;
    lenght = 35;
    ```
* Shadowing is allowed

    ```
    let color = "blue";
    let color = "red";
    println!("Color is {}", color)  // Color is red
    ```

* Declaring multiple variables simultaneously
    `let (a, b, c) = (2, 3, 4);`

### Scalar Data Types
#### Data Types
##### Int
|Size|Signed|Unsigned|
|----|------|--------|
|8 bit|i8|u8|
|16 bit|i16|u16|
|**32 bit**| **i32** |u32|
|64 bit|i64|u64|
|128 bit|i128|u128|
|arch|isize|usize|
##### Float
|Size|Float|
|----|-----|
|32 bit|f32|
|64 bit|f64|
##### Boolean
`let is_day = true;`

`let is_night = false;`
##### Character
`let char1 = 'A';`

`let smiley_face = '\u{1F601}'; // =)`
#### Type Casting
`let pi: f32 = 4;    //mismatched types error`
#### Number Separator
`let million = 1_000_000;`

`let random = 3_52.34_2234_334;`
#### String
##### String slices
`let cat: &str = "Fluffy";`

`let cat: &'static str = "Fluffy";`

String slices are immutable
##### String objects
`let dog = String::new();`

`let mut dog = String::from("Max");`
##### format! (string builder)
`format!("Hi {} how are you", "Mark");`
#### String methods
##### len
`println!("{}", dog.len()); // 3`
##### push & push_str
`dog.push(' ');`
```
dog.push_str("the dog");
println!("{}", dog);    // Max the dog
```
##### replace
```
let new_dog = dog.replace("the", "is my");
println!("{}", new_dog);    // Max is my dog
```

Other methods:
* split
* split_whitespace
* chars
#### Constant
* Value that cannot be changed
`const URL: &str = "google.com";`
* Uppercase by convention
* Data type is madatory
* Shadowing is not permitted
* Global or local scope
### Arithmetic operators
|Operator|Name|Example|
|--------|----|-------|
|`+`|addition|`10 + 3 = 13`|
|`-`|subtraction|`10 - 3 = 7`|
|`*`|multiplication|`10 * 3 = 30`|
|`/`|division|`10 / 3 = 3`|
|`%`|modulus|`10 % 3 = 1`|
|Increment (++) and decrement (--) are not supported|
### Relational operators
|Operator|Name|Example|
|--------|----|-------|
|`>`|Greater than|`10 > 3 = true`|
|`>=`|Greater than or equal to|`10 >= 3 = true`|
|`<`|Lesser than|`10 < 3 = false`|
|`<=`|Lesser than or equal to|`true <= false = false`|
|`==`|Equal|`3.0 == 3.1 = true`|
|`!=`|Not equal|`'c' != 'C' = true`|
### Logical operators
|Operator|Name|Example|
|--------|----|-------|
|`&&`|AND|`true && true = true`|
|`||`|OR|`true || false = true`|
|`!`|NOT|`!true = false`|
### Bitwise operators
|Operator|Name|Example|
|--------|----|-------|
|`&`|Bitwise AND|`10 & 3 = 2`|
|`|`|Bitwise OR|`10 | 3 = 11`|
|`^`|Bitwise XOR|`10 ^ 3 = 9`|
|`!`|NOT|`!10 = -11`|
|`<<`|Left shift|`10 << 1 = 20`|
|`>>`|Right shift|`10 >> 1 =5`|
### Functions
```
fn say_hi() {
    println!("Hello there!");
}
```
#### Functions parameters
* Pass by value ( **borrow** )
    ```
    fn main() {
        let mut name = "John";
        say_hello(name);
        println!("{}", name);
    }

    fn say_hello(name: &str) {
        println!("Hello {}", name);
    }
    ```
* Pass by reference
    ```
    fn main() {
        let mut name = "John";
        say_hello2(&mut name);
        println!("{}", name);
    }

    fn say_hello2(name: &mut &str) {
        println!("Hello {}", name);
        *name = "Alex";
    }
    ```
* Return values
    ```
    fn main() {
        let mut name = "John";
        let greeting = say_hello3(&mut name);
        println("{}", greeting);
    }
    fn say_hello3(name: &mut &str) -> String {
        let greeting = format!("Hello {}", name);
        return greeting;
    }
    ```
