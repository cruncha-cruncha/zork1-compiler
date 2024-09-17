;"It's the end of summer..."

<DIRECTIONS UP DOWN IN OUT NORTH EAST SOUTH WEST>

;"3 = sunny, 2 = cloudy, 1 = rainy, 0 = thunder"
<GLOBAL WEATHER 3>

;"aka days since last rain"
<GLOBAL DRY 2>

;"0 = none, 1-11 = waxing, 12 = full, 13-23 = waning"
<GLOBAL MOON-PHASE 2>

;"starts at 1 and just keeps going"
<GLOBAL DAY 1>

;"night starts at -1 and goes to -17, day starts at 1 and goes to 33"
;"2 ticks per hour, 24 hours in a day"
<GLOBAL TIME 1>

;"Max number of items the player can carry"
<GLOBAL P-CAPACITY 10>

;"count all the sleeps a player has taken (at night)"
<GLOBAL TOTAL-SLEEPS 0>

;"count all the sleeps a player has taken (during the day)"
<GLOBAL TOTAL-NAPS 0>

;"count all the food a player has eaten"
<GLOBAL TOTAL-FOOD 0>

<PLAYER 
      (ROOM FOREST-1)
      (ACT-ENTER <V-DESC-ROOM>)
      (ACT-ALWAYS <TIME-PASSES>)
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

<ROUTINE TIME-PASSES (NEW-W) 
      <COND (
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
            )>
      )(
            <IS-ASC TIME 0>
            <SET-VAR TIME <ADD TIME -1>>
            <COND (
                  <IS-EQUAL TIME -18>
                  <SET-VAR TIME 1>
            )(
                  <IS-EQUAL TIME -8>
                  <SET-VAR DAY <ADD DAY 1>>
                  <SET-VAR MOON-PHASE <ADD MOON-PHASE 1>>
                  <COND (
                        <IS-EQUAL MOON-PHASE 24>
                        <SET-VAR MOON-PHASE 0>
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
            <AND <IS-DES MOON-PHASE 0> <IS-ASC MOON-PHASE 12>> ;"waxing"
            <SET-VAR UNTIL-FULL <SUBTRACT MOON-PHASE 11>>
            <TELL "the moon is waxing (" UNTIL-FULL " day">
            <COND (
                <NOT <IS-EQUAL UNTIL-FULL 1>>
                <TELL "s">
            )>
            <TELL " until full), ">

      )(
            <IS-EQUAL MOON-PHASE 12> ;"full"
            <TELL "full moon, ">
      )(
            <AND <IS-DES MOON-PHASE 12> <IS-ASC MOON-PHASE 24>> ;"waning"
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
