# Lists and dicts

Lists are very similar to arrays.
They can contain as many variables as you want.
The lists can also be iterated in a very simple way.
Here is an example.

```rust
fn main(argc: int, argv: list) {
    let mylist = [1, 2, 3, "Hello"];

    for e in mylist {
        println("{}", e);
    }
}
```

You also learn how to use the `for` loop,
but more details will be provided later.

As you probably noticed, the lists can contain any types.
However, if you want to specify a type and avoid
other types in your list, you can do it in a simple way.

```rust
let mylist: List[int] = [1, 2, 3, "Hello"];
```

This code will cause error:

```ruby
error: list item 3 has incompatible type "str"; expected "int"
 --> dist/test.fi:2
  |
  |     let mylist: List[int] = [1, 2, 3, "Hello"];
  |     ^^^
```
