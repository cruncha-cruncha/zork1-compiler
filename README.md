# zork1-compiler

This is actually more like a game engine, and it accepts a zil-like (zork implementation language -like) syntax, not actually zil (anywhere you read 'zil', think 'zil-like'). At first I thought it was a compiler, then a transpiler. Takes source code and converts it to js, doing some mild error checking in the process. It uses a recursive-descent parser (maybe).

There are three steps:

1. Parse zil code into an Abstract Syntax Tree. This code lives in the 'zil' folder.
2. Convert data from that tree into a format usable by js (find classes, functions, and variables). This code lives in the 'stats' folder.
3. Write js. This code lives in the 'js' folder.

## I/O

Every file in ./intput-files is assumed to be a .zil file. Output is a bunch of .js files in ./output-files.

`cargo run`
