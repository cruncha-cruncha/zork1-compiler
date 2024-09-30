<GLOBAL FOREST-BURNED-DOWN 0>

;"don't spawn animals until later"
<GLOBAL FIRST-FOREST-PATH 0>

<ROOM FOREST-1
      (DESC <DESC-FOREST-1>)
      (VARS FIRST-TIME 1 ABOVE-GROUND 1)
      (NORTH TO FOREST-2)
      (EAST TO FOREST-2)
      (WEST TO CAVE-ENTRANCE-1)
      (ACT-ENTER <FOREST-1-ENTER>)>

<ROOM FOREST-2
      (DESC <DESC-FOREST-2>)
      (VARS ABOVE-GROUND 1)
      (NORTH TO CABIN-EXTERIOR)
      (SOUTH TO FOREST-1)
      (ACT-ENTER <FOREST-2-ENTER>)>

<ROOM FOREST-3
      (DESC <DESC-FOREST-3>)
      (VARS ABOVE-GROUND 1)
      (NORTH TO FOREST-4)
      (EAST TO CABIN-EXTERIOR)
      (SOUTH TO FOREST-5)
      (WEST TO LAKE-1)
      (ACT-ENTER <FOREST-3-ENTER>)>

<ROOM FOREST-4
      (DESC <DESC-FOREST-4>)
      (VARS ABOVE-GROUND 1)
      (NORTH TO CAVE-ENTRANCE-2)
      (EAST TO FIELD-2)
      (SOUTH TO FOREST-3)
      (WEST TO FOREST-6)
      (ACT-ENTER <FOREST-4-ENTER>)>

<ROOM FOREST-5
      (DESC <DESC-FOREST-5>)
      (VARS ABOVE-GROUND 1)
      (NORTH TO LAKE-1)
      (EAST TO FOREST-2)
      (SOUTH TO CAVE-ENTRANCE-1)
      (WEST "Think again")
      (ACT-ENTER <FOREST-5-ENTER>)
      (ACT-EXIT <FOREST-5-EXIT>)>

<ROOM FOREST-6
      (DESC <DESC-FOREST-6>)
      (VARS ABOVE-GROUND 1)
      (NORTH "The forest gets thicker, closing in authoritatively. No chance of getting through.")
      (EAST TO CAVE-ENTRANCE-2)
      (SOUTH TO LAKE-1)
      (WEST "Not a chance.")
      (ACT-ENTER <FOREST-6-ENTER>)
      (ACT-EXIT <FOREST-6-EXIT>)>

<ROOM LAKE-1
      (DESC <DESC-LAKE-1>)
      (VARS ABOVE-GROUND 1)
      (NORTH TO FOREST-6)
      (EAST TO FOREST-3)
      (SOUTH TO FOREST-5)
      (WEST PER <LAKE-1-WEST>)
      (ACT-ENTER <LAKE-1-ENTER>)
      (ACT-ALWAYS <LAKE-1-ALWAYS>)>

<ROOM FIELD-1
      (DESC <DESC-FIELD-1>);"A massive field stretches out all around you."
      (VARS ABOVE-GROUND 1)
      (NORTH TO FIELD-2)
      (EAST TO CLIFF-1)
      (SOUTH TO FOREST-1)
      (WEST TO CABIN-EXTERIOR)
      (ACT-ENTER <FIELD-1-ENTER>)>

<ROOM FIELD-2
      (DESC <DESC-FIELD-2>);"This field seems to go on forever, gently rolling past the horizon."
      (VARS ABOVE-GROUND 1)
      (NORTH "Nothing but endless field lies north. Best kept for another time.")
      (EAST TO CLIFF-1)
      (SOUTH TO CABIN-EXTERIOR)
      (WEST TO FOREST-4)
      (ACT-ENTER <FIELD-2-ENTER>)>

<ROOM CLIFF-1
      (DESC <DESC-CLIFF-1>)
      (VARS ABOVE-GROUND 1)
      (EAST PER <CLIFF-1-EAST>)
      (SOUTH TO FIELD-1)
      (WEST TO FIELD-1)
      (DOWN PER <CLIFF-1-DOWN>)
      (ACT-ENTER <CLIFF-1-ENTER>)>

<ROOM CAVE-ENTRANCE-1
      (DESC <DESC-CAVE-ENTRANCE-1>)
      (VARS ABOVE-GROUND 1)
      (NORTH PER <CAVE-ENTRANCE-1-NORTH>)
      (EAST PER <CAVE-ENTRANCE-1-EAST>)
      (SOUTH PER <CAVE-ENTRANCE-1-SOUTH>)
      (WEST PER <CAVE-ENTRANCE-1-WEST>)
      (DOWN TO LONG-HALL-1)>

<ROOM CAVE-ENTRANCE-2
      (DESC <DESC-CAVE-ENTRANCE-2>)
      (NORTH "There's nothing but dense forest")
      (EAST PER <CAVE-ENTRANCE-2-EAST>)
      (SOUTH PER <CAVE-ENTRANCE-2-SOUTH>)
      (WEST PER <CAVE-ENTRANCE-2-WEST>)
      (DOWN TO PASSAGE-1)
      (VARS ABOVE-GROUND 1)>

<OBJECT BOAT-FRAME
      (AKA BOAT FRAME BOAT-FRAME)
      (DESC <DESC-BOAT-FRAME>)
      (VARS NO-TAKE 1)
      (COPY <ROOM LAKE-1>)>

<OBJECT BOAT
      (DESC "a BOAT")>

<OBJECT TREE
      (AKA TREE)
      (DESC "a TREE")
      (VARS HEALTH 20 IS-SOFT 1)>
;"IS-SOFT actually means that this object is meant to be hit"

<OBJECT LOG
      (AKA LOG LOGS)
      (DESC "a LOG")
      (VARS HEALTH 10 IS-SOFT 1)>

<OBJECT ROUGH-BOARD
      (AKA BOARD ROUGH-BOARD)
      (DESC "a rough wooden BOARD")
      (VARS IS-SOFT 1)>

<OBJECT DETRITUS
      (AKA BRUSH DETRITUS)
      (DESC <DESC-DETRITUS>)
      (COPY <ROOM CABIN-EXTERIOR>
            <ROOM CAVE-ENTRANCE-1>
            <ROOM CAVE-ENTRANCE-2>)
      (VARS OWN-TAKE 1)>

<OBJECT STICK
      (AKA STICK BRANCH)
      (DESC "a STICK")
      (COPY <ROOM FOREST-2>
            <ROOM FOREST-4>
            <ROOM CAVE-ENTRANCE-1>)
      (VARS IS-SOFT 1)>

<OBJECT TORCH 
      (AKA TORCH)
      (DESC <DESC-TORCH>)
      (VARS IS-LIT 0 FUEL 8)>

<OBJECT BULRUSH
      (AKA REED REEDS BULRUSH BULRUSHES CATTAIL CATTAILS TYPHA)
      (DESC "some REEDS")>

<OBJECT STRAP
      (AKA STRAP)
      (DESC "a strong cloth-like STRAP")>

<OBJECT SAP 
      (AKA SAP)
      (COPY <ROOM STORAGE>)
      (DESC "tree SAP")>

<OBJECT BOILED-SAP
      (AKA BOILED-SAP)
      (DESC "BOILED-SAP")>

<OBJECT BERRIES
      (AKA BERRY BERRIES)
      (DESC "some BERRIES")
      (VARS IS-EDIBLE 1)
      (COPY <ROOM FIELD-2>)>

<OBJECT HERBS
      (AKA HERB HERBS)
      (DESC "some HERBS")
      (VARS IS-EDIBLE 1)
      (COPY <ROOM FIELD-1>
            <ROOM FIELD-2>)>

<OBJECT NUTS
      (AKA NUT NUTS)
      (DESC "some NUTS")
      (VARS IS-EDIBLE 1)
      (COPY <ROOM FOREST-6>)>

<OBJECT MUSHROOM
      (AKA MUSHROOM MUSHROOMS)
      (DESC "a MUSHROOM")
      (VARS IS-EDIBLE 1)>

<OBJECT ROOT
      (AKA ROOT ROOTS)
      (DESC "a ROOT")
      (VARS IS-EDIBLE 1)>

