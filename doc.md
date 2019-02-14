<!--
	$size:	a4
	page_number: true
	footer: Rust Programming Language
    ![bg](./paper.jpg)
-->

<center>
  
# Rust Programming Language
Around year start of 2019

</center>

![bg original](./autumn.jpg)

<small>

---
# Getting Started

![bg](./autumn.jpg)

---
## Installation
- `rustc --version` to get current version.
- `rustup doc` to open local documentation

![bg](./paper.jpg)

---
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

![bg](./paper.jpg)

---
## Hello, Cargo!
- `cargo new proj-name --vcs` vcs annoted new project 
- `cargo new proj-name --bin` initial git annoted new project 

```toml
proj-name
    |-- .git          // Automatically generated git initial file
    |   |
    |   `-- ...
    |-- src           // source files are supposed to put here
    |   |
    |   `-- main.rs 
    |-- ...
    |
    `-- Cargo.toml    // TOML (Tom's Obvious, Minimal Language)
```

![bg](./paper.jpg)

---
## Cargo! (continue)
- Cargo.toml 
```toml
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

![bg](./paper.jpg)

---
# Programming a Guessing Game
- source code and detail @ ch2-number-guessing

![bg](./autumn.jpg)

---
# Common Programming Concepts

![bg](./autumn.jpg)

---
## Identifiers
1. Start with letter and others are alphanumeric or _
2. Start with `_` and others are alphanumeric, at least 2 charactor
3. raw identifiers, start with `r#`, used when
	1. need to use a name that's a keyword for another purpose
	2. Maybe call a function named match that is coming from a C library

		```rust
		let r#fn = "this variable is named 'fn' even though that's a keyword";
		
		// call a function named 'match';
		r#match();
		```
![bg](./paper.jpg)

---
## Variables and Mutability
1. By default, variable are immutable. You can use `mut` to make them mutable  
2. constant vs inmutable variable  
	1. definition: use `const` for constant, use `let` for variable  
	2. no `mut` for constant
	3. `const` type need to be annotated  
	4. `const` can be declared in any scope, including global  
	5. constants may be set only to a constant expression
	6. ex: `const PI: f64 = 3.1415`  

![bg](./paper.jpg)

---
## Variables and Mutability (cont.)

3. Shadowing  
	1. We can declare new variable with the same name as previous variable  
		1. Rustaceans say that the first variable is shadowed by the second  
	2. Difference between Shadow and `mut`  
		1. Transformation can be made, but variable are still imutable by shadowing.  
		2. Type changes can be performed by shadowing, ex:   
	3. We can change the type of value, and reuse the same name   

		```
		// Following code is ok:
		let spaces = "     ";
		let spaces = spaces.len();
		
		// Following code is invalid:
		let mut mspaces = "     ";
		mspaces = mspaces.len();
		```

![bg](./paper.jpg)

---
## Data Types
1. Rust is static type. --> It must know types of all variables at compile time.
2. Compiler usually can infer the type by the value and usage. 
   But it my fail when many types are possible. Eg:
	```
	// It will fail if the type : u32 is missed:
	let guess: u32 = "42".parse().expect("Need a Number!");
	```

![bg](./paper.jpg)

---
### Scalar Types (`a single value`) ###

1. Integer Type:  (default: `i32`)
	1.1. Rust Integer Type List:
	| Length  | Signed | Unsigned | comment                        |
	|---------|--------|----------|--------------------------------|
	|   8-bit | i8     | u8       | unsigned, do not need `-` sign |
	|  16-bit | i16    | u16      | signed, can use `-` sign       |
	|  32-bit | i32    | u32      |                                |
	|  64-bit | i64    | u64      |                                |
	| 128-bit | i128   | u128     |                                |
	| arch    | isize  | usize    | determinded by OS              |

	1.2. Integer literals:
	| Number literals | Example | Number literals | Example |
	|-----------------|---------|-----------------|---------|
	| Decimal         | 98_222  | Hex             | 0xff    |
	| Binary          | 0b1111  | Byte (u8 only)  | b'A'    |
    
    1.3. suffix allowed (except byte), ex: `12u32`
    
![bg](./paper.jpg)

---
### Scalar Types (cont.) ###
	
2. Overflow  
	2.1. The variable value exceed its range  
	2.2. Halt in debug mode  
	2.3. `two's complement wrapping` will happen in release edition.  

![bg](./paper.jpg)

---
### Scalar Types (cont.) ###

3. Floating-Point Types (default: `f64`)  
	3.1. `f32` --> Single-precision float  
	3.2. `f64` --> Double-precision float  

