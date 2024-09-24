;"It's the end of summer..."

<DIRECTIONS UP DOWN NORTH EAST SOUTH WEST>

;"3 = sunny, 2 = cloudy, 1 = rainy, 0 = thunder"
<GLOBAL WEATHER 3>

;"aka days since last rain"
<GLOBAL DRY 2>

;"0 = new, 1-3 = waxing, 4 = full, 5-7 = waning"
<GLOBAL MOON-PHASE 1>

;"starts at 1 and just keeps going"
<GLOBAL DAY 1>

;"night starts at -1 and goes to -17, day starts at 1 and goes to 33"
;"2 ticks per hour, 24 hours in a day"
<GLOBAL TIME 1>

;"Max number of items the player can carry"
<GLOBAL P-CAPACITY 10>

;"count all the sleeps a player has taken (at night)"
<GLOBAL TOTAL-SLEEPS 0>

;"count all the sleeps a player has taken (during this day)"
<GLOBAL NAPS-TODAY 0>

;"count all the food a player has eaten"
<GLOBAL TOTAL-FOOD 0>

;"good for storing objects that can be cloned"
<ROOM STORAGE>

<PLAYER 
      (ROOM FOREST-1)
      (ACT-ENTER <PLAYER-ACT-ENTER>)
      (ACT-ALWAYS <PLAYER-ALWAYS>)
      (VARS HEALTH 40 MAX-HEALTH 50)>

<OBJECT AXE 
      (AKA AXE AX)
      (DESC "an AXE")
      (COPY <PLAYER>)
      (VARS DAMAGE 3 MAX-DAMAGE 5 HEALTH 10 MAX-HEALTH 10)>

<OBJECT CLOAK 
      (AKA CLOAK COAT)
      (DESC "a CLOAK")
      (COPY <PLAYER>)>

<OBJECT FLINT 
      (AKA FLINT)
      (DESC "FLINT")
      (COPY <PLAYER>)>

<OBJECT CUP 
      (AKA CUP)
      (DESC "a CUP")
      (COPY <PLAYER>)>

<OBJECT KETTLE 
      (AKA KETTLE POT)
      (DESC "a KETTLE")
      (COPY <PLAYER>)>

<OBJECT KNIFE 
      (AKA KNIFE)
      (DESC "a KNIFE")
      (COPY <PLAYER>)
      (VARS DAMAGE 2 MAX-DAMAGE 2 HEALTH 10 MAX-HEALTH 10)>

<OBJECT WATER
      (DESC "water")
      ;(ACT-PRSI <PRSI-WATER>)>

<ROUTINE PLAYER-ALWAYS () 
      <BURN-FIRE>
      <INVENTORY-LIMIT>
      <TIME-PASSES>
      <CHECK-PULSE>
>

<ROUTINE CHECK-PULSE () 
      <COND (
            <IS-ASC <GET-VAR PLAYER HEALTH> 1>
            <TELL "You are dead" CR>
            ;"TODO: tell objectives"
      )>
>

<ROUTINE INVENTORY-LIMIT (COUNT)
      <EACH-OBJ PLAYER (OBJ)
            <SET-VAR COUNT <ADD COUNT 1>>
      >
      <COND (
            <IS-DES COUNT 7>
            <TELL "You're trying to carry too much. Some items fall on the ground." CR>
            <EACH-OBJ PLAYER (OBJ)
                  <COND (
                        <IS-DES COUNT 7>
                        <MOVE OBJ C-ROOM>
                  )>
                  <SET-VAR COUNT <SUBTRACT COUNT 1>>
            >
      )>
> 

