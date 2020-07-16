# brainfuck-llvm
After I wrote my little [brainfuck interpreter in C](https://github.com/LevitatingBusinessMan/brainfuck) I decided to take the next step and make a brainfuck compiler using LLVM.

Although I read a bit of the LLVM Language Reference Manual, I learned the language mainly by writing what I wanted to achieve in C and then using `clang` to generate valid IR code.
The brainfuck has a stack of a thousand cells each containing an 8 bit signed integer, athough this may change in the feature.
