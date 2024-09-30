<GLOBAL FIRST-SOUP 1>

<GLOBAL FIRE-IN-DRAFTY-CABIN 0>

<ROOM CABIN
      (DESC <DESC-CABIN>)
      (VARS ABOVE-GROUND 1)
      (NORTH TO CABIN-EXTERIOR)
      (EAST TO CABIN-EXTERIOR)
      (SOUTH TO CABIN-EXTERIOR)
      (WEST TO CABIN-EXTERIOR)
      (OUT TO CABIN-EXTERIOR)
      (ACT-ENTER <CABIN-ENTER>)>

<ROOM CABIN-EXTERIOR
      (DESC <DESC-CABIN-EXTERIOR>)
      (VARS ABOVE-GROUND 1 FIRST-TIME 1)
      (NORTH TO FIELD-2)
      (EAST TO FIELD-1)
      (SOUTH TO FOREST-2)
      (WEST TO FOREST-3)
      (ACT-ENTER <CABIN-EXTERIOR-ENTER>)>

<OBJECT CABIN-DOOR
      (AKA DOOR CABIN-DOOR)
      (DESC "the cabin DOOR")
      (COPY <ROOM CABIN-EXTERIOR>)
      (VARS NO-TAKE 1 HEALTH 6 IS-LOCKED 1 HAS-BOARDS 2 IS-SOFT 1)>

<OBJECT CABIN-WINDOW
      (AKA WINDOW)
      (DESC <DESC-CABIN-WINDOW>)
      (VARS NO-TAKE 1 IS-BROKEN 0)
      (COPY <ROOM CABIN-EXTERIOR>)>

<OBJECT CABIN-DOOR-KEY
      (AKA KEY DOOR-KEY CABIN-KEY)
      (DESC "a KEY")
      (COPY <DETRITUS 1>)>

<OBJECT BOOK
    (AKA BOOK)
    (COPY <ROOM CABIN>)
    (VARS HEALTH 2 TINDER 1 IS-SOFT 1)
    (DESC <DESC-BOOK>)>

;"can turn into a note"
<OBJECT BOOK-PAGE
    (AKA PAPER PAGE BOOK-PAGE)
    (COPY <BOOK 1>)
    (DESC <DESC-BOOK-PAGE>)>

<OBJECT NOTE
    (AKA MESSAGE NOTE)
    (DESC <DESC-NOTE>)>

<OBJECT TABLE
    (AKA TABLE)
    (COPY <ROOM CABIN>)
    (DESC "a wooden TABLE")
    (VARS HEALTH 4 HAS-BOARDS 2 IS-SOFT 1)>

<OBJECT CHAIR
    (AKA CHAIR)
    (COPY <ROOM CABIN>
          <ROOM CABIN>)
    (DESC "a wooden CHAIR")
    (VARS HEALTH 6 HAS-BOARDS 1 IS-SOFT 1)>

<OBJECT BED-FRAME
    (AKA BED BED-FRAME)
    (COPY <ROOM CABIN>)
    (DESC "a wooden BED frame")
    (VARS HEALTH 6 HAS-BOARDS 2 IS-SOFT 1)>

<OBJECT BUCKET
    (AKA BUCKET)
    (COPY <ROOM CABIN>)
    (VARS IS-HARD 1)
    (DESC "a BUCKET")>

<OBJECT NAILS
    (AKA NAILS)
    (COPY <ROOM CABIN>)
    (DESC "some NAILS")>

<OBJECT SOUP
    (AKA SOUP)
    (DESC "SOUP")
    (VARS IS-EDIBLE 1)>

<OBJECT TEA
    (AKA TEA)
    (DESC "TEA")
    (VARS IS-EDIBLE 1)>

<OBJECT FIRE
    (AKA FIRE)
    (DESC <DESC-FIRE>)
    (VARS FUEL 3 NO-TAKE 1)>

<OBJECT CHARCOAL
    (AKA COAL CHARCOAL)
    (DESC "some CHARCOAL")>

<ROUTINE DESC-CABIN ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "You're inside a rustic log cabin. It's well built. There's a window looking out over the fields.
        The furnishings are spartan, but it's clearly been lived in. The smell of smoke lingers in the air." CR> 
    )(
        <IS-ASC WIN-ENTER-CABIN 1>
        <TELL "You're inside a log cabin. There's signs of a previous occupant.
        It's safe to make a fire in here." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "You're inside a log cabin." CR>
    )>
