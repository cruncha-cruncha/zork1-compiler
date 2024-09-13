;"It's the end of summer..."

<DIRECTIONS UP DOWN IN OUT NORTH EAST SOUTH WEST>

;"3 = sunny, 2 = cloudy, 1 = rainy, 0 = thunder"
<GLOBAL WEATHER 3>

;"aka days since last rain"
<GLOBAL DRY 2>

;"0 = none, 1-11 = waxing, 12 = full, 13-23 = waning"
<GLOBAL MOON 2>

;"starts at 1 and just keeps going"
<GLOBAL DAY 1>

;"night starts at -1 and goes to -17, day starts at 1 and goes to 33"
;"2 ticks per hour, 24 hours in a day"
<GLOBAL TIME 1>

;"if DESC is a function, it can look for this global to optionally provide a detailed description"
<GLOBAL DETAILED-DESC 0>

;"Explain more the first time we look around"
<GLOBAL FIRST-LOOK-AROUND 1>

;"Explain more the first time we examine an object"
<GLOBAL FIRST-EXAMINE 1>

<PLAYER 
      (ROOM FOREST-1)
      (ACT-ENTER V-DESC-ROOM)
      (ACT-ALWAYS TIME-PASSES)
      (VARS HEALTH 40)>

<OBJECT AXE 
      (AKA AXE AX)
      (DESC "an AXE")
      (IN PLAYER)
      (VARS DAMAGE 3 MAX-DAMAGE 5 HEALTH 10)>

<OBJECT BOOTS 
      (AKA BOOTS BOOT)
      (DESC "BOOTS")
      (IN PLAYER)>

<OBJECT CLOAK 
      (AKA CLOAK COAT)
      (DESC "a CLOAK")
      (IN PLAYER)>

<OBJECT FLINT 
      (AKA FLINT)
      (DESC "FLINT")
      (IN PLAYER)>

<OBJECT BOWL 
      (AKA BOWL)
      (DESC "a BOWL")
      (IN PLAYER)>

<OBJECT CUP 
      (AKA CUP)
      (DESC "a CUP")
      (IN PLAYER)>

<OBJECT KETTLE 
      (AKA KETTLE POT)
      (DESC "a KETTLE")
      (IN PLAYER)>

<OBJECT CUTLERY 
      (AKA CUTLERY)
      (DESC "some CUTLERY")
      (IN PLAYER)>

<OBJECT KNIFE 
      (AKA KNIFE)
      (DESC "a KNIFE")
      (IN PLAYER)>


<OBJECT CABIN-DOOR
      (AKA DOOR CABIN-DOOR)
      (DESC "the cabin DOOR")
      (IN CABIN-EXTERIOR)
      (VARS NO-TAKE 1 CAN-OPEN 1 IS-LOCKED 1)
      (ACT-PRSO F-CABIN-DOOR-PRSO)>

<OBJECT CABIN-WINDOW
      (AKA WINDOW)
      (DESC "a cabin WINDOW")
      (VARS NO-TAKE 1)
      (IN CABIN-EXTERIOR)>

<OBJECT CABIN-DOOR-KEY
      (AKA KEY DOOR-KEY CABIN-DOOR-KEY)
      (DESC "a KEY")
      (IN DETRITUS-1)>

<OBJECT KNOTTED-ROOT-1
      (AKA KNOTTED-ROOT ROOT)
      (DESC "a knotted ROOT")
      (VARS NO-TAKE 1)
      (IN DETRITUS-1)>

<OBJECT DETRITUS-1
      (AKA DETRITUS BRUSH)
      (VARS CAN-UNPACK 1)
      (DESC "leafy DETRITUS on the ground")
      (IN CABIN-EXTERIOR)
      (ACT-PRSO V-DETRITUS-1-PRSO)>

<ROOM FOREST-1
      (DESC F-DESC-FOREST-1)
      (VARS FIRST-TIME 1 ABOVE-GROUND 1)
      (NORTH TO FOREST-2)
      (EAST TO FOREST-2)
      (WEST TO CAVE-ENTRANCE-1)>

<ROOM FOREST-2
      (DESC "You're in a forest, the trees are thinner here. There's a trail heading NORTH (and back SOUTH)." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO CABIN-EXTERIOR)
      (SOUTH TO FOREST-1)>

<ROOM CABIN-EXTERIOR
      (DESC F-DESC-CABIN-EXTERIOR)
      (VARS ABOVE-GROUND 1 FIRST-TIME 1)
      (NORTH TO FIELD-2)
      (EAST TO FIELD-1)
      (SOUTH TO FOREST-2)
      (IN PER CAN-ENTER-CABIN)
      (WEST TO FOREST-3)>

