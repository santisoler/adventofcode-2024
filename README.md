# Solutions to Advent of Code 2024

## About

These are my solutions to [Advent of Code 2024][aoc].
I wrote them all in Rust 🦀, using [Neovim][neovim] as my text editor on
a [MacBookPro5,5 (Mid-2009)][macbookpro] running [Debian] 12 (XFCE).

I take AoC as an opportunity to learn more about languages I don't get to use
so often. This year I chose Rust again, for third year in a row.
No AI tools were used while solving AoC.
I did read some of the solutions in the AoC Reddit to get inspiration and learn
when I felt stuck.

I think this repo is a way of showing that we can still have a modern
development environment (Neovim + LSP + autocompletion + Telescope
+ Treesitter, etc) using old hardware and the right choice of software that
minimizes the computational load required to run them.
We can still achieve modern computational tasks through [**frugal
computing**][frugal] ([Wim Vanderbauwhede, 2023][vanderbauwhede2023], [Low carbon
and sustainable computing][low-carbon-computing]).

### References

- Wim Vanderbauwhede (2023). Frugal Computing -- On the need for low-carbon and
  sustainable computing and the path towards zero-carbon computing.
  doi: [10.48550/arXiv.2303.06642][vanderbauwhede2023]

[aoc]: https://adventofcode.com/2024
[neovim]: https://neovim.io/
[macbookpro]: https://everymac.com/systems/apple/macbook_pro/specs/macbook-pro-core-2-duo-2.26-aluminum-13-mid-2009-sd-firewire-800-unibody-specs.html
[debian]: https://www.debian.org/
[vanderbauwhede2023]: https://doi.org/10.48550/arXiv.2303.06642
[low-carbon-computing]: https://www.dcs.gla.ac.uk/~wim/low-carbon-computing/index.html


## How to run the solutions

To run my solutions to AoC 2024 you need the Rust compiler. Since I decided not
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
