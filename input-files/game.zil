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

;"aka day modulo 3"
<GLOBAL DAY-M-3 0>

;"night starts at -1 and goes to -17, day starts at 1 and goes to 33"
;"2 ticks per hour, 24 hours in a day"
<GLOBAL TIME 1>

;"count all the sleeps a player has taken (at night)"
<GLOBAL TOTAL-SLEEPS 0>

;"count all the sleeps a player has taken (during this day)"
<GLOBAL NAPS-TODAY 0>

;"count all the food a player has eaten"
<GLOBAL TOTAL-FOOD 0>

;"max max-health the player can have"
<GLOBAL MAX-MAX-HEALTH 100>

;"explain more then first time we hit the inventory limit"
<GLOBAL FIRST-INVENTORY-LIMIT 1>

;"win conditions aka objectives aka progress, reported (recap) when the player dies or wins"
<GLOBAL WIN-ENTER-CABIN 0>
<GLOBAL WIN-LIGHT-FIRE 0>
<GLOBAL WIN-COOK-MEAL 0>
<GLOBAL WIN-FIND-GEM 0>
<GLOBAL WIN-ENTER-CAVE 0>
<GLOBAL WIN-KILL-MONSTER 0>
;<GLOBAL WIN-WRITE-NOTE 0> ;"is calculated at the end"
<GLOBAL WIN-BUILD-BOAT 0>

;"if the user used CHEAT"
<GLOBAL WIN-CHEAT 0>

;"good for storing objects that can be cloned"
<ROOM STORAGE>

<PLAYER 
      (ROOM FOREST-1)
      (ACT-ENTER <PLAYER-ACT-ENTER>)
      (ACT-ALWAYS <PLAYER-ALWAYS>)
      (VARS HEALTH 45 MAX-HEALTH 50)>

<OBJECT AXE 
      (AKA AXE AX)
      (DESC <DESC-AXE>)
      (COPY <PLAYER>)
      (VARS DAMAGE 8 MAX-DAMAGE 10 HEALTH 10 MAX-HEALTH 10)>

<OBJECT CLOAK 
      (AKA CLOAK COAT)
      (DESC <DESC-CLOAK>)
      (COPY <PLAYER>)>

<OBJECT FLINT 
      (AKA FLINT)
      (DESC <DESC-FLINT>)
      (COPY <PLAYER>)>

<OBJECT CUP 
      (AKA CUP)
      (DESC <DESC-CUP>)
      (COPY <PLAYER>)>

<OBJECT KETTLE 
      (AKA KETTLE POT)
      (DESC <DESC-KETTLE>)
      (VARS IS-HARD 1)
      (COPY <PLAYER>)>

<OBJECT KNIFE 
      (AKA KNIFE)
      (DESC <DESC-KNIFE>)
      (COPY <PLAYER>)
      (VARS DAMAGE 4 MAX-DAMAGE 4 HEALTH 10 MAX-HEALTH 10)>

<OBJECT WATER
      (AKA WATER)
      (DESC "WATER")
      (COPY <ROOM LAKE-1>
            <ROOM CAVE-LAKE>)
      (VARS OWN-TAKE 1)>

<ROUTINE PLAYER-ALWAYS () 
      <COND (
            <NOT <IS-EQUAL <CHECK-PULSE> 1>>
            <WEATHER-REPORT>
            <TELL "Good game! Thank you for playing" CR>
            <END>
      )(
            <IS-EQUAL 1 1>
            <INVENTORY-LIMIT>
            <CLEAR-QUESTIONS>
            <MONSTER-HEALTH>
            <BURN-FIRE>
            <CHECK-PULSE>
            <TIME-PASSES>
      )>
>

<ROUTINE CHECK-PULSE () 
      <COND (
            <NOT <IS-DES <GET-VAR PLAYER HEALTH> 0>>
            <TELL "You have died." CR>
            <RECAP>
            <RETURN 0>
      )>
      <RETURN 1>
>

