# Loops

Fire has two types of loops:
- **`while` loops**
- **`for` loops**

## While loop

The code block will be repeated until the condition ceases to be true.

```rust
let i = 0;
while i < 5 {
    println("{}", i);
    i += 1;
}
```

With each block execution, we increase `i`. When `i` is equal to 5
(won't be less than 5), the loop will break.

The result is:
```
0
1
2
3
4
```

## For loop

The for loop is used to sequence iterations.

With the for loop, we can execute a set of
instructions for each element of iterated sequence.

For example:

- List:
  ```rust
  let fruits = ["apple", "banana", "cherry"];
  for fruit in fruits {
      println("{}", fruit);
  }
  ```
- String:
  ```rust
  let apple = "apple";
  for c in apple {
      println("{}", c);
  }
  ```

## Ranges

To go through the code set a certain number of times, we can use the range:

```rust
for i in 0..10 {
    println("{}", i);
}
```

This will display numbers from 0 to 9 (not 10!).

If you want to use a variable or any other
expression within the range, you must use parentheses.

```rust
let n = 5;
for i in (1..n+1) {
    println("{}", i);
}
```

This one displays numbers from 1 to 5.

## Breaking loop

Using the break statement,
we can stop the loop,
even if the condition is true.

```rust
let n = 0;
while n < 5 {
    println("{}", n);
    n += 1;

    if n == 3 {
        break;
    }
}
```

Output is:

```
0
1
2
```

## Continue

Using continue, we can stop the current
iteration and continue with the next one:

```rust
let n = 0;
while n < 5 {
    n += 1;
    if n == 3 {
        continue;
    }
    println("{}", n);
}
```

Output is:

```
1
2
4
5
```
