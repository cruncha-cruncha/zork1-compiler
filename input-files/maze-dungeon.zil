<GLOBAL FOUND-MAGIC-RING 0>

<OBJECT MAGIC-RING
    (AKA RING MAGIC-RING)
    (DESC <DESC-MAGIC-RING>)>

;"entrance, has a bunch of rocks in it"
<ROOM LONG-HALL-1
    (DESC "You're at the south end of a large hall." CR)
    (NORTH TO LONG-HALL-2)
    (UP TO CAVE-ENTRANCE-1)
    (ACT-ENTER <LONG-HALL-1-ENTER>)>

<ROOM LONG-HALL-2
    (DESC "You're still in a large hall." CR)
    (NORTH TO LONG-HALL-3)
    (EAST TO GROTTO-2)
    (SOUTH TO LONG-HALL-1)
    (WEST TO GROTTO-1)>

<ROOM LONG-HALL-3
    (DESC "You're at the north end of a large hall." CR)
    (EAST TO CORRIDOR-1)
    (SOUTH TO LONG-HALL-2)
    (WEST TO GROTTO-3)>

<ROOM GROTTO-1
    (DESC "You're in a small grotto." CR)
    (EAST TO LONG-HALL-2)>

<ROOM GROTTO-2
    (DESC "You're in a small grotto. There's a massive hole in the ground." CR)
    (WEST TO LONG-HALL-2)
    (ACT-ENTER <ENTER-HOLE-RM>)>

<ROOM GROTTO-3
    (DESC "You're in a small grotto. There's a massive hole in the ground." CR)
    (EAST TO LONG-HALL-3)
    (ACT-ENTER <ENTER-HOLE-RM>)>

<ROOM CORRIDOR-1
    (DESC "You're in a long corridor." CR)
    (NORTH TO CORRIDOR-2)
    (EAST TO GROTTO-5)
    (SOUTH TO GROTTO-4)
    (WEST TO LONG-HALL-3)>

<ROOM CORRIDOR-2
    (DESC "You're in a long corridor." CR)
    (NORTH TO CORRIDOR-3)
    (EAST TO GROTTO-6)
    (SOUTH TO CORRIDOR-1)>

<ROOM CORRIDOR-3
    (DESC "You're in a long corridor." CR)
    (NORTH TO CORRIDOR-4)
    (EAST TO LONG-HALL-4)
    (SOUTH TO CORRIDOR-2)
    (WEST TO GROTTO-7)>

<ROOM CORRIDOR-4
    (DESC "You're in a long corridor." CR)
    (NORTH TO GROTTO-9)
    (SOUTH TO CORRIDOR-3)
    (WEST TO GROTTO-8)>

<ROOM GROTTO-4
    (DESC "You're in a small grotto. There's a massive hole in the ground." CR)
    (NORTH TO CORRIDOR-1)
    (ACT-ENTER <ENTER-HOLE-RM>)>

<ROOM GROTTO-5
    (DESC "You're in a small grotto. There's a massive hole in the ground." CR)
    (WEST TO CORRIDOR-1)
    (ACT-ENTER <ENTER-HOLE-RM>)>  

<ROOM GROTTO-6
    (DESC "You're in a small grotto." CR)
    (WEST TO LONG-HALL-2)>

<ROOM GROTTO-7
    (DESC "You're in a small grotto. There's a massive hole in the ground." CR)
    (WEST TO CORRIDOR-2)
    (ACT-ENTER <ENTER-HOLE-RM>)>

<ROOM GROTTO-8
    (DESC "You're in a small grotto." CR)
    (EAST TO CORRIDOR-4)
    (WEST TO SUB-GROTTO-8)>

;"has an obsidian shard"
<ROOM SUB-GROTTO-8
    (DESC "You're in a smaller grotto." CR)
    (EAST TO GROTTO-7)>

<ROOM GROTTO-9
    (DESC "You're in a small grotto. There's a massive hole in the ground." CR)
    (SOUTH TO CORRIDOR-4)
    (ACT-ENTER <ENTER-HOLE-RM>)>

<ROOM LONG-HALL-4
    (DESC "You're in a large hall." CR)
    (NORTH TO GROTTO-10)
    (EAST TO LONG-HALL-5)
    (SOUTH TO GROTTO-11)
    (WEST TO LONG-HALL-5)>

<ROOM LONG-HALL-5
    (DESC "You're in a large hall." CR)
    (NORTH TO GROTTO-12)
    (EAST TO LONG-HALL-4)
    (SOUTH TO GROTTO-13)
    (WEST TO LONG-HALL-4)>

<ROOM GROTTO-10
    (DESC "You're in a small grotto." CR)
    (SOUTH TO LONG-HALL-4)> 

;"back to corridor"
<ROOM GROTTO-11
    (DESC "You're in a small grotto." CR)
    (NORTH TO LONG-HALL-4)
    (WEST TO CORRIDOR-3)>

;"has exit"
<ROOM GROTTO-12
    (DESC "You're in a small grotto." CR)
    (NORTH TO MAZE-REST-1)
    (SOUTH TO LONG-HALL-5)>