<ROUTINE CAN-ENTER-CABIN ()
      <COND (
            <AND <IS-EQUAL <GET-VAR CABIN-DOOR IS-LOCKED> 1>
                  <IS-EQUAL <GET-VAR CABIN-WINDOW IS-SMASHED> 0>>
            <TELL "There's no way into the cabin, you need to break something or find a key." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE PLAYER CABIN>
      )>
>

<ROOM CABIN
      (DESC "You're inside a log cabin. It's rustic, but has a lovely fireplace.")
      (VARS ABOVE-GROUND 1)
      (NORTH TO CABIN-EXTERIOR)
      (EAST TO CABIN-EXTERIOR)
      (SOUTH TO CABIN-EXTERIOR)
      (WEST TO CABIN-EXTERIOR)
      (OUT TO CABIN-EXTERIOR)>

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
      (DESC "Amidst boulders and tree trunks a black space beckons you forward. A cave. You could GO DOWN, but there is no telling whether you would come back up.")
      (VARS ABOVE-GROUND 1)
      (NORTH TO FOREST-5)
      (EAST TO FOREST-1)
      (SOUTH TO FOREST-1)
      (WEST TO FOREST-5)>

<ROOM CAVE-ENTRANCE-2
      (DESC "Out of the forest appears a large, rocky hole in the ground. A cave. You could GO DOWN.")
      (NORTH "There's nothing but dense forest")
      (EAST TO FOREST-4)
      (SOUTH TO FOREST-4)
      (WEST TO FOREST-6)>

<ROUTINE TIME-PASSES (NEW-W) 
      <COND (
            <IS-DES <GET-VAR TIME> 0>
            <SET-VAR TIME <ADD <GET-VAR TIME> 1>>
            <COND (
                  <IS-EQUAL <GET-VAR TIME> 34>
                  <SET-VAR TIME -1>
                  <COND (
                        ;"thunder starts at night"
                        <IS-EQUAL <GET-VAR WEATHER> 0>
                        <SET-VAR DRY 0>
                  )>
            )>
      )(
            <IS-ASC <GET-VAR TIME> 0>
            <SET-VAR TIME <ADD <GET-VAR TIME> -1>>
            <COND (
                  <IS-EQUAL <GET-VAR TIME> -18>
                  <SET-VAR TIME 1>
            )(
                  <IS-EQUAL <GET-VAR TIME> -8>
                  <SET-VAR DAY <ADD <GET-VAR DAY> 1>>
                  <SET-VAR MOON <ADD <GET-VAR MOON> 1>>
                  <COND (
                        <IS-EQUAL <GET-VAR MOON> 24>
                        <SET-VAR MOON 0>
                  )>
                  <SET-VAR NEW-W <RAND>>
                  <COND (
                        <IS-DES 30 <GET-VAR NEW-W>>
                        <SET-VAR WEATHER 3>
                        <SET-VAR DRY <ADD <GET-VAR DRY> 1>>
                  )(
                        <IS-DES 60 <GET-VAR NEW-W>>
                        <SET-VAR WEATHER 2>
                        <SET-VAR DRY <ADD <GET-VAR DRY> 1>>
                  )(
                        <IS-DES 90 <GET-VAR NEW-W>>
                        <SET-VAR WEATHER 1>
                        <SET-VAR DRY 0>
                  )(
                        <IS-DES 100 <GET-VAR NEW-W>>
                        <SET-VAR WEATHER 0>
                        <SET-VAR DRY <ADD <GET-VAR DRY> 1>>
                  )>
            )> 
      )>
>

<ROUTINE V-DETRITUS-1-PRSO ()
      <COND (
            <IS-EQUAL CMD-PRSA "UNPACK">
            <EACH-OBJ DETRITUS-1 (OBJ)
                  <COND(
                        <NOT <IS-EQUAL <GET-VAR OBJ NO-TAKE> 1>>
                        <MOVE OBJ CURRENT-ROOM>
                  )>
            >
            <TELL "Unpackable items are now on the ground." CR>
      )>
>