<ROUTINE INVENTORY-LIMIT (COUNT)
      <EACH-OBJ PLAYER (OBJ)
            <SET-VAR COUNT <ADD COUNT 1>>
      >
      <COND (
            <IS-DES COUNT 7>
            <TELL "You're trying to carry too much. Some items fall on the ground." CR>
            <COND (
                  <IS-EQUAL FIRST-INVENTORY-LIMIT 1>
                  <TELL "Can DROP items you don't want, and TAKE the ones you do." CR>
                  <SET-VAR FIRST-INVENTORY-LIMIT 0>
            )>
            <EACH-OBJ PLAYER (OBJ)
                  <COND (
                        <IS-DES COUNT 7>
                        <MOVE OBJ C-ROOM>
                  )>
                  <SET-VAR COUNT <SUBTRACT COUNT 1>>
            >
      )>
> 

<ROUTINE CLEAR-QUESTIONS () 
      <EACH-OBJ Q-STORAGE (OBJ)
            <COND (
                  <IS-EQUAL YN-Q OBJ>
                  <COND (
                        <IS-DES <GET-VAR OBJ EXP> 0>
                        <SET-VAR OBJ EXP <SUBTRACT <GET-VAR OBJ EXP> 1>>
                  )(
                        <IS-EQUAL <GET-VAR OBJ EXP> 0>
                        <COND (
                              <AND
                                    <IS-EQUAL <GET-VAR OBJ FROM-BEAR> 1>
                                    <IS-IN C-ROOM BEAR>
                              >
                              <TELL "The bear takes your silence as an answer, and sits down to forage" CR>
                        )>
                        <MOVE OBJ>
                  )>
            )>
      >
>