<ROUTINE BURN-FIRE () 
      ;"decrement all instances of fire and lit torches"
      ;"if zero, it disappears (burned up)"

      <EACH-OBJ FIRE (OBJ)
            <COND (
                  ;"if in cabin and door and window not broken, subtract 1"
                  <AND
                        <IS-IN CABIN OBJ>
                        ;"BUG: should be CABIN-EXTERIOR, not CABIN, but I want to see what happens"
                        <IS-ASC <GET-VAR <INST CABIN CABIN-DOOR> HEALTH> 1>
                        <IS-ASC <GET-VAR <INST CABIN CABIN-WINDOW> HEALTH> 1>
                  >
                  <SET-VAR OBJ FUEL <SUBTRACT <GET-VAR OBJ FUEL> 1>>
            )(
                  ;"if above-ground, subtract 2"
                  <IS-EQUAL <GET-VAR <LOC OBJ N> ABOVE-GROUND> 1>
                  <SET-VAR OBJ FUEL <SUBTRACT <GET-VAR OBJ FUEL> 2>>
                  <COND (
                        <AND 
                              <IS-IN CABIN OBJ>
                              <OR 
                                    <IS-IN CABIN PLAYER>
                                    <IS-IN CABIN-EXTERIOR PLAYER>
                              >
                        >
                        <TELL "The cabin is drafty from a broken door or window, and fire burns more quickly." CR> 
                  )>
            )(
                  <IS-EQUAL 1 1>
                  <SET-VAR OBJ FUEL <SUBTRACT <GET-VAR OBJ FUEL> 1>>
            )>

            <COND (
                  <IS-ASC <GET-VAR OBJ FUEL> 1>
                  <MOVE OBJ>
                  <COND (
                        <OR 
                              <IS-IN PLAYER OBJ>
                              <IS-IN C-ROOM OBJ>
                        >
                        <TELL "The fire dies." CR>
                  )>
            )(
                  <AND 
                        <IS-EQUAL <GET-VAR <LOC OBJ> ABOVE-GROUND> 1>
                        <NOT <IS-IN CABIN OBJ>>
                        <IS-DES DRY 3>
                  >
                  <TELL "It hasn't rained in awhile. Fire catches, and the forest burns down." CR>
                  <SET-VAR FOREST-BURNED-DOWN 1>
                  <COND (
                        <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 1>
                        <TELL "You die." CR>
                        <SET-VAR PLAYER HEALTH -1>
                        <RETURN 1>
                  )>
            )>
      >

      <EACH-OBJ TORCH (OBJ)
            <COND (
                  <IS-EQUAL <GET-VAR OBJ LIT> 1>
                  <SET-VAR OBJ FUEL <SUBTRACT <GET-VAR OBJ FUEL> 1>>
                  <COND ( 
                        <IS-ASC <GET-VAR OBJ FUEL> 1>
                        <MOVE OBJ>
                        <COND (
                              <OR 
                                    <IS-IN PLAYER OBJ>
                                    <IS-IN C-ROOM OBJ>
                              >
                              <TELL "The fire dies." CR>
                        )>
                  )(
                        <AND 
                              <IS-EQUAL <GET-VAR <LOC OBJ> ABOVE-GROUND> 1>
                              <NOT <IS-IN CABIN OBJ>>
                              <IS-DES DRY 3>
                        >
                        <TELL "It hasn't rained in awhile. Fire catches, and the forest burns down." CR>
                        <SET-VAR FOREST-BURNED-DOWN 1>
                        <COND (
                              <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 1>
                              <TELL "You die." CR>
                              <SET-VAR PLAYER HEALTH -1>
                              <RETURN 1>
                        )>
                  )>
            )>
      >
>

<ROUTINE TIME-PASSES (NEW-W) 
      <COND (
            ;"if is daytime"
            <IS-DES TIME 0>
            <SET-VAR TIME <ADD TIME 1>>
            <COND (
                  <IS-EQUAL TIME 34>
                  <SET-VAR TIME -1>
                  <COND (
                        ;"thunder starts at night"
                        <IS-EQUAL WEATHER 0>
                        <SET-VAR DRY 0>
                  )>
                  <TELL "Night falls." CR>
                  <CHECK-SLEEP>
                  <CHECK-HEALTH>
            )>
      )(
            ;"if is nighttime"
            <IS-ASC TIME 0>
            <SET-VAR TIME <ADD TIME -1>>
            <COND (
                  <IS-EQUAL TIME -18>
                  <SET-VAR TIME 1>
                  <TELL "Morning comes." CR>
                  <CHECK-FOOD>
                  <CHECK-HEALTH>
            )(
                  ;"midnight"
                  <IS-EQUAL TIME -8>
                  <SET-VAR NAPS-TODAY 0>
                  <SET-VAR DAY <ADD DAY 1>>
                  <SET-VAR MOON-PHASE <ADD MOON-PHASE 1>>
                  <COND (
                        <IS-EQUAL MOON-PHASE 8>
                        <SET-VAR MOON-PHASE 0>
                  )(
                        <IS-EQUAL MOON-PHASE 4>
                        <TELL "It's a full moon. Be careful, all your weapon and tool actions are more erratic." CR>
                  )>
                  <SET-VAR NEW-W <RAND>>
                  <COND (
                        <IS-DES 30 NEW-W>
                        <SET-VAR WEATHER 3>
                        <SET-VAR DRY <ADD DRY 1>>
                  )(
                        <IS-DES 60 NEW-W>
                        <SET-VAR WEATHER 2>
                        <SET-VAR DRY <ADD DRY 1>>
                  )(
                        <IS-DES 90 NEW-W>
                        <SET-VAR WEATHER 1>
                        <SET-VAR DRY 0>
                  )(
                        <IS-DES 100 NEW-W>
                        <SET-VAR WEATHER 0>
                        <SET-VAR DRY <ADD DRY 1>>
                  )>
            )> 
      )>