4. Numeric Operations:  `+ - * / %`  
  
5. Boolean Type (`true` or `false`)  
	5.1 Specified by `bool`  
	5.2 one byte in size  

6. Character Type  
	6.1. `char` type is the language's most primitive alphabetic type.  
	6.2. `char` literal use single quotes, as opposed, string literals use double quotes  
	6.3. Rust's `char` type represents a Unicode Scalar Value.  

![bg](./paper.jpg)

---
### Compound Type ###
> Compound types can group multiple values into one type  

1. Tuple Type  
	1.1. Groups together some values with a variety of type s into one compound type.  
	1.2. Fixed length, cannot grow or shrink once declared.  
	
	```rust
	// Creation: comma-separated list of values inside parentheses
	let tup: (i32, f64, u8) = (500, 6.4, 1);

	let (x, y, z)    = tup;   // Accessing by destructuring 
	let five_hundred = tup.0; // Accessing by name.index (index from 0)
	```

![bg](./paper.jpg)

---

### Compound Type (cont.) ###

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

	2.3. Compiler not produce error when accessing an outbound element, but result in runtime error

![bg](./paper.jpg)

---

### Typecasting
1. No Typecasting by default  
2. Use `as` keyword to typecast to another type, ex `let x = y as i64`  

![bg](./paper.jpg)

---
## Function ##
- example
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

![bg](./paper.jpg)

---
## Function (cont.) ##
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

![bg](./paper.jpg)

---

## Control Flow
- `if` Expression
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

- Repetition with Loops
	1. `loop {}`
	2. `break`
	3. `while condition {}`

![bg](./paper.jpg)

---
# Ownership
- Rust's central and unique feature, enable memory safety w/o garbage collector
- memory managed by ownership system with a rule set checks at compile time.
- Rust has same variable scope rule as others

![bg](./autumn.jpg)

---

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

![bg](./paper.jpg)

---

## Ownership Rules
1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

![bg](./paper.jpg)

---

## The `String` Type
- Only one `String` type in the core language --> string literal or string slice (`&str`)  
- The `String` type is provided by std lib rather than in the core
- `String` is growable, mutable, UTF-8 encoded type.  
- Create a new String:-  String::new()` or `String::from("a_string")`  
- Placed in Heap
- ex: `String` vs string literal
	```rust
	let mut h = String::from("Hello"); // String Type,      mutable, SLOW!
	let     w = "world!";              // String literal, immutable, FAST!
	h.push_str(",");                   // Mutalbe
	println!("{} {}", h, w);
	```

![bg](./paper.jpg)

---
## The `String` Type (cont.)
- `String` is made up by 3 parts
	1. A point to the memory that holds the content
	2. Length: how much memory curently used, in bytes
	3. Capacity: total amount of memory, in bytes

- The group data 1-3 is stored in the Stack
	```markdown
	          h                    Heap Data
	
	| name     | value |        | index | value |
	|----------|-------|        |-------|-------|
	| ptr      |       | -----> |   0   |   H   |
	| length   |   5   |        |   1   |   e   |
	| capacity |   5   |        |   2   |   l   |
	                            |   3   |   l   |
	                            |   4   |   o   |
	```

![bg](./paper.jpg)

---

## Memory and Allocation
- Steps to allocate an amount of memory on the heap:  
	1. Request --> at runtime by `String::from` (similar in every language)  
	2. Return  --> after the variable that owns it goes out of scope  
		- Function `drop` is called  

![bg](./paper.jpg)

---
## Ways Variables and Data Interact

- ==**Move**==
```rust
let x  = 5;
let s1 = String::from("Hello");
{
    let y  = x; // Make a copy of the value in x and bind it to y. 2 copy of 5
    let s2 = s1; // string assignment see fig-move:

    println!("{}, world!", s1); // Error: borrow of moved value
} // Scope end, call `drop`

```
```toml
                           fig-move

 String data is copied, means ptr, length, capacity, not inclue heap data

 ........................
 :          s1          :          Heap Data                   s2         
 : | name     | value | :      | index | value |      | name     | value |
 : |----------|-------| :      |-------|-------|      |----------|-------|
 : | ptr      |       | : ---> |   0   |   H   | <--- | ptr      |       |
 : | length   |   5   | :      |   1   |   e   |      | length   |   5   |
 : | capacity |   5   | :      |   2   |   l   |      | capacity |   5   |
 :......................:      |   3   |   l   |                                
                               |   4   |   o   |                                
 DATA **move**d to s2, s1 no longer valid
