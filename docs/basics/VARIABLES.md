# Variables

Fire is statically-typed. You have to declare variables before using them and declare it's type (if it's not obvious).

The simplest types in Fire:

| **Type** | **Example values**       |
|----------|--------------------------|
| `int`    | `123`, `1000`, `-344`    |
| `float`  | `3.14`, `82.543`, `4.0`  |
| `str`    | `"Hello, world!"`        |

To declare a variable, write `let` and its name.
You can assign a value to it, using the `=` operator.

```rust
fn main(argc: int, argv: list) {
    let myint = 120;
    let myfloat = 3.14;
    let mystr = "Hello, world!";
    println("My int: {}", myint);
    println("My float: {}", myfloat);
    println("My str: {}", mystr);
}
```

Simple operators can be performed on numbers and strings:

```rust
fn main(argc: int, argv: list) {
    let one = 1;
    let two = 2;
    let three = one + two;
    println("{} + {} = {}", one, two, three);
    
    let hello = "hello"
    let world = "world"
    let helloworld = hello + " " + world
    println("{}", helloworld);
}
```

Mixing operators between different types causes an error:

```rust
fn main(argc: int, argv: list) {
    let a = 120;
    let b = "Hello";
    println("{}", a + b);
}
```

```ruby
error: unsupported operands
 --> dist/test.fi:4
  |
  |     println("{}", a + b);
  |     ^^^ unsupported operand type(s) for +: 'int' and 'str'
  |
  = note: Function __add__ on int expects int
```