<OBJECT FERN
      (AKA FERN FERNS)
      (DESC "a FERN")
      (VARS IS-EDIBLE 1)
      (COPY <ROOM FOREST-3>)>

<OBJECT RIVER-STONE
      (AKA STONE WHETSTONE RIVER-STONE)
      (DESC <DESC-RIVER-STONE>)
      (COPY <ROOM FOREST-3>)
      (VARS WETNESS 0 IS-HARD 1)>

<OBJECT TREE-HOLLOW
      (AKA HOLE HOLLOW TREE-HOLLOW)
      (DESC "a tree HOLLOW")
      (VARS NO-TAKE 1)
      (COPY <ROOM STORAGE>)>

<OBJECT GEM
      (AKA GEM)
      (DESC <DESC-GEM>)
      (COPY <TREE-HOLLOW 1>)
      (ACT-ADD <GEM-ENTER-PLAYER>)>

<ROUTINE DESC-FOREST-1 ()
      <COND (
            <IS-EQUAL <GET-VAR FOREST-1 FIRST-TIME> 1>
            <SET-VAR FOREST-1 FIRST-TIME 0>
            <TELL "It's the end of summer. Birds and other small creatures are rustling through
            the forest. Light filters down through long green needles, and the smell of pine
            hangs thick in the air." CR>
            <WEATHER-REPORT>
            <TELL "There's a trail up ahead, it looks like you could GO NORTH" CR>
      )(
            <OR 
                  <IS-EQUAL DETAILED-DESC 1>
                  <IS-DES 10 <RAND>>
            >
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "You've followed a trail deep into the forest. You crouch down, and see nothing but the trunks of black spruce
                  and jack pine. Even during a thunderstorm, the wind will barely penetrate this far down,
                  leaving the air hot and thick with the smell of sap." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "You're deep in the forest, following a well-defined trail. The ground is matted
                  in a solid layer of orange needles, long and slender, dotted with small pine cones and broken
                  by the occasional outcropping of white limestone. Black spruce and jack pine sway their branches overhead.
                  You spot a clump of dark brown hair caught on a branch." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "You've walked far into the forest. It's been nothing but black, white, and red spruce for awhile now.
                  The trail is firm and well-packed under your feet. The terrain here is fairly level, which makes for easy walking." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "You're in a dense forest. Well, the canopy is dense, and you often have to duck under branches. The undergrowth is
                  surprisingly spare. Most small plants are unable to push there way through the thick matt of pine needles, long and orange.
                  You accidentally step on a snail." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're in a dense forest" CR>
      )>
>

<ROUTINE DESC-RIVER-STONE ()
      <COND (
            <IS-EQUAL DETAILED-DESC 1>
            <TELL "A flat and smooth river stone. Perfect for sharpening weapons:
            WORK <weapon> WITH STONE. But make sure it's wet first: ADD WATER TO STONE." CR>
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
                  <MOVE <CMD 1>>
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
            ;"if the tree isn't dead and <CMD 2> has max-damage (is weapon), get sap"
            <AND
                  <IS-DES <GET-VAR <CMD 1> HEALTH> 0>
                  <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
                  <NOT <IS-IN C-ROOM SAP>>
            >
            <COPY-MOVE SAP C-ROOM>
      )>
>

<HIT LOG (DMG)
      <COND (
            ;"if <CMD 2> has max-damage (is a weapon), hit. If log is dead, get sticks"
            <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
            <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
            <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>

            <SET-VAR DMG <ROLL-DMG>>
            <COND (
                  <IS-EQUAL DMG 0>
                  <TELL "The log takes no damage" CR>
                  <RETURN 1>
            )>

            <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
            <TELL "The log takes " DMG " damage" CR>

            <COND (
                  <IS-ASC <GET-VAR <CMD 1> HEALTH> 1>
                  ;"replace log with four rough-boards"
                  <MOVE <CMD 1>>
                  <COPY-MOVE ROUGH-BOARD C-ROOM>
                  <COPY-MOVE ROUGH-BOARD C-ROOM>
                  <COPY-MOVE ROUGH-BOARD C-ROOM>
                  <COPY-MOVE ROUGH-BOARD C-ROOM>
                  <TELL "You've chopped the log into four rough BOARDs" CR>
            )>
      )>
>

<ROUTINE CAVE-ENTRANCE-1-NORTH ()
      <COND (
            <IS-EQUAL FOREST-BURNED-DOWN 1>
            <TELL "There's nothing but charred debris as far as the eye can see." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE PLAYER FOREST-5>
      )>
>

<ROUTINE CAVE-ENTRANCE-1-EAST ()
      <COND (
            <IS-EQUAL FOREST-BURNED-DOWN 1>
            <TELL "There's nothing but charred debris as far as the eye can see." CR>
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
            <TELL "There's nothing but charred debris as far as the eye can see." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE PLAYER FOREST-4>
      )>
>

<ROUTINE CLIFF-1-EAST ()
      <TELL "You leap off the cliff." CR>
      <SET-VAR PLAYER HEALTH 0>
>

<ROUTINE CLIFF-1-DOWN ()
      <COND (
            <IS-IN <INST PLAYER PICK-AXE> STRAP>
            <TELL "Using the improvised grappling hook, you climb down the waterfall." CR>
            <MOVE PLAYER WATERFALL-PASSAGE>
      )(
            <IS-EQUAL 1 1>
            <TELL "It's too dangerous to go down without some sort of climbing equipment. Perhaps you could improvise a grappling hook with some things?" CR>
      )>
>

<ROUTINE LAKE-1-WEST ()
      <TELL "You can't swim that far" CR>
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

<WORK RIVER-STONE ()
      <COND (
            <NOT <IS-EQUAL <CMD 2> RIVER-STONE>>
            <RETURN 0>
      )>
      <COND (
            ;"can work weapon to sharpen it"
            <IS-DES <GET-VAR <CMD 1> MAX-DAMAGE> 0>

            ;"cannot sharpen obsidian"
            <COND (
                  <IS-EQUAL OBSIDIAN-SHARD <CMD 2>>
                  <TELL "The obsidian shatters." CR>
                  <MOVE <CMD 2>>
            )>

            ;"cannot sharpen pick-axe"
            <COND (
                  <IS-EQUAL PICK-AXE <CMD 2>>
                  <RETURN 0>
            )>

            ;"if stone is not wet, damages the item"
            <COND (
                  <IS-ASC <GET-VAR <CMD 2> WET> 1>
                  <TELL "The stone isn't wet, so it just dulls the edge." CR>
                  <SET-VAR <CMD 1> DAMAGE <DIVIDE <GET-VAR <CMD 1> DAMAGE> 2>>
                  <RETURN 1>
            )>

            ;"sharpen item by 5"
            <SET-VAR <CMD 1> DAMAGE <ADD <GET-VAR <CMD 1> DAMAGE> 5>>
            <COND (
                  <IS-DES <GET-VAR <CMD 1> DAMAGE> <GET-VAR <CMD 1> MAX-DAMAGE>>
                  <SET-VAR <CMD 1> DAMAGE <GET-VAR <CMD 1> MAX-DAMAGE>>
            )>

            ;"working removes wetness"
            <SET-VAR <CMD 2> WETNESS <SUBTRACT <GET-VAR <CMD 2> WETNESS> 1>>
            <COND (
                  <IS-ASC <GET-VAR <CMD 2> WETNESS> 0>
                  <SET-VAR <CMD 2> WETNESS 0>
            )>
      )>
>