>

<ROUTINE WEATHER-REPORT (IS-NIGHT UNTIL-FULL)
      <COND (
            <IS-DES TIME 48> ;"evening"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's evening, ">
      )(
            <IS-DES TIME 32> ;"afternoon"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's the afternoon, ">
      )(
            <IS-DES TIME 16> ;"morning"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's morning, ">
      )(
            <IS-DES TIME 0> ;"early morning"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's early morning, ">
      )(
            <IS-ASC TIME -16> ;"night"
            <SET-VAR IS-NIGHT 1>
            <TELL "It's night, ">
      )(
            <IS-ASC TIME 0> ;"late night"
            <SET-VAR IS-NIGHT 1>
            <TELL "It's late at night, ">
      )>
      <COND (
            <IS-EQUAL MOON-PHASE 0> ;"new"
            <TELL "new moon, ">
      )(
            <AND <IS-DES MOON-PHASE 0> <IS-ASC MOON-PHASE 4>> ;"waxing"
            <SET-VAR UNTIL-FULL <SUBTRACT 3 MOON-PHASE>>
            <TELL "the moon is waxing (" UNTIL-FULL " day">
            <COND (
                <NOT <IS-EQUAL UNTIL-FULL 1>>
                <TELL "s">
            )>
            <TELL " until full), ">

      )(
            <IS-EQUAL MOON-PHASE 4> ;"full"
            <TELL "full moon, ">
      )(
            <AND <IS-DES MOON-PHASE 4> <IS-ASC MOON-PHASE 8>> ;"waning"
            <TELL "the moon is waning, ">
      )>
      <COND (
            <IS-EQUAL WEATHER 3> ;"sunny"
            <COND (
                  <IS-EQUAL IS-NIGHT 0>
                  <TELL "and there's a clear blue sky overhead. ">
            )(
                  <IS-EQUAL IS-NIGHT 1>
                  <TELL "and there are thousands of stars above you. ">

            )>
      )(
            <IS-EQUAL WEATHER 2> ;"cloudy"
            <COND (
                  <IS-EQUAL IS-NIGHT 0>
                  <TELL "but a dim, overcast sky presses down on you. ">
            )(
                  <IS-EQUAL IS-NIGHT 1>
                  <TELL "but no light penetrates through the cloudy sky. ">
            )>
      )(
            <IS-EQUAL WEATHER 1> ;"rainy"
            <COND (
                  <IS-EQUAL IS-NIGHT 0>
                  <TELL "a persistent drizzle dampens everything around. ">
            )(
                  <IS-EQUAL IS-NIGHT 1>
                  <TELL "heavy raindrops fall haphazardly. ">
            )>
      )(
            <IS-EQUAL WEATHER 0> ;"thunder"
            <COND (
                  <IS-EQUAL IS-NIGHT 0>
                  <TELL "storm clouds are building. ">
            )(
                  <IS-EQUAL IS-NIGHT 1>
                  <TELL "lightning and thunder beat down savagely around you. ">
            )>
      )>
      <COND (
            <IS-ASC DRY 1>
            ;"do nothing, it's actively raining (weather 1 or night of weather 0)"
      )(
            <IS-ASC DRY 2>
            <TELL "There are still puddles on the ground from recent rain." CR>
      )(
            <IS-ASC DRY 4>
            <TELL "The ground is still damp from rain." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "It's been " DRY " days since the last rain." CR>
      )>
