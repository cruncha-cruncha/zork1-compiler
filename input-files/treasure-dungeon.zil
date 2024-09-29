<ROOM PASSAGE-1
    (DESC "You're in a series of tunnels." CR)
    (EAST TO CAVERN-1)
    (WEST TO DEN-1)
    (UP TO CAVE-ENTRANCE-2)
    (DOWN TO MAZE-REST-1)
    (ACT-ENTER <PASSAGE-1-ENTER>)>

<ROOM CAVERN-1
    (DESC <DESC-CAVERN-1>) ;"A large cavern"
    (NORTH PER <CAVERN-1-NORTH>)
    (WEST TO PASSAGE-1)
    (SOUTH TO CAVE-LAKE)>

<ROOM CRYPT
    (DESC <DESC-CRYPT>);"You're in a low-ceiled space. From the coffin in the center, you can tell it's a crypt."
    (SOUTH PER <CRYPT-SOUTH>)
    (EAST TO WATERFALL-PASSAGE)>

<ROOM WATERFALL-PASSAGE
    (DESC <DESC-WATERFALL-PASSAGE>);"You're in a long passage winding through the rock. The sound of rushing water echos along it."
    (WEST TO CRYPT)
    (EAST PER <WATERFALL-PASSAGE-UP>)
    (UP PER <WATERFALL-PASSAGE-UP>)
    (ACT-ENTER <WATERFALL-PASSAGE-ENTER>)>

<ROOM CAVE-LAKE
    (DESC <DESC-CAVE-LAKE>);"an underground lake"
    (NORTH TO CAVERN-1)
    (ACT-ALWAYS <CAVE-LAKE-ALWAYS>)>

<OBJECT PICK-AXE
    (AKA PICK PICK-AXE)
    (DESC <DESC-PICK-AXE>)
    (VARS DAMAGE 10 MAX-DAMAGE 4)
    (COPY <ROOM CAVERN-1>)>

<OBJECT SWORD
    (AKA SWORD)
    (DESC <DESC-SWORD>)
    (VARS DAMAGE 30 MAX-DAMAGE 30 OWN-TAKE 1)
    (COPY <ROOM CRYPT>)>

<OBJECT ROCK
    (AKA ROCK ROCKS)
    (DESC "a ROCK")
    (VARS IS-HARD 1 DAMAGE 2 MAX-DAMAGE 2)
    (COPY <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM LONG-HALL-1>
          <ROOM CAVERN-1>
          <ROOM DEN-2>
          <ROOM DEN-3>
          <ROOM CAVE-LAKE>
          <ROOM CRYPT>
          <ROOM CAVE-ENTRANCE-1>)
>

<OBJECT BONES
    (AKA BONE BONES)
    (DESC <DESC-BONES>)
    (COPY <ROOM DEN-1>
          <ROOM DEN-1>
          <ROOM DEN-1>
          <ROOM DEN-2>
          <ROOM DEN-3>
          <ROOM CAVERN-1>
          <ROOM CAVE-LAKE>
          <COFFIN 1>)
    (VARS IS-EDIBLE 1)>
    
<OBJECT MASTER-KEY
    (AKA MASTER-KEY)
    (DESC "a MASTER-KEY")>

<OBJECT GOLD-LUMP
    (AKA GOLD GOLD-LUMP)
    (DESC "a GOLD lump")>

<OBJECT STONE-DOOR
    (AKA DOOR STONE-DOOR)
    (DESC <DESC-STONE-DOOR>)
    (COPY <ROOM CAVERN-1>)
    (VARS IS-HARD 1 NO-TAKE 1 HEALTH 10 IS-LOCKED 1)>

<OBJECT COFFIN
    (AKA COFFIN)
    (DESC "a COFFIN")
    (VARS IS-HARD 1)
    (COPY <ROOM CRYPT>)>

<OBJECT CURSED-SKULL
    (AKA SKULL CURSED-SKULL)
    (DESC <DESC-CURSED-SKULL>)
    (COPY <COFFIN 1>)
    (ACT-IN-PLAYER <CURSED-SKULL-IN-PLAYER>)
    (ACT-REMOVE <CURSED-SKULL-EXIT-PLAYER>)>

<OBJECT OBSIDIAN-SHARD
    (AKA OBSIDIAN OBSIDIAN-SHARD)
    (DESC <DESC-OBSIDIAN>)
    (COPY <ROOM SUB-GROTTO-8>
          <WATER 2>)
    (VARS DAMAGE 50 MAX-DAMAGE 50)>

<ROUTINE DESC-OBSIDIAN () 
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "This obsidian shard is extremely sharp but very brittle; you might only get one good hit with it.">
    )(
        <IS-EQUAL 1 1>
        <TELL "an OBSIDIAN shard">
    )>
