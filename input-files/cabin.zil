<GLOBAL FIRST-SOUP 1>

<ROOM CABIN
      (DESC "You're inside a log cabin. It's rustic, but has a lovely fireplace." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO CABIN-EXTERIOR)
      (EAST TO CABIN-EXTERIOR)
      (SOUTH TO CABIN-EXTERIOR)
      (WEST TO CABIN-EXTERIOR)
      (OUT TO CABIN-EXTERIOR)>

<ROOM CABIN-EXTERIOR
      (DESC <DESC-CABIN-EXTERIOR>)
      (VARS ABOVE-GROUND 1 FIRST-TIME 1)
      (NORTH TO FIELD-2)
      (EAST TO FIELD-1)
      (SOUTH TO FOREST-2)
      (WEST TO FOREST-3)>

<OBJECT CABIN-DOOR
      (AKA DOOR CABIN-DOOR)
      (DESC "the cabin DOOR")
      (COPY <ROOM CABIN-EXTERIOR>)
      ;(ACT-PRSO <PRSO-CABIN-DOOR>)
      (VARS NO-TAKE 1 HEALTH 8 IS-LOCKED 1 HAS-BOARDS 3)>

<OBJECT CABIN-WINDOW
      (AKA WINDOW)
      (DESC <DESC-CABIN-WINDOW>)
      (VARS NO-TAKE 1 IS-BROKEN 0)
      ;(ACT-PRSO <PRSO-CABIN-WINDOW>)
      (COPY <ROOM CABIN-EXTERIOR>)>

<OBJECT CABIN-DOOR-KEY
      (AKA KEY DOOR-KEY CABIN-KEY)
      (DESC "a KEY")
      (COPY <DETRITUS 1>)>

<OBJECT BOOK
    (AKA BOOK)
    (COPY <ROOM CABIN>)
    (VARS HEALTH 2 TINDER 1)
    ;(ACT-PRSO <PRSO-BOOK>)
    (DESC <DESC-BOOK>)>

;"can turn into a note"
<OBJECT BOOK-PAGE
    (AKA PAPER PAGE BOOK-PAGE)
    (COPY <BOOK 1>)
    (DESC "some PAPER")
    (VARS TINDER 1)>

<OBJECT NOTE
    (AKA NOTE)
    (DESC "a NOTE")
    (VARS TINDER 1)>

<OBJECT TABLE
    (AKA TABLE)
    (COPY <ROOM CABIN>)
    (DESC "a wooden TABLE")
    ;(ACT-PRSO <PRSO-TABLE>)
    (VARS HEALTH 4 HAS-BOARDS 2)>

<OBJECT CHAIR
    (AKA CHAIR)
    (COPY <ROOM CABIN>
          <ROOM CABIN>)
    (DESC "a wooden CHAIR")
    ;(ACT-PRSO <PRSO-CHAIR>)
    (VARS HEALTH 6 HAS-BOARDS 1)>

<OBJECT BED-FRAME
    (AKA BED BED-FRAME)
    (COPY <ROOM CABIN>)
    (DESC "a wooden BED frame")>

<OBJECT BUCKET
    (AKA BUCKET)
    (COPY <ROOM CABIN>)
    (DESC "a BUCKET")>

<OBJECT NAILS
    (AKA NAILS)
    (COPY <ROOM CABIN>)
    (DESC "some NAILS")>

<OBJECT GLASS-SHARD>

<OBJECT SOUP
    (AKA SOUP)
    (DESC "SOUP")
    (VARS EDIBLE 1)>

<OBJECT TEA
    (AKA TEA)
    (DESC "TEA")
    (VARS EDIBLE 1)>

<OBJECT FIRE
    (AKA FIRE)
    (DESC "a FIRE")
    (VARS FUEL 2 NO-TAKE 1)>

<OBJECT CHARCOAL> ;"special case where 1 + 1 = 1"

<ROUTINE DESC-CABIN-EXTERIOR ()
      <COND (
            <IS-EQUAL <GET-VAR CABIN-EXTERIOR FIRST-TIME> 1>
            <SET-VAR CABIN-EXTERIOR FIRST-TIME 0>
            <TELL "You're at the transition between a forest and a field.
            There are trails in all directions, with a cabin at the center.
            It's got a scenic window facing the fields. You could try to
            OPEN DOOR" CR>
            <RETURN 1>
      )>
      <TELL "You're outside a cabin on the edge of a forest, overlooking some fields." CR>
      <COND (
            <IS-EQUAL <GET-VAR <INST CABIN-EXTERIOR CABIN-DOOR> IS-LOCKED> 1>
            <TELL "The cabin door is locked. Where's the key? It's probably hidden under some leaves." CR>
      )>
>