<ROUTINE FOREST-1-ENTER (DC)
      ;"count things"
      <EACH-OBJ C-ROOM (OBJ)
            <COND (
                  <IS-EQUAL OBJ DETRITUS>
                  <SET-VAR DC <ADD DC 1>>
            )>
      >

      ;"spawn trees"
      <COND (
            <NOT <IS-IN C-ROOM TREE>>
            <COPY-MOVE TREE C-ROOM>
      )>

      ;"spawn detritus"
      <COND (
            <IS-ASC <MULTIPLY DC 45> <RAND>>
            <COPY-MOVE DETRITUS C-ROOM>
      )>

      ;"spawn sticks"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM STICK>>
                  <IS-DES 34 <RAND>>
            >
            <COPY-MOVE STICK C-ROOM>
      )>

      ;"spawn roots"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM ROOT>>
                  <IS-DES 12 <RAND>>
            >
            <COPY-MOVE ROOT C-ROOM>
      )>

      ;"spawn mushroom"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM MUSHROOM>>
                  <IS-DES 10 <RAND>>
            >
            <COPY-MOVE MUSHROOM C-ROOM>
      )>

      ;"spawn rabbit"
      <COND (
            <AND 
                  <IS-ASC 1 FIRST-FOREST-PATH>
                  <NOT <IS-IN C-ROOM RABBIT>>
                  <IS-DES 25 <RAND>>
            >
            <COPY-MOVE RABBIT C-ROOM>
      )(
            <IS-DES 75 <RAND>>
            <MOVE <INST C-ROOM RABBIT>>
      )>

      ;"spawn owl"
      <COND (
            <AND
                  <IS-DES 0 TIME>
                  <NOT <IS-IN C-ROOM OWL>>
                  <IS-DES 20 <RAND>> 
            >
            <COPY-MOVE OWL C-ROOM>
      )(
            <IS-DES 25 <RAND>>
            <MOVE <INST C-ROOM OWL>>
      )>

      <SET-VAR FIRST-FOREST-PATH <ADD FIRST-FOREST-PATH 1>>
>

<ROUTINE FOREST-2-ENTER (DC HAS-DEAD) 
      ;"count things"
      <EACH-OBJ C-ROOM (OBJ)
            <COND (
                  <IS-EQUAL OBJ DETRITUS>
                  <SET-VAR DC <ADD DC 1>>
            )(
                  <AND 
                        <IS-EQUAL <GET-VAR OBJ IS-ANIMAL> 1>
                        <IS-ASC <GET-VAR OBJ HEALTH> 1>
                  >
                  <SET-VAR HAS-DEAD 1>
            )>
      >

      ;"spawn sticks"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM STICK>>
                  <IS-DES 20 <RAND>>
            >
            <COPY-MOVE STICK C-ROOM>
      )>

      ;"spawn ferns"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM FERN>>
                  <IS-DES 34 <RAND>>
            >
            <COPY-MOVE FERN C-ROOM>
      )>

      ;"spawn nuts"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM NUTS>>
                  <IS-DES 25 <RAND>>
            >
            <COPY-MOVE NUTS C-ROOM>
      )>

      ;"spawn mushroom"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM MUSHROOM>>
                  <IS-DES 15 <RAND>>
            >
            <COPY-MOVE MUSHROOM C-ROOM>
      )>

      ;"spawn rabbit"
      <COND (
            <AND 
                  <IS-ASC 1 FIRST-FOREST-PATH>
                  <NOT <IS-IN C-ROOM RABBIT>>
                  <IS-DES 40 <RAND>>
            >
            <COPY-MOVE RABBIT C-ROOM>
            <COND (
                  <IS-ASC TALK-PROMPTS 2>
                  <SET-VAR TALK-PROMPTS <ADD TALK-PROMPTS 1>>
                  <TELL "There's a rabbit here, bouncing up and down like it has something to say. You could try to TALK TO it" CR>
            )>
      )(
            <IS-DES 60 <RAND>>
            <MOVE <INST C-ROOM RABBIT>>
      )>

      ;"spawn owl"
      <COND (
            <AND
                  <IS-DES 0 TIME>
                  <NOT <IS-IN C-ROOM OWL>>
                  <NOT <IS-EQUAL WEATHER 0>>
                  <OR
                        <IS-DES 80 <RAND>> 
                        <IS-EQUAL HAS-DEAD 1>
                  >
            >
            <COPY-MOVE OWL C-ROOM>
            <COND (
                  <IS-DES 20 <RAND>>
                  <TELL "An owl hoots in the trees." CR>
            )>
      )(
            <IS-DES 25 <RAND>>
            <MOVE <INST C-ROOM OWL>>
      )>

      ;"spawn crow"
      <COND (
            ;"can't already be in the room"
            ;"can't have killed it's sibling"
            <AND
                  <IS-ASC 1 FIRST-FOREST-PATH>
                  <NOT <IS-IN C-ROOM CROW>>
                  <NOT <IS-EQUAL KILLED-CROW 1>>
            >

            <COND (
                  <AND
                        <IS-EQUAL HAS-DEAD 1>
                        <IS-DES 75 <RAND>>
                  >
                  <COPY-MOVE CROW C-ROOM>
                  <COND (
                        <IS-DES 50 <RAND>>
                        <TELL "A crow squawks at you" CR>
                  )>
            )(
                  <IS-DES 10 <RAND>>
                  <COPY-MOVE CROW C-ROOM>
            )>
      )(
            <OR 
                  <IS-DES 90 <RAND>>
                  <IS-EQUAL KILLED-CROW 1>
            >
            <MOVE <INST C-ROOM CROW>>
      )>

      <SET-VAR FIRST-FOREST-PATH <ADD FIRST-FOREST-PATH 1>>
>

<ROUTINE LAKE-1-ENTER (BLC HAS-DEAD) 
      ;"count things"
      <EACH-OBJ C-ROOM (OBJ)
            <COND (
                  <IS-EQUAL OBJ BULRUSH>
                  <SET-VAR BLC <ADD BLC 1>>
            )(
                  <AND 
                        <IS-EQUAL <GET-VAR OBJ IS-ANIMAL> 1>
                        <IS-ASC <GET-VAR OBJ HEALTH> 1>
                  >
                  <SET-VAR HAS-DEAD 1>
            )>
      >

      ;"spawn bulrushes"
      <EACH-VAL <SUBTRACT 2 BLC> (VAL)
            <COPY-MOVE BULRUSH C-ROOM>
      >

      ;"spawn frog"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM FROG>>
                  <IS-DES 70 <RAND>>
                  <NOT <IS-EQUAL WEATHER 0>>
            >
            <COPY-MOVE FROG C-ROOM>
            <COND (
                  <IS-ASC TALK-PROMPTS 2>
                  <SET-VAR TALK-PROMPTS <ADD TALK-PROMPTS 1>>
                  <TELL "There's a frog here, opening and closing it's mouth as if trying to say something. You could try to TALK TO it" CR>
            )>
      )>

      ;"spawn rabbit"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM RABBIT>>
                  <IS-DES 25 <RAND>>
            >
            <COPY-MOVE RABBIT C-ROOM>
      )(
            <IS-EQUAL 1 1>
            <MOVE <INST C-ROOM RABBIT>>
      )>

      ;"spawn snake"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM SNAKE>>
                  <NOT <IS-EQUAL WEATHER 0>>
                  <IS-DES 25 <RAND>>
            >
            <COPY-MOVE SNAKE C-ROOM>
      )(
            <IS-DES 75 <RAND>>
            <MOVE <INST C-ROOM SNAKE>>
      )>

      ;"spawn owl"
      <COND (
            <AND
                  <IS-DES 0 TIME>
                  <NOT <IS-IN C-ROOM OWL>>
                  <NOT <IS-EQUAL WEATHER 0>>
                  <OR
                        <IS-DES 50 <RAND>> 
                        <IS-EQUAL HAS-DEAD 1>
                  >
            >
            <COPY-MOVE OWL C-ROOM>
            <COND (
                  <IS-DES 25 <RAND>>
                  <TELL "An owl blinks in the moonlight." CR>
            )>
      )(
            <IS-DES 25 <RAND>>
            <MOVE <INST C-ROOM OWL>>
      )>

      ;"spawn crow"
      <COND (
            <AND 
                  ;"can't already be in the room"
                  <NOT <IS-IN C-ROOM CROW>>
                  ;"can't be thundering"
                  <NOT <IS-EQUAL WEATHER 0>>
                  ;"can't have killed it's sibling"
                  <NOT <IS-EQUAL KILLED-CROW 1>>
            >

            <COND (
                  <AND
                        <IS-EQUAL HAS-DEAD 1>
                        <IS-DES 75 <RAND>>
                  >
                  <COPY-MOVE CROW C-ROOM>
                  <COND (
                        <IS-DES 50 <RAND>>
                        <TELL "A crow squawks at you" CR>
                  )>
            )(
                  <IS-DES 10 <RAND>>
                  <COPY-MOVE CROW C-ROOM>
            )>
      )(
            <OR 
                  <IS-DES 90 <RAND>>
                  <IS-EQUAL KILLED-CROW 1>
            >
            <MOVE <INST C-ROOM CROW>>
      )>
