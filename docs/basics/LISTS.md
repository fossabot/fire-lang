# Lists and Dicts

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
 --> example.fi:2
  |
  |     let mylist: List[int] = [1, 2, 3, "Hello"];
  |     ^^^
```

## Dictionaries

A dictionary is a data type similar to arrays,
but works with keys and values instead of indexes.
Access to any value stored in the dictionary
can be obtained using a key that is any type of object
(string, number, list, etc.) instead of
using its index for addressing.

For example:

```rust
fn main(argc: int, argv: list) {
    let mydict = dict({
        "a": 100,
        "b": "Hello",
    });

    mydict["c"] = 200;

    println("{}", mydict["a"]);
    println("{}", mydict["b"]);
    println("{}", mydict["c"]);
}
```
displays:
```
100
Hello
200
```

As you probably noticed (just like with lists),
you can assign any types to dict.
But if you want, you can specify a type yourself.

```rust
fn main(argc: int, argv: list) {
    let mydict: Dict[str, int] = dict({
        "a": 100,
        "b": "Hello",
    });

    mydict["c"] = 200;

    println("{}", mydict["a"]);
    println("{}", mydict["b"]);
    println("{}", mydict["c"]);
}
```
causes:
```ruby
error: dict entry 1 has incompatible type "str": "str"; expected "str": "int"
 --> example.fi:2
  |
  |     let mydict: Dict[str, int] = dict({
  |     ^^^
```

It is worth remembering that to declare a dict,
don't just write `{}`. You must write `dict ()`
for empty dict or `dict ({...})` as shown above.