<OPEN CABIN-DOOR ()
    <COND (
        <OR
            <IS-ASC <GET-VAR <CMD 1> HEALTH> 1>
            <IS-EQUAL <GET-VAR <CMD 1> IS-LOCKED> 0>
        >
        <MOVE PLAYER CABIN>
    )(
        <AND
            <OR 
                <IS-IN PLAYER CABIN-DOOR-KEY>
                <IS-IN PLAYER MASTER-KEY>
            >
            <IS-EQUAL <GET-VAR <CMD 1> IS-LOCKED> 1>
        >
        <SET-VAR <CMD 1> IS-LOCKED 0>
        <TELL "The key works." CR>
        <MOVE PLAYER CABIN>
    )(
        <IS-EQUAL 1 1>
        <TELL "The door is locked. You should LOOK AROUND for a key." CR> 
    )>
>

<HIT CABIN-DOOR (DMG) 
    <COND (
        <NOT <IS-EQUAL <CMD 1> CABIN-DOOR>>
        <RETURN 0>
    )>
    <COND (
        <IS-DES <GET-VAR <CMD 1> HEALTH> 0>

        <COND (
            ;"weapons deal damage with their damage"
            <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
            <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
            <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>
        )(
            ;"tools deal double damage to doors"
            <IS-DES <GET-VAR <CMD 2> MAX-HEALTH> 0>
            <SET-VAR COMBAT-DAMAGE <MULTIPLY <GET-VAR <CMD 2> HEALTH> 2>>
            <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-HEALTH>>
        )(
            ;"can only hit with a weapon or tool"
            <IS-EQUAL 1 1>
            <RETURN 0>
        )>
                
        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "The door takes no damage" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
        <TELL "You hit the door for " DMG " damage." CR>

        <COND (
            ;"if dead, drop some rough-boards"
            <IS-ASC <GET-VAR <CMD 1> HEALTH> 1>
            <EACH-VAL <GET-VAR <CMD 1> HAS-BOARDS> (V)
                <COPY-MOVE ROUGH-BOARD C-ROOM>
            >
            <SET-VAR <CMD 1> HAS-BOARDS 0>
            <TELL "It's broken into some ROUGH-BOARDs" CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "It's already broken." CR>
    )>
>

<WORK CABIN-DOOR (DMG)
    <COND (
        <NOT <IS-EQUAL <CMD 1> CABIN-DOOR>>
        <RETURN 0>
    )(
        <NOT <IS-DES <GET-VAR <CMD 2> MAX-HEALTH> 0>>
        <TELL "You can only work on this door with a tool." CR>
        <RETURN 0>
    )(
        <NOT <IS-ASC <GET-VAR <CMD 1> HAS-BOARDS> 3>>
        <TELL "Already as repaired as it's going to be." CR>
        <RETURN 0>
    )>
    <COND (
        ;"can work with a tool, increasing health by dmg"
        <AND
            <OR <IS-IN PLAYER ROUGH-BOARD> <IS-IN C-ROOM ROUGH-BOARD>>
            <OR <IS-IN PLAYER NAILS> <IS-IN C-ROOM NAILS>>
        >
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> HEALTH>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-HEALTH>>
        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "Nothing happens to the door" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <ADD <GET-VAR <CMD 1> HEALTH> DMG>>
        <SET-VAR <CMD 1> HAS-BOARDS <ADD <GET-VAR <CMD 1> HAS-BOARDS> 1>>
        <MOVE <INST PLAYER ROUGH-BOARD>>
        <TELL "You repair the door with a rough-board for " DMG " health." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "You need a rough-board and nails in order to repair this door." CR>
    )>
>

<ROUTINE DESC-BOOK () 
    <COND (
        <IS-ASC <GET-VAR <CMD 1> HEALTH> 1>
        <TELL "a torn-up book">
        <RETURN 1>
    )(
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "This leather-bound journal's yellowing pages are covered in
        scratchy handwriting, probably done with charcoal. You can make out a
        few passages:" CR>
        <TELL "...boat is coming along, should be ready in..." CR>
        <TELL "...beast is hungry, what does it eat down there? No use mining any more..." CR>
        <TELL "...lost the gem, a crow must have taken it..." CR>
        <RETURN 1>
    )>
    <TELL "a BOOK">
>

<EMPTY BOOK ()
    <COND (
        <IS-IN <CMD 1> BOOK-PAGE>
        <MOVE <INST <CMD 1> BOOK-PAGE> PLAYER>
        <TELL "You've ripped out some blank pages from the book" CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "The book is already empty." CR>
    )>
>

<HIT BOOK ()
    <COND (
        <NOT <IS-EQUAL <CMD 1> BOOK>>
        <RETURN 0>
    )>
    <COND (
        <IS-DES <GET-VAR <CMD 1> HEALTH> 0>
        <TELL "Why are you hitting this book?" CR>
        <COND (
            <IS-IN <CMD 1> BOOK-PAGE>
            <MOVE <INST <CMD 1> BOOK-PAGE> C-ROOM>
            <TELL "You've cut out some pages onto the floor." CR>
        )>
        <SET-VAR <CMD 1> HEALTH <ADD <GET-VAR <CMD 1> HEALTH> -1>>
    )(
        <IS-EQUAL 1 1>
        <TELL "The book is dead." CR>
        <COND (
            <IS-IN <CMD 1> BOOK-PAGE>
            <MOVE <INST <CMD 1> BOOK-PAGE>>
        )>
    )>
>

<HIT TABLE (DMG)
    <COND (
        <NOT <IS-EQUAL <CMD 1> TABLE>>
        <RETURN 0>
    )>
    <COND (
        <IS-DES <GET-VAR <CMD 1> HEALTH> 0>

        <COND (
            ;"weapons can deal damage to tables"
            <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
            <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
            <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>
        )(
            ;"tools deal double damage to tables"
            <IS-DES <GET-VAR <CMD 2> MAX-HEALTH> 0>
            <SET-VAR COMBAT-DAMAGE <MULTIPLY <GET-VAR <CMD 2> HEALTH> 2>>
            <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-HEALTH>>
        )(
            ;"can only hit with a weapon or tool"
            <IS-EQUAL 1 1>
            <RETURN 0>
        )>
                
        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "The table takes no damage" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
        <TELL "You hit the table for " DMG " damage." CR>

        <COND (
            ;"if dead, drop some rough-boards"
            <IS-ASC <GET-VAR <CMD 1> HEALTH> 1>
            <EACH-VAL <GET-VAR <CMD 1> HAS-BOARDS> (V)
                <COPY-MOVE ROUGH-BOARD C-ROOM>
            >
            <SET-VAR <CMD 1> HAS-BOARDS 0>
            <TELL "It's broken into some ROUGH-BOARDs" CR>
        )>
    )>
>

<WORK TABLE (DMG)
    <COND (
        ;"can work with a tool, increasing health by dmg"
        <AND
            <IS-DES <GET-VAR <CMD 2> MAX-HEALTH> 0>
            <OR <IS-IN PLAYER ROUGH-BOARD> <IS-IN C-ROOM ROUGH-BOARD>>
            <OR <IS-IN PLAYER NAILS> <IS-IN C-ROOM NAILS>>
            <IS-ASC <GET-VAR <CMD 1> HAS-BOARDS> 2>
        >
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> HEALTH>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-HEALTH>>

        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "Nothing happens to the table" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <ADD <GET-VAR <CMD 1> HEALTH> DMG>>
        <SET-VAR <CMD 1> HAS-BOARDS <ADD <GET-VAR <CMD 1> HAS-BOARDS> 1>>
        <MOVE <INST PLAYER ROUGH-BOARD>>
        <TELL "You repair the table with a rough-board for " DMG " health." CR>
    )>
>

;"identical to table, but a chair only has 1 rough-board"
<HIT CHAIR (DMG)
    <COND (
        <IS-DES <GET-VAR <CMD 1> HEALTH> 0>

        <COND (
            ;"weapons can deal damage to chairs"
            <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
            <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
            <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>
        )(
            ;"tools deal double damage to chairs"
            <IS-DES <GET-VAR <CMD 2> MAX-HEALTH> 0>
            <SET-VAR COMBAT-DAMAGE <MULTIPLY <GET-VAR <CMD 2> HEALTH> 2>>
            <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-HEALTH>>
        )(
            ;"can only hit with a weapon or tool"
            <IS-EQUAL 1 1>
            <RETURN 0>
        )>
                
        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "The chair takes no damage" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
        <TELL "You hit the chair for " DMG " damage." CR>

        <COND (
            ;"if dead, drop one rough-board"
            <IS-ASC <GET-VAR <CMD 1> HEALTH> 1>
            <SET-VAR <CMD 1> HAS-BOARDS 0>
            <COPY-MOVE ROUGH-BOARD C-ROOM>
            <TELL "It's broken into a ROUGH-BOARD" CR>
        )>
    )>
>

<WORK CHAIR (DMG)
    <COND (
        ;"can work with a tool, increasing health by dmg"
        <AND
            <IS-DES <GET-VAR <CMD 2> MAX-HEALTH> 0>
            <OR <IS-IN PLAYER ROUGH-BOARD> <IS-IN C-ROOM ROUGH-BOARD>>
            <OR <IS-IN PLAYER NAILS> <IS-IN C-ROOM NAILS>>
            <IS-ASC <GET-VAR <CMD 1> HAS-BOARDS> 1>
        >
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> HEALTH>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-HEALTH>>
        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "Nothing happens to the chair" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <ADD <GET-VAR <CMD 1> HEALTH> DMG>>
        <SET-VAR <CMD 1> HAS-BOARDS <ADD <GET-VAR <CMD 1> HAS-BOARDS> 1>>
        <MOVE <INST PLAYER ROUGH-BOARD>>
        <TELL "You repair the chair with a rough-board for " DMG " health." CR>
    )>
>

<HIT CABIN-WINDOW () 
    <COND (
        <NOT <IS-EQUAL <GET-VAR <CMD 1> IS-BROKEN> 1>>
        <TELL "You smash the window, taking 1 damage in the process." CR>
        <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 1>>
        <COPY-MOVE GLASS-SHARD CABIN>
        <MOVE PLAYER CABIN>
    )>
>

<ROUTINE DESC-CABIN-WINDOW ()
    <COND (
        <IS-EQUAL <GET-VAR <CMD 1> IS-BROKEN> 1>
        <TELL "a broken WINDOW" CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "a WINDOW" CR>
    )>
>
