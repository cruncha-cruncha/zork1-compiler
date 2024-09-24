<GLOBAL FOREST-BURNED-DOWN 0>

<ROOM FOREST-1
      (DESC <DESC-FOREST-1>)
      (VARS FIRST-TIME 1 ABOVE-GROUND 1)
      (NORTH TO FOREST-2)
      (EAST TO FOREST-2)
      (WEST TO CAVE-ENTRANCE-1)>

<ROOM FOREST-2
      (DESC "You're in a forest, the trees are thinner here. There's a trail heading NORTH (and back SOUTH)." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO CABIN-EXTERIOR)
      (SOUTH TO FOREST-1)>

<ROOM FOREST-3
      (DESC "You're in a lightly-populated forest, and a small stream burbles nearby." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO FOREST-4)
      (EAST TO CABIN-EXTERIOR)
      (SOUTH TO FOREST-2)
      (WEST TO FOREST-5)>

<ROOM FOREST-4
      (DESC "The forest is thicker again here, but the trail still looks good, and the you can hear a stream." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO CAVE-ENTRANCE-2)
      (EAST TO FIELD-2)
      (SOUTH TO FOREST-3)
      (WEST TO FOREST-5)>

<ROOM FOREST-5
      (DESC "You're in the forest. The trail is faint here, and barely-worn." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO LAKE-1)
      (EAST TO CAVE-ENTRANCE-1)
      (SOUTH TO CAVE-ENTRANCE-1)
      (WEST "Think again")>

<ROOM FOREST-6
      (DESC "This is deep forest, but there is trail running SOUTH and EAST" CR)
      (VARS ABOVE-GROUND 1)
      (NORTH "The forest gets thicker, closing in authoritatively. No chance of getting through.")
      (EAST TO CAVE-ENTRANCE-2)
      (SOUTH TO LAKE-1)
      (WEST "Not a chance.")>

<ROOM LAKE-1
      (DESC "You find yourself at the edge a large, calm lake" CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO FOREST-6)
      (EAST TO FOREST-4)
      (SOUTH TO FOREST-5)
      (WEST TO FOREST-5)>

<ROOM FIELD-1
      (DESC "A massive field stretches out all around you, with wild grass and sage.")
      (VARS ABOVE-GROUND 1)
      (NORTH TO FIELD-2)
      (EAST TO CLIFF-1)
      (SOUTH TO FOREST-1)
      (WEST TO CABIN-EXTERIOR)>

<ROOM FIELD-2
      (DESC "This field seems to go on forever, gently rolling past the horizon." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH "Nothing but endless field lies north. Best kept for another time.")
      (EAST TO CLIFF-1)
      (SOUTH TO CABIN-EXTERIOR)
      (WEST TO FOREST-4)>

<ROOM CLIFF-1
      (DESC "Grassy fields abruptly stop at the edge of a cliff, a stream hurtling over. You can taste more adventure awaiting in the land below.")
      (VARS ABOVE-GROUND 1)
      (NORTH TO FIELD-2)
      (EAST "You're not yet prepared for climbing down this sheer drop.")
      (SOUTH TO FIELD-1)
      (WEST TO FIELD-1)>

<ROOM CAVE-ENTRANCE-1
      (DESC "Amidst boulders and tree trunks a black space beckons you forward. A cave. You could GO DOWN, but there is no telling whether you would come back up. Some sort of light source might help." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH PER <CAVE-ENTRANCE-1-NORTH>)
      (EAST PER <CAVE-ENTRANCE-1-EAST>)
      (SOUTH PER <CAVE-ENTRANCE-1-SOUTH>)
      (WEST PER <CAVE-ENTRANCE-1-WEST>)
      (DOWN TO LONG-HALL-1)>

<ROOM CAVE-ENTRANCE-2
      (DESC "Out of the forest appears a large, rocky hole in the ground. A cave. You could GO DOWN. Some sort of light source might help you navigate.")
      (NORTH "There's nothing but dense forest")
      (EAST PER <CAVE-ENTRANCE-2-EAST>)
      (SOUTH PER <CAVE-ENTRANCE-2-SOUTH>)
      (WEST PER <CAVE-ENTRANCE-2-WEST>)
      (DOWN TO PASSAGE-1)
      (VARS ABOVE-GROUND 1)>

<OBJECT BOAT-FRAME>
<OBJECT BOAT>

<OBJECT TREE
      (AKA TREE)
      (DESC "a TREE")
      (VARS HEALTH 20)
      ;(ACT-PRSO <PRSO-TREE>)>

