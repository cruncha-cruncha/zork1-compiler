# zork1-compiler

This is actually more like a game engine. It accepts a zil-like (zork implementation language -like) syntax, not actually zil (anywhere you read 'zil', think 'zil-like'). Takes source code and converts it to js, doing some error checking in the process. It uses a recursive-descent parser (I think).

There are three steps:

1. Parse zil code (./input-files) into an Abstract Syntax Tree. This code lives in the 'zil' folder.
2. Convert data from that tree into a format more usable by js. This code lives in the 'stats' folder.
3. Write js (./output-files). This code lives in the 'js' folder.

## Concepts

Aka top-level keywords in the zil-like language.

Player

- `<PLAYER ... >`
- always in a room
- has variables
- has an inventory (aka objects can be stored in the player)

Room

- `<ROOM ... >`
- has variables
- can have objects inside

Object

- `<OBJECT ... >`
- in the player, a room, or another object
- has variables
- can contain nested objects, and those objects can contain nested objects, and so on

Globals

- `<GLOBAL ... >`
- aka global variables
- no new variables can be added at game time (unlike player, object, or room variables which can exist before the game starts or be added in the process of playing the game)

All 'variables' can only store integer values. If you try to read a variable that doesn't exist, you'll get a zero. So treat zero values accordingly (as if they were boolean false or don't exist).

Syntax

- `<SYNTAX ... >`
- defines an input command string
- first word is the action, this needs to be known for defining handlers

Synonym

- `<SYNONYM ... >`
- used to define alternatives to any word in a syntax

Buzz

- `<BUZZ ... >`
- used to ignore words when matching any syntax. These are effectively erased before being passed to the game-time parser.

Directions

- `<DIRECTIONS ... >`
- directions which can be used with the special 'GO' command. Usually NORTH, SOUTH, EAST, and WEST, but can really be anything.

Routines

- `<ROUTINES ... >`
- aka handlers, functions, algorithms, or code that does stuff
- can define a set of local variables to use only within this routine. These, like other variables, can only hold integer values.
- always has access to the player, current room, PRSA, PRSO, and PRSI. Can also read and modify any global variable

## Hooks

A routine can be assigned to any of the following 'hooks', which will be fired (aka the routine will be executed) when certain conditions are met. Any routine can be assigned to any hook, but a hook can only be assigned to once (or zero times, they're optional).
Player has three hooks:

- ACT-ENTER: fires when the player enters any room
- ACT-EXIT: fires when the player exits any room
- ACT-ALWAYS: fires after every command

Every room has a similar set of three hooks:

- ACT-ENTER: fires when the player enters this room
- ACT-EXIT: fires when the player leaves this room
- ACT-ALWAYS: fires after every command while the player is in this room

Every object has four hooks:

- ACT-IN-ROOM: fires after every command while this object is in the same room as the player
- ACT-IN-PLAYER: fires after every command while this object is in the player's inventory
- ACT-ADD: fires when this object is added to the player's inventory
- ACT-REMOVE: fires when this object is removed from the player's inventory

### Firing order

- object ACT-ADD
- object ACT-REMOVE
- room ACT-EXIT
- player ACT-EXIT
- room ACT-ENTER
- player ACT-ENTER
- object ACT-IN-PLAYER
- object ACT-IN-ROOM
- room ACT-ALWAYS
- player ACT-ALWAYS

Object hooks generally fire for every object that was moved (including objects not explicitly moved by the player), and firing order is always respected. So if a theatre's (a room) ACT-ENTER hook removes a concert ticket (an object) from the players inventory, the concert ticket's ACT-REMOVE action will fire before the theatre's ACT-ALWAYS action.

When a hook is inserted, it removes all upcoming hook calls of lower priority. This an opaque process which can lead to unexpected behaviour, so always test your hooks. Details of exactly when exactly hooks are inserted can muddled out of `./js-boilerplate/engine.js`.

## Syntax

Let's look at an example: `<SYNTAX HIT OBJECT WITH OBJECT = V-HIT>`. A player could enter the command "HIT NAIL WITH HAMMER", in which case:

- PRSA = 'HIT'
- PRSO = the NAIL object
- PRSI = the HAMMER object
  (assuming NAIL and HAMMER exist and are accessible). Objects are accessible if they are in the PLAYER or CURRENT-ROOM. First OBJECT is always PRSO, second OBJECT is always PRSI. If there are no OBJECTS in a syntax, the PRSO and PRSI are (fake) empty objects in PLAYER.

A special 'GO' command allows the player to move on their own, for example: "GO NORTH".

## Routines

What are the illegal values? What can you do? What are some examples?

# Compiling

`cargo run`
