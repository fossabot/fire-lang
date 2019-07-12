# Conditions

An "if statement" is written by using the `if` keyword:

```rust
let a = 50;
let b = 60;
if b > a {
    println("b is greater than a");
}
```

Remember that you can't skip `{` and `}` like in C:

```c
if (a>b)
    puts("a is greater than b");
```

Omitting them will result:

```ruby
error: compiler error
 --> example.fi:4
  |
  |     if b > a
  |     ^^^ invalid syntax
  |
```

The else keyword catches anything that hasn't been caught by the previous conditions.

```rust
if b > a {
    println("b is greater than a");
} else {
    println("a is equal to or greater than b");
}
```

You can combine else and if:

```rust
if b > a {
    println("b is greater than a");
} else if a == b {
    println("a is equal to b");
}
```