<OBJECT LOG
      (AKA LOG LOGS)
      (DESC "a LOG")>

<OBJECT ROUGH-BOARD
      (AKA BOARD ROUGH-BOARD)
      (DESC "a rough wooden BOARD")>

<OBJECT DETRITUS
      (AKA BRUSH DETRITUS)
      (DESC "leafy DETRITUS")
      (COPY <ROOM CABIN-EXTERIOR>)
      (VARS TINDER 1)>

<OBJECT STICK
      (AKA STICK BRANCH)
      (DESC "a STICK")
      (VARS TINDER 1)>

<OBJECT TORCH 
      (AKA TORCH)
      (DESC "a makeshift TORCH")
      (VARS TINDER 1 LIT 1 FUEL 8)>

<OBJECT BULRUSH
      (AKA REED REEDS BULRUSH BULRUSHES CATTAIL CATTAILS TYPHA)
      (DESC "some REEDS")
      (VARS TINDER 1)>

<OBJECT STRAP
      (AKA STRAP)
      (DESC "a strong cloth-like STRAP")
      (VARS TINDER 1)>

<OBJECT SAP 
      (AKA SAP)
      (COPY <ROOM STORAGE>)
      (DESC "tree SAP")
      (VARS TINDER 1)>

<OBJECT BOILED-SAP
      (AKA BOILED-SAP)
      (DESC "BOILED-SAP")>

<OBJECT BERRIES
      (AKA BERRY BERRIES)
      (DESC "some BERRIES")
      (VARS EDIBLE 1)>

<OBJECT HERBS
      (AKA HERB HERBS)
      (DESC "some HERBS")
      (VARS EDIBLE 1)>
      
<OBJECT NUTS
      (AKA NUT NUTS)
      (DESC "some NUTS")
      (VARS EDIBLE 1)>

<OBJECT MUSHROOM
      (AKA MUSHROOM MUSHROOMS)
      (DESC "a MUSHROOM")
      (VARS EDIBLE 1)>

<OBJECT ROOTS
      (AKA ROOT ROOTS)
      (DESC "a ROOT")
      (VARS EDIBLE 1)>

<OBJECT FERNS
      (AKA FERN FERNS)
      (DESC "a FERN")
      (VARS EDIBLE 1)>

<OBJECT RIVER-STONE
      (AKA STONE WHETSTONE RIVER-STONE)
      (DESC <DESC-RIVER-STONE>)
      (COPY <ROOM FOREST-4>)
      (VARS IS-WET 0)
      ;(ACT-PRSI <PRSI-RIVER-STONE>)>

<OBJECT TREE-HOLLOW>
<OBJECT GEM>
<OBJECT OWL>
<OBJECT CROW>
<OBJECT FISH>
<OBJECT FROG>
<OBJECT BEETLE>
<OBJECT RABBIT>
<OBJECT SNAKE>
<OBJECT BEAR>

<ROUTINE DESC-FOREST-1 ()
      <COND (
            <IS-EQUAL <GET-VAR FOREST-1 FIRST-TIME> 1>
            <SET-VAR FOREST-1 FIRST-TIME 0>
            <TELL "It's the end of summer, and you're in a dense forest. Birds
            and other small creatures can be heard rustling, buzzing, and chirping
            through the undergrowth. The smell of pine hangs thick in the air. Light
            filters down through needles and leaves." CR>
            <WEATHER-REPORT>
            <TELL "There's a trail up ahead, it looks like you could GO NORTH" CR>
      )(
            <IS-EQUAL DETAILED-DESC 1>
            <TELL "You're in dense forest. The undergrowth is thick with dead
            branches, leaves, and other detritus. Vines, bushes, and brush all
            vie for a spot in the sunlight. Even during a thunderstorm, the wind
            barely penetrates this far down, leaving the air hot and thick with
            the smell of pine." CR>
            <TELL "Despite the foliage, there's a well-defined trail heading
            NORTH and WEST" CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're in a dense forest. There's a trail heading NORTH and WEST." CR>
      )>
>

<ROUTINE DESC-RIVER-STONE ()
      <COND (
            <IS-EQUAL DETAILED-DESC 1>
            <TELL "A flat and smooth river stone. Perfect for sharpening weapons or tools when wet." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "a smooth STONE">
      )>
>