>

<ROUTINE CHECK-HEALTH () 
      <COND (
            <IS-ASC <GET-VAR PLAYER HEALTH> 1>
            <TELL "You are dead." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You have " <GET-VAR PLAYER HEALTH> " health." CR>
      )>
>

<ROUTINE CHECK-FOOD (E)
      <COND (
            <IS-ASC <GET-VAR PLAYER HEALTH> 1>
            <RETURN 1>
      )>
      <SET-VAR E <SUBTRACT TOTAL-FOOD <MULTIPLY DAY 2>>>
      <COND (
            <IS-ASC E -5>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 20>>
            <TELL "You lost 20 health from hunger." CR>
      )(
            <IS-ASC E -3>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 10>>
            <TELL "You lost 10 health from hunger." CR>
      )(
            <IS-ASC E 0>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 2>>
            <TELL "You lost 2 health from hunger." CR>
      )(
            <IS-DES E 6>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> <ADD E DAY>>>
            <TELL "You lost some health from over-eating." CR>
      )>
>

<ROUTINE CHECK-SLEEP (Z)
      <COND (
            <IS-DES NAPS-TODAY 5>
            <TELL "You lost 10 health from napping too much today, but also gained 1 sleep" CR>
            <SET-VAR TOTAL-SLEEPS <ADD TOTAL-SLEEPS 1>> 
      )(
            <IS-DES NAPS-TODAY 2>
            <TELL "You gained 1 sleep from the naps you took today" CR>
            <SET-VAR TOTAL-SLEEPS <ADD TOTAL-SLEEPS 1>> 
      )>
      <SET-VAR Z <SUBTRACT TOTAL-SLEEPS DAY>>
      <COND (
            <IS-ASC Z -2>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 40>>
            <TELL "You lost 40 health from lack of sleep." CR>
      )(
            <IS-ASC Z -1>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 20>>
            <TELL "You lost 20 health from lack of sleep." CR>
      )(
            <IS-ASC Z 0>          
            <TELL "You will lose health if you don't get enough sleep." CR>
      )(
            <IS-DES Z 3>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 20>>
            <TELL "You lost 20 health from over-sleeping." CR>
      )>
>

<ROUTINE PLAYER-ACT-ENTER () 
      <V-DESC-ROOM>
      <COND (
            <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 0>
            ;"TODO"
            ;"if haven't had first encounter, can spawn monster"
            ;"if not carrying light, can spawn a cave-spider or two or three"
            ;"if not carrying light, can spawn the monster"
      )>
>

<SLEEP ()
;"if night, sleep until morning, increment TOTAL-SLEEPS, maybe dream"
;"if day, sleep for 2 hours, increment NAPS-TODAY"
      <TELL "// TODO" CR>
>

<ROUTINE PRSI-WATER ()
      <COND (
            <OR <IS-EQUAL CMD "ADD"> <IS-EQUAL CMD "PUT">>
            <COND (
                  <IS-EQUAL <CMD 1> WATER>
                  <TELL "The water gets more watery." CR>
                  <MOVE  <CMD 1>>
                  <RETURN 1>
            )(
                  <IS-EQUAL <CMD 1> TORCH>
                  <SET-VAR <CMD 1> LIT 0>
                  <TELL "The torch hisses and dies as it lands in the water." CR>
            )(
                  <IS-EQUAL <CMD 1> RIVER-STONE>
                  <SET-VAR <CMD 1> WET 10>
                  <TELL "The river stone is wet. You could WORK KNIFE WITH STONE to sharpen it." CR>
            )(
                  <IS-EQUAL 1 1>
                  <TELL "You put it in the water." CR>
            )>
            <MOVE <CMD 1> <CMD 2>>
      )(
            <IS-EQUAL CMD "FILL">
            <COND (
                  <OR
                        <IS-EQUAL <CMD 1> KETTLE>
                        <IS-EQUAL <CMD 1> BUCKET>
                        <IS-EQUAL <CMD 1> CUP>
                        <IS-EQUAL <CMD 1> TREE-HOLLOW>
                        <IS-EQUAL <CMD 1> BOAT>
                  >
                  <COPY-MOVE WATER  <CMD 1>>
                  <TELL "You fill it with water." CR>
            )>
      )>
>
