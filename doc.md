# Getting Started

## Installation
- `rustc --version` to get current version.
- `rustup doc` to open local documentation
- 

## Hello World!
- `rustc main.rs` to compile
- code
```rust
// Line  comments --> end of line
/* Block comments --> closing delimiter --> */

/// <-- Doc comment --> Generate library docs for the following item.
//! <-- Doc comment --> Generate library docs for the enclosing item.
/* Doc comments are useful for big projects that require documentation.
 * run rustdoc to generate the doc file
 */

// an automatic formatter tool called `rustfmt` 

fn main() { // main is special function will always be firstly called
    /* 
     * 1. rust style is to indent with 4 spaces, not 1 tab
     * 2. ! indicates a macro call, no ! is a normal function call 
     * 3. line ends with a semicolon (;)
     */

    println!("Hello world!"); 
}
```

## Hello, Cargo!
- `cargo new proj-name --vcs` vcs annoted new project 
- `cargo new proj-name --bin` initial git annoted new project 
```
proj-name
    |-- .git          // Automatically generated git initial file
    |   |
    |   `-- ...
    |-- src           // source files are supposed to put here
    |   |
    |   `-- main.rs 
    |-- ...
    |
    `-- Cargo.toml    // The cargo config file. TOML (Tom's Obvious, Minimal Language)
```

- Cargo.toml 
```
[package]             // section to address the information of package
name = "proj-name"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]        // the crates to use
```

- `cargo build` to compile and build the program
- `cargo run`   to compile and execute, no program file built
- `cargo check` syntax check, fast but not execute
- `cargo build --release` to compile with optimizations

# Programming a Guessing Game
- source code and detail @ ch2-number-guessing

# Common Programming Concepts

## Identifiers
1. Start with letter and others are alphanumeric or _
2. Start with `_` and others are alphanumeric, at least 2 charactor
3. raw identifiers, start with `r#`, used when
3.1. need to use a name that's a keyword for another purpose
3.2. Maybe call a function named match that is coming from a C library, where match is not a keyword
```rust
let r#fn = "this variable is named 'fn' even though that's a keyword";

// call a function named 'match'
r#match();
```

## Variables and Mutability
1. By default, variable are immutable. You can use `mut` to make them mutable  
2. constant vs inmutable variable  
2.1. definition: use `const` -> constant, use `let` for variable  
2.2. `const` type need to be annotated  
2.3. `const` can be assigned in any scope, including global  
2.4. ex: `const PI: f64 = 3.1415`  
3. Shadowing  
3.1. declare new variable with same name as previous, the newer shadows previous  
3.2. Difference between Shadow and `mut`  
3.2.1. Transformation can be made, but variable are still imutable by shadowing.  
3.2.2. Type changes can be performed by shadowing, ex:   
```
// Following code is ok:
let spaces = "     ";
let spaces = spaces.len();

// Following code is invalid:
let mut mspaces = "     ";
mspaces = mspaces.len();
```

## Data Types
1. Rust is static type. --> It must know types of all variables at compile time.
2. Compiler usually can infer the type by the value and usage.
   But it my fail when many types are possible. Ex:
```
// It will fail if the type : u32 is missed:
let guess: u32 = "42".parse().expect("Need a Number!");
```

### Scalar Types ###
> Represents a single value.

1. Integer Type
> default type is `i32`

1.1. Integer Types in Rust:

| Length  | Signed | Unsigned |
|---------|--------|----------|
|   8-bit | i8     | u8       |
|  16-bit | i16    | u16      |
|  32-bit | i32    | u32      |
|  64-bit | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

>   signed --> can use '-' sign  
> unsigned --> DO NOT need sign  
> arch     --> determinded by os, 64-bit or 32-bit  

1.2. Range
>   signed --> -(2^(n-1)) ~> 2^(n-1) - 1  
> unsigned --> 0 ~> 2^n - 1  

1.3. Integer literal

| Number literals | Example |
|-----------------|---------|
| Decimal         | 98_222  |
| Hex             | 0xff    |
| Binary          | 0b1111  |
| Byte (u8 only)  | b'A'    |

> suffix allowed (except byte), ex: `12u32`

1.4. Overflow  
1.4.1. The variable value exceed its boundary  
1.4.2. It will halt in debug mode  
1.4.3. two's complement wrapping will happen in release edition.  

2. Floating-Point Types  
2.1. `f32` --> Single-precision float  
2.2. `f64` --> Double-precision float  
2.2. `f64` is default  