```

![bg](./autumn.jpg)

---
## Ways Variables and Data Interact (cont.)

- ==**Clone**==
	```rust
	let s1 = String::from("Hello");
	let s2 = s1.clone;  // use common method `close` if deeply copy is required;
	println!("s1 is '{}', s2 is '{}'", s1, s2);
	```

- ==**Copy**==  
	1. Special annotation -> `Copy`.  
	2. Type with `Copy` trait, old var still valid after assignment  
	3. Some of the types has `Copy` trait:  
		1. Integer types  
		2. `bool` type  
		3. floating point types  
		4. `char` type  
		5. tuples that only contain `Copy` types  

![bg](./paper.jpg)

---
## References and Borrowing 
- Reference allows to refer to value without ownership of it, use `&value_name`
```rust
fn main() {
    let s1 = String::from("Hello");
    let l1 = calculate_length(&s1); //&s1 creates a reference to s1 but not own it
    // let (s2, l2) = calculate_length(s1);

    println!("length of '{}' is {}.", s1, l1);
}
/* // Original function, need to return the string reference back             *
 * fn calculate_length(s: String) -> usize {                                  *
 *     let length = s.len();                                                  *
 *     (s, length)             // must give back ownership to avoid `drop` s  *
 * }                                                                          */
fn calculate_length(s: &String) -> usize { // take reference as input
    s.len()
} // scope ends, nothing happened, as s is not owner
```
```markdown
....................      ........................      .....................
:       &s1        :      :          s1          :      :     Heap Data     :
: | name | value | :      : | name     | value | :      : | index | value | :
: |------|-------| :      : |----------|-------| :      : |-------|-------| :
: |  ptr |       | : ---> : | ptr      |       | : ---> : |   0   |   H   | :
:..................:      : | length   |   5   | :      : |   1   |   e   | :
                          : | capacity |   5   | :      : |   2   |   l   | :
                          :......................:      : |   3   |   l   | : 
                                                        : |   4   |   o   | : 
                                                        :...................:
```

![bg](./paper.jpg)

---
## References and Borrowing (cont.)
- Dereference is the opposite of reference, use `*ref_name`
- Mutable References allow to mutate data through referece, use `&mut var`
	- Only one mutable reference to a particular piece of data in particular scope.
- Data race happens when
	- (cause undefined behavior. difficult to diagnose and fix at runtime)
	1. Two or more pointers access the same data at the same time.
	2. At least one of the pointers is being used to write to the data.
	3. There's no mechanism being used to synchronize access to the data.
- Dangling reference 
	- A pointer that references a location of memory that may have been given to someone else
	- Rust prevent this
- The Rules of References 
	1. You can have either one mutable reference or any number of immutable references
	2. References must always be valid (no dangling reference)

![bg](./paper.jpg)

---
## Lifetime
```rust
let mut book = Vec::new();
book.push(...);             // <-- book **mutable** here

/********* Don't break your friend's toys ***********/
{
    let r = &book;          // <-- book **borrow**ed here
    // book.push(...);      // <-- cannot mutate while shared
    r.len();                // <-- `&` reference
}                           // <-- r goes out of scope; borrow ends
book.push(...);             // <-- now book is **mutable** again