<ROUTINE BURN-FIRE () 
      ;"decrement all instances of fire and lit torches"
      ;"if zero, it disappears (burned up)"

      <EACH-OBJ FIRE (OBJ)
            <COND (
                  ;"if in cabin and door and window not broken, subtract 1"
                  <AND
                        <IS-IN CABIN OBJ>
                        <IS-ASC <GET-VAR <INST CABIN-EXTERIOR CABIN-DOOR> HEALTH> 1>
                        <IS-ASC <GET-VAR <INST CABIN-EXTERIOR CABIN-WINDOW> HEALTH> 1>
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
                        <COND (
                              <IS-EQUAL FIRE-IN-DRAFTY-CABIN 1>
                              <TELL "The cabin is drafty from a broken door or window, and fire burns more quickly." CR> 
                              <SET-VAR FIRE-IN-DRAFTY-CABIN 0>
                        )>
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
                  <BURN-FOREST-DOWN>
            )>
      >

      <EACH-OBJ TORCH (OBJ)
            <COND (
                  <IS-EQUAL <GET-VAR OBJ IS-LIT> 1>
                  <SET-VAR OBJ FUEL <SUBTRACT <GET-VAR OBJ FUEL> 1>>
                  <COND ( 
                        <IS-ASC <GET-VAR OBJ FUEL> 1>
                        <COND (
                              <IS-IN PLAYER OBJ>
                              <TELL "Your torch dies." CR>
                        )>
                        <MOVE OBJ>
                  )(
                        <AND 
                              <IS-EQUAL <GET-VAR <LOC OBJ N> ABOVE-GROUND> 1>
                              <NOT <IS-IN PLAYER OBJ N>>
                              <NOT <IS-IN CABIN OBJ N>>
                              <IS-DES DRY 3>
                        >
                        <BURN-FOREST-DOWN>
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

                  <COND (
                        <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 1>
                        <TELL "Night falls." CR>
                  )(
                        <IS-EQUAL 1 1>
                        <TELL "Outside, night falls." CR>
                  )>
                  
                  <CHECK-SLEEP>

                  <SET-VAR MOON-PHASE <ADD MOON-PHASE 1>>
                  <COND (
                        <IS-EQUAL MOON-PHASE 8>
                        <SET-VAR MOON-PHASE 0>
                  )(
                        <IS-EQUAL MOON-PHASE 4>
                        <TELL "It's a full moon. Be careful, all your weapon and tool actions are more erratic." CR>
                  )>
            )>
      )(
            ;"if is nighttime"
            <IS-ASC TIME 0>
            <SET-VAR TIME <ADD TIME -1>>
            <COND (
                  <IS-EQUAL TIME -18>
                  <SET-VAR TIME 1>
                  <COND (
                        <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 1>
                        <TELL "Morning breaks." CR>
                  )(
                        <IS-EQUAL 1 1>
                        <TELL "Outside, morning comes." CR>
                  )>
                  <CHECK-FOOD>
            )(
                  ;"midnight"
                  <IS-EQUAL TIME -8>
                  <SET-VAR NAPS-TODAY 0>
                  <SET-VAR DAY <ADD DAY 1>>
                  <SET-VAR DAY-M-3 <ADD DAY-M-3 1>>
                  <COND (
                        <IS-EQUAL DAY-M-3 3>
                        <SET-VAR DAY-M-3 0>
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
            <IS-DES TIME 23> ;"evening"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's evening, ">
      )(
            <IS-DES TIME 15> ;"afternoon"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's the afternoon, ">
      )(
            <IS-DES TIME 7> ;"morning"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's morning, ">
      )(
            <IS-DES TIME 0> ;"early morning"
            <SET-VAR IS-NIGHT 0>
            <TELL "It's early morning, ">
      )(
            <IS-ASC TIME -8> ;"late night"
            <SET-VAR IS-NIGHT 1>
            <TELL "It's late at night, ">
      )(
            <IS-ASC TIME 0> ;"night"
            <SET-VAR IS-NIGHT 1>
            <TELL "It's night, ">
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

<ROUTINE CHECK-FOOD (E)
      <COND (
            <NOT <IS-DES <GET-VAR PLAYER HEALTH> 0>>
            <RETURN 1>
      )>
      <SET-VAR E <SUBTRACT TOTAL-FOOD <MULTIPLY DAY 2>>>
      <COND (
            <IS-DES E 6>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> <ADD E DAY>>>
            <TELL "You lose some health from over-eating." CR>
      )(
            <IS-ASC <GET-VAR PLAYER HEALTH> 20>
            <TELL "EAT some food to re-gain health" CR>
      )>
>

<ROUTINE CHECK-SLEEP (Z)
      <COND (
            <IS-DES NAPS-TODAY 5>
            <TELL "You lose 10 health from napping too much today, but also gained 1 sleep" CR>
            <SET-VAR TOTAL-SLEEPS <ADD TOTAL-SLEEPS 1>> 
      )(
            <IS-DES NAPS-TODAY 2>
            <TELL "You gain 1 sleep from the naps you took today" CR>
            <SET-VAR TOTAL-SLEEPS <ADD TOTAL-SLEEPS 1>> 
      )>
      <SET-VAR Z <SUBTRACT TOTAL-SLEEPS DAY>>
      <COND (
            <IS-ASC Z -2>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 40>>
            <TELL "You lose 40 health from lack of sleep." CR>
      )(
            <IS-ASC Z -1>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 20>>
            <TELL "You lose 20 health from lack of sleep." CR>
      )(
            <IS-ASC Z 0> 
            <COND (
                  <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 1>
                  <TELL "You will lose health if you don't get enough SLEEP." CR>
            )>    
      )(
            <IS-DES Z 3>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 20>>
            <TELL "You lose 10 health from over-sleeping." CR>
      )>
>

<ROUTINE PLAYER-ACT-ENTER (HAS-LIGHT) 
      <V-DESC-ROOM>
      <SET-VAR HAS-LIGHT <ROOM-HAS-LIGHT>>

      ;"spawn child-monster"
      <COND (
            <IS-EQUAL CHILD-MONSTER-WILL-CHASE 1>
            <EACH-OBJ CHILD-MONSTER (OBJ)
                  ;"this is the only way to reliably get the child-monster instance"
                  <MOVE OBJ C-ROOM>
            >
      )(
            ;"if in den, good chance to spawn child monster"
            <AND
                  <OR
                        <IS-EQUAL C-ROOM DEN-1>
                        <IS-EQUAL C-ROOM DEN-2>
                        <IS-EQUAL C-ROOM DEN-3>
                        <IS-EQUAL C-ROOM DEN-4>
                  >
                  <IS-DES 45 <RAND>>
            >
            <MOVE <INST STORAGE CHILD-MONSTER> C-ROOM>
      )>

      ;"spawn parent-monster"
      <COND (
            <IS-EQUAL PARENT-MONSTER-WILL-CHASE -1> 
            <SET-VAR PARENT-MONSTER-WILL-CHASE 0>
            <EACH-OBJ PARENT-MONSTER (OBJ)
                  <SET-VAR OBJ ENCOUNTER <ADD <GET-VAR OBJ ENCOUNTER> 1>>
                  <SET-VAR OBJ MAX-HEALTH <ADD <GET-VAR OBJ MAX-HEALTH> PARENT-MONSTER-HEALTH-STEP>>
                  <COND (
                        <IS-DES <GET-VAR OBJ MAX-HEALTH> 1500>
                        <SET-VAR OBJ MAX-HEALTH 1500>
                  )>
            >
      )(
            <IS-EQUAL PARENT-MONSTER-WILL-CHASE 1>
            <EACH-OBJ PARENT-MONSTER (OBJ)
                  ;"this is the only way to reliably get the parent-monster instance"
                  <MOVE OBJ C-ROOM>
            >
      )(
            <AND 
                  <IS-EQUAL HAS-LIGHT 1>
                  <IS-EQUAL PARENT-MONSTER-FIRST-ENCOUNTER 1>
                  <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 0>
                  <IS-DES 25 <RAND>>
            >
            ;"first encounter: spawn only if light underground"
            <MOVE <INST STORAGE PARENT-MONSTER> C-ROOM>
      )(
            <AND
                  <IS-EQUAL HAS-LIGHT 1>
                  <IS-EQUAL PARENT-MONSTER-FIRST-ENCOUNTER 0>
                  <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 0>
                  <IS-DES 15 <RAND>>
            >
            ;"will increment encounter, but won't hit"
            <MOVE <INST STORAGE PARENT-MONSTER> C-ROOM>
      )(
            <AND
                  ;"can spawn if dark, including above-ground if new moon and no torch, fire, or gem"
                  <IS-EQUAL HAS-LIGHT 0>
                  ;"need first encounter in the light, otherwise won't spawn in the dark"
                  <IS-EQUAL PARENT-MONSTER-FIRST-ENCOUNTER 0>
            >
            <COND (
                  <OR
                        <IS-IN DEN-1 PLAYER>
                        <IS-IN DEN-2 PLAYER>
                        <IS-IN DEN-3 PLAYER>
                        <IS-IN DEN-4 PLAYER>
                  >
                  <COND (
                        <IS-DES 34 <RAND>>
                        <MOVE <INST STORAGE PARENT-MONSTER> C-ROOM> 
                  )>
            )(
                  <IS-DES 20 <RAND>>
                  <MOVE <INST STORAGE PARENT-MONSTER> C-ROOM> 
            )>
      )>

      ;"spawn cave-spiders"
      <COND (
            <AND 
                  <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 0>
                  ;"they're attracted to the light"
                  <IS-EQUAL HAS-LIGHT 1>
            >
            ;"can spawn a cave-spider or three"
            <COND (
                  <IS-DES 20 <RAND>>
                  <COPY-MOVE CAVE-SPIDER C-ROOM>
                  <COND (
                        <IS-DES 50 <RAND>>
                        <COPY-MOVE CAVE-SPIDER C-ROOM>
                  )>
                  <COND (
                        <IS-DES 10 <RAND>>
                        <COPY-MOVE CAVE-SPIDER C-ROOM>
                  )>
            )>
      )>

      ;"spawn bat, only underground, and higher chance at night"
      <COND (
            <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 0>
            <COND (
                  <AND
                        <IS-DES 0 TIME>
                        <IS-DES 25 <RAND>>
                  >
                  <COPY-MOVE BAT C-ROOM>
            )(
                  <IS-DES 5 <RAND>>
                  <COPY-MOVE BAT C-ROOM>
            )>
      )>

      ;"if there's a dead animal in this room, chance to clear it away (was eaten by other things)"
      <EACH-OBJ C-ROOM (OBJ)
            <COND (
                  <AND
                        <IS-EQUAL <GET-VAR OBJ IS-ANIMAL> 1>
                        <NOT <IS-DES <GET-VAR OBJ HEALTH> 0>>
                        <IS-DES 34 <RAND>>
                  >
                  <MOVE OBJ>
            )>
      >
>

<SLEEP (ST)
      ;"can only sleep if you have you're cloak"
      ;"if night, sleep until morning, increment TOTAL-SLEEPS, maybe dream"
      ;"if day, sleep for 2 hours, increment NAPS-TODAY"

      <COND (
            <NOT <IS-IN PLAYER CLOAK>>
            <TELL "It's not safe to sleep without your cloak." CR>
            <RETURN 0>
      )(
            <IS-DES -9 TIME>
            <SET-VAR NAPS-TODAY <ADD NAPS-TODAY 1>>
            <TELL "You settle down for what sleep you can get." CR>
            <SET-VAR ST <SUBTRACT TIME -17>> ;"sleep until morning"
      )(
            <IS-DES 0 TIME>
            <SET-VAR TOTAL-SLEEPS <ADD TOTAL-SLEEPS 1>>
            <TELL "You settle down for the night." CR>
            ;"TODO: maybe dream"
            <SET-VAR ST <SUBTRACT TIME -17>> ;"sleep until morning"
      )(
            <IS-EQUAL 1 1>
            <SET-VAR NAPS-TODAY <ADD NAPS-TODAY 1>>
            <TELL "You settle in for a nice nap." CR>
            <SET-VAR ST 4> ;"sleep for two hours"
      )>

      <EACH-VAL ST (VAL)
            <BURN-FIRE>
            <CHECK-PULSE>
            <TIME-PASSES>
      >
>

<TAKE WATER (CC) 
      <COND (
            <OR 
                  <IS-IN PLAYER BUCKET>
                  <IS-IN PLAYER KETTLE>
                  <IS-IN PLAYER CUP>
                  <IS-IN C-ROOM BUCKET>
                  <IS-IN C-ROOM KETTLE>
                  <IS-IN C-ROOM CUP>
            >
            <SET-VAR CC 1>
      )>

      <COND (
            <AND
                  <IS-IN PLAYER BUCKET>
                  <NOT <IS-IN <INST PLAYER BUCKET> WATER>>
            >
            <TELL "You fill the bucket with water" CR>
            <MOVE <CMD 1> <INST PLAYER BUCKET>>
      )(
            <AND
                  <IS-IN PLAYER KETTLE>
                  <NOT <IS-IN <INST PLAYER KETTLE> WATER>>
            >
            <TELL "You fill the kettle with water" CR>
            <MOVE <CMD 1> <INST PLAYER KETTLE>>
      )(
            <AND
                  <IS-IN PLAYER CUP>
                  <NOT <IS-IN <INST PLAYER CUP> WATER>>
            >
            <TELL "You fill the cup with water" CR>
            <MOVE <CMD 1> <INST PLAYER CUP>>
      )(
            <AND
                  <IS-IN C-ROOM BUCKET>
                  <NOT <IS-IN <INST C-ROOM BUCKET> WATER>>
            >
            <TELL "You fill the bucket with water" CR>
            <MOVE <CMD 1> <INST C-ROOM BUCKET>>
      )(
            <AND
                  <IS-IN C-ROOM KETTLE>
                  <NOT <IS-IN <INST C-ROOM KETTLE> WATER>>
            >
            <TELL "You fill the kettle with water" CR>
            <MOVE <CMD 1> <INST C-ROOM KETTLE>>
      )(
            <AND
                  <IS-IN C-ROOM CUP>
                  <NOT <IS-IN <INST C-ROOM CUP> WATER>>
            >
            <TELL "You fill the cup with water" CR>
            <MOVE <CMD 1> <INST C-ROOM CUP>>
      )(
            <IS-EQUAL CC 1>
            <TELL "All the containers are already full." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You need a BUCKET, KETTLE, or CUP to carry the water." CR>
      )>
>

<ROUTINE RECAP ()
      <COND (
            <IS-EQUAL WIN-ENTER-CABIN 1>
            <TELL "You entered the cabin." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You did not enter the cabin." CR>
      )>
      <COND (
            <IS-EQUAL WIN-LIGHT-FIRE 1>
            <TELL "You built a fire." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You did not build a fire." CR>
      )>
      <COND (
            <IS-EQUAL WIN-COOK-MEAL 1>
            <TELL "You cooked and ate a meal." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You did not eat a cooked meal." CR>
      )>
      <COND (
            <IS-EQUAL WIN-FIND-GEM 1>
            <TELL "You found the gem." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You did not find a gem." CR>
      )>
      <COND (
            <IS-EQUAL WIN-ENTER-CAVE 1>
            <TELL "You entered the caves." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You did not enter the caves." CR>
      )>
      <COND (
            <IS-DES WIN-KILL-MONSTER 1>
            <TELL "You killed both monsters." CR>
      )(
            <IS-EQUAL WIN-KILL-MONSTER 1>
            <TELL "You killed a monster." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You did not kill a monster." CR>
      )>
      <COND (
            <IS-IN CABIN NOTE>
            <TELL "You left a message." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You did not leave a message." CR>
      )>
      <COND (
            <IS-EQUAL WIN-BUILD-BOAT 1>
            <TELL "You built a boat." CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You did not build a boat." CR>
      )>
      <COND (
            <IS-EQUAL WIN-CHEAT 1>
            <TELL "You cheated." CR>
      )>
>

<ROUTINE ROOM-HAS-LIGHT ()
      <COND (
            <AND 
                  <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 1>
                  <NOT <AND
                        <IS-DES 0 TIME>
                        <IS-EQUAL MOON-PHASE 0>
                  >> 
            >
            ;"above ground, and not the night of a new moon"
            <RETURN 1>
      )(
            <OR
                  <IS-IN C-ROOM FIRE>
                  <IS-EQUAL <GET-VAR <INST C-ROOM TORCH> IS-LIT> 1>
                  <IS-EQUAL <GET-VAR <INST PLAYER TORCH> IS-LIT> 1>
            >
            ;"there's a fire in the room, a lit torch, or a lit torch in the player"
            <RETURN 1>
      )(
            <OR 
                  <IS-IN C-ROOM GEM>
                  <IS-IN PLAYER GEM>
            >
            ;"gem is in room or player"
            <RETURN 1>
      )>
      <RETURN 0>
>

<ROUTINE MONSTER-HEALTH ()
      <EACH-OBJ PARENT-MONSTER (INST)
            <COND (
                  <NOT <IS-IN C-ROOM INST>>
                  <SET-VAR INST HEALTH <ADD <GET-VAR INST HEALTH> 1>>
                  <COND (
                        <IS-DES <GET-VAR INST HEALTH> <GET-VAR INST MAX-HEALTH>>
                        <SET-VAR INST HEALTH <GET-VAR INST MAX-HEALTH>>
                  )>
            )>
      >

      <EACH-OBJ CHILD-MONSTER (INST)
            <COND (
                  <NOT <IS-IN C-ROOM INST>>
                  <SET-VAR INST HEALTH <ADD <GET-VAR INST HEALTH> 1>>
                  <COND (
                        <IS-DES <GET-VAR INST HEALTH> <GET-VAR INST MAX-HEALTH>>
                        <SET-VAR INST HEALTH <GET-VAR INST MAX-HEALTH>>
                  )>
            )>
      >
>

<ROUTINE BURN-FOREST-DOWN ()
      <TELL "It hasn't rained in awhile. Fire catches, and the forest burns down." CR>
      <SET-VAR FOREST-BURNED-DOWN 1>
      <EACH-OBJ CAVE-ENTRANCE-1 (OBJ)
            <MOVE OBJ>
      >
      <EACH-OBJ CAVE-ENTRANCE-2 (OBJ)
            <MOVE OBJ>
      >
      <COND (
            <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 1>
            <SET-VAR PLAYER HEALTH 0>
            <RETURN 1>
      )>
>