>

<ROUTINE FOREST-3-ENTER (HAS-DEAD)
      ;"count things"
      <EACH-OBJ C-ROOM (OBJ)
            <COND (
                  <AND 
                        <IS-EQUAL <GET-VAR OBJ IS-ANIMAL> 1>
                        <IS-ASC <GET-VAR OBJ HEALTH> 1>
                  >
                  <SET-VAR HAS-DEAD 1>
            )>
      >

      ;"spawn sticks"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM STICK>>
                  <IS-DES 34 <RAND>>
            >
            <COPY-MOVE STICK C-ROOM>
      )>

      ;"spawn ferns"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM FERN>>
                  <IS-DES 25 <RAND>>
            >
            <COPY-MOVE FERN C-ROOM>
      )>

      ;"spawn herbs"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM HERBS>>
                  <IS-DES 25 <RAND>>
            >
            <COPY-MOVE HERBS C-ROOM>
      )>

      ;"spawn mushroom"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM MUSHROOM>>
                  <IS-DES 10 <RAND>>
            >
            <COPY-MOVE MUSHROOM C-ROOM>
      )>

      ;"spawn owl"
      <COND (
            <AND
                  <IS-DES 0 TIME>
                  <NOT <IS-IN C-ROOM OWL>>
                  <NOT <IS-EQUAL WEATHER 0>>
                  <OR
                        <IS-DES 80 <RAND>> 
                        <IS-EQUAL HAS-DEAD 1>
                  >
            >
            <COPY-MOVE OWL C-ROOM>
            <COND (
                  <IS-DES 20 <RAND>>
                  <TELL "An owl hoots in the trees." CR>
            )>
      )(
            <IS-DES 50 <RAND>>
            <MOVE <INST C-ROOM OWL>>
      )>

      ;"spawn crow"
      <COND (
            ;"can't already be in the room"
            ;"can't have killed it's sibling"
            <AND
                  <NOT <IS-IN C-ROOM CROW>>
                  <NOT <IS-EQUAL KILLED-CROW 1>>
            >

            <COND (
                  <AND
                        <IS-EQUAL HAS-DEAD 1>
                        <IS-DES 75 <RAND>>
                  >
                  <COPY-MOVE CROW C-ROOM>
                  <COND (
                        <IS-DES 50 <RAND>>
                        <TELL "A crow squawks at you" CR>
                  )>
            )(
                  <IS-DES 10 <RAND>>
                  <COPY-MOVE CROW C-ROOM>
            )>
      )(
            <OR 
                  <IS-DES 75 <RAND>>
                  <IS-EQUAL KILLED-CROW 1>
            >
            <MOVE <INST C-ROOM CROW>>
      )>

      ;"tattle about tree-hollow"
      <COND (
            <IS-IN <INST C-ROOM TREE-HOLLOW> GEM>
            <TELL "There's a hollow in that tree over there, looks like a good place to hide things." CR>
      )>
>

<ROUTINE FOREST-4-ENTER (HAS-DEAD)
      ;"count things"
      <EACH-OBJ C-ROOM (OBJ)
            <COND (
                  <AND 
                        <IS-EQUAL <GET-VAR OBJ IS-ANIMAL> 1>
                        <IS-ASC <GET-VAR OBJ HEALTH> 1>
                  >
                  <SET-VAR HAS-DEAD 1>
            )>
      >

      ;"spawn sticks"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM STICK>>
                  <IS-DES 34 <RAND>>
            >
            <COPY-MOVE STICK C-ROOM>
      )>

      ;"spawn berries"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM BERRIES>>
                  <IS-DES 25 <RAND>>
            >
            <COPY-MOVE BERRIES C-ROOM>
      )>

      ;"spawn nuts"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM NUTS>>
                  <IS-DES 12 <RAND>>
            >
            <COPY-MOVE NUTS C-ROOM>
      )>

      ;"spawn ferns"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM FERN>>
                  <IS-DES 20 <RAND>>
            >
            <COPY-MOVE FERN C-ROOM>
      )>

      ;"spawn mushroom"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM MUSHROOM>>
                  <IS-DES 10 <RAND>>
            >
            <COPY-MOVE MUSHROOM C-ROOM>
      )>
      
      ;"spawn frog"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM FROG>>
                  <IS-DES 33 <RAND>>
            >
            <COPY-MOVE FROG C-ROOM>
      )(
            <IS-DES 18 <RAND>>
            <MOVE <INST C-ROOM FROG>>
      )>

      ;"spawn rabbit"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM RABBIT>>
                  <IS-DES 40 <RAND>>
            >
            <COPY-MOVE RABBIT C-ROOM>
            <COND (
                  <IS-ASC TALK-PROMPTS 2>
                  <SET-VAR TALK-PROMPTS <ADD TALK-PROMPTS 1>>
                  <TELL "There's a rabbit here, bouncing up and down like it has something to say. You could try to TALK TO it" CR>
            )>
      )(
            <IS-DES 60 <RAND>>
            <MOVE <INST C-ROOM RABBIT>>
      )>

      ;"spawn snake"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM SNAKE>>
                  <IS-DES 67 <RAND>>
            >
            <COPY-MOVE SNAKE C-ROOM>
      )(
            <IS-DES 50 <RAND>>
            <MOVE <INST C-ROOM SNAKE>>
      )>

      ;"spawn owl"
      <COND (
            <AND
                  <IS-DES 0 TIME>
                  <NOT <IS-IN C-ROOM OWL>>
                  <NOT <IS-EQUAL WEATHER 0>>
                  <OR
                        <IS-DES 80 <RAND>> 
                        <IS-EQUAL HAS-DEAD 1>
                  >
            >
            <COPY-MOVE OWL C-ROOM>
            <COND (
                  <IS-DES 20 <RAND>>
                  <TELL "An owl hoots in the trees." CR>
            )>
      )(
            <IS-DES 25 <RAND>>
            <MOVE <INST C-ROOM OWL>>
      )>

      ;"spawn crow"
      <COND (
            ;"can't already be in the room"
            ;"can't have killed it's sibling"
            <AND
                  <NOT <IS-IN C-ROOM CROW>>
                  <NOT <IS-EQUAL KILLED-CROW 1>>
            >

            <COND (
                  <AND
                        <IS-EQUAL HAS-DEAD 1>
                        <IS-DES 75 <RAND>>
                  >
                  <COPY-MOVE CROW C-ROOM>
                  <COND (
                        <IS-DES 50 <RAND>>
                        <TELL "A crow squawks at you" CR>
                  )>
            )(
                  <IS-DES 10 <RAND>>
                  <COPY-MOVE CROW C-ROOM>
            )>
      )(
            <OR 
                  <IS-DES 75 <RAND>>
                  <IS-EQUAL KILLED-CROW 1>
            >
            <MOVE <INST C-ROOM CROW>>
      )>
>

<ROUTINE FOREST-5-ENTER (B-IN)
      ;"spawn trees"
      <COND (
            <NOT <IS-IN C-ROOM TREE>>
            <COPY-MOVE TREE C-ROOM>
      )>

      ;"spawn detritus"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM DETRITUS>>
                  <IS-DES 67 <RAND>>
            >
            <COPY-MOVE DETRITUS C-ROOM>
      )>

      ;"spawn roots"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM ROOT>>
                  <IS-DES 40 <RAND>>
            >
            <COPY-MOVE ROOT C-ROOM>
      )>

      ;"spawn mushroom"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM MUSHROOM>>
                  <IS-DES 10 <RAND>>
            >
            <COPY-MOVE MUSHROOM C-ROOM>
      )>

      ;"spawn beetle"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM BEETLE>>
                  <IS-DES 40 <RAND>>
            >
            <COPY-MOVE BEETLE C-ROOM>
      )(
            <IS-DES 67 <RAND>>
            <MOVE <INST C-ROOM BEETLE>>
      )>

      ;"spawn owl"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM OWL>>
                  <IS-DES 20 <RAND>> 
            >
            <COPY-MOVE OWL C-ROOM>
            <COND (
                  <IS-DES 90 <RAND>>
                  <TELL "You somehow spot an owl in the dense trees." CR>
            )>
      )(
            <IS-EQUAL 40 <RAND>>
            <MOVE <INST C-ROOM OWL>>
      )>

      ;"spawn bear"
      <COND (
            <NOT <IS-IN C-ROOM BEAR>>
            <EACH-OBJ BEAR (B)
                  <COND (
                        <AND
                              <IS-IN STORAGE B>
                              <IS-DES <GET-VAR B HEALTH> 0>
                              <IS-DES 25 <RAND>>
                              <IS-EQUAL B-IN 0>
                        >
                        <MOVE B C-ROOM>
                        <SET-VAR B ASKED 0>
                        <SET-VAR B-IN 1>
                  )>
            >
      )(
            <IS-EQUAL 1 1>
            <MOVE <INST C-ROOM BEAR>>
      )>