/********* No, it's **my** turn now! *********/
{
    let r = &mut book;      // <-- book **borrow**ed here
    // book.len();          // <-- cannot **access** while borrow
    r.push(...);
}                           // <-- r goes out of scope; borrow ends
book.push(...);             // <-- borrow ended; **accessible** again
```

	| Type   | Ownership       | Alias? | Mutate? |
	|--------|-----------------|:------:|:-------:|
	| T      | Ownered         |        |   Yes   |
	| &T     | Shared  Refence |  Yes   |         |
	| &mut T | Mutable Refence |        |   Yes   |

![bg](./paper.jpg)

---
## The Slice Type
- Slice type does not have ownership
- code example:
```rust
fn first_word(s: &String) -> usize { // return the index of word end.
    let bytes = s.as_bytes(); // convert String to an array of bytes.
    
    for (i, &item) in bytes.iter().enumerate() { // (i, &item) is a tuple struct
        // iter() creates an iterator over the array of bytes
        // enumerate() wrap the result of iter(), return each elements as a tuple
    
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}
```
- General Slice type, like array/tuple
```rust
let a     = [1, 2, 3, 4, 5];
let slice = &a[1..3];        // Slice type is &[i32];
```


![bg](./paper.jpg)

---
## The Slice Type (cont.)
- String Slices 
	- A reference to part of a `String`, look like `let hello = &s[0..5];`
	- `start..end` Range: start to end, not include end
	- `start..=end` Range: start to end
	- start_index can be dropped if it is 0, end_index works similar
	- String slice range indices must occur at valid UTF-8 character boundaries.
	- String literals (`&str`) are Slices
	```rust
	fn main() {
	    let my_string         = String::from("hello world");
	    let my_string_literal = "hello world"; 
        
	    let word = first_word(&my_string[..]);         // Slices of `String`
	    let word = first_word(&my_string_literal[..]); // Slices
	    let word = frist_word(my_string_literal);      // already String Slices
	}
	fn first_word(s: &str) -> &str {
	...
	```

![bg](./paper.jpg)

---
# Using Structs to Sturcture Related Data
- `Struct` is a custom data type that groups related and make up meaningful group
- A `Struct` is lke an object's data attributes.

![bg](./autumn.jpg)

---
## Definining and Instantiating Structs
- define 
	```rust
	struct Sname {
	     sproperty1: Type,
	     ...
	}
	```
- Instantiation
	```rust
	 Explictly Init   | dot notation     | FIS *            | Update Syntax **
	------------------|------------------|------------------|------------------
	 let s1 = Sname { | s1.o_prop = val; | let s2 = Sname { | let s3 = Sname {
         prop1: val1, |                  |     val3,        |    prop1: val4,
	     .......      |                  |     .......      |    ..s2
	 };               |                  | };               | };
     
     *  Field Init Shorthand
     ** Creating Instances From Other Instances With Struct Update Syntax (..)
     ```
![bg](./paper.jpg)

--- 
## Tuple like Struct
- No named field and similar to tuple.
- Useful when you want to give the tuple name, eg `struct Color(i32, i32, i32);`
- use index to access value, eg `x.1`

## Unit-like Struct
- Structs without any field.
- Useful when to impl a trait on some type but no data stored.

## Ownership of Struct Data
- Possible to store references to data owned by others, but need to use *lifetime*

![bg](./paper.jpg)

--- 
## Method Syntax
- Same syntax as function
- defined w/i the context of a struct, an enum, or a trait object.
- 1st parameter is alway `self`, represents the instance of the struct
- code example:
	```rust
	#[derive(Debug)]
	struct Rectangle {
	    width: u32,
	    height: u32,
	}
	impl Rectangle { // multiple `impl` blocks are allowed
	    fn area(&self) -> u32 {      // just need to read data, not ownership.
	        self.width * self.height
	    }
	}
	fn main() {
	    let rect1 = Rectangle { width: 30, height: 50 };
	    println!("The area of the rectangle is {} square pixels.", rect1.area());
	}
	```
- Associated Function
	- Function in `impl` don't take `self` as a parameter.
	- use `StructName::function_name` to access

![bg](./paper.jpg)

--- 
# Enums and Pattern Matching
> To define a type by enumerating its possible values.
> ```rust
> enum EnumName {
>     // enumerate all possible values
> }
> ```

![bg](./autumn.jpg)

---
## Something about Enum
- Use `EnumName::member` to access the variants
- Use with `struct` to bind values together
- Each variant can have different types and amounts of associated data.
- Any data type is allowed for enum variants
- We're able to define methods on enums by `impl` just like `struct`

![bg](./paper.jpg)

---
## `option` Enum
> An `enum` that can encode the concept of a value being present or absent
> ```rust
> enum option<T> {
>     Some(T),
>     None,
> }
> ```
- "_Null_ references: The Bilion Dollar Mistake"
- `option` Enum is defined by the standard library, and included in the prelude.
- `Some` example
	```rust
	let x: i8 = 5;
	let y: Option<i8> = Some(5);
    // let z = x + y; // Option<T> values cannot be used directly (even with <T>).

	let absent_num: Option<i32> = None; //When use None, Option<T> type required
	```

![bg](./paper.jpg)

---
## `option` Enum (cont.)
- When use a value of a type, compiler ensures it has a valid value
- Only when using an `Option<T>` do we have to worry about possibly not having a value
- Convert `Option<T>` to a `T` before performing `T` operations with it.

![bg](./paper.jpg)

---
## Control Flow Operator
- `match`
	```rust
	match an_enum {
	    TheEnum::branch1 => {...},
	    TheEnum::another =>  ... ,  
	    _                =>  ... ,  // _ placeholder for other/default variant
	}
	```
- `if let` one branch `match`
	```rust
	if let A = B {                      match B {
        ...                 ==              A => {...},
	}                                       _ => {},
                                        }
    // example: 
    if let Some(3) = some_u8_value {
        println!("Three");
    }
	```
    
![bg](./paper.jpg)

---
# Package, Crates, and Modules

![bg](./autumn.jpg)

---

