<GLOBAL FOUND-MAGIC-RING 0>

<OBJECT MAGIC-RING
    (AKA RING MAGIC-RING)
    (DESC <DESC-MAGIC-RING>)>

;"entrance, has a bunch of rocks in it"
<ROOM LONG-HALL-1
    (DESC "You're at the south end of a large hall." CR)
    (NORTH TO LONG-HALL-2)
    (UP TO CAVE-ENTRANCE-1)>

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
    (DESC "You're in a small grotto." CR)
    (WEST TO LONG-HALL-2)>

<ROOM GROTTO-3
    (DESC "You're in a small grotto." CR)
    (EAST TO LONG-HALL-3)>

<ROOM CORRIDOR-1
    (DESC "You're in a long corridor." CR)
    (NORTH TO CORRIDOR-2)
    (EAST TO HOLE-1)
    (WEST TO LONG-HALL-3)>

<ROOM CORRIDOR-2
    (DESC "You're in a long corridor." CR)
    (NORTH TO CORRIDOR-3)
    (EAST TO GROTTO-5)
    (SOUTH TO CORRIDOR-1)>

<ROOM CORRIDOR-3
    (DESC "You're in a long corridor." CR)
    (NORTH TO CORRIDOR-4)
    (EAST TO LONG-HALL-4)
    (SOUTH TO CORRIDOR-2)
    (WEST TO HOLE-2)>

<ROOM CORRIDOR-4
    (DESC "You're in a long corridor." CR)
    (SOUTH TO CORRIDOR-3)
    (WEST TO GROTTO-7)>

<ROOM HOLE-1
    (DESC "You're in a small grotto. There's a massive hole in the ground." CR)
    (WEST TO CORRIDOR-1)
    (ACT-EXIT <EXIT-HOLE-RM>)>  

<ROOM GROTTO-5
    (DESC "You're in a small grotto." CR)
    (WEST TO CORRIDOR-2)>

<ROOM HOLE-2
    (DESC "You're in a small grotto. There's a massive hole in the ground." CR)
    (EAST TO CORRIDOR-3)
    (ACT-EXIT <EXIT-HOLE-RM>)>

<ROOM GROTTO-7
    (DESC "You're in a small grotto." CR)
    (EAST TO CORRIDOR-4)
    (WEST TO SUB-GROTTO-7)>

;"has an obsidian shard"
<ROOM SUB-GROTTO-7
    (DESC "You're in a smaller grotto." CR)
    (EAST TO GROTTO-7)>

<ROOM LONG-HALL-4
    (DESC "You're in a large hall.")
    (NORTH TO GROTTO-10)
    (EAST TO LONG-HALL-5)
    (SOUTH TO GROTTO-9)
    (WEST TO CORRIDOR-3)>

<ROOM LONG-HALL-5
    (DESC "You're in a large hall.")
    (NORTH TO GROTTO-12)
    (SOUTH TO HOLE-3)
    (WEST TO LONG-HALL-4)>
    
<ROOM GROTTO-9
    (DESC "You're in a small grotto." CR)
    (NORTH TO LONG-HALL-4)>

;"has exit"
<ROOM GROTTO-10
    (DESC "You're in a small grotto." CR)
    (NORTH TO MAZE-REST-1)
    (SOUTH TO LONG-HALL-4)> 

<ROOM HOLE-3
    (DESC "You're in a small grotto. There's a massive hole in the ground." CR)
    (NORTH TO LONG-HALL-5)
    (ACT-EXIT <EXIT-HOLE-RM>)>

<ROOM GROTTO-12
    (DESC "You're in a small grotto." CR)
    (SOUTH TO LONG-HALL-5)>

<ROOM MAZE-REST-1
    (DESC "You're in a lovely rocky cave." CR)
    (NORTH TO BTN-RM-1)
    (SOUTH TO GROTTO-10)
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
    (DESC "You're in a lovely rocky cave." CR)
    (NORTH TO BTN-RM-2)
    (SOUTH TO BTN-RM-1)>

<ROOM BTN-RM-2
    (DESC <DESC-BTN-RM-2>)
    (EAST PER <BTN-RM-2-EAST>)
    (SOUTH PER <BTN-RM-2-SOUTH>)
    (WEST PER <BTN-RM-2-WEST>)
    (VARS START 0 STEP 0)>

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
        <IS-EQUAL <GET-VAR BTN-RM-2 STEP> 3>
        <SET-VAR BTN-RM-2 STEP 2>
        <TELL "You appear to be in the same room, but something has changed." CR>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR BTN-RM-2 STEP <GET-VAR BTN-RM-2 START>>
        <TELL "You appear to be in the same room that you started from." CR>
    )>
>

<ROUTINE BTN-RM-2-SOUTH ()
    <COND (
        <OR 
            <IS-EQUAL <GET-VAR BTN-RM-2 STEP> 0>
            <IS-EQUAL <GET-VAR BTN-RM-2 STEP> 3>
        >
        <MOVE PLAYER MAZE-REST-2>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR BTN-RM-2 STEP <GET-VAR BTN-RM-2 START>>
        <TELL "You appear to be in the same room that you started from." CR>
    )>
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
            <IS-ASC <RAND> 67>
            <COPY-MOVE OBSIDIAN-SHARD C-ROOM>
            <TELL "And something shiny appeared on the ground." CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR BTN-RM-2 STEP <GET-VAR BTN-RM-2 START>>
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

<ROUTINE EXIT-HOLE-RM ()
    <COND (
        <AND
            <NOT <IS-IN PLAYER TORCH>>
            <NOT <IS-IN PLAYER GEM>> 
        >
        <TELL "Without light to see where you're going, you fall into the hole and die." CR>
        <SET-VAR PLAYER HEALTH -1>
    )>
>
