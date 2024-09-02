# zork1-compiler

This is actually a transpiler but I didn't know that word when I started the project. Takes zil (zork implementation language) source code and converts it to js, doing some mild error checking in the process. It uses a recursive-descent parser (maybe).

There are three steps:

1. Parse zil code into an Abstract Syntax Tree. This code lives in the 'zil' folder.
2. Convert data from that tree into a format usable by js (find classes, functions, and variables). This code lives in the 'stats' folder, so named because it collects statistics on the zil code in order to understand it better.
3. Write js. This code lives in the 'js' folder.

## I/O

Every file in ./intput-files is assumed to be a .zil file. Output is a bunch of .js files in ./output-files.

`cargo run`
