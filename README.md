# FizzBuzz Program by Rust

This program implements the famous FizzBuzz problem by Rust.

**Usage**  
Input the range of numbers,  the number you output `Fizz`, and the number you want to output `Buzz`.

```bash
% cargo run -- -n 10 -f 3 -b 5
---FizzBuzz---
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
--------------
```

If you want to calculate the numbers that did not fire in FizzBuzz, you can do so with the `-c` option.

```bash
% cargo run -n 10 -f 3 -b 5 -c
---FizzBuzz---
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
--------------
No FizzBuzz Number Sum: 22
```

Thank you for watching!  

<br>

## Next challenge
- Caluculates the sum of transformered numbers to `Fizz`, `Buzz` or `FizzBuzz`.