<ROOM GROTTO-13
    (DESC "You're in a small grotto." CR)
    (NORTH TO LONG-HALL-5)>

<ROOM MAZE-REST-1
    (DESC <DESC-MAZE-REST-1>)
    (NORTH TO BTN-RM-1)
    (SOUTH TO GROTTO-12)
    (UP TO PASSAGE-1)>

<ROOM BTN-RM-1
    (DESC <DESC-BTN-RM-1>)
    (NORTH PER <BTN-RM-1-NORTH>)
    (EAST PER <BTN-RM-1-EAST>)
    (SOUTH PER <BTN-RM-1-SOUTH>)
    (WEST PER <BTN-RM-1-WEST>)
    (VARS START 0 STEP 0)>

<ROUTINE DESC-BTN-RM-1 ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "You're in a room. There's a symbol on the floor:" CR>
        <TELL "a small circle, with a line going east" CR>
        <TELL "at the end of that line, a second small circle" CR>
        <TELL "coming out of the second small circle, a line going north" CR>
        <TELL "at the end of that line, an X" CR>
        <COND (
            <IS-EQUAL <GET-VAR BTN-RM-1 STEP> 0>
            <TELL "The first circle is glowing blue" CR>
        )(
            <IS-EQUAL <GET-VAR BTN-RM-1 STEP> 1>
            <TELL "The second circle is glowing blue" CR>
        )(
            <IS-EQUAL <GET-VAR BTN-RM-1 STEP> 2>
            <TELL "The X is glowing blue" CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "You're in a room. There's a symbol on the floor." CR>
    )>
>

<ROUTINE BTN-RM-1-NORTH ()
    <COND (
        <IS-EQUAL <GET-VAR BTN-RM-1 STEP> 1>
        <SET-VAR BTN-RM-1 STEP 2>
        <COND (
            <IS-EQUAL <GET-VAR BTN-RM-1 START> 0>
            <SET-VAR BTN-RM-1 START 2>
            <MOVE PLAYER MAZE-REST-2>
            <RETURN 1>
        )>
        <TELL "You appear to be in the same room, but something has changed." CR>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR BTN-RM-1 STEP <GET-VAR BTN-RM-1 START>>
        <TELL "You appear to be in the same room that you started from." CR>
    )>
>

<ROUTINE BTN-RM-1-EAST ()
    <COND (
        <IS-EQUAL <GET-VAR BTN-RM-1 STEP> 0>
        <SET-VAR BTN-RM-1 STEP 1>
        <TELL "You appear to be in the same room, but something has changed." CR>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR BTN-RM-1 STEP <GET-VAR BTN-RM-1 START>>
        <TELL "You appear to be in the same room that you started from." CR>
    )>
>

<ROUTINE BTN-RM-1-SOUTH ()
    <COND (
        <IS-EQUAL <GET-VAR BTN-RM-1 STEP> 2>
        <SET-VAR BTN-RM-1 STEP 1>
        <TELL "You appear to be in the same room, but something has changed." CR>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR BTN-RM-1 STEP <GET-VAR BTN-RM-1 START>>
        <TELL "You appear to be in the same room that you started from." CR>
    )>
>

<ROUTINE BTN-RM-1-WEST ()
    <COND (
        <IS-EQUAL <GET-VAR BTN-RM-1 STEP> 1>
        <SET-VAR BTN-RM-1 STEP 0>
        <COND (
            <IS-EQUAL <GET-VAR BTN-RM-1 START> 2>
            <SET-VAR BTN-RM-1 START 0>
            <MOVE PLAYER MAZE-REST-1>
            <RETURN 1>
        )>
        <TELL "You appear to be in the same room, but something has changed." CR>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR BTN-RM-1 STEP <GET-VAR BTN-RM-1 START>>
        <TELL "You appear to be in the same room that you started from." CR>
    )>
>

<ROOM MAZE-REST-2
    (DESC "You're in a lovely cave, with a huge pile of rocks." CR)
    (NORTH TO MAZE-GRID-RED)
    (SOUTH TO BTN-RM-1)>

;"an infinite pile of rocks"
<OBJECT ROCK-PILE
    (AKA ROCK ROCKS)
    (DESC "a large rock pile")
    (COPY <ROOM MAZE-REST-2>)
    (ACT-IN-ROOM <ROCK-PILE-IN-ROOM>)>

<ROUTINE ROCK-PILE-IN-ROOM ()
    ;"clean up (delete) all stray rocks (put them in the infinite pile)"
    <EACH-OBJ C-ROOM (OBJ)
        <COND (
            <IS-EQUAL OBJ ROCK>
            <MOVE OBJ>
        )>
    >
>

<TAKE () ROCK 
    ;"this handler will be called if the player already has a rock in their inventory"
    <COND (
        <IS-IN C-ROOM ROCK-PILE>
        <COPY-MOVE ROCK PLAYER>
    )>
>

<TAKE () ROCK-PILE 
    ;"can't pick up the whole pile"
    <MOVE <CMD 1> C-ROOM>
    ;"can pick up a single rock from the pile"
    <COPY-MOVE ROCK PLAYER>
>

