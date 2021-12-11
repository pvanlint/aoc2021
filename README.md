# aoc2021

These are my solutions to https://adventofcode.com/2021 problems.

I decided to use this years competition to learn a new language. I decided on Rust.

The solutions to earlier problems are probably naive, and I often found myself tweaking various combinations of * and & to fix borrowing issues, before taking the time to read more of the documentation. :)

I used the termion library when I wanted text formating in my console output, so I compiled and ran each program using something like this:

```
$ rustc -L ~/.cargo/registry/src/github.com-1ecc6299db9ec823/termion-1.5.6/target/debug/deps aoc91.rs
$ ./aoc9a aoc9.input
```
