# zork1-compiler

This is actually more like a game engine. It accepts a zil-like (zork implementation language -like) syntax, not actually zil (anywhere you read 'zil', think 'zil-like'). At first I thought it was a compiler, then a transpiler. Takes source code and converts it to js, doing some mild error checking in the process. It uses a recursive-descent parser (maybe).

There are three steps:

1. Parse zil code into an Abstract Syntax Tree. This code lives in the 'zil' folder.
2. Convert data from that tree into a format more usable by js. This code lives in the 'stats' folder.
3. Write js. This code lives in the 'js' folder.

## I/O

Every file in ./intput-files is assumed to be a .zil file. Output is a bunch of .js files in ./output-files.

## Concepts

Aka top-level keywords in the zil language.

Player

- `<PLAYER ... >`
- always in a room
- has variables
- has an inventory (aka can store objects in the player)

Room

- `<ROOM ... >`
- has variables
- can have objects inside

Object

- in the player, a room, or another object
- has variables
- can contain nested objects, and those objects can contain nested objects, and so on

Globals

- aka global variables
- no new variables can be added at game time (unlike player, object, or room variables which can exist before the game starts or be added in the process of playing the game)

All 'variables' can only store integer values. If you try to read a variable that doesn't exist, you'll get a zero. So treat zero values accordingly (as if they were boolean false, or don't exist).

Syntax

- defines an input string
- first word is the 'action'
- can work with up to two objects (PRSO and PRSI). Explained in more detail later on.

Synonym

- used to define alternatives to any word in a syntax

Buzz

- used to ignore words when matching any syntax

Directions

- directions which can be used with the special 'GO' command (explained later on). Usually NORTH, SOUTH, EAST, and WEST (at minimum), but can really be anything.

Routines

- aka actions
- always has access to the player, current room, action, PRSO, and PRSI. Can also read and modify any global variable
- can define a set of local variables to use only within this routine

## Hooks

A routine can be assigned to any of the following hooks, which will be fired when certain conditions are met.
Player has four hooks:

- ACT-FIRST: fires when the player enters any room for the first time (it's the player's first time in that room).
- ACT-ENTER: fires when the player enters any room
- ACT-EXIT: fires when the player exits any room
- ACT-ALWAYS: fires on every command

Every room has a similar set of four hooks:

- ACT-FIRST: fires when the player enters this room for the first time (it's the player's first time in this room)
- ACT-ENTER: fires when the player enters this room
- ACT-EXIT: fires when the player leaves this room
- ACT-ALWAYS: fires on every command while the player is in this room

Every object has six hooks:

- ACT-IN-ROOM: fires on every command while this object is in the same room as the player
- ACT-IN-PLAYER: fires on every command while this object is in the player's inventory
- ACT-ADD: fires when this object is added to the player's inventory
- ACT-REMOVE: fires when this object is removed from the player's inventory
- ACT-PRSO: fires if this object is successfully used as the PRSO in a command
- ACT-PRSI: fires if this object is successfully used as the PRSI in a command

### Firing order

- object ACT-PRSI
- object ACT-PRSO
- syntax action
- object ACT-REMOVE
- object ACT-ADD
- object ACT-IN-PLAYER
- room ACT-EXIT
- player ACT-EXIT
- room ACT-FIRST
- player ACT-FIRST
- object ACT-IN-ROOM
- room ACT-ENTER
- player ACT-ENTER
- room ACT-ALWAYS
- player ACT-ALWAYS

Object hooks fire for every object that was moved (including objects not explicitly moved by the player), and firing order is always respected. So if a theatre's (a room) ACT-ENTER hook removes a concert ticket (an object) from the players inventory, the concert ticket's ACT-REMOVE action will fire before the theatre's ACT-ALWAYS action.

## Syntax

How does it work? What are PRSO and PRSI? What's the special GO command?

## Routines

What are the illegal values? What can you do? What are some examples?

`cargo run`
