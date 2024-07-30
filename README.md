An implementation of the Hack Assembler from the 'Nand to Tetris' course,
written in Rust!

The architecture of the assembler follows a fairly functional approach: mostly
functions doing the heavy work, not relying on objects, and trying to use
mostly immutable variables. But a few mutable references are used when it makes
the everything easier.

The whole thing is done in a little more than 200 lines!

# References

- [From Nand to Tetris: Building a Modern Computer From First Principles](https://www.nand2tetris.org/)