>

<ROUTINE FOREST-6-ENTER (DC B-IN)
      ;"count things"
      <EACH-OBJ C-ROOM (OBJ)
            <COND (
                  <IS-EQUAL OBJ DETRITUS>
                  <SET-VAR DC <ADD DC 1>>
            )>
      >

      ;"spawn trees"
      <COND (
            <NOT <IS-IN C-ROOM TREE>>
            <COPY-MOVE TREE C-ROOM>
      )>

      ;"spawn detritus"
      <COND (
            <IS-ASC <MULTIPLY DC 14> <RAND>>
            <COPY-MOVE DETRITUS C-ROOM>
      )>

      ;"spawn nuts"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM NUTS>>
                  <IS-DES 34 <RAND>>
            >
            <COPY-MOVE NUTS C-ROOM>
      )>

      ;"spawn roots"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM ROOT>>
                  <IS-DES 40 <RAND>>
            >
            <COPY-MOVE ROOT C-ROOM>
      )>

      ;"spawn mushroom"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM MUSHROOM>>
                  <IS-DES 25 <RAND>>
            >
            <COPY-MOVE MUSHROOM C-ROOM>
      )>

      ;"spawn rabbit"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM RABBIT>>
                  <IS-DES 60 <RAND>>
            >
            <COPY-MOVE RABBIT C-ROOM>
            <COND (
                  <IS-ASC TALK-PROMPTS 2>
                  <SET-VAR TALK-PROMPTS <ADD TALK-PROMPTS 1>>
                  <TELL "There's a rabbit here, bouncing up and down like it has something to say. You could try to TALK TO it" CR>
            )>
      )(
            <IS-DES 50 <RAND>>
            <MOVE <INST C-ROOM RABBIT>>
      )>

      ;"spawn bear"
      <COND (
            <NOT <IS-IN C-ROOM BEAR>>
            <EACH-OBJ BEAR (B)
                  <COND (
                        <AND
                              <IS-IN STORAGE B>
                              <IS-DES <GET-VAR B HEALTH> 0>
                              <IS-DES 25 <RAND>>
                              <IS-EQUAL B-IN 0>
                        >
                        <MOVE B C-ROOM>
                        <SET-VAR B ASKED 0>
                        <SET-VAR B-IN 1>
                  )>
            >
      )(
            <IS-EQUAL 1 1>
            <MOVE <INST C-ROOM BEAR>>
      )>
>

<ROUTINE FIELD-1-ENTER (BC HAS-DEAD) 
      ;"count things"
      <EACH-OBJ C-ROOM (OBJ)
            <COND (
                  <IS-EQUAL OBJ BERRIES>
                  <SET-VAR BC <ADD BC 1>>
            )(
                  <AND 
                        <IS-EQUAL <GET-VAR OBJ IS-ANIMAL> 1>
                        <IS-ASC <GET-VAR OBJ HEALTH> 1>
                  >
                  <SET-VAR HAS-DEAD 1>
            )>
      >

      ;"spawn berries"
      <COND (
            <AND
                  <IS-ASC BC 2>
                  <IS-DES 25 <RAND>>
            >
            <COPY-MOVE BERRIES C-ROOM>
      )>

      ;"spawn herbs"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM HERBS>>
                  <IS-DES 67 <RAND>>
            >
            <COPY-MOVE HERBS C-ROOM>
      )>

      ;"spawn snake"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM SNAKE>>
                  <NOT <IS-EQUAL WEATHER 0>>
                  <IS-DES 67 <RAND>>
            >
            <COPY-MOVE SNAKE C-ROOM>
      )(
            <IS-EQUAL WEATHER 0>
            <MOVE <INST C-ROOM SNAKE>>
      )>

      ;"spawn beetle"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM BEETLE>>
                  <IS-DES 40 <RAND>>
                  <NOT <IS-EQUAL WEATHER 0>>
            >
            <COPY-MOVE BEETLE C-ROOM>
      )>

      ;"spawn rabbit"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM RABBIT>>
                  <NOT <IS-EQUAL WEATHER 0>>
                  <IS-DES 67 <RAND>>
            >
            <COPY-MOVE RABBIT C-ROOM>
            <COND (
                  <IS-ASC TALK-PROMPTS 2>
                  <SET-VAR TALK-PROMPTS <ADD TALK-PROMPTS 1>>
                  <TELL "There's a rabbit here, bouncing up and down like it has something to say. You could try to TALK TO it" CR>
            )>
      )(
            <IS-DES 50 <RAND>>
            <MOVE <INST C-ROOM RABBIT>>
      )>

      ;"spawn crow"
      <COND (
            <AND 
                  ;"can't already be in the room"
                  <NOT <IS-IN C-ROOM CROW>>
                  ;"can't be thundering"
                  <NOT <IS-EQUAL WEATHER 0>>
                  ;"can't have killed it's sibling"
                  <NOT <IS-EQUAL KILLED-CROW 1>>
            >

            <COND (
                  <AND
                        <IS-EQUAL HAS-DEAD 1>
                        <IS-DES 75 <RAND>>
                  >
                  <COPY-MOVE CROW C-ROOM>
                  <COND (
                        <IS-DES 50 <RAND>>
                        <TELL "A crow squawks at you" CR>
                  )>
            )(
                  <IS-DES 12 <RAND>>
                  <COPY-MOVE CROW C-ROOM>
            )>
      )(
            <OR 
                  <IS-DES 90 <RAND>>
                  <IS-EQUAL KILLED-CROW 1>
            >
            <MOVE <INST C-ROOM CROW>>
      )>
>

<ROUTINE FIELD-2-ENTER (BC HAS-DEAD)
      ;"count things"
      <EACH-OBJ C-ROOM (OBJ)
            <COND (
                  <IS-EQUAL OBJ BERRIES>
                  <SET-VAR BC <ADD BC 1>>
            )(
                  <AND 
                        <IS-EQUAL <GET-VAR OBJ IS-ANIMAL> 1>
                        <IS-ASC <GET-VAR OBJ HEALTH> 1>
                  >
                  <SET-VAR HAS-DEAD 1>
            )>
      >

      ;"spawn berries"
      <COND (
            <AND
                  <IS-ASC BC 2>
                  <IS-DES 40 <RAND>>
            >
            <COPY-MOVE BERRIES C-ROOM>
      )>

      ;"spawn herbs"
      <COND (
            <AND
                  <NOT <IS-IN C-ROOM HERBS>>
                  <IS-DES 40 <RAND>>
            >
            <COPY-MOVE HERBS C-ROOM>
      )>

      ;"spawn beetle"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM BEETLE>>
                  <IS-DES 40 <RAND>>
            >
            <COPY-MOVE BEETLE C-ROOM>
      )>

      ;"spawn rabbit"
      <COND (
            <AND 
                  <NOT <IS-IN C-ROOM RABBIT>>
                  <NOT <IS-EQUAL WEATHER 0>>
                  <IS-DES 75 <RAND>>
            >
            <COPY-MOVE RABBIT C-ROOM>
            <COND (
                  <IS-ASC TALK-PROMPTS 2>
                  <SET-VAR TALK-PROMPTS <ADD TALK-PROMPTS 1>>
                  <TELL "There's a rabbit here, bouncing up and down like it has something to say. You could try to TALK TO it" CR>
            )>
      )(
            <IS-DES 50 <RAND>>
            <MOVE <INST C-ROOM RABBIT>>
      )>

      ;"spawn crow"
      <COND (
            <AND 
                  ;"can't already be in the room"
                  <NOT <IS-IN C-ROOM CROW>>
                  ;"can't be thundering"
                  <NOT <IS-EQUAL WEATHER 0>>
                  ;"can't have killed it's sibling"
                  <NOT <IS-EQUAL KILLED-CROW 1>>
            >

            <COND (
                  <AND
                        <IS-EQUAL HAS-DEAD 1>
                        <IS-DES 75 <RAND>>
                  >
                  <COPY-MOVE CROW C-ROOM>
                  <COND (
                        <IS-DES 50 <RAND>>
                        <TELL "A crow squawks at you" CR>
                  )>
            )(
                  <IS-DES 12 <RAND>>
                  <COPY-MOVE CROW C-ROOM>
            )>
      )(
            <OR 
                  <IS-DES 90 <RAND>>
                  <IS-EQUAL KILLED-CROW 1>
            >
            <MOVE <INST C-ROOM CROW>>
      )>
