<ROOM PASSAGE-1
    (DESC "A passageway in the rock" CR)
    (EAST TO CAVERN-1)
    (WEST TO DEN-1)
    (UP TO CAVE-ENTRANCE-2)
    (DOWN TO MAZE-REST-1)>

<ROOM CAVERN-1
    (DESC "A massive cavern" CR)
    (NORTH PER <CAVERN-1-NORTH>)
    (WEST TO PASSAGE-1)
    (SOUTH TO CAVE-LAKE)>

<ROOM CRYPT
    (DESC "a crypt" CR)
    (SOUTH TO CAVERN-1)>

<ROOM CAVE-LAKE
    (DESC "an underground lake")
    (NORTH TO CAVERN-1)>

<ROOM DEN-1
    (DESC "the den of a monster")
    (NORTH TO DEN-2)
    (EAST TO PASSAGE-1)
    (SOUTH TO DEN-4)>

<ROOM DEN-2
    (DESC "the den of a monster")
    (EAST TO DEN-1)
    (WEST TO DEN-3)>

<ROOM DEN-3
    (DESC "the den of a monster")
    (NORTH TO DEN-2)
    (SOUTH TO DEN-4)>

<ROOM DEN-4
    (DESC "the den of a monster")
    (EAST TO DEN-1)
    (WEST TO DEN-3)>

<OBJECT WIRE
    (AKA WIRE WIRES)
    (DESC "some WIRE")
    (COPY <ROOM CAVERN-1>)>

<OBJECT PICK-AXE
    (AKA PICK PICK-AXE)
    (DESC "a PICK-AXE")
    (COPY <ROOM CAVERN-1>)>

<OBJECT SWORD
    (AKA SWORD)
    (DESC "a SWORD")
    (COPY <ROOM CAVERN-1>)>

<OBJECT ROCK
    (AKA ROCK ROCKS)
    (DESC "a ROCK")
    (COPY <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM CAVERN-1>
          <ROOM CAVERN-1>
          <ROOM DEN-2>
          <ROOM DEN-3>
          <ROOM CAVE-LAKE>
          <ROOM CAVE-LAKE>
          <ROOM CAVE-LAKE>
          <ROOM CRYPT>
          <ROOM CAVE-ENTRANCE-1>)
>

<OBJECT BONES
    (AKA BONE BONES)
    (DESC "some BONES")
    (COPY <ROOM DEN-1>
          <ROOM DEN-1>
          <ROOM DEN-1>
          <ROOM DEN-2>
          <ROOM DEN-3>
          <ROOM CAVERN-1>
          <ROOM CAVE-LAKE>
          <COFFIN 1>)
    (VARS EDIBLE 1)>
    
<OBJECT MASTER-KEY
    (AKA MASTER-KEY)
    (DESC "a MASTER-KEY")>

<OBJECT BAT>

<OBJECT GOLD-LUMP
    (AKA GOLD GOLD-LUMP)
    (DESC "a GOLD lump")>

<OBJECT STONE-DOOR
    (AKA DOOR STONE-DOOR)
    (DESC "a stone DOOR")
    (COPY <ROOM CAVERN-1>)
    (VARS NO-TAKE 1)>

<OBJECT COFFIN
    (AKA COFFIN)
    (DESC "a COFFIN")
    (COPY <ROOM CRYPT>)>

<OBJECT CURSED-SKULL
    (AKA SKULL CURSED-SKULL)
    (DESC "a cursed SKULL")
    (COPY <COFFIN 1>)>

<OBJECT OBSIDIAN-SHARD
    (AKA OBSIDIAN OBSIDIAN-SHARD)
    (DESC <DESC-OBSIDIAN>)
    (COPY <ROOM SUB-GROTTO-7>)
    (VARS DAMAGE 60 MAX-DAMAGE 60)>

<ROUTINE DESC-OBSIDIAN () 
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "This obsidian shard is extremely sharp but very brittle; you might only get one good hit with it.
        And be careful: you can hurt yourself by using a dull implement.">
    )(
        <IS-EQUAL 1 1>
        <TELL "an OBSIDIAN shard">
    )>
>

<ROUTINE CAVERN-1-NORTH ()
    ;"TODO"
    ;"if stone-door health = 0, can go north to crypt"
    ;"if player has master key, can go north to crypt"
    ;"if player has talked to the bat"
>