<ROUTINE F-DESC-FOREST-1 ()
      <COND (
            <IS-EQUAL <GET-VAR FOREST-1 FIRST-TIME> 1>
            <SET-VAR FOREST-1 FIRST-TIME 0>
            <TELL "It's the end of summer. You're in a dense forest. Birds and other
            small creatures can be heard rustling through the undergrowth and the
            smell of pine hangs thick in the air. Light filters down through
            needles and leaves." CR>
            <V-WEATHER-REPORT>
            <TELL "There's a trail up ahead, it looks like you could GO NORTH" CR>
      )(
            <IS-EQUAL <GET-VAR DETAILED-DESC> 1>
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

<ROUTINE V-WEATHER-REPORT (IS-NIGHT)
      <COND (
            <IS-DES <GET-VAR TIME> 48> ;"evening"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's evening, ">
      )(
            <IS-DES <GET-VAR TIME> 32> ;"afternoon"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's the afternoon, ">
      )(
            <IS-DES <GET-VAR TIME> 16> ;"morning"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's morning, ">
      )(
            <IS-DES <GET-VAR TIME> 0> ;"early morning"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's early morning, ">
      )(
            <IS-ASC <GET-VAR TIME> -16> ;"night"
            <SET-VAR IS-NIGHT 1>
            <TELL "It's night, ">
      )(
            <IS-ASC <GET-VAR TIME> 0> ;"late night"
            <SET-VAR IS-NIGHT 1>
            <TELL "It's late at night, ">
      )>
      <COND (
            <IS-EQUAL <GET-VAR MOON> 0> ;"new"
            <TELL "new moon, ">
      )(
            <AND <IS-DES <GET-VAR MOON> 0> <IS-ASC <GET-VAR MOON> 12>> ;"waxing"
            <TELL "the moon is waxing, ">
      )(
            <IS-EQUAL <GET-VAR MOON> 12> ;"full"
            <TELL "full moon, ">
      )(
            <AND <IS-DES <GET-VAR MOON> 12> <IS-ASC <GET-VAR MOON> 24>> ;"waning"
            <TELL "the moon is waning, ">
      )>
      <COND (
            <IS-EQUAL <GET-VAR WEATHER> 3> ;"sunny"
            <COND (
                  <IS-EQUAL <GET-VAR IS-NIGHT> 0>
                  <TELL "and there's a clear blue sky overhead. ">
            )(
                  <IS-EQUAL <GET-VAR IS-NIGHT> 1>
                  <TELL "and there are thousands of stars above you. ">

            )>
      )(
            <IS-EQUAL <GET-VAR WEATHER> 2> ;"cloudy"
            <COND (
                  <IS-EQUAL <GET-VAR IS-NIGHT> 0>
                  <TELL "but a dim, overcast sky presses down on you. ">
            )(
                  <IS-EQUAL <GET-VAR IS-NIGHT> 1>
                  <TELL "but no light penetrates through the cloudy sky. ">
            )>
      )(
            <IS-EQUAL <GET-VAR WEATHER> 1> ;"rainy"
            <COND (
                  <IS-EQUAL <GET-VAR IS-NIGHT> 0>
                  <TELL "a persistent drizzle dampens everything around. ">
            )(
                  <IS-EQUAL <GET-VAR IS-NIGHT> 1>
                  <TELL "heavy raindrops fall haphazardly. ">
            )>
      )(
            <IS-EQUAL <GET-VAR WEATHER> 0> ;"thunder"
            <COND (
                  <IS-EQUAL <GET-VAR IS-NIGHT> 0>
                  <TELL "storm clouds are building. ">
            )(
                  <IS-EQUAL <GET-VAR IS-NIGHT> 1>
                  <TELL "lightning and thunder beat down savagely around you. ">
            )>
      )>
      <COND (
            <IS-ASC <GET-VAR DRY> 1>
            ;"do nothing, it's actively raining (weather 1 or night of weather 0)"
      )(
            <IS-ASC <GET-VAR DRY> 2>
            <TELL "There are still puddles on the ground from recent rain." CR>
      )(
            <IS-ASC <GET-VAR DRY> 4>
            <TELL "The ground is still damp from rain." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "It's been " <GET-VAR DRY> " days since the last rain." CR>
      )>
>

<ROUTINE F-DESC-CABIN-EXTERIOR ()
      <COND (
            <IS-EQUAL <GET-VAR CABIN-EXTERIOR FIRST-TIME> 1>
            <SET-VAR CABIN-EXTERIOR FIRST-TIME 0>
            <TELL "You're at the transition between a forest and a field.
            There are trails in all directions, but at the center a cabin!
            It's got a scenic window facing the fields. You could try to
            OPEN DOOR" CR>
            <RETURN 1>
      )>
      <TELL "You're outside a cabin on the edge of a forest, overlooking some fields." CR>
      <COND (
            <IS-EQUAL <GET-VAR CABIN-DOOR IS-LOCKED> 1>
            <TELL "The cabin door is locked. Where's the key?" CR>
      )>
>

<ROUTINE V-DESC-ROOM ()
      <DESC CURRENT-ROOM>
>

<ROUTINE V-ROOM-DETAIL () 
      <SET-VAR DETAILED-DESC 1>
      <DESC CURRENT-ROOM>
      <SET-VAR DETAILED-DESC 0>
>

<ROUTINE V-DESC-OBJECTS-IN-ROOM (COUNT)
      <COND (
            <IS-EQUAL <GET-VAR FIRST-LOOK-AROUND> 1>
            <SET-VAR FIRST-LOOK-AROUND 0>
            <TELL "This command lists interactable objects in the immediate vicinity.
            Interactions may not be obvious." CR>
            <TELL "Objects nested inside other objects are not listed, but might show up
            if you EXAMINE their container." CR>
      )>
      <EACH-OBJ CURRENT-ROOM (OBJ)
            <SET-VAR COUNT <ADD <GET-VAR COUNT> 1>>
            <DESC OBJ>
            <TELL CR>
      >
      <COND (
            <IS-EQUAL COUNT 0>
            <TELL "This space appears to be empty." CR>
      )>
>

<ROUTINE V-INVENTORY (COUNT)
      <EACH-OBJ PLAYER (OBJ)
            <SET-VAR COUNT <ADD <GET-VAR COUNT> 1>>
            <DESC OBJ>
            <TELL CR>
      >
      <COND (
            <IS-EQUAL COUNT 0>
            <TELL "There's nothing in your inventory." CR>
      )>
>

<ROUTINE F-CABIN-DOOR-PRSO ()
      <COND (
            <IS-EQUAL CMD-PRSA "OPEN">
            <COND (
                  <AND <IS-IN PLAYER CABIN-DOOR-KEY> <IS-EQUAL <GET-VAR CABIN-DOOR IS-LOCKED> 1>>
                  <SET-VAR CABIN-DOOR IS-LOCKED 0>
                  <TELL "The key works!" CR>
                  <MOVE PLAYER CABIN>
            )(
                  <IS-EQUAL <GET-VAR CABIN-DOOR IS-LOCKED> 0>
                  <MOVE PLAYER CABIN>
            )(
                  <IS-EQUAL 1 1>
                  <TELL "The door is locked. You should LOOK AROUND for a key." CR> 
            )>
      )>
>

<ROUTINE V-THROW-AT ()
      <TELL "// TODO" CR>
>

<ROUTINE V-SPARK-AT ()
      <TELL "Spark it at what?" CR>
>

<ROUTINE V-HIT-WITH ()
      <TELL "// TODO" CR>
>

<ROUTINE V-OPEN ()
      ;"do nothing, let the object's ACT-PRSO handle it"
>

<ROUTINE V-CLOSE ()
      <TELL "// TODO" CR>
>

<ROUTINE V-EXAMINE (COUNT R)
      <SET-VAR DETAILED-DESC 1>
      <DESC CMD-PRSO>
      <SET-VAR DETAILED-DESC 0> 
      <TELL CR>

      <COND(
            <IS-EQUAL FIRST-EXAMINE 1>
            <SET-VAR FIRST-EXAMINE 0>
            <TELL "If there's any interesting item in this object, you can
            UNPACK it to remove the nested items. After that, can
            TAKE the item off the ground." CR>
      )>

      <TELL "...contains:" CR>
      <EACH-OBJ CMD-PRSO (OBJ) 
            <DESC OBJ>
            <TELL CR>
            <SET-VAR COUNT <ADD COUNT 1>>
      >
      <COND (
            <IS-EQUAL COUNT 0>
            <SET-VAR R <RAND>>
            <COND (
                  <IS-DES 10 R>
                  <TELL "a whole lotta nuthin" CR>
            )(
                  <IS-DES 30 R>
                  <TELL "zilch" CR>
            )(
                  <IS-DES 50 R>
                  <TELL "nada" CR>
            )(
                  <IS-DES 100 R>
                  <TELL "nothing" CR>
            )>
      )>
>

<ROUTINE V-EAT ()
      <TELL "// TODO" CR>
>

<ROUTINE V-PUT-IN ()
      <TELL "// TODO" CR>
>

<ROUTINE V-TALK-TO ()
      <TELL "// TODO" CR>
>

<ROUTINE V-SING ()
      <TELL "// TODO" CR>
>

<ROUTINE V-WHISPER ()
      <TELL "// TODO" CR>
>

<ROUTINE V-TAKE ()
      <COND (
            <IS-EQUAL <GET-VAR CMD-PRSO NO-TAKE> 1>
            <TELL "This can't be picked up." CR>
      )(
            <IS-EQUAL 1 1>
            <MOVE CMD-PRSO PLAYER>
            <TELL "Picked up!" CR>
      )>
>

<ROUTINE V-TAKE-OUT ()
      ;"do nothing, let the object's ACT-PRSO handle it"
>

<ROUTINE V-PEE-ON ()
      <TELL "// TODO" CR>
>

<ROUTINE V-COOK ()
      <TELL "// TODO" CR>
>

<ROUTINE V-WRITE-NOTE ()
      <TELL "// TODO" CR>
>

<ROUTINE V-SLEEP ()
;"if night, sleep until morning"
;"if day, sleep for 2 hours"
      <TELL "// TODO" CR>
>

<ROUTINE V-DROP ()
      <TELL "// TODO" CR>
>

<ROUTINE V-WORK-WITH ()
      <TELL "// TODO" CR>
>

<ROUTINE V-BAIT ()
      <COND (
            <NOT <IS-EQUAL <GET-VAR CMD-PRSO CAN-BAIT> 1>>
            <TELL "Can't bait that" CR>
            <RETURN 0>
      )>
      <SET-VAR CMD-PRSO BAITED 1>
      <TELL "Baited, SET OBJECT DOWN to see if it will catch anything." CR>
      <RETURN 1>
>

<ROUTINE V-SWIM ()
      <TELL "The water looks a little chilly, it would be better if we had a boat, or maybe built one?" CR>
>

<ROUTINE V-JUMP ()
      <TELL "No thank you" CR>
>

<ROUTINE V-WHERE-TO-GO ()
      <TELL "There's only one way to find out. Directions to try are: UP DOWN IN OUT NORTH SOUTH EAST WEST." CR>
>

<BUZZ A OF ALL SOME THE AGAIN>

<SYNTAX WHERE AM I = V-ROOM-DETAIL>
<SYNTAX LOOK AROUND = V-DESC-OBJECTS-IN-ROOM>
<SYNTAX INVENTORY = V-INVENTORY>
<SYNTAX WEATHER = V-WEATHER-REPORT>
<SYNTAX SLEEP = V-SLEEP>

<SYNTAX EXAMINE OBJECT = V-EXAMINE>
<SYNONYM EXAMINE INSPECT READ INVESTIGATE>
<SYNTAX TAKE OBJECT = V-TAKE>
<SYNONYM TAKE GATHER GET>
<SYNTAX UNPACK OBJECT = V-TAKE-OUT>
<SYNTAX DROP OBJECT = V-DROP>
<SYNTAX PUT OBJECT INTO OBJECT = V-PUT-IN>
<SYNONYM PUT PLACE POUR>
<SYNTAX FILL OBJECT WITH OBJECT = V-PUT-IN>
<SYNTAX THROW OBJECT AT OBJECT = V-THROW-AT>
<SYNTAX THROW OBJECT OVER OBJECT = V-THROW-AT>
<SYNTAX OPEN OBJECT = V-OPEN>
<SYNTAX CLOSE OBJECT = V-CLOSE>
<SYNTAX HIT OBJECT WITH OBJECT = V-HIT-WITH>
<SYNTAX COOK OBJECT = V-COOK>
<SYNONYM COOK BOIL ROAST>
<SYNTAX EAT OBJECT = V-EAT>
<SYNONYM EAT TASTE LICK DRINK IMBIBE>
<SYNTAX WORK ON OBJECT WITH OBJECT = V-WORK-WITH>

<SYNTAX SPARK FLINT AT OBJECT = V-SPARK-AT>
<SYNTAX ENTER CABIN = CAN-ENTER-CABIN>
<SYNTAX BAIT OBJECT = V-BAIT>
<SYNTAX TALK TO OBJECT = V-TALK-TO>
<SYNTAX SING OBJECT = V-SING>
<SYNTAX WHISPER OBJECT = V-WHISPER>
<SYNTAX PEE ON OBJECT = V-PEE-ON>
<SYNTAX WRITE NOTE = V-WRITE-NOTE>

<SYNTAX WHERE CAN I GO = V-WHERE-TO-GO>
<SYNTAX SWIM = V-SWIM>
<SYNTAX DIVE IN = V-SWIM>
<SYNTAX JUMP IN = V-JUMP>
<SYNTAX JUMP DOWN = V-JUMP>