>

<ROUTINE CABIN-EXTERIOR-ENTER (HAS-DEAD)
      <MOVE <INST CABIN CABIN-WINDOW> C-ROOM>

      ;"spawn owl"
      <COND (
            <AND
                  <IS-DES 0 TIME>
                  <NOT <IS-IN C-ROOM OWL>>
                  <NOT <IS-EQUAL WEATHER 0>>
                  <OR
                        <IS-DES 75 <RAND>> 
                        <IS-EQUAL HAS-DEAD 1>
                  >
            >
            <COPY-MOVE OWL C-ROOM>
            <COND (
                  <IS-DES 10 <RAND>>
                  <TELL "An owl watches you closely." CR>
            )>
      )(
            <IS-DES 50 <RAND>>
            <MOVE <INST C-ROOM OWL>>
      )>

      ;"spawn crow"
      <COND (
            ;"can't already be in the room"
            <NOT <IS-IN C-ROOM CROW>>

            ;"check for a dead animal in the room"
            <EACH-OBJ C-ROOM (OBJ)
                  <COND (
                        <AND 
                              <IS-EQUAL <GET-VAR OBJ IS-ANIMAL> 1>
                              <IS-ASC <GET-VAR OBJ HEALTH> 1>
                        >
                        <SET-VAR HAS-DEAD 1>
                  )>
            >

            <COND (
                  <IS-EQUAL <GET-VAR <INST C-ROOM CABIN-WINDOW> IS-BROKEN> 1>
            )>

            <COND (
                  <AND
                        <OR
                              <IS-EQUAL HAS-DEAD 1>
                              ;"is attracted to the shiny broken glass"
                              <IS-EQUAL <GET-VAR <INST C-ROOM CABIN-WINDOW> IS-BROKEN> 1>
                        >
                        <IS-DES 90 <RAND>>
                  >
                  <COPY-MOVE CROW C-ROOM>
                  <COND (
                        <IS-DES 50 <RAND>>
                        <TELL "A crow squawks at you" CR>
                  )>
            )>
      )(
            <OR 
                  <IS-DES 25 <RAND>>
                  <IS-EQUAL KILLED-CROW 1>
            >
            <MOVE <INST C-ROOM CROW>>
      )>
>

<ROUTINE FOREST-5-EXIT ()
      ;"return bear to storage"
      <EACH-OBJ BEAR (B)
            <COND (
                  <AND
                        <IS-IN C-ROOM B>
                        <IS-DES <GET-VAR B HEALTH> 0>
                  >
                  <MOVE B STORAGE>
            )>
      >
>

<ROUTINE FOREST-6-EXIT () 
      ;"return bear to storage"
      <EACH-OBJ BEAR (B)
            <COND (
                  <AND
                        <IS-IN C-ROOM B>
                        <IS-DES <GET-VAR B HEALTH> 0>
                  >
                  <MOVE B STORAGE>
            )>
      >
>

<ROUTINE CLIFF-1-ENTER (HAS-DEAD) 
      ;"spawn crow"
      <COND (
            ;"can't already be in the room"
            <NOT <IS-IN C-ROOM CROW>>

            ;"check for a dead animal in the room"
            <EACH-OBJ C-ROOM (OBJ)
                  <COND (
                        <AND 
                              <IS-EQUAL <GET-VAR OBJ IS-ANIMAL> 1>
                              <IS-ASC <GET-VAR OBJ HEALTH> 1>
                        >
                        <SET-VAR HAS-DEAD 1>
                  )>
            >

            <COND (
                  <AND
                        <IS-EQUAL HAS-DEAD 1>
                        <IS-DES 90 <RAND>>
                  >
                  <COPY-MOVE CROW C-ROOM>
                  <COND (
                        <IS-DES 50 <RAND>>
                        <TELL "A crow squawks at you" CR>
                  )>
            )(
                  <IS-DES 34 <RAND>>
                  <COPY-MOVE CROW C-ROOM>
            )>
      )(
            <IS-DES 50 <RAND>>
            <MOVE <INST C-ROOM CROW>>
      )>
>

<ROUTINE DESC-DETRITUS ()
      <COND (
            <IS-EQUAL DETAILED-DESC 1>
            <TELL "Various grasses, leaves, and twigs are strewn about. This detritus could be good for starting a fire,
            or weaving into some rope-like straps (WORK with the right tool)." CR>
            <TELL "The word DETRITUS is a little long, you could use BRUSH instead." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "leafy DETRITUS">
      )>
>

<WORK DETRITUS (F-2) 
      <COND (
            <AND
                  <IS-EQUAL DETRITUS <CMD 1>>
                  <IS-EQUAL DETRITUS <CMD 2>>
            >

            ;"cmd 1 and cmd 2 are the same instance, so we have to go looking for another one"
            <MOVE <CMD 1> STORAGE> ;"move temporarily, we just need it out of the way"
            <COND(
                  <IS-IN PLAYER DETRITUS N>
                  <EACH-OBJ <INST PLAYER DETRITUS N> (OBJ)
                        <MOVE OBJ C-ROOM>
                  >
                  <MOVE <INST PLAYER DETRITUS N>>
                  <SET-VAR F-2 1>
            )(
                  <IS-IN C-ROOM DETRITUS N>
                  <EACH-OBJ <INST C-ROOM DETRITUS N> (OBJ)
                        <MOVE OBJ C-ROOM>
                  >
                  <MOVE <INST C-ROOM DETRITUS N>>
                  <SET-VAR F-2 1>
            )>

            <COND (
                  <IS-EQUAL F-2 1>
                  <MOVE <CMD 1>>
                  <COPY-MOVE STRAP C-ROOM>
                  <TELL "You weave the grasses together to form a STRAP" CR>
            )(
                  <IS-EQUAL 1 1>
                  ;"<CMD 1> could have been in player, this will have to do"
                  <MOVE <CMD 1> C-ROOM>
            )>

      )(
            <AND
                  <IS-EQUAL DETRITUS <CMD 1>>
                  <IS-EQUAL BONES <CMD 2>>
            >
            <MOVE <CMD 1>>
            <COPY-MOVE STRAP C-ROOM>
            <TELL "You stretch the grasses over an ad-hoc bone loom, then pick them together into a STRAP" CR>
      )>
>

<WORK PICK-AXE ()
      <COND (
            <AND
                  <IS-EQUAL PICK-AXE <CMD 1>>
                  <IS-EQUAL STRAP <CMD 2>>
            >
            <COND (
                  <IS-IN <CMD 1> STRAP>
                  <TELL "This pick-axe already has a strap tied to it" CR>
            )(
                  <IS-EQUAL 1 1>
                  <MOVE <CMD 2> <CMD 1>>
                  <TELL "You tie a strap to the pick-axe. It reminds you of mountain-climbing expeditions, of days spent rappelling down cliffs." CR> 
            )>
      )>
>

<ROUTINE DESC-GEM ()
      <COND (
            <IS-EQUAL DETAILED-DESC 1>
            <TELL "This gem sparkles with unnatural brilliance. Carrying it will light up even the darkest caverns." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "a GEM">
      )>
>

<ROUTINE GEM-ENTER-PLAYER ()
      <SET-VAR WIN-FIND-GEM 1>
>

