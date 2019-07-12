# Functions

Function is a sequence of program instructions that performs a specific task.

Syntax for a function:

```rust
fn name(arg1: Type, arg2: Type) -> RetType {
}
```

Example:

```rust
fn myfunc(arg: int) -> int {
    return arg * 2;
}

fn main(argc: int, argv: list) {
    println("{}", myfunc(5));
}
```

The result is 10, the `main` function calls the function` myfunc`,
giving it the number 5, and the function `myfunc` returns
this argument multiplied by 2. The result of the` myfunc`
function is passed to the `println` function, which displays this result.
