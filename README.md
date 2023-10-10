# Rust tutorials

Goals: Safety, productivity and Control 
Features: Performance, Concurrency and Memory efficiency

Rust developer: Rustacean
Motto: Fearless concurrency

## Development Environment setup
* Rustup: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
* Vim plugin: git clone https://github.com/rust-lang/rust.vim ~/.vim/pack/plugins/start/rust.vim

## Tool chain
* rustup => Manages rust installation
* rustc => Rust compiler
* rustfmt => Code formatter
* Clippy =>
* cargo => Package manager and Project builder.
```sh
    # Creating new project with cargo: 
    $ cargo new project-name

    # Building Project
    $ cargo build
    $ cargo check  # Compile project without generating executable

    # Running Project
    $ cargo run
```

## Variables and Mutability
Variables are immutable by default. Declared using the `let` keyword.
Type of a variable can be inferred without explicit annotation.
A variable can be mutable by adding the `mut` keyword in the variable declaration
or initialisation.

```rust
fn main()
{
    // let IMMUTABLE_VARIABLE: [TYPE] [= VALUE];
    let var_name: &str = "string variable";

    //let mut MUTABLE_VARIABLE: [TYPE] [= VALUE];
    let mut my_age: u8 = 30;
    my_age = my_age + 1;
}
```

### Constants
Constants are always immutable and can be declared in any scope.
Declared using the `const` keyword and must be annotated. Value must
be assigned not computed at runtime and are valid in scope within the
entire program running time.

```rust
fn main()
{
    // const CONSTANT_NAME: TYPE = VALUE;
    const CONST_NAME: u128 = 10_000_000e12;
}
```

### Shadowing
Redeclaring a variable using previously declared variable name.
Shadowing is different from marking a variable as `mut` (mutable).
It allows the type of a variable to be changed. 

```rust
fn main()
{
    // let some_variable: [Type] [= value];
    let some_variable: &str = "    ";
    let some_variable: u32 = some_variable.len();
}
```

## Data Types
Rust is statically typed language but sometimes it can infer the type of a variable
at compile time. Supports scalar types and compound types.

### Scalar Types
* Integers: Signed(i8,i16,i32,i64,i128), Unsigned(u8,u16,u32,u64,u128)
    and Arch(isize, usize)
    Integer literals can be written in binary(0b), hexadecimal(0x), octal(0x) and in
    byte(b'b' for u8 only). Values can be separated with an underscore(_) for clarity.

* Floating-point Numbers: f32 and f64
* Booleans: Represents true or false values
* Characters: Literals put in single quotes. Represents Unicode value. Size = 4 bytes.

### Compound Types
Compound types can group multiple values into one type.
Rust has two primitive compound types: tuples and arrays.

#### Tuple Type
Groups variety values with variety of types into one compound type with a fixed length. Each value in a tuple has an associated type. Indexing starts from zero (0).
Individual values can be obtained using pattern matching (destructuring) or using the index.
A _unit_ is a special tuple without a value and type. Expressions implicitly return a unit.

```rust
fn main(){
    // Declaring and Assigning a tuple
    let tup: (&str, u8, f64) = ("ASCII", 254, 82.8273);

    // Destructuring
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    //      OR
    // Using the period (.) followed by the index
    println!("The value at location.2 is: {tup.2}");
}
```

#### The Array Type
An _array_ handles multiple values but unlike a tuple, all the values must be of the same type with a fixed length. Arrays are stored on the stack.

```rust
fn main() {
    // let array_name: [type; size] = [init_values; element]
    let months: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                              "Jul", "Aug", "Sept", "Oct", "Nov", "Dec"];

    let five_ones: u8 = [1; 5]; // Initialise the array with five ones
}
```

## Functions
The most important function in a rust program is the `main` function. It is the entry point for rust program. The `fn` keyword is used to declare a new function.
Example: `fn func_name([args]) {// function body}`.
Functions in rust can be defined before or after the main function.

```rust
fn main()
{
    // Calling function
    say_hello();
}

// Function definition
fn say_hello()
{
    println!("Hello");
}
```

### Function Parameters
Parameters are special function variables that are part of a function's signature.
Function signature must declare the type of each parameter _(Annotate parameter)_.
Multiple parameters are separated by commas.
Concrete values given to a function call are called arguments.

```rust
fn say_hello(name: &str)
{
    println!("Hello, {}!", name);
}

// main function
fn main()
{
    let name = "Taufiq Gh";
    say_hello(name); // Calling function with arguments
}
```

### Return Values
Function can return a value at the end of execution or early by using return keyword.
The return type must be declared with `(-> type_name)`.
Last expression result is returned implicitly.
```rust
fn func(x: u32) -> u32 {
    if x > 0 {
        return x
    }
    x * 1
} 

fn main() {
    let x = func(0);
    println!("func(0) return {:?}",x);
}
```