>

<ROUTINE DESC-CABIN-EXTERIOR ()
      <COND (
            <IS-EQUAL <GET-VAR CABIN-EXTERIOR FIRST-TIME> 1>
            <SET-VAR CABIN-EXTERIOR FIRST-TIME 0>
            <TELL "You're outside a cabin. You could try to OPEN DOOR" CR>
            <RETURN 1>
      )>
      <TELL "You're outside a cabin on the edge of a forest, overlooking some fields." CR>
      <COND (
            <IS-EQUAL <GET-VAR <INST CABIN-EXTERIOR CABIN-DOOR> IS-LOCKED> 1>
            <TELL "The cabin door is locked. Where's the key? It's probably hidden under some leaves." CR>
      )>
>

<ENTER () 
    <COND (
        <IS-IN CABIN-EXTERIOR PLAYER>
        <COND (
            <IS-EQUAL <GET-VAR <INST CABIN-EXTERIOR CABIN-DOOR> IS-LOCKED> 0>
            <MOVE PLAYER CABIN>
        )(
            <IS-EQUAL <GET-VAR <INST CABIN-EXTERIOR CABIN-WINDOW> IS-BROKEN> 1>
            <MOVE PLAYER CABIN>
        )(
            <IS-EQUAL 1 1>
            <TELL "The door is locked. Where's the key?" CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "You can't" CR>
    )>
>

<EXIT ()
    <COND (
        <IS-IN CABIN PLAYER>
        <SET-VAR <INST CABIN-EXTERIOR CABIN-DOOR> IS-LOCKED 0>
        <MOVE PLAYER CABIN-EXTERIOR>
    )(
        <IS-EQUAL 1 1>
        <TELL "You're not in the cabin" CR>
    )>
>

<OPEN ()
    <COND (
        <AND
            <IS-EQUAL C-ROOM CABIN>
            <IS-EQUAL <CMD 1> <CMD 2>>
        >
        <TELL "Assuming you typed OPEN DOOR, try 'GO ...' instead" CR>
        <COND (
            <IS-DES 10 <RAND>>
            <TELL "You see, an instance can only exist in one room at a time. That leaves us with two options; A: one 
            CABIN-DOOR in CABIN and a second CABIN-DOOR in CABIN-EXTERIOR; and B: one CABIN-DOOR that follows the player
            back-and-forth between CABIN and CABIN-EXTERIOR (this is what CABIN-WINDOW does). Option A is annoying if
            the player HITs a door. Option B is annoying because the instance could get lost. Having already spent too
            much time on this game, I chose option C." CR>
        )>
    )>
>

<OPEN CABIN-DOOR ()
    <COND (
        <IS-EQUAL C-ROOM CABIN>
        <SET-VAR <CMD 1> IS-LOCKED 0>
        <MOVE PLAYER CABIN-EXTERIOR>
    )(
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
            ;"can only hit with a weapon"
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
            <TELL "It's broken into some rough BOARDs" CR>
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
        <NOT <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>>
        <TELL "You can only work on this door with a weapon." CR>
        <RETURN 0>
    )(
        <NOT <IS-ASC <GET-VAR <CMD 1> HAS-BOARDS> 3>>
        <TELL "Already as repaired as it's going to be." CR>
        <RETURN 0>
    )>
    <COND (
        <AND
            <OR <IS-IN PLAYER ROUGH-BOARD> <IS-IN C-ROOM ROUGH-BOARD>>
            <OR <IS-IN PLAYER NAILS> <IS-IN C-ROOM NAILS>>
        >
        ;"can work with a tool, increasing health by dmg"
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>
        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "Nothing happens to the door" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <ADD <GET-VAR <CMD 1> HEALTH> DMG>>
        <SET-VAR <CMD 1> HAS-BOARDS <ADD <GET-VAR <CMD 1> HAS-BOARDS> 1>>
        <COND (
            <IS-IN PLAYER ROUGH-BOARD>
            <MOVE <INST PLAYER ROUGH-BOARD>>
        )(
            <IS-EQUAL 1 1>
            <MOVE <INST C-ROOM ROUGH-BOARD>>
        )>
        <TELL "You repair the door with a rough board for " DMG " health." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "You need a rough board and nails in order to repair this door." CR>
    )>
>

<WORK CABIN-WINDOW (DMG)
    <COND (
        <NOT <IS-EQUAL <CMD 1> CABIN-WINDOW>>
        <RETURN 0>
    )(
        <NOT <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>>
        <TELL "You can only work on this window with a weapon." CR>
        <RETURN 0>
    )(
        <NOT <IS-EQUAL <GET-VAR <CMD 1> IS-BROKEN> 1>>
        <TELL "Already as repaired as it's going to be." CR>
        <RETURN 0>
    )>
    <COND (
        <AND
            <OR <IS-IN PLAYER ROUGH-BOARD> <IS-IN C-ROOM ROUGH-BOARD>>
            <OR <IS-IN PLAYER NAILS> <IS-IN C-ROOM NAILS>>
        >
        <COND (
            <IS-IN PLAYER ROUGH-BOARD>
            <MOVE <INST PLAYER ROUGH-BOARD>>
        )(
            <IS-EQUAL 1 1>
            <MOVE <INST C-ROOM ROUGH-BOARD>>
        )>
        <TELL "You repair the window with a rough board" CR>
        <SET-VAR <CMD 1> IS-BROKEN 0>
    )(
        <IS-EQUAL 1 1>
        <TELL "You need a rough board and nails in order to repair this window." CR>
    )>
>

<ROUTINE DESC-BOOK () 
    <COND (
        <IS-ASC <GET-VAR <CMD 0> HEALTH> 1>
        <TELL "a torn-up book">
        <RETURN 1>
    )(
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "The delicate pages of this leather-bound journal are covered in
        scratchy handwriting, probably done with charcoal. You can make out a
        few passages:" CR>
        <TELL "...boat is coming along, should be ready..." CR>
        <TELL "...lost the gem, a crow must have..." CR>
        <TELL "...beast is hungry, what does it eat down there? No use mining any more..." CR>
        <RETURN 1>
    )>
    <TELL "a BOOK">
>

<EMPTY BOOK ()
    <COND (
        <IS-IN <CMD 1> BOOK-PAGE>
        <MOVE <INST <CMD 1> BOOK-PAGE> PLAYER>
        <TELL "You've ripped out some blank pages from the book." CR>
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
            ;"can only hit with a weapon"
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
            <TELL "It's broken into some rough BOARDs" CR>
            <MOVE <CMD 1>>
        )>
    )>