<WORK BOAT-FRAME (SC BC NC LC)
      <COND (
            <IS-EQUAL BOAT-FRAME <CMD 2>>
            <TELL "Try it the other way around" CR>
            <RETURN 0>
      )>
      <COND (
            <OR
                  <IS-EQUAL STRAP <CMD 2>>
                  <IS-EQUAL ROUGH-BOARD <CMD 2>>
                  <IS-EQUAL NAILS <CMD 2>>
                  <IS-EQUAL BOILED-SAP <CMD 2>>
            >
            <MOVE <CMD 2> <CMD 1>>
            <EACH-OBJ <CMD 1> (OBJ) 
                  <COND (
                        <IS-EQUAL STRAP OBJ>
                        <SET-VAR SC <ADD SC 1>>
                  )(
                        <IS-EQUAL ROUGH-BOARD OBJ>
                        <SET-VAR BC <ADD BC 1>>
                  )(
                        <IS-EQUAL NAILS OBJ>
                        <SET-VAR NC <ADD NC 1>>
                  )(
                        <IS-EQUAL BOILED-SAP OBJ>
                        <SET-VAR LC <ADD LC 1>>
                  )>
            >
            <COND (
                  <AND
                        <IS-ASC 1 SC>
                        <IS-ASC 9 BC>
                        <IS-ASC 0 NC>
                        <IS-ASC 0 SC>
                  >
                  <SET-VAR WIN-BUILD-BOAT 1>
                  <TELL "You've build a boat." CR>
                  <RECAP>
                  <TELL "It's day " DAY CR>
                  <WEATHER-REPORT>
                  <TELL "You push the boat off, heading west across lake, to adventures unknown." CR>
                  <TELL "Good game! Thank you for playing" CR>
                  <END>
            )(
                  <IS-EQUAL 1 1>
                  <TELL "The boat needs:" CR>
                  <TELL "2 straps, has " SC CR>
                  <TELL "10 boards, has " BC CR>
                  <TELL "1 nails, has " NC CR>
                  <TELL "1 boiled-sap, has " LC CR>
            )>
      )(
            <IS-EQUAL <CMD 2> <CMD 3>>
            <TELL "I'm not sure what you mean. Do you have that?" CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "Building a boat requires STRAPs, BOARDs, NAILS, and BOILED-SAP" CR>
      )>
>

<TAKE () DETRITUS 
      <EACH-OBJ <CMD 1> (OBJ)
            <MOVE OBJ C-ROOM>
      >
      <TELL "Picked up" CR>
      <MOVE <CMD 1> PLAYER>
>

<ROUTINE DESC-BOAT-FRAME ()
      <COND (
            <IS-EQUAL DETAILED-DESC 1>
            <TELL "This is the skeleton of a small wooden boat. It's missing some BOARDs,
            NAILS, STRAPs, and BOILED-SAP. Can WORK BOAT WITH <item>. In general, can
            WORK <thing> WITH <thing>." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "a BOAT frame">
      )>
>

<ROUTINE DESC-CAVE-ENTRANCE-1 ()
      <COND (
            <IS-EQUAL DETAILED-DESC 1>
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "The forest hums and buzzes around you. Soft earth gives way to a jagged maw, beckoning you DOWN." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "Twigs snap underfoot. Amidst boulders and tree trunks a black void beckons
                  you forward. A cave. You could GO DOWN, but there is no telling whether you would come back up." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "Lot's of creatures are crawling around the forest at this time of year. Some might slither. A few might 
                  stalk. Off to the side of the path is the entrance to a cave, large enough for you to slip DOWN, or for something to 
                  come up." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "The air is a little cooler at this end of the forest, but still hangs with summer smells. There's no break in the trees,
                  save for a clump of rocks, with a black hole at the center. It leads DOWN. How far?" CR>
            )>
            <COND (
                  <NOT <OR
                        <IS-IN PLAYER GEM>
                        <IS-IN PLAYER TORCH> ;"hopefully lit"
                  >>
                  <TELL "Some sort of light source might help you navigate below." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're at the entrace to a cave." CR>
      )>
>

<ROUTINE DESC-CAVE-ENTRANCE-2 ()
      <COND (
            <IS-EQUAL DETAILED-DESC 1>
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "This was clearly a well-trodden path, but has fallen into dis-use. How many feet have walked this
                  ground? How many hooves? Not many flowers this far into the forest, not enough light for them. Or they've all
                  been eaten by a fawn. You hear water dripping faintly, echoing against rock. It could be coming from this large
                  hole in the ground. GO DOWN to find out." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "You notice some broken branches off of the path, signs an animal has come this way. There are animals all
                  over the forest this time of year. It could be a deer, or a bear, or something that lives in that hole. You can't
                  see much, would have to GO DOWN to investigate." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "The path is flanked by high brush on both sides, the forest canopy thick overhead. It's almost like walking
                  down a tunnel. But through the foliage you spot an even darker area. A wide gap in the ground, as if the top
                  has come off a bottle. You could GO DOWN." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "A large hole in the ground is hidden behind a screen of brush. You could GO DOWN.
                  It doesn't look like there's anything living there, no path connecting this hole to the main trail, but who's 
                  to say it's not some careful creature." CR>
            )>
            <COND (
                  <NOT <OR
                        <IS-IN PLAYER GEM>
                        <IS-IN PLAYER TORCH> ;"hopefully lit"
                  >>
                  <TELL "Some sort of light source might help you navigate below." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're at the entrace to a cave." CR>
      )>
>

<ROUTINE DESC-CLIFF-1 ()
      <COND (
            <OR 
                  <IS-EQUAL DETAILED-DESC 1>
                  <IS-DES 10 <RAND>>
            >
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "You're standing on the edge of a cliff. The fields slope up behind you, the cabin just barely visible back WEST.
                  A fresh, clear stream runs between tall grass." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "Grass dances violently in the wind here. Birds wheel overhead. The stream, flowing so gently through
                  the fields, suddenly crashes off the edge of a cliff. Without climbing gear, EAST surely means death." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "The fields stretching away to your north, east, and west are cut off by fresh air. The ground drops away,
                  forming a rocky cliff. A stream, following the cleft of the fields, falls of the cliff edge." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "Grassy fields abruptly stop at the edge of a cliff, facing east. The drop would surely be fatal.
                  A stream hurtles over, mist rising gently up from it's landing below." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're at the edge of a cliff." CR>
      )>
>

<ROUTINE DESC-LAKE-1 ()
      <COND (
            <OR 
                  <IS-EQUAL DETAILED-DESC 1>
                  <IS-DES 10 <RAND>>
            >
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "You find yourself on the eastern edge of a large lake. It stretches off into the distance, shrouded
                  in fog. The shoreline is mostly granite, with some patches of coarse yellow sand. The water is cool and clear.
                  Frogs croak amidst the bulrushes." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "Trees peel back to reveal a large lake, spreading out luxuriously towards the west.">
                  <COND (
                        <AND
                              <NOT <IS-EQUAL MOON-PHASE 0>>
                              <IS-DES 0 TIME>
                        >
                        <TELL " Moonlight reflects off the surface." >
                  )>
                  <TELL " The crisp snap of fall is beginning to clear away the heat of summer. Some leaves are starting to turn yellow, orange, red." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "The forest is never silent, but here sound carries perfectly across the surface of a wide lake. It carries
                  on to the west, before losing itself in mist. Small fish dart around, never breaking the surface. It's two worlds,
                  above and below." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "A large, wide lake takes over from endless tree bark. The contrast is startling, as if stumbling into
                  an empty ballroom. It's never quiet, but always peaceful here. Sumac crowds around the water's edge." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're on the shore of a lake" CR>
      )>
>

<ROUTINE DESC-FOREST-2 ()
      <COND (
            <OR 
                  <IS-EQUAL DETAILED-DESC 1>
                  <IS-DES 10 <RAND>>
            >
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "The trail weaves haphazardly through thinning trees. A dense carpet of bracken gently transitions into
                  dandelions, wild carrot, clover, and virginia rye. A snake slithers past. A group of ants struggle against a twig.
                  The trail you're walking on is littered with maple keys." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "Wafts of fresh air cut through the stench of pine. You're in a forest, but not as deep as elsewhere. It feels safe and playful.
                  Young sugar maples are climbing up towards the sky, stretching their leaves, and swaying in the wind. Virginia rye brushes
                  against your boots." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "This path runs generally NORTH to SOUTH but enjoys it's time in getting there. Wild carrot dots the forest floor like little
                  white fireworks. Broad leaves brush against needles, and the earth is a rich brown under your boots." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "Many creatures have walked this path before you. You pick the most-trodden route, but grasses and bracken are
                  flattened all over. Most of the trees here are younger, with slender trunks. They stand tall, hopeful, and defiant. Older aspen
                  and elm persevere quietly." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're in a forest, the trees are thinner here. There's a trail heading NORTH (and SOUTH)." CR>
      )>
