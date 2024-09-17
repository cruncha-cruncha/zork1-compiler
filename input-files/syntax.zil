<BUZZ A OF ALL SOME THE AGAIN>

<SYNTAX WHERE AM I = V-ROOM-DETAIL>
<SYNTAX LOOK AROUND = V-DESC-OBJECTS-IN-ROOM>
<SYNTAX INVENTORY = V-INVENTORY>
<SYNTAX WEATHER = WEATHER-REPORT>
<SYNTAX SLEEP = V-SLEEP>

<SYNTAX EXAMINE OBJECT = V-EXAMINE>
<SYNONYM EXAMINE INSPECT READ INVESTIGATE>
<SYNTAX TAKE OBJECT = V-TAKE>
<SYNONYM TAKE GATHER GET>
<SYNTAX UNPACK OBJECT = V-TAKE-OUT>
<SYNTAX DROP OBJECT = V-DROP>
<SYNTAX PUT OBJECT (CAN-CONTAIN) INTO OBJECT = V-PUT-IN>
<SYNONYM PUT PLACE POUR>
<SYNTAX FILL OBJECT (CAN-CONTAIN) WITH OBJECT = V-PUT-IN>
<SYNTAX OPEN OBJECT = V-OPEN>
<SYNTAX CLOSE OBJECT = V-CLOSE>
<SYNTAX HIT OBJECT WITH OBJECT (MAX-DAMAGE) = V-HIT-WITH>
<SYNTAX COOK OBJECT = V-COOK>
<SYNONYM COOK BOIL ROAST>
<SYNTAX EAT OBJECT (EDIBLE) = V-EAT>
<SYNONYM EAT TASTE LICK DRINK IMBIBE>
<SYNTAX WORK OBJECT WITH OBJECT (MAX-HEALTH) = V-WORK-WITH>

<SYNTAX SPARK FLINT AT OBJECT = V-SPARK-AT>
<SYNTAX TALK TO OBJECT = V-TALK-TO>
<SYNTAX PEE ON OBJECT = V-PEE-ON>
<SYNTAX WRITE NOTE = V-WRITE-NOTE>

<SYNTAX WHERE CAN I GO = V-WHERE-TO-GO>
<SYNTAX SWIM = V-SWIM>
<SYNTAX DIVE IN = V-SWIM>
<SYNTAX JUMP IN = V-JUMP>
<SYNTAX JUMP DOWN = V-JUMP>

;"if DESC is a function, it can look for this global to optionally provide a detailed description"
<GLOBAL DETAILED-DESC 0>

;"Explain more the first time we look around"
<GLOBAL FIRST-LOOK-AROUND 1>

;"Explain more the first time we examine an object"
<GLOBAL FIRST-EXAMINE 1>

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
            <IS-EQUAL FIRST-LOOK-AROUND 1>
            <SET-VAR FIRST-LOOK-AROUND 0>
            <TELL "This command lists interactable objects in the immediate vicinity.
            Interactions may not be obvious." CR>
            <TELL "Objects nested inside other objects are not listed, but might show up
            if you EXAMINE their container." CR>
      )>
      <EACH-OBJ CURRENT-ROOM (OBJ)
            <COND(
                  <IS-EQUAL <GET-VAR OBJ HIDDEN> 0>
                  <SET-VAR COUNT <ADD COUNT 1>>
                  <DESC OBJ>
                  <TELL CR>
            )>
      >
      <COND (
            <IS-EQUAL COUNT 0>
            <TELL "This space appears to be empty." CR>
      )>
>

<ROUTINE V-INVENTORY (COUNT)
      <EACH-OBJ PLAYER (OBJ)
            <COND(
                  <IS-EQUAL <GET-VAR OBJ HIDDEN> 0>
                  <SET-VAR COUNT <ADD COUNT 1>>
                  <DESC OBJ>
                  <TELL CR>
            )>
      >
      <COND (
            <IS-EQUAL COUNT 0>
            <TELL "There's nothing in your inventory." CR>
      )>
>

<ROUTINE V-SPARK-AT ()
      ;"do nothing, let the object's ACT-PRSO handle it"
>

<ROUTINE V-HIT-WITH ()
      ;"do nothing, let the object's ACT-PRSO or ACT-PRSI handle it"
>

<ROUTINE V-OPEN ()
      ;"do nothing, let the object's ACT-PRSO handle it"
>

<ROUTINE V-CLOSE ()
      ;"do nothing, let the object's ACT-PRSO handle it"
>

<ROUTINE V-EXAMINE (COUNT R)
      <SET-VAR DETAILED-DESC 1>
      <DESC PRSO>
      <SET-VAR DETAILED-DESC 0> 
      <TELL CR>

      <COND(
            <IS-EQUAL FIRST-EXAMINE 1>
            <SET-VAR FIRST-EXAMINE 0>
            <TELL "The EXAMINE command will list items nested inside the object, and might also
            tell you more about the object itself. If there's any interesting
            items in this object, you can UNPACK it to remove the nested items.
            After that, can TAKE an item off the ground." CR>
      )>

      <TELL "and inside:" CR>
      <EACH-OBJ PRSO (OBJ) 
            <COND(
                  <IS-EQUAL <GET-VAR OBJ HIDDEN> 0>
                  <SET-VAR COUNT <ADD COUNT 1>>
                  <DESC OBJ>
                  <TELL CR>
            )>
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
      ;"do nothing, let the object's ACT-PRSO handle it"
>

<ROUTINE V-PUT-IN ()
      ;"do nothing, let the object's ACT-PRSO or ACT-PRSI handle it"
>

<ROUTINE V-TALK-TO ()
      ;"do nothing, let the object's ACT-PRSI handle it"
>

<ROUTINE V-TAKE ()
      <COND (
            <IS-EQUAL <GET-VAR PRSO NO-TAKE> 1>
            <TELL "This can't be picked up." CR>
      )(
            <NOT <IS-EQUAL <GET-VAR PRSO HIDDEN> 0>>
            ;"do nothing"
      )(
            <IS-EQUAL 1 1>
            <MOVE PRSO PLAYER>
            <TELL "Picked up!" CR>
      )>
>

<ROUTINE V-TAKE-OUT ()
      ;"do nothing, let the object's ACT-PRSO handle it"
>

<ROUTINE V-PEE-ON ()
      ;"do nothing, let the object's ACT-PRSO handle it"
>

<ROUTINE V-COOK ()
      ;"do nothing, let the object's ACT-PRSO handle it"
>

<ROUTINE V-WRITE-NOTE ()
      ;"must have charcoal and paper in player or room"
      <TELL "// TODO" CR>
>

<ROUTINE V-SLEEP ()
;"if night, sleep until morning, increment TOTAL-SLEEPS, maybe dream"
;"if day, sleep for 2 hours, increment TOTAL-NAPS"
      <TELL "// TODO" CR>
>

<ROUTINE V-DROP ()
      <TELL "// TODO" CR>
>

<ROUTINE V-WORK-WITH ()
      ;"do nothing, let the object's ACT-PRSO or ACT-PRSI handle it"
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