>

<WORK TABLE (DMG)
    <COND (
        ;"can work with a tool, increasing health by dmg"
        <AND
            <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
            <OR <IS-IN PLAYER ROUGH-BOARD> <IS-IN C-ROOM ROUGH-BOARD>>
            <OR <IS-IN PLAYER NAILS> <IS-IN C-ROOM NAILS>>
            <IS-ASC <GET-VAR <CMD 1> HAS-BOARDS> 2>
        >
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>

        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "Nothing happens to the table" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <ADD <GET-VAR <CMD 1> HEALTH> DMG>>
        <SET-VAR <CMD 1> HAS-BOARDS <ADD <GET-VAR <CMD 1> HAS-BOARDS> 1>>
        <COND (
            <IS-IN PLAYER ROUGH-BOARD>
            <MOVE <INST PLAYER ROUGH-BOARD>>
        )(
            <IS-EQUAL 1 1>
            <MOVE <INST C-ROOM ROUGH-BOARD>>
        )>
        <TELL "You repair the table with a rough board for " DMG " health." CR>
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
            ;"can only hit with a weapon"
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
            <TELL "It's broken into a rough BOARD" CR>
            <MOVE <CMD 1>>
        )>
    )>
>

<WORK CHAIR (DMG)
    <COND (
        ;"can work with a tool, increasing health by dmg"
        <AND
            <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
            <OR <IS-IN PLAYER ROUGH-BOARD> <IS-IN C-ROOM ROUGH-BOARD>>
            <OR <IS-IN PLAYER NAILS> <IS-IN C-ROOM NAILS>>
            <IS-ASC <GET-VAR <CMD 1> HAS-BOARDS> 1>
        >
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>
        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "Nothing happens to the chair" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <ADD <GET-VAR <CMD 1> HEALTH> DMG>>
        <SET-VAR <CMD 1> HAS-BOARDS <ADD <GET-VAR <CMD 1> HAS-BOARDS> 1>>
                <COND (
            <IS-IN PLAYER ROUGH-BOARD>
            <MOVE <INST PLAYER ROUGH-BOARD>>
        )(
            <IS-EQUAL 1 1>
            <MOVE <INST C-ROOM ROUGH-BOARD>>
        )>
        <TELL "You repair the chair with a rough board for " DMG " health." CR>
    )>
