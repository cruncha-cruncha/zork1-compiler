;"add the step to max-health after every encounter, up to 1500"
<GLOBAL PARENT-MONSTER-HEALTH-STEP 8>

;"monsters don't attack like us, their damage is either this or nothing"
<GLOBAL PARENT-MONSTER-ATK-DMG 40>
<GLOBAL CHILD-MONSTER-ATK-DMG 3>

;"parent monster is very surprised the first time it meets you"
<GLOBAL PARENT-MONSTER-FIRST-ENCOUNTER 1>

;"parent-monster won't chase you in the light"
<GLOBAL PARENT-MONSTER-WILL-CHASE 0>

;"child-monster will chase you in the den"
<GLOBAL CHILD-MONSTER-WILL-CHASE 0>

<ROOM DEN-1
    (DESC "You're in a monster's den" CR)
    (NORTH TO DEN-2)
    (EAST TO PASSAGE-1)
    (SOUTH TO DEN-4)>

<ROOM DEN-2
    (DESC "You're in a monster's den" CR)
    (EAST TO DEN-1)
    (WEST TO DEN-3)>

<ROOM DEN-3
    (DESC "You're in a monster's den" CR)
    (NORTH TO DEN-2)
    (SOUTH TO DEN-4)>

<ROOM DEN-4
    (DESC "You're in a monster's den" CR)
    (EAST TO DEN-1)
    (WEST TO DEN-3)>

;"can spawn anywhere underground"
;"always hits for 1 health"
<OBJECT CAVE-SPIDER
    (AKA SPIDER CAVE-SPIDER)
    (DESC "a SPIDER")
    (VARS HEALTH 2 OWN-TAKE 1) ;"is not soft"
    (ACT-IN-ROOM <CAVE-SPIDER-IN-ROOM>)>

;"only spawns in the monster den, will not leave it"
;"usually hits for 3 health"
;"max 100 health"
;"heals itself +1 every turn when the player is not around"
<OBJECT CHILD-MONSTER
    (AKA MONSTER)
    (DESC <DESC-CHILD-MONSTER>)
    (COPY <ROOM STORAGE>)
    (VARS HEALTH 100 MAX-HEALTH 100 IS-SOFT 1 NO-TAKE 1)
    (ACT-IN-ROOM <CHILD-MONSTER-IN-ROOM>)>

;"can spawn anywhere underground, but will not follow you if you're carrying light"
;"on first encounter, will hit player once then run away. Will hit for min(40, (player health - 1))"
;"always hits for 40 or 0 damage"
;"heals itself +1 every turn when the player is not around"
<OBJECT PARENT-MONSTER
    (AKA MONSTER)
    (DESC <DESC-PARENT-MONSTER>)
    (COPY <ROOM STORAGE>)
    (VARS HEALTH 60 MAX-HEALTH 70 IS-SOFT 1 NO-TAKE 1)
    (ACT-IN-ROOM <PARENT-MONSTER-IN-ROOM>)>

<ROUTINE CAVE-SPIDER-IN-ROOM ()
    <COND (
        <AND 
            <IS-DES 50 <RAND>>
            <IS-ASC 0 <GET-VAR PLAYER HEALTH>> 
        >
        <TELL "A spider attacks, you take one damage" CR>
        <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 1>>
    )(
        <NOT <IS-DES <GET-VAR <CMD 0> HEALTH> 0>>
        <TELL "There's a spider in here" CR>
    )>
>

<HIT CAVE-SPIDER (DMG)
    <COND (
        <AND
            <IS-EQUAL CAVE-SPIDER <CMD 1>>
            <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
        >
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>
        <SET-VAR DMG <ROLL-DMG>>

        <COND (
            <IS-DES DMG 0>
            <TELL "You hit the spider for " DMG " damage." CR>
            <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
        )>

        <COND (
            <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
            <TELL "It's dead." CR>
            <MOVE <CMD 1>>
        )>
    )>
>

<ROUTINE CHILD-MONSTER-IN-ROOM ()
    <COND (
        ;"if parent monster in room, run away silently"
        <IS-IN C-ROOM PARENT-MONSTER>
        <SET-VAR CHILD-MONSTER-WILL-CHASE 0>
        <MOVE <CMD 0> STORAGE>
        <RETURN 0>
    )(
        ;"if dead, won't do anything"
        <NOT <IS-DES <GET-VAR <CMD 0> HEALTH> 0>>
        <RETURN 0>
    )>

    <COND (
        <NOT <OR
            <IS-EQUAL C-ROOM DEN-1>
            <IS-EQUAL C-ROOM DEN-2>
            <IS-EQUAL C-ROOM DEN-3>
            <IS-EQUAL C-ROOM DEN-4>
        >>
        <TELL "The monster runs back to it's den" CR>
        <SET-VAR CHILD-MONSTER-WILL-CHASE 0>
        <MOVE <CMD 0> STORAGE>
        <RETURN 1>
    )>

    <COND (
        <IS-DES 80 <RAND>>
        <TELL "A monster hits you for " CHILD-MONSTER-ATK-DMG " health" CR>
        <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> CHILD-MONSTER-ATK-DMG>>
    )(
        <IS-EQUAL 1 1>
        <TELL "A monster attacks, but misses." CR>
    )>
    <SET-VAR CHILD-MONSTER-WILL-CHASE 1>