>

<ROUTINE CAVERN-1-NORTH ()
    <COND (
        <OR
            <IS-DES 1 <GET-VAR <INST C-ROOM STONE-DOOR> HEALTH>>
            <IS-EQUAL <GET-VAR <INST C-ROOM STONE-DOOR> IS-LOCKED> 0>
        >
        <MOVE PLAYER CRYPT>
    )(
        <IS-IN PLAYER MASTER-KEY>
        <TELL "You unlock the stone door." CR>
        <MOVE PLAYER CRYPT>
    )(
        <IS-EQUAL 1 1>
        <TELL "The stone door blocks your path." CR>
    )>
>

<ROUTINE CRYPT-SOUTH ()
    <COND (
        <OR
            <IS-EQUAL <GET-VAR <INST CRYPT STONE-DOOR> IS-LOCKED> 0>
            <IS-ASC <GET-VAR <INST CRYPT STONE-DOOR> HEALTH> 1>
        >
        <MOVE PLAYER CAVERN-1>
    )(
        <IS-EQUAL 1 1>
        <TELL "You press a small panel in the wall. A stone door slides open." CR>
        <SET-VAR <INST CRYPT STONE-DOOR> IS-LOCKED 0>
        <MOVE PLAYER CAVERN-1>
    )>
>

<OPEN STONE-DOOR ()
    <COND (
        <IS-EQUAL C-ROOM CRYPT>
        <TELL "You press a small panel in the wall. The door slides open." CR>
        <SET-VAR <INST CRYPT STONE-DOOR> IS-LOCKED 0>
        <MOVE PLAYER CAVERN-1>
    )(
        <IS-IN PLAYER MASTER-KEY>
        <TELL "The key fits into a crevice of the door. It slides open." CR>
        <SET-VAR <INST CRYPT STONE-DOOR> IS-LOCKED 0>
        <MOVE PLAYER CRYPT>
    )>
>

<HIT STONE-DOOR (DMG)
    <COND (
        <IS-EQUAL STONE-DOOR <CMD 2>>
        ;"do nothing"
        <RETURN 0>
    )(
        <IS-EQUAL <GET-VAR <CMD 2> MAX-DAMAGE> 0>
        ;"can only hit with a weapon"
        <RETURN 0>
    )(
        <IS-DES 1 <GET-VAR <CMD 1> HEALTH>>
        <TELL "It's already cracked." CR>
        <RETURN 0>
    )>

    <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
    <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>         
    <SET-VAR DMG <ROLL-DMG>>

    <COND (
        <IS-EQUAL DMG 0>
        <TELL "The door takes no damage" CR>
        <RETURN 0>
    )>

    <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
    <COND (
        <NOT <IS-ASC 0 <GET-VAR <CMD 1> HEALTH>>>
        <TELL "You manage to crack the rock, breaking the door." CR>
        <MOVE PLAYER CRYPT>
    )>
>

<ROUTINE DESC-CURSED-SKULL ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "The ancient king this skull belongs to was known to be jealous. Taking it can't be good." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "a SKULL">
    )>
>

<ROUTINE CURSED-SKULL-IN-PLAYER (AF)
    <EACH-OBJ PLAYER (OBJ)
        <COND (
            <IS-DES <GET-VAR OBJ MAX-DAMAGE> 0>
            <SET-VAR OBJ DAMAGE 0>
            <SET-VAR AF 1>
        )>
    >
    <EACH-OBJ C-ROOM (OBJ)
        <COND (
            <IS-DES <GET-VAR OBJ MAX-DAMAGE> 0>
            <SET-VAR OBJ DAMAGE 0>
            <SET-VAR AF 1>
        )>
    >

    <COND (
        <IS-EQUAL AF 1>
        <TELL "The skull makes all your weapons go dull." CR>
    )>
>

<ROUTINE CURSED-SKULL-EXIT-PLAYER () 
    ;"sword turns to dust if not wearing the magic ring"
    <COND (
        <AND 
            <IS-IN PLAYER SWORD>
            <IS-EQUAL <GET-VAR PLAYER HAS-MAGIC-RING> 0>
        >
        <TELL "The sword crumbles from your hand. You're left clenching an empty fist." CR>
        <MOVE <INST PLAYER SWORD>>
    )>
>

