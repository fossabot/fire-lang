<img src="https://i.imgur.com/SSYGMnA.png"/>

Fire is simple, fast, statically-typed programming language.

## Faster than C
```rust
fn fib(i: int) -> int {
    if i < 2 {
        return 1;
    }
    return fib(i-1) + fib(i-2);
}

fn main(argc: i64, argv: list) {
    for i in 0..45 {
        print("{} ", fib(i));
    }

    println("");
}
```
the same code in C:
```c
#include <stdio.h>

int fib(int i) {
    if (i < 2) {
        return 1;
    }
    return fib(i-1) + fib(i-2);
}

int main(void) {
    for (int i = 0; i < 45; i++) {
        printf("%d ", fib(i));
    }

    printf("\n");
}
```
compile the code with the following command and run:
```
$ gcc test.c -o test -Ofast -march=native
$ time ./test
1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 6765 10946 17711 28657 46368 75025 121393 196418 317811 514229 832040 1346269 2178309 3524578 5702887 9227465 14930352 24157817 39088169 63245986 102334155 165580141 267914296 433494437 701408733 1134903170

real    0m2.726s
user    0m2.719s
sys     0m0.000s
```
and now fire:
```
$ python3 fire test.fi
$ time ./test
1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 6765 10946 17711 28657 46368 75025 121393 196418 317811 514229 832040 1346269 2178309 3524578 5702887 9227465 14930352 24157817 39088169 63245986 102334155 165580141 267914296 433494437 701408733 1134903170

real    0m0.053s
user    0m0.031s
sys     0m0.016s
```
