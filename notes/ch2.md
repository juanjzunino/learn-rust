# Learn Rust

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

## Chapter 2

Variables and references are immutable by default

`Result` types are enumerations, often referred to as enums, which can have a fixed set of posibilites known as variants

`let mut guess = String::new();` line has created a mutable variable that is currently bound to a new, empty instance of a String.

Crate is a collection of Rust source code files. The `rand` crate is a library crate, which contains code intended to be used in other programs, and can’t be executed on its own.

Cargo’s coordination of external crates is where Cargo really shines. Before we can write code that uses `rand`, we need to modify the _Cargo.toml_ file to include the `rand` crate as a dependency. Open that file now and add the following line to the bottom beneath the `[dependencies]` section header that Cargo created for you. Be sure to specify `rand` exactly as we have here, with this version number, or the code examples in this tutorial may not work.