<TAKE SWORD ()
    <COND (
        <IS-EQUAL <GET-VAR PLAYER HAS-MAGIC-RING> 1>
        <TELL "You pick up the sword. It hums with power." CR>
        <MOVE <CMD 1> PLAYER>
    )(
        <IS-IN PLAYER CURSED-SKULL>
        <TELL "You pick up the sword. It seems to recognize it's owner." CR>
        <MOVE <CMD 1> PLAYER>
    )(
        <IS-EQUAL 1 1>
        <TELL "As you pick the sword up, it turns to dust in your hands." CR>
        <MOVE <CMD 1>>
    )>
>

<ROUTINE DESC-WEAPON-SHARPNESS ()
    <COND (
        <IS-IN PLAYER <CMD 0>>
        <TELL " Damage: " <GET-VAR <CMD 0> DAMAGE> ", Max: " <GET-VAR <CMD 0> MAX-DAMAGE> CR>
    )>
>

<ROUTINE WATERFALL-PASSAGE-UP ()
    <COND (
        <IS-IN <INST PLAYER PICK-AXE> STRAP>
        <TELL "Using the improvised grappling hook, you climb up the waterfall." CR>
        <MOVE PLAYER CLIFF-1>
    )(
        <IS-EQUAL 1 1>
        <TELL "You're behind a waterfall, with no way to climb up. Perhaps you could jury-rig some sort of grappling hook?" CR>
    )>
>

<WORK BONES ()
    <COND (
        <AND
            <IS-EQUAL BONES <CMD 1>>
            <IS-EQUAL OBSIDIAN-SHARD <CMD 2>>
        >
        <COND (
            <IS-DES <GET-VAR <CMD 2> DAMAGE> <DIVIDE <GET-VAR <CMD 2> MAX-DAMAGE> 2>>
            ;"obsidian is at least half sharp"
            <TELL "With obsidian's precision, you're able to carve out a master key." CR>
            <COPY-MOVE MASTER-KEY PLAYER>
            <MOVE <CMD 1>>
        )(
            <IS-EQUAL 1 1>
            <TELL "The obsidian is too dull" CR>
        )>
    )>
>

<HIT ROCK ()
    <COND (
        <IS-DES 5 <RAND>>
        <TELL "You found a lump of GOLD." CR>
        <COPY-MOVE GOLD-LUMP C-ROOM>
    )>
>

<ROUTINE DESC-STONE-DOOR ()
    <COND (
        <AND 
            <IS-EQUAL DETAILED-DESC 1>
            <IS-EQUAL <GET-VAR PLAYER T-BAT> 2>
        >
        <TELL "This heavy door is carved with jagged runes. It tells of an old king, jealous of all those around him.
        He died unfulfilled." CR>
    )(
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "This heavy door is carved with runes. It tells of an old king." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "a stone DOOR">
    )>
>

<ROUTINE DESC-BONES ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "These bones could be human, but some of them look larger.
        You could use one of the chips as a needle, to help you weave some straps (WORK).
        You could make a nice bone broth. Or, with a sharp enough tool, whittle a key." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "some BONES">
    )>
>

<ROUTINE PASSAGE-1-ENTER ()
    <SET-VAR WIN-ENTER-CAVE 1>
>

<ROUTINE WATERFALL-PASSAGE-ENTER ()
    <SET-VAR WIN-ENTER-CAVE 1>
>

<ROUTINE DESC-CAVE-LAKE ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "You're on the shore of a large underground lake. The ceiling is low.
        Water drips from stalactites. Some hang down into the surface of the water,
        forming pillars with their reflection." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "You're on the shore of an underground lake." CR>
    )>
>

<ROUTINE DESC-CAVERN-1 ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "You're in a massive cavern, partially natural, partially carved by human hand. It must have taken
        a lot of work. Is that the ceiling? Or is it just darkness? You voice echoes indeterminately." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "You're in a massive cavern." CR>
    )>
>

<ROUTINE DESC-CRYPT ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "You're in a crypt with a stone coffin in the center. The walls are plain. Significant effort
        must have gone into making them so." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "You're in a crypt." CR>
    )>
>

<ROUTINE DESC-WATERFALL-PASSAGE ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "You're in a narrow passage. To the west, a crypt. To the east, a wall of water.
        You must be behind a waterfall. There's no way UP unless you can jury-rig some climbing
        equipment. Have you found anything hook-like down here?" CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "You're in a narrow passage." CR>
    )>
>

<ROUTINE CAVE-LAKE-ALWAYS ()
    <COND (
        <NOT <IS-IN C-ROOM WATER>>
        ;"the lake never dries up (but it might temporarily while the game is processing hooks)"
        <COPY-MOVE WATER C-ROOM>
    )>
>

<HIT CURSED-SKULL ()
    <TELL "The skull splits into three pieces, and spirit rises up." CR>
    <SET-VAR PLAYER HEALTH 0>
>