<HIT TREE (DMG)
      <COND (
            ;"if <CMD 2> has max-damage (is a weapon), hit. If tree is dead, get logs"
            <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
            <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
            <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>

            <SET-VAR DMG <ROLL-DMG>>
            <COND (
                  <IS-EQUAL DMG 0>
                  <TELL "The tree takes no damage" CR>
                  <RETURN 1>
            )>

            <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
            <TELL "The tree takes " DMG " damage" CR>

            <COND (
                  <IS-ASC <GET-VAR <CMD 1> HEALTH> 1>
                  ;"replace tree with four logs and two sticks"
                  <MOVE  <CMD 1>>
                  <COPY-MOVE LOG C-ROOM>
                  <COPY-MOVE LOG C-ROOM>
                  <COPY-MOVE LOG C-ROOM>
                  <COPY-MOVE LOG C-ROOM>
                  <COPY-MOVE STICK C-ROOM>
                  <COPY-MOVE STICK C-ROOM>
                  <TELL "You've chopped the tree into four LOGs and two STICKs" CR>
            )>
      )>
      <COND (
            ;"if the tree isn't dead and <CMD 2> has max-damage or max-health (is weapon or tool), get sap"
            <AND
                  <IS-DES <GET-VAR <CMD 1> HEALTH> 0>
                  <OR
                        <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
                        <IS-DES <GET-VAR <CMD 2> MAX-HEALTH> 0>
                  >
            >
            <COPY-MOVE SAP PLAYER>
            <TELL "You pick up some tree SAP" CR>
      )>
>

<ROUTINE CAVE-ENTRANCE-1-NORTH ()
      <COND (
            <IS-EQUAL FOREST-BURNED-DOWN 1>
            <TELL "Charred debris blocks your way." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE PLAYER FOREST-5>
      )>
>

<ROUTINE CAVE-ENTRANCE-1-EAST ()
      <COND (
            <IS-EQUAL FOREST-BURNED-DOWN 1>
            <TELL "Charred debris blocks your way." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE PLAYER FOREST-1>
      )>
>

<ROUTINE CAVE-ENTRANCE-1-SOUTH ()
      <COND (
            <IS-EQUAL FOREST-BURNED-DOWN 1>
            <TELL "Charred debris blocks your way." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE PLAYER FOREST-1>
      )>
>

<ROUTINE CAVE-ENTRANCE-1-WEST ()
      <COND (
            <IS-EQUAL FOREST-BURNED-DOWN 1>
            <TELL "Charred debris blocks your way." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE PLAYER FOREST-5>
      )>
>

<ROUTINE CAVE-ENTRANCE-2-EAST ()
      <COND (
            <IS-EQUAL FOREST-BURNED-DOWN 1>
            <TELL "Charred debris blocks your way." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE PLAYER FOREST-4>
      )>
>

<ROUTINE CAVE-ENTRANCE-2-SOUTH ()
      <COND (
            <IS-EQUAL FOREST-BURNED-DOWN 1>
            <TELL "Charred debris blocks your way." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE PLAYER FOREST-4>
      )>
>

<ROUTINE CAVE-ENTRANCE-2-WEST ()
      <COND (
            <IS-EQUAL FOREST-BURNED-DOWN 1>
            <TELL "Charred debris blocks your way." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE PLAYER FOREST-6>
      )>
>

<ROUTINE PRSI-RIVER-STONE ()
      <COND (
            ;"can work tool/weapon to sharpen it"
            <AND
                  <IS-EQUAL CMD "WORK">
                  <OR
                        <IS-DES <GET-VAR <CMD 1> MAX-DAMAGE> 0>
                        <IS-DES <GET-VAR <CMD 1> MAX-HEALTH> 0>
                  >
            >

            ;"if stone is not wet, damages the item"
            <COND (
                  <IS-ASC <GET-VAR <CMD 2> WET> 1>
                  <TELL "The stone isn't wet, so it just dulls the edge." CR>
                  ;"TODO"
                  <RETURN 1>
            )>

            ;"cannot sharpen obsidian"
            ;"working removes wetness"
      )(
            ;"can add water to (or pour on) this stone to wet it"
            <AND
                  <OR
                        <IS-EQUAL CMD "ADD">
                        <IS-EQUAL CMD "POUR">
                  >
                  <IS-EQUAL <CMD 1> WATER>
            >
            <SET-VAR <CMD 2> WET 10>
            <MOVE  <CMD 1>>
            <TELL "The river stone is wet. You could WORK KNIFE WITH STONE to sharpen it." CR>
      )>
>