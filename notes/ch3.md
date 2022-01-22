# Learn Rust

https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html

## Chapter 3

### Variables and mutability

Like immutable variables, constants are values that are bound to a name and are not allowed to change. They are always immutable and the type must be annotated.

Constants can be declared in any scope, including the global scope.

Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between `mut` and shadowing is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name

### Data Types

**Scalar types**

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters

_Integer type_

Length  | Signed | Unsigned |
--------|--------|----------|
8-bit   | i8     | u8       |
16-bit  | i16    | u16      |
32-bit  | i32    | u32      |
64-bit  | i64    | u64      |
128-bit | i128   | u128     |
arch    | isize  | usize    |

Each variant can be either signed or unsigned and has an explicit size. Signed and unsigned refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned).

Integer types default to `i32`.

Rust does not check integer overflow in release mode.

_Floating-Point types_

Rust’s floating-point types are `f32` and `f64`

#### Compound type

_Tuple type_

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value
    let (x, y, z) = tup;
    
    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access
    let five_hundred = x.0;
}
```

_Array type_

Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

```rust
fn main() {
    // Create an array
    let a = [1, 2, 3, 4, 5];

    // You write an array’s type using square brackets with the type of each element, a semicolon,
    // and then the number of elements in the array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // You can also initialize an array to contain the same value for each element by specifying the initial value,\
    // followed by a semicolon, and then the length of the array in square brackets
    let a = [3, 5];

    // Accessing array element
    let first = a[0];
}
```
