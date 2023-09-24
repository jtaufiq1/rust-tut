# Rust tutorials

## Development Environment setup
* Rustup: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
* Vim plugin: git clone https://github.com/rust-lang/rust.vim ~/.vim/pack/plugins/start/rust.vim

## Tool chain
* rustc => Rust compiler:
* rustfmt => Code formatter
* cargo => Package manager and Project builder.

* Creating new project with cargo: 
    ```sh
    $ cargo new project-name
    ```

* Building Project
    ```sh
    $ cargo build
    $ cargo check  # Compile project without generating executable
    ```

* Running Project
    ```sh
    $ cargo run
    ```

## Rust Concepts

### Variables and Mutability
Variables are immutable by default. Declared using the `let` keyword.
Type of a variable can be inferred without explicit annotation.
A variable can be mutable by adding the `mut` keyword in the variable declaration
or initialisation.

```rust
// let IMMUTABLE_VARIABLE: [TYPE] [= VALUE];
let var_name: &str = "string variable";

//let mut MUTABLE_VARIABLE: [TYPE] [= VALUE];
let mut my_age: u8 = 30;
my_age = my_age + 1;
```

#### Constants
Constants are always immutable and can be declared in any scope.
Declared using the `const` keyword and must be annotated. Value must
be assigned not computed at runtime and are valid in scope within the
entire program running time.

```rust
// const CONSTANT_NAME: TYPE = VALUE;
const CONST_NAME: u128 = 10_000_000e12;
```

#### Shadowing
Redeclaring a variable using previously declared variable name.
Shadowing is different from marking a variable as `mut` (mutable).
It allows the type of a variable to be changed. 

```rust
// let some_variable: [Type] [= value];
let some_variable: &str = "    ";
let some_variable: u32 = some_variable.len();
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
