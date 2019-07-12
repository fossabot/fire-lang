# Structs

Struct is a complex data type, grouping data of different types in one area of memory.
The structure components - fields - are labeled, i.e. they have their unique names;
by giving the name you get access to the field.

Declaring the struct:

```rust
struct Person {
    age: int,
    name: str
}
```

Excellently! Now you can use this struct in your program.

```rust
fn main(argc: int, argv: list) {
    let p = Person();
    p.age = 18;
    p.name = "John";
    println("{}", p);
    println("{}'s age is {}", p.name, p.age);
}
```

Let me explain.

```rust
let p = Person();
```

You create a struct as you create other variables.
Instead of assigning type 5 or "Hello" values, you assign a struct object.
You can write the same `int()` to create an int with a default value (0).

Next, you assign values to the struct fields:

```rust
p.age = 18;
p.name = "John";
```

You display the entire structure and specific fields:
```rust
println("{}", p);
println("{}'s age is {}", p.name, p.age);
```
Fire has a nice option to display the entire structure.

The result of the entire program is:

```
Person { age = 18, name = 'John' }
John's age is 18
```