>

;"identical to table"
<HIT BED-FRAME (DMG)
    <COND (
        <NOT <IS-EQUAL <CMD 1> BED-FRAME>>
        <RETURN 0>
    )>
    <COND (
        <IS-DES <GET-VAR <CMD 1> HEALTH> 0>

        <COND (
            ;"weapons can deal damage to beds"
            <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
            <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
            <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>
        )(
            ;"can only hit with a weapon"
            <IS-EQUAL 1 1>
            <RETURN 0>
        )>
                
        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "The bed takes no damage" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
        <TELL "You hit the bed for " DMG " damage." CR>

        <COND (
            ;"if dead, drop some rough-boards"
            <IS-ASC <GET-VAR <CMD 1> HEALTH> 1>
            <EACH-VAL <GET-VAR <CMD 1> HAS-BOARDS> (V)
                <COPY-MOVE ROUGH-BOARD C-ROOM>
            >
            <SET-VAR <CMD 1> HAS-BOARDS 0>
            <TELL "It's broken into some rough BOARDs" CR>
        )>
    )>
>

<WORK BED-FRAME (DMG)
    <COND (
        ;"can work with a tool, increasing health by dmg"
        <AND
            <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
            <OR <IS-IN PLAYER ROUGH-BOARD> <IS-IN C-ROOM ROUGH-BOARD>>
            <OR <IS-IN PLAYER NAILS> <IS-IN C-ROOM NAILS>>
            <IS-ASC <GET-VAR <CMD 1> HAS-BOARDS> 2>
        >
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>

        <SET-VAR DMG <ROLL-DMG>>
        <COND (
            <IS-EQUAL DMG 0>
            <TELL "Nothing happens to the bed" CR>
            <RETURN 1>
        )>

        <SET-VAR <CMD 1> HEALTH <ADD <GET-VAR <CMD 1> HEALTH> DMG>>
        <SET-VAR <CMD 1> HAS-BOARDS <ADD <GET-VAR <CMD 1> HAS-BOARDS> 1>>
        <COND (
            <IS-IN PLAYER ROUGH-BOARD>
            <MOVE <INST PLAYER ROUGH-BOARD>>
        )(
            <IS-EQUAL 1 1>
            <MOVE <INST C-ROOM ROUGH-BOARD>>
        )>
        <TELL "You repair the bed with a rough board for " DMG " health." CR>
    )>
>

<HIT CABIN-WINDOW () 
    <COND (
        <NOT <IS-EQUAL <GET-VAR <CMD 1> IS-BROKEN> 1>>
        <TELL "You smash the window, taking 1 damage in the process." CR>
        <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 1>>
        <SET-VAR <CMD 1> IS-BROKEN 1>
        <SET-VAR <INST CABIN-EXTERIOR CABIN-DOOR> IS-LOCKED 0>
        <MOVE PLAYER CABIN>
    )>
>

<ROUTINE DESC-CABIN-WINDOW ()
    <COND (
        <IS-EQUAL <GET-VAR <CMD 0> IS-BROKEN> 1>
        <TELL "a broken WINDOW" CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "a WINDOW" CR>
    )>
>

<ROUTINE DESC-AXE ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "Your trusty axe. If you find one, you might be able to HIT TREE WITH AXE, get
        some logs to keep the fire going. But be careful, tools dull with use." CR> 
        <DESC-WEAPON-SHARPNESS> 
    )(
        <IS-EQUAL 1 1>
        <TELL "an AXE">
    )>
>

<ROUTINE DESC-KNIFE ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "A long knife. When in danger, you can HIT <thing> WITH KNIFE. But be careful, weapons
        dull with use." CR>
        <DESC-WEAPON-SHARPNESS> 
    )(
        <IS-EQUAL 1 1>
        <TELL "a KNIFE">
    )>