>

<ROUTINE DESC-FOREST-3 ()
      <COND (
            <OR 
                  <IS-EQUAL DETAILED-DESC 1>
                  <IS-DES 10 <RAND>>
            >
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "You're in a forest, on a trail following a small stream. The trees aren't too dense but you can't
                  see very far in any direction, due to sharp elevation change in the rocky terrain. Some maples are starting
                  to lose their leaves, coating the path in a collage of decay." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "This forest reminds you of so many autumns before, but it's familiarity does not dull your pleasure nor succeed in extinguishing a
                  flickering sense of wonder. The smell of pine, sap, earth, and fresh water swirl through the air. Water wears away granite ever
                  so slowly." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "You're in a forest. A stream burbles alongside the path, leading generally EAST to WEST, occasionally branching
                  confidently from a rise. Patches of sage, mint, and thyme spill over the landscape. Granite peaks through fallen leaves, 
                  brown-pink-white-gold." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "You're in a light forest of poplar, birch, and maple. It seems like an excellent area for cultivating mushrooms.
                  Through the next sudden valley, a prime spot for herbs. A diminutive but determined brook burbles nearby. You avoid
                  stepping in some rabbit droppings." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're in a forest, following a small stream." CR>
      )>
>

<ROUTINE DESC-FOREST-4 ()
      <COND (
            <OR 
                  <IS-EQUAL DETAILED-DESC 1>
                  <IS-DES 10 <RAND>>
            >
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "As summer turns to fall, animals in the forest prepare for winter. Every nut, root, grub, and flower is worth
                  contemplation. The squirrel has made a lovely home in an oak tree. You're surrounded by tall trees, with dandelion, marigold,
                  and bittercress fighting up out of the dirt." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "Trees thin and thicken. You could stand four people around that oak and not touch hands. The forest floor is
                  populated with dandelion, marigold, and bittercress. Oyster mushrooms grow on tree bark, and continue right on growing
                  after the tree falls down dead. Birches shrug of their skin." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "You're in a forest of oak, hemlock, dogwood, and cedar. Daisies wave from atop shelves of limestone. Small
                  creatures fight amongst themselves for dominance of the area. Acorns crunch under your boots as you walk alone the trail.
                  The smell of possibility is in the air." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "Who broke this trail? It's not as well-worn as others, but maintains a consistent heading; there's a definite destination
                  in mind. An oak leaf falls to the ground as you step over a tree root. What brought them here? Were they looking for the same thing
                  you are?" CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're in a forest." CR>
      )>
>

<ROUTINE DESC-FOREST-5 ()
      <COND (
            <OR 
                  <IS-EQUAL DETAILED-DESC 1>
                  <IS-DES 10 <RAND>>
            >
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "You're deep into a forest of spruce, pine, and cedar. Despite a thick coating of needles
                  on the ground, the trail is clearly discernable: it's worn markedly lower than the rest of the forest
                  floor. When rain falls, it gathers in puddles and washes away debris down the trail." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "You've walked down many forest trails. The smell of pine, now thick in your nose, conjures up
                  memories of songs around the campfire, cold nights, and fireflies in the dark. I wonder where your
                  cedar-strip canoe is now. Is that water in the distance?" CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "You're deep into a conifer forest. The ground is matted with needles, and they stick to the soles of
                  your boots with sap. You spot a snake skin coiled up on near the base of a red pine. It's pretty small though.
                  The forest just gets thicker and thicker." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "You hum to yourself while walking down the trail. Tree limbs here are crowded for space, but the trunks
                  themselves are well apart, so the walking is easy as roots seldom crop up. That trunk has clearly been marked
                  by a bear. A spruce needle pricks you as you go by." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're deep in a forest." CR>
      )>
>

<ROUTINE DESC-FOREST-6 ()
      <COND (
            <OR 
                  <IS-EQUAL DETAILED-DESC 1>
                  <IS-DES 10 <RAND>>
            >
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "You're in a dense forest. The undergrowth is thick with fallen branches,
                  leaves, and other detritus. Everything green vies for a spot in the sunlight." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "You're in a wild forest. Many generations of tree have lived and died here. Beech, cedar, oak, and maple
                  feast on the remains of one another. Each gets their turn in the sun. A spray of asters shines smartly amidst
                  brown sticks and leaves." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "You could get lost out here if you're not careful. The trail is faint and barely worn. Lightening-struck
                  trees lean against those still standing, threatening to topple them in turn. You choose your footing carefully.
                  The air is crisp with notes of mint and clover, but always undertones of decay." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "Are you still on a trail? As you head more north west, the path becomes harder to follow. You doubt many
                  other people have come this way. Trees are closing in around you, they loom and lean menacingly. Pine sap forms little
                  stalagmites on top of some roots. That cracked tree-stump over there reminds you of your grandfather." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're deep in a forest." CR>
      )>
>

<ROUTINE DESC-FIELD-1 ()
      <COND (
            <OR 
                  <IS-EQUAL DETAILED-DESC 1>
                  <IS-DES 10 <RAND>>
            >
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "A grassy field sprawls before you. You trace the wind across it's face. Goldenrod,
                  virginia rye, wild carrot, milkweed, thistle, and nettle all sway in the breeze. The air is a
                  practiced conductor; everyone playing in perfect harmony. A stream tinkles nearby." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "You're in a meadow, waist-deep in wild grass. The smooth curves or earth are interrupted only
                  by a thin stream running east. The wind whips against your face. A whisp of milkweed flies into your
                  eye, you stop momentarily to blink it out." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "You're in a field. A furrow runs beside the path, carved out by a determined trickle of
                  a stream. The wind races along. Where does it have to go? The dirt is soft beneath your boots.
                  Goldenrod slips through your fingers." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "You're walking along a trail through rolling fields. You've walked far enough to avoid any
                  thistle and nettle, but this meadow is mostly grass. It grows higher near a little stream. The path
                  follows on it's south, occasionally crossing over north." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're in a field." CR>
      )>
>

<ROUTINE DESC-FIELD-2 ()
      <COND (
            <OR 
                  <IS-EQUAL DETAILED-DESC 1>
                  <IS-DES 10 <RAND>>
            >
            <COND (
                  <IS-DES 12 <RAND>>
                  <TELL "You're in a large field. It rolls away north to the horizon. A slug is munching
                  on the stem of a wild carrot. You munch on some goldenrod. Is that a voice on the wind?
                  No, it's just the wind." CR>
            )(
                  <IS-EQUAL DAY-M-3 0>
                  <TELL "You're in a large meadow, with seeds rolling down into your boots. The trail is faint here,
                  and not very playful; it proceeds in straight lines. Not that there's much reason for curves.
                  A meadow is diverse in the micro but monotonous in the macro. At least you'll see anything coming
                  from a ways off, if anything comes." CR>
            )(
                  <IS-EQUAL DAY-M-3 1>
                  <TELL "You're in a field, spilling away from you in all directions. The goldenrod is vibrant, the thistles sharp,
                  and the rye tall. You could spend all day here, wrapped in your cloak. The harvest is good for this time of year." CR>
            )(
                  <IS-EQUAL DAY-M-3 2>
                  <TELL "You're in a large field, unbroken by fence or tree. If you weren't already, it would make you want to
                  wander. Every time you see milkweed pods you have an urge to open them." CR>
            )>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're in a large field." CR>
      )>
>

<HIT MUSHROOM ()
      <TELL "'can you feel your heart burning? can you feel the struggle within? the fear within me is beyond anything
      your soul can make. you cannot kill me in a way that matters'" CR>
>

<ROUTINE LAKE-1-ALWAYS ()
    <COND (
        <NOT <IS-IN C-ROOM WATER>>
        ;"the lake never dries up (but it might temporarily while the game is processing hooks)"
        <COPY-MOVE WATER C-ROOM>
    )>
>