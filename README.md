# Solutions to Advent of Code 2024

## How to run the solutions

To run my solutions to AoC 2023 you need the Rust compiler. Since I decided not
to use any crate and stick with the standard libraries, that would be enough.
But I actually used `cargo` every day of the challenge to get used to the tool.
With it you can build, run and test each solution.
Check the
[Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)
section in the aforementioned book.

For example, if you want to run the solution to day one, clone this repo,
and navigate to the `day-01` folder:

```
git clone https://www.github.com/santisoler/adventofocode-2024
cd adventofcode-2024
cd day-01
```

In there you can use `cargo` to test the code (if there are tests available):

```
cargo test
```

Or run the code to obtain solutions for both days:

```
cargo run
```

## License

Copyright © 2024 Santiago Soler

Source code available through the [MIT License](LICENSE).