>

<ROUTINE DESC-FLINT ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "Can try to SPARK FLINT AT <something> to start a fire. The fire won't
        last very long without more substantial fuel though." CR> 
    )(
        <IS-EQUAL 1 1>
        <TELL "FLINT">
    )>
>

<ROUTINE DESC-CLOAK ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "This cloak keeps you warm, and is very comfortable for sleeping." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "a CLOAK">
    )>
>

<ROUTINE DESC-KETTLE ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "You've cooked many a meal in this old, dented kettle. Start with ADD WATER TO KETTLE, then something edible." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "a KETTLE">
    )>
>

<ROUTINE CABIN-ENTER ()
    <SET-VAR WIN-ENTER-CABIN 1>
    <MOVE <INST CABIN-EXTERIOR CABIN-WINDOW> C-ROOM>
>

<SPARK FLINT ()
    <COND (
        <NOT <IS-EQUAL FLINT <CMD 1>>>
        ;"nothing happens"
        <RETURN 0>
    )>

    <COND (
        <OR
            <IS-EQUAL STICK <CMD 2>>
            <IS-EQUAL DETRITUS <CMD 2>>
            <IS-EQUAL SAP <CMD 2>>
            <IS-EQUAL BULRUSH <CMD 2>>
            <IS-EQUAL BOOK <CMD 2>>
            <IS-EQUAL BOOK-PAGE <CMD 2>>
            <IS-EQUAL NOTE <CMD 2>>
            <IS-EQUAL STRAP <CMD 2>>
        >
        <TELL "It catches. Keep the fire alive by ADDing some logs TO it." CR>
        <MOVE <CMD 2>>
        <COPY-MOVE FIRE C-ROOM>
        <EACH-OBJ C-ROOM (OBJ)
            <COND (
                <IS-EQUAL CHARCOAL OBJ>
                <MOVE OBJ>
            )>
        >
        <COND (
            <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 0>
            <TELL "You take 1 damage from smoke" CR>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 1>>
        )(
            <AND
                <IS-EQUAL C-ROOM CABIN>
                <OR
                    <IS-DES 1 <GET-VAR <INST CABIN-EXTERIOR CABIN-DOOR> HEALTH>>
                    <AND
                        ;"the cabin window moves around, so need to check both places"
                        <IS-DES 1 <GET-VAR <INST CABIN CABIN-WINDOW> HEALTH>>
                        <IS-DES 1 <GET-VAR <INST CABIN-EXTERIOR CABIN-WINDOW> HEALTH>>
                    >
                >
            > 
            <SET-VAR FIRE-IN-DRAFTY-CABIN 1>
        )>
    )(
        <IS-EQUAL TORCH <CMD 2>>
        <TELL "It catches, but torches burn up pretty quickly. You won't have light for long." CR>
        <SET-VAR <CMD 2> IS-LIT 1>
    )(
        <IS-EQUAL <CMD 2> <CMD 3>>
        <TELL "Nope." CR>
    )>
>

<ROUTINE DESC-NOTE ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "I'm not sure what you wrote down, hopefully something meaningful. You could leave it
        in the cabin for the next person who comes through here." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "a NOTE">
    )>
>

<ROUTINE DESC-BOOK-PAGE ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "You could try to WRITE NOTE with this paper." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "some PAPER">
    )>
>

<ROUTINE DESC-CUP ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "This cup can hold tea, and tea is just soup, but it's known to provide
        strength beyond that of it's contents." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "a CUP">
    )>
>

<ROUTINE DESC-TORCH ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <COND (
            <IS-EQUAL <GET-VAR <CMD 0> IS-LIT> 1>
            <TELL "This torch has " <GET-VAR <CMD 0> FUEL> " fuel, and is burning." CR>
        )(
            <IS-EQUAL 1 1>
            <TELL "This unlit torch has " <GET-VAR <CMD 0> FUEL> " fuel." CR>
        )>
    )(
        <IS-EQUAL <GET-VAR <CMD 0> IS-LIT> 1>
        <TELL "a TORCH" CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "an unlit TORCH" CR>
    )>
>

<ROUTINE DESC-FIRE ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "This fire has " <GET-VAR <CMD 0> FUEL> " fuel left." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "a FIRE" CR>
    )>
>