>

<ROUTINE PARENT-MONSTER-IN-ROOM (DMG)
    <COND (
        ;"if dead, do nothing"
        <NOT <IS-DES <GET-VAR <CMD 0> HEALTH> 0>>
        <RETURN 0>
    )(
        <IS-EQUAL PARENT-MONSTER-FIRST-ENCOUNTER 1>
        <TELL "There's a monster here. It hits you for ">
        <COND (
            <IS-ASC PARENT-MONSTER-ATK-DMG <GET-VAR PLAYER HEALTH>>
            <TELL PARENT-MONSTER-ATK-DMG>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> PARENT-MONSTER-ATK-DMG>>
        )(
            <IS-EQUAL 1 1>
            <SET-VAR DMG <SUBTRACT <GET-VAR PLAYER HEALTH> 1>>
            <TELL DMG>
            <SET-VAR PLAYER HEALTH 1>
        )>
        <TELL " health, then runs away. It must be scared of the light." CR>
        <SET-VAR PARENT-MONSTER-FIRST-ENCOUNTER 0> 
        <MOVE <CMD 0> STORAGE>
        <RETURN 1>
    )(
        <IS-EQUAL <ROOM-HAS-LIGHT> 1>
        <TELL "A monster runs from the room. It looks larger than it did last time." CR>
        <SET-VAR PARENT-MONSTER-WILL-CHASE -1>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR PARENT-MONSTER-WILL-CHASE 1>
    )>

    <COND (
        <IS-DES 50 <RAND>>
        <TELL "A monster hits you for " PARENT-MONSTER-ATK-DMG " health" CR>
        <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> PARENT-MONSTER-ATK-DMG>>
    )(
        <IS-EQUAL 1 1>
        <TELL "A monster attacks, but misses." CR>
    )>
>

<HIT CHILD-MONSTER (DMG)
    <COND (
        <NOT <IS-EQUAL CHILD-MONSTER <CMD 1>>>
        ;"do nothing"
    )(
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"do nothing"
    )(
        <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>
        <SET-VAR DMG <ROLL-DMG>>

        <COND (
            <IS-DES DMG 0>
            <TELL "You hit the monster for " DMG " damage." CR>
            <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
        )>

        <COND (
            <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
            <TELL "It's dead." CR>
            <SET-VAR WIN-KILL-MONSTER <ADD WIN-KILL-MONSTER 1>>
            <SET-VAR CHILD-MONSTER-WILL-CHASE 0>
        )>
    )>
>

<HIT PARENT-MONSTER (DMG) 
    <COND (
        <NOT <IS-EQUAL PARENT-MONSTER <CMD 1>>>
        ;"do nothing"
    )(
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"do nothing"
    )(
        <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>
        <SET-VAR DMG <ROLL-DMG>>

        <COND (
            <IS-DES DMG 0>
            <TELL "You hit the monster for " DMG " damage." CR>
            <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
        )>

        <COND (
            <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
            <TELL "It's dead." CR>
            <SET-VAR WIN-KILL-MONSTER <ADD WIN-KILL-MONSTER 1>>
            <SET-VAR PARENT-MONSTER-WILL-CHASE 0>
        )>
    )>
>

<TAKE () CAVE-SPIDER 
    <TELL "You crush all of the cave spiders." CR>
    <EACH-OBJ C-ROOM (OBJ)
        <COND (
            <IS-EQUAL CAVE-SPIDER OBJ>
            <MOVE OBJ>
        )>
    >
>

<ROUTINE DESC-CHILD-MONSTER ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "Health: " <GET-VAR <CMD 0> HEALTH> ", Max: " <GET-VAR <CMD 0> MAX-HEALTH> ", Damage: " CHILD-MONSTER-ATK-DMG CR>
        <COND (
            <NOT <IS-DES <GET-VAR <CMD 0> HEALTH> 0>>
            <TELL "This monster was small but well fed." CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "a MONSTER">
    )>
>

<ROUTINE DESC-PARENT-MONSTER ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "Health: " <GET-VAR <CMD 0> HEALTH> ", Max: " <GET-VAR <CMD 0> MAX-HEALTH> ", Damage: " PARENT-MONSTER-ATK-DMG CR>
        <COND (
            <NOT <IS-DES <GET-VAR <CMD 0> HEALTH> 0>>
            <TELL "This monster was experienced." CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "a MONSTER">
    )>
>