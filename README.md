![](https://raw.githubusercontent.com/writeonlycode/hack-assembler/main/Screenshot%20from%202024-07-30%2012-34-28.png)

# Hack Assembler in Rust

This project is an implementation of the Hack Assembler from the 'Nand to
Tetris' course, written in Rust!

The architecture of the assembler follows a functional approach: mostly
functions doing the heavy work, not relying on objects, and trying to use
mostly immutable variables. However, a few mutable references are used when it
makes everything easier.

The entire implementation is done in just over 200 lines of code!


# References

- [From Nand to Tetris: Building a Modern Computer From First Principles](https://www.nand2tetris.org/)
