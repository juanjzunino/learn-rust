
# Learn Rust

## Chapter 3

### Variables and mutability

Like immutable variables, constants are values that are bound to a name and are not allowed to change. They are always immutable and the type must be annotated.

Constants can be declared in any scope, including the global scope.

Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between `mut` and shadowing is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name

### Data Types

