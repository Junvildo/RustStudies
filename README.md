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
### Modules
#### Modules
* Create a module
    ```
    pub mod mod_name {
        pub fn do_something() {
            ...
        }
    }
    ```

    ```
    mod mod_name;

    fn main() {
        mod_name::do_something();
    }
    ```

`pub mod mod_name`
* Nested modules
    
    ```
    pub mod mod_name {
        pub mod sub_mod {
            fn fun_sub_module() {
                ...
            }
        }
    }
    ```

    ```
    mod mod_name;

    fn main() {
        mod_name::sub_mod::fun_sub_module();
    }
    ```
#### Crates
* Multiple modules are grouped into a crate
* Two types:
    * binary crates (have main fn).
    * library crates (doesn't have main fn).
* Cargo is used to manage crates.
* External crates are imported into the project must be added to the toml file.
## Data Types
### Arrays
A collection of values of the same type

`let primes = [2, 3, 5, 7, 11];`

`let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];`

* Static - cannot be resized
* Element values can be modified but not deleted
* Indexed
Create array with default values

`let mut numbers = [0;15];`

```
const DEFAULT: i32 = 3;
let numbers = [DEFAULT;15];
```

Updating elements

`numbers[3] = 5;`

Using an iterator

```
for number in numbers.iter() {
    println!("{}", number);
}
```
### Vectors
Arrays of variable size

`let mut primes: Vec<i32> = Vec::new();`

`let mut primes = vec![2, 3, 5];`

Create vectors with default values

`let mut numbers = vec![2;10];`

```
const DEFAULT: i32 = 6;
let mut numbers = [DEFAULT;8];
```

Updating elements

`number[3] = 5;`

Using an iterator

```
for number in numbers.iter() {
    println!("{}", number);
}
```
### Slices
A slice is a pointer to a block of memory

```
let numbers = [1, 2, 3, 4, 5];
let slice = &numbers[1..4];
```
* Size is determined at runtime
* Can be used on arrays, vectors and strings
* Indexed

Mutable slices allowed us to changed values
### Tuples
A collection of values of various types

`let person = ("John", 27, true);`

`let person: (&str, i32, bool) = ("John", 27, true);`
* Static - cannot be resized
* Element values can be updated
* Indexed
* Limited to 12 elements

Accessing elements

`println!("Name: {}", person.0);`

Updating elements

`person.0 = "Jack";`

Destructuring a tuple (number of variables must correspond to number of elements)

`let (name, age, employed) = person;`
### Structures
A collection of key-value pairs

```
struct Employee {
    name:String,
    company:String,
    age:u32
}
```

```
let emp1 = Employee {
    name:String::from("John"),
    company:String::from("Google"),
    age:35
};
```

`println!("{}", emp1.name);`

Adding methods to a structures

```
impl Employee {
    fn fn_detail(&self) -> String {
        ...
    }
}
```

A structure can have static methods

```
impl Employee {
    fn static_fn_detail() -> String {
        ...
    }
}
```
### Enums
A collection of values

```
enum Color {
    Red,
    Green,
    Blue
}
```

`let my_color = Color::Red;`

`let my_color = Red;`

We can add data types ti enum elements

```
enum Person{
    Name(String),
    Surname(String),
    Age(u32)
}
```

`let person = Name(String::from("Alex"));`
### Generics
Allow us to have variable data types

```
struct Point<T> {
    x:T,
    y:T
}
```

```
let p1: Point<i32> = Point {x : 6, y : 8};
let p2: Point<f64> = Point {x : 3.25, y : 8.43};
```

```
struct Point<T,V> {
    x:T,
    y:V
}
```

`let p3: Point<i32, f64> = Point{x:5,y:5.34};`
## Control Structures
### If statement
```
if logical_expression {
    functionality for true
}
```

```
if logical_expression {
    functionality for true
} else {
        functionality for false
}
```

```
if expression1 {
    functionality for expression1 true
} else if expression2 {
    functionality for expression2 true
} else {
    functionality for both expression false
}
```

If statement can return a result

```
let res = if expr1 {
    result for true
} else {
    result for false
}
```
### Match statement
Similar to when or switch in other languages

```
match expression {
    expr1 => {...}
    expr2 => {...}
    _ => {...}
}
```

Can return a reuslt

Ranges are allowed
### Pattern Matching
Multiple values 1|2

Ranges 1..=5

Conditions x if a > b

Tuple matching
### For loop
Loop through a collection or range, execute code for each element

```
for element in collection {
    functionality
}
```

Continue will skip a step

Break will stop the loop
### While loop
Loop as long as a condition is true

```
while condition {
    ...
}
```

Continue will skip a step

Break will stop the loop

```
loop {              //while true {
    ...
}
```
## Functions
### Functions and Scope
#### Functions
```
fn main() {
    say_hi();
}

fn say_hi() {
println!("Hello there!!!");
}
```

```
fn main() {
    say_hi("John");
}

fn say_hi(name: &str) {
    println!("Hello {}!", name);
}
```

```
fn main() {
    let mut name = "John";
    say_hi(&mut name);
    println!("The new name is {}", name);
}

fn say_hi(name: &mut &str) {
    *name = "Alex";
    println!("Hello {}!", name);
}
```

```
fn main() {
    let mut name = "John";
    let greeting = say_hello(&mut name);
    println!("{}", greeting);
}

fn say_hello(name: &mut &str) -> String {
    let greeting = format!("Hello {}", name);
    greeting
}
```

#### Scope
No memory leaks - no need to manually deallocate variables

```
{
    let a = 3;
}
println!("a = {}", a);  //error
```

Global variables can be declared but they are unsafe

`let a = 3;`

```
fn main() {
    unsafe {println!("{}", a);}
}
```
### Closures
A function within a function

An anonymous function, lambda expression

```
|a: i32, b: i32| println!("{}", a + b);
|a: i32, b: i32| -> i32 {a + b};
```

A function can be assigned to a variable

```
let sum = |a: i32, b: i32| -> i32 { a + b };
sum(2, 3);
```

A closure can be generic

```
let gen = |x| {println!("received {}", x)};
gen(3);
```
### Higher Order Functions
Functions that take another function as a parameter

```
fn apply(f: fn(i32) -> i32, a: i32) {

}
```

`apply(|x| -> x + 1, a);`
### Macros
Write code that writes code - meta programming

Match an expression and perform some operation

Code is expanded and compiled

```
macro_rules! my_macro {
    (match) => ( code to run )
}
```

`my_macro!`

`println!("This is an {} macro", "awesome");`

We can match multiple expressors

```
macro_rules! my_macro {
    (match1) => (code to run)
    (match2) => (code to run)
}
```

Designators
* expr
* ident
* block
* stmt
* pat
* path
* meta
* ty
* tt
## Traits
### Traits
Similar to an interface or abstract class

Add a definition to a structure

```
trait Name {
    fn must_implement(&self) -> i32;
    fn do_action(&self) {...}
    fn do_non_instance_action() {...}
}
```

Can have definition only or default implementation

Can have instance and non-instance action

Implement a trait

```
impl Name for Person {
    fn must_implement(&self) -> {42}
    fn new(name: &str) -> Person {
        Person{name: name}
    }
}
```

Can provide a constructor

```
trait Name {
    fn New(name: &str) -> Self;
}
```

`let john = Person::new("John");`

We can add a trait to a structure we didn't create

```
impl My_Trait for Vec<i32> {
    ...
}
```
### Generics
Generics can be limited by traits

```
fn color<T: Colorable> (a:T) {
    ...
}
```
### dynu (returning traits)

The compiler needs to know the space required for a function return type

A workaround is to return a box with a dyn trait.

```
fn get_animal() -> Box<dyn Animal> {
    ...
}
```

dyn is a new addition to the language, old code might not have it
### Operator overloading
We can implement standard operators for our custom structs

`use std::ops::Add;`

```
struct Custom {
    ...
}
```

```
impl Add for Custom {
    type Output = Custom;
    fn add(self: Custom, rhs: Custom) -> Custom {
        ...
    }
}
```

`custom1 + custom2`
### Static dispatch (Monomorphization - converting to one form)
A generic trait will be converted to the required type at compile time
### Dynamic dispatch
A generic trait will be converted to the required type at run time
## Memory management
### Ownership
Only one variable can own a piece of memory

For primitive types, copying data is cheap

For complex types, ownership is transferred
### Borrowing
Only one variable can own a piece of memory

Variables can borrow ownership to other pieces of memory

```
let a = 6;
let b = &a;
```

`println!("{}", *b);`

`a += 2;    //error`

The borrow has to match the mutability
### Lifetimes
An indication of how long an object will live

Rust prevents parts of objects outliving the object

```
struct Object<'lifetime> {
    field: &'lifetime str
}
```

Lifetime elision - compiler builds lifetimes for us when evident
### Reference counted variables
A structure that can hold multiple references to a variable

Can be shared in different places

```
use std::rc::Rc;
fn do_smth(var: Rc<String>) ...
```

`let var = Rc::new(String::from("test"));`

`var.clone()`

Count the variable pointers

`Rc::strong_count(&var)`