3. Numeric Operations  
> `+ - * / %`  
  
4. Boolean Type  
> Specified by `bool`, has 2 possible value `true` or `false`  
> one byte in size  

5. Character Type  
5.1. `char` type is the language's most primitive alphabetic type.  
5.2. `char` literal use single quotes, as opposed, string literals use double quotes  
5.3. Rust's `char` type represents a Unicode Scalar Value.  
  

### Compound Type ###
> Compound types can group multiple values into one type  

1. Tuple Type  
1.1. A tuple groups together some values with a variety of type s into one compound type.  
1.2. Fixed length, cannot grow or shrink once declared.  
```rust
// Creation: comma-separated list of values inside parentheses
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z)    = tup;   // Accessing by destructuring 
let five_hundred = tup.0; // Accessing by name.index (index from 0)
```

2. Array Type  
2.1. Every item in same type  
2.2. Fixed length  
```rust
// Creation: comma-separated list of values inside square brackets
let a = [1, 2, 3, 4, 5];
let five = a[4]; // Accessing

// let a: [i32: 5] = [1, 2, 3, 4, 5]; // when type needed

// my: Only might usable array:
let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep",
              "Oct", "Nov", "Dec"];
```

2.3 Compile not produce error when accessing an outbound element, but result in runtime error;  


## Function
- ex: 
```rust
fn main () {
    println!("Hello world!");
    another_function();        // call by name and set of parentheses
    other_function(1, 5);      // provided values are called arguments
}

// use snake_case convention
fn another_function () {
    println!("Another Function");
}

// function can have parameters
// types must be declared
fn other_function (x: i32, y: i32) {
    println!("x is {}", x);
    println!("y is {}", y);
}

```

- Return a value
```rust
fn main () {
    let x = 5; // this is a statement, it has no return value
    // let x = y = 5; // this is invalid in Rust

    let y = add5(x);
}

fn add5(a: i32) -> i32 { // Function return value:  -> type
    // a + 5 ;  // a statement, has no return value, compile error with this
    a + 5       // an expression, it return its value
}
```


## Control Flow
### `if` Expression
```rust
// each are called **arm** in Rust
if condition      { } 
else if condition { }
else              { }

// if can be used in let, type must be compatible 
let x = if condition {
        } else {
        };
```

### Repetition with Loops
- `loop {}`
- `break`
- `while condition {}`



## Ownership
- Rust's central and unique feature, enable memory safety w/o garbage collector
- memory managed by ownership system with a rule set checks at compile time.
- Rust has same variable scope rule as others

## The Stack and the Heap
- Stack and Heap are parts of memory available at runtime

### Stack
- Last In, First Out: Stores values in the order it gets and removes in opposite order
- Fast!!! 
1. No need to search places to put data, alway on top
2. Stores only known, fixed-sized data
- term: Add    data --> pushing onto the stack
- term: Remove data --> popping off  the stack

### Heap
- Store data with unknown size at compiler time, or changable size
- term: allocating (on the heap)  
  DATA -> require memory space -> OS to find a suitable slot -> return an adress point  
- SLOW!!! and less organized
1. Need to follow point to access real data


## Ownership Rules
1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.


## The `String` Type
- Placed in Heap
- ex: vs string literal
```rust
let mut h = String::from("Hello"); // String Type,                 mutable, SLOW!                  
let     w = "world!";              // String literal, hardcoded, immutable, FAST!
h.push_str(",");                   // Mutalbe
println!("{} {}", h, w);
```

- `String` is made up by 3 parts
1. A point to the memory that holds the content
2. Length: how much memory curently used, in bytes
3. Capacity: total amount of memory, in bytes
- The group data 1-3 is stored in the Stack

```
          h                    Heap Data

| name     | value |        | index | value |
|----------|-------|        |-------|-------|
| ptr      |       | -----> |   0   |   H   |
| length   |   5   |        |   1   |   e   | 
| capacity |   5   |        |   2   |   l   |
                            |   3   |   l   |
                            |   4   |   o   |
```

## Memory and Allocation
- Steps to allocate an amount of memory on the heap:  
1. Request --> at runtime by `String::from` (similar in every language)  
2. Return  --> after the variable that owns it goes out of scope  
2.1. Function `drop` is called  


