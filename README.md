# Zil-Like

This is actually more like a game engine. It accepts a zil-like (zork implementation language -like) syntax, not actual zil. Takes source code and converts it to javascript, doing some error checking in the process. It uses a recursive-descent parser (I think).

## Further Reading

- For the [demo game](https://cruncha-cruncha.github.io/zork1-compiler/): [player guide](https://github.com/cruncha-cruncha/zork1-compiler/blob/master/DEMO.md)
- For developers: [language spec](https://github.com/cruncha-cruncha/zork1-compiler/blob/master/LANG.md)

## Compiler

The game runs in js, but the compiler is written in rust. There are three steps:

1. Parse zil code (./input-files) into an Abstract Syntax Tree. This code lives in the `./src/zil`.
2. Convert data from that tree into a format more usable by js, and validate cross-references. This code lives in `./src/stats`.
3. Write js (./output-files). This code lives in `./src/js`.