<ROOM MAZE-GRID-RED
    (DESC "a red room" CR)
    (NORTH TO MAZE-GRID-BLUE)
    (EAST TO MAZE-GRID-TEAL)
    (SOUTH TO MAZE-REST-2)
    (WEST TO MAZE-GRID-BLUE)>

<ROOM MAZE-GRID-BLUE
    (DESC "a blue room" CR)
    (NORTH TO MAZE-GRID-RED)
    (EAST TO MAZE-GRID-RED)
    (SOUTH TO MAZE-REST-2)
    (WEST TO MAZE-GRID-RED)>

<ROOM MAZE-GRID-TEAL
    (DESC "a blue room" CR)
    (NORTH TO MAZE-REST-3)
    (EAST TO MAZE-GRID-RED)
    (SOUTH TO MAZE-REST-2)
    (WEST TO MAZE-GRID-RED)>

<ROOM MAZE-REST-3
    (DESC "You're in a lovely rocky cave." CR)
    (NORTH TO BTN-RM-2)
    ;"magically teleport back to maze-rest-2"
    (SOUTH TO MAZE-REST-2)>

<ROOM BTN-RM-2
    (DESC <DESC-BTN-RM-2>)
    (EAST PER <BTN-RM-2-EAST>)
    (SOUTH PER <BTN-RM-2-SOUTH>)
    (WEST PER <BTN-RM-2-WEST>)
    (VARS STEP 0)>

<ROUTINE DESC-BTN-RM-2 ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "You're in a room. There are some symbols on the floor:" CR>
        <TELL "two small circles, one west and east, connected by a line" CR>
        <TELL "the number 3 is written just above the line" CR>
        <COND (
            <IS-EQUAL <GET-VAR BTN-RM-2 STEP> 0>
            <TELL "The east circle glows faintly blue" CR>
        )(
            <IS-EQUAL <GET-VAR BTN-RM-2 STEP> 1>
            <TELL "The west circle glows faintly blue" CR>
        )(
            <IS-EQUAL <GET-VAR BTN-RM-2 STEP> 2>
            <TELL "The east circle glows a strong blue" CR>
        )(
            <IS-EQUAL <GET-VAR BTN-RM-2 STEP> 3>
            <TELL "The west circle glows a strong blue" CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "You're in a room. There are some symbols on the floor." CR>
    )>
>

<ROUTINE BTN-RM-2-EAST ()
    <COND (
        <IS-EQUAL <GET-VAR BTN-RM-2 STEP> 1>
        <SET-VAR BTN-RM-2 STEP 2>
        <TELL "You appear to be in the same room, but something has changed." CR>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR BTN-RM-2 STEP 0>
        <TELL "You appear to be in the same room that you started from." CR>
    )>
>

<ROUTINE BTN-RM-2-SOUTH ()
    <SET-VAR BTN-RM-2 STEP 0>
    <MOVE PLAYER MAZE-REST-2>
>

<ROUTINE BTN-RM-2-WEST ()
    <COND (
        <IS-EQUAL <GET-VAR BTN-RM-2 STEP> 0>
        <SET-VAR BTN-RM-2 STEP 1>
        <TELL "You appear to be in the same room, but something has changed." CR>
    )(
        <IS-EQUAL <GET-VAR BTN-RM-2 STEP> 2>
        <SET-VAR BTN-RM-2 STEP 3>
        <TELL "You appear to be in the same room, but something has changed." CR>
        <COND (
            <IS-EQUAL FOUND-MAGIC-RING 0>
            <SET-VAR FOUND-MAGIC-RING 1>
            <COPY-MOVE MAGIC-RING C-ROOM>
            <TELL "And something shiny appeared on the ground." CR>
        )(
            <IS-ASC <RAND> 50>
            <COPY-MOVE <INST STORAGE OBSIDIAN-SHARD> C-ROOM>
            <TELL "And something shiny appeared on the ground." CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR BTN-RM-2 STEP 0>
        <TELL "You appear to be in the same room that you started from." CR>
    )>
>

<ROUTINE DESC-MAGIC-RING ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "This intricately-carved silver ring weighs much more than it looks like it should. It may be magic.">
    )(
        <IS-EQUAL 1 1>
        <TELL "a RING">
    )>
>

<TAKE () MAGIC-RING 
    <TELL "The ring fits perfectly onto you're finger, then painlessly melts into your skin." CR>
    <SET-VAR PLAYER HAS-MAGIC-RING 1>
    <MOVE <CMD 1>>
>

<ROUTINE ENTER-HOLE-RM ()
    <COND (
        <AND
            <NOT <IS-IN PLAYER TORCH>>
            <NOT <IS-IN PLAYER GEM>> 
        >
        <TELL "Without light to see where you're going, you fall into the hole." CR>
        <SET-VAR PLAYER HEALTH -1>
    )>
>

<ROUTINE LONG-HALL-1-ENTER ()
    <SET-VAR WIN-ENTER-CAVE 1>
>

<ROUTINE DESC-MAZE-REST-1 ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "You're in a lovely rocky cave. There's another space UP." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "You're in a lovely rocky cave." CR>
    )>
>