## Ways Variables and Data Interact
- **Move**
```rust
let x  = 5;
let s1 = String::from("Hello");
{
    let y  = x; // Make a copy of the value in x and bind it to y. 2 copy of 5
    
    let s2 = s1; // string assignment see fig-move:

    println!("{}, world!", s1); // Error: borrow of moved value
} // Scope end, call `drop`

/*                            fig-move
 *
 * String data is copied, means ptr, length, capacity, not inclue heap data
 *
 * ........................
 * :          s1          :           Heap Data                      s2         
 * : | name     | value | :       | index | value |         | name     | value |
 * : |----------|-------| :       |-------|-------|         |----------|-------|
 * : | ptr      |       | : ----> |   0   |   H   | <------ | ptr      |       |
 * : | length   |   5   | :       |   1   |   e   |         | length   |   5   |
 * : | capacity |   5   | :       |   2   |   l   |         | capacity |   5   |
 * :......................:       |   3   |   l   |                                
 *                                |   4   |   o   |                                
 * DATA **move**d to s2, s1 no longer valid
 */
```

- **Clone**
```rust
let s1 = String::from("Hello");
let s2 = s1.clone;  // common method `close` is used if deeply copy is required;
println!("s1 is '{}', s2 is '{}'", s1, s2);
```

- Stack-only Data: **Copy**  
1. Special annotation -> `Copy`.  
2. Type with `Copy` trait, old var still valid after assignment  
3. Some of the types has `Copy` trait:  
3.1. Integer types  
3.2. `bool` type  
3.3. floating point types  
3.4. `char` type  
3.5. tuples that only contain `Copy` types  


## References and Borrowing 
### Reference allows to refer to value without ownership of it, use `&value_name`
```rust
fn main() {
    let s1 = String::from("Hello");
    let l1 = calculate_length(&s1); // &s1 creates a reference to s1 but now own it

    // let (s2, l2) = calculate_length(s1);

    println!("length of '{}' is {}.", s1, l1);
}

/* Original function, need to return the string reference back
 * 
 * fn calculate_length(s: String) -> usize {
 *     let length = s.len();
 *     (s, length)             // must give back ownership to avoid `drop` s
 * }
 */

fn calculate_length(s: &String) -> usize { // take reference as input
    s.len()
} // scope ends, nothing happened, as s is not owner

/*
 * ....................      ........................      .....................
 * :       &s1        :      :          s1          :      :     Heap Data     :
 * : | name | value | :      : | name     | value | :      : | index | value | :
 * : |------|-------| :      : |----------|-------| :      : |-------|-------| :
 * : |  ptr |       | : ---> : | ptr      |       | : ---> : |   0   |   H   | :
 * :..................:      : | length   |   5   | :      : |   1   |   e   | :
 *                           : | capacity |   5   | :      : |   2   |   l   | :
 *                           :......................:      : |   3   |   l   | : 
 *                                                         : |   4   |   o   | : 
 *                                                         :...................:
 */
```

### Dereference is the opposite of reference, use `*ref_name`
### Mutable References allow to mutate data through referece, use `&mut var`
- Only one mutable reference to a particular piece of data in particular scope.
### Data race happens when
- (cause undefined behavior. difficult to diagnose and fix at runtime)
1. Two or more pointers access the same data at the same time.
2. At least one of the pointers is being used to write to the data.
3. There's no mechanism being used to synchronize access to the data.
### Dangling reference 
- A pointer that references a location of memory that may have been given to someone else
- Rust prevent this
### The Rules of References 
1. You can have either one mutable reference or any number of immutable references
2. References must always be valid (no dangling reference)

### Lifetime
```rust
let mut book = Vec::new();
book.push(...);             // <-- book **mutable** here

/********************************** 
 * Don't break your friend's toys * 
 **********************************/
{
    let r = &book;          // <-- book **borrow**ed here
    // book.push(...);      // <-- cannot mutate while shared
    r.len();                // <-- `&` reference
}                           // <-- r goes out of scope; borrow ends
book.push(...);             // <-- now book is **mutable** again


/*****************************
 * No, it's **my** turn now! *
 *****************************/
{
    let r = &mut book;      // <-- book **borrow**ed here
    // book.len();          // <-- cannot **access** while borrow
    r.push(...);
}                           // <-- r goes out of scope; borrow ends
book.push(...);             // <-- borrow ended; **accessible** again

/* 
 *
 * | Type   | Ownership       | Alias? | Mutate? |
 * |--------|-----------------|--------|---------|
 * | T      | Ownered         |        |   Yes   |
 * | &T     | Shared  Refence |  Yes   |         |
 * | &mut T | Mutable Refence |        |   Yes   |
 *
 */
```



