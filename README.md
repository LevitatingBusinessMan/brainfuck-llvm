# brainfuck-llvm
After I wrote my little [brainfuck interpreter in C](https://github.com/LevitatingBusinessMan/brainfuck) I decided to take the next step and make a brainfuck compiler using LLVM.

Although I read a bit of the LLVM Language Reference Manual, I learned the language mainly by writing what I wanted to achieve in C and then using `clang` to generate valid IR code. I looked at that code to learn the various functions used in LLVM.

### Details
The brainfuck uses a tape consisting of 30.000 signed bytes.
The compilers works simply by assigning each brainfuck instruction to a constant snippet of LLVM IR.
These instructions get stringed together, and then placed inside a premade body (found in `skel.ll`).
This means that no optimizations are done, like combining multiple `+` instructions together.
The resulting programs sets aside 30.000 signed bytes of memory to be used as a tape.

### Usage
You can run the crate directly via `cargo run`, or build via `cargo build --release`. 

To get the LLVM IR of a brainfuck program, simply use `brainfuck-llvm your_brainfuck_file`. Or to get the brainfuck code from stdin, use a `-` instead of the filename.
When using the output (`-o`) flag, the program will attempt to compile the IR and save it as a binary. This requires that you have `llvm` and `gcc` installed. Example: `brainfuck-llvm hello_world.bf -o hello_world`

#### Creating a Conway's game of life executable
Here we download and compile an implementation of Conway's Game of Life by Linus Akesson
`curl https://www.linusakesson.net/programming/brainfuck/life.bf | brainfuck-llvm - -o life`
This results in a binary called `life` to be created.
