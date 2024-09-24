<BUZZ A OF MY ALL SOME THE AGAIN>

<SYNTAX WHERE AM I>
<SYNTAX WHERE CAN I GO>
<SYNTAX WHAT IS HERE>
<SYNTAX WHAT CAN I DO>
<SYNTAX LOOK AROUND>
<SYNTAX INVENTORY>
<SYNTAX WEATHER>
<SYNTAX WEATHER REPORT>
<SYNTAX SLEEP>

<SYNTAX EXAMINE OBJECT>
<SYNONYM EXAMINE INSPECT READ INVESTIGATE>
<SYNTAX TAKE OBJECT>
<SYNONYM TAKE GATHER GET>
<SYNTAX DROP OBJECT>
<SYNTAX EMPTY OBJECT>
<SYNONYM EMPTY UNPACK>
<SYNTAX ADD OBJECT TO OBJECT>
<SYNTAX PUT OBJECT IN OBJECT>
<SYNTAX FILL OBJECT WITH OBJECT>
<SYNTAX POUR OBJECT ON OBJECT>
<SYNTAX HIT OBJECT WITH OBJECT>
<SYNTAX WORK OBJECT WITH OBJECT>

<SYNTAX EAT OBJECT>
<SYNONYM EAT TASTE LICK DRINK IMBIBE>
<SYNTAX OPEN OBJECT>
<SYNTAX SPARK OBJECT AT OBJECT>
<SYNTAX TALK TO OBJECT>
<SYNTAX PEE ON OBJECT>
<SYNTAX WRITE NOTE>

<SYNTAX SWIM>
<SYNTAX JUMP IN>
<SYNTAX JUMP DOWN>

;"if DESC is a function, it can look for this global to optionally provide a detailed description"
<GLOBAL DETAILED-DESC 0>

;"Explain more the first time we look around"
<GLOBAL FIRST-LOOK-AROUND 1>

;"Explain more the first time we desc room in detail"
<GLOBAL FIRST-WHERE-AM-I 1>

;"Explain more the first time we list objects in the room"
<GLOBAL FIRST-WHAT-IS-HERE 1>

;"Explain more the first time we examine an object"
<GLOBAL FIRST-EXAMINE 1>

<ROUTINE V-DESC-ROOM ()
    <DESC C-ROOM>
>

<WEATHER ()
    <WEATHER-REPORT>
>

<LOOK ()
    <COND (
        <IS-EQUAL FIRST-LOOK-AROUND 1>
        <TELL "LOOK AROUND runs two commands for us:" CR>
    )>
    <V-ROOM-DETAIL>
    <COND (
        <NOT <IS-EQUAL FIRST-LOOK-AROUND 1>>
        <TELL "items:" CR>
    )>
    <V-DESC-OBJECTS-IN-ROOM>
    <COND (
        <IS-EQUAL FIRST-LOOK-AROUND 1>
        <SET-VAR FIRST-LOOK-AROUND 0>
    )>
>

<WHERE () 
    <V-ROOM-DETAIL>
>

<ROUTINE V-ROOM-DETAIL () 
    <COND (
        <IS-EQUAL FIRST-WHERE-AM-I 1>
        <SET-VAR FIRST-WHERE-AM-I 0>
        <TELL "WHERE AM I describes the current room, and may go into more detail than
        a regular description of the room." CR>
    )>
    <SET-VAR DETAILED-DESC 1>
    <DESC C-ROOM>
    <SET-VAR DETAILED-DESC 0>
>

<WHAT () 
    <V-DESC-OBJECTS-IN-ROOM>
>

<ROUTINE V-DESC-OBJECTS-IN-ROOM (COUNT)
    <COND (
        <IS-EQUAL FIRST-WHAT-IS-HERE 1>
        <SET-VAR FIRST-WHAT-IS-HERE 0>
        <TELL "WHAT IS HERE lists interactable objects in the immediate vicinity.
        Interactions may not be obvious." CR>
        <TELL "Objects nested inside other objects are not listed,
        but might show up if you EXAMINE their container." CR>
    )>
    
    <EACH-OBJ C-ROOM (OBJ)
        <SET-VAR COUNT <ADD COUNT 1>>
        <DESC OBJ>
        <TELL CR>
    >
    <COND (
        <IS-EQUAL COUNT 0>
        <TELL "This space appears to be empty." CR>
    )>
>

<INVENTORY (COUNT)
    <EACH-OBJ PLAYER (OBJ)
        <SET-VAR COUNT <ADD COUNT 1>>
        <DESC OBJ>
        <TELL CR>
    >
>

<EXAMINE (COUNT R)
    <COND(
        <IS-EQUAL FIRST-EXAMINE 1>
        <SET-VAR FIRST-EXAMINE 0>
        <TELL "The EXAMINE command will list items nested inside an object, and might also
        tell you more about the object itself. If there's any interesting
        items in this object, you can try to EMPTY it, then TAKE items off the ground." CR>
    )>

    <SET-VAR DETAILED-DESC 1>
    <DESC  <CMD 1>>
    <SET-VAR DETAILED-DESC 0> 
    <TELL CR>

    <EACH-OBJ <CMD 1> (OBJ) 
        <SET-VAR COUNT <ADD COUNT 1>>
    >

    <COND (
        <IS-EQUAL COUNT 0>
        <TELL "nothing inside" CR>
        <RETURN 1>
    )>

    <TELL "items inside:" CR>
    <EACH-OBJ <CMD 1> (OBJ) 
        <DESC OBJ>
        <TELL CR>
    >
>

<EAT ()
    ;"TODO"
    <COND (
        <NOT <IS-EQUAL <GET-VAR <CMD 1> EDIBLE> 1>>
        <TELL "You can't eat that." CR>
    )>
>

<TAKE ()
    <COND (
        <IS-EQUAL <GET-VAR <CMD 1> NO-TAKE> 1>
        <TELL "This can't be picked up" CR>
    )(
        <IS-EQUAL 1 1>
        <MOVE <CMD 1> PLAYER>
        <TELL "Picked up" CR>
    )>
>

<EMPTY (COUNT)
    <EACH-OBJ <CMD 1> (OBJ)
        <COND (
            <NOT <IS-EQUAL <GET-VAR OBJ NO-TAKE> 1>>
            <MOVE OBJ C-ROOM>
            <SET-VAR COUNT <ADD COUNT 1>>
        )>
    >
    <COND (
        <IS-DES COUNT 0>
        <TELL "Emptied" CR>
    )>
>

<PEE ()
    ;"if fire or torch, put out"
    ;"else, print cool"
    <COND (
        <IS-EQUAL <CMD 1> FIRE>
        <MOVE  <CMD 1>>
        <TELL "The fire goes out." CR>
    )(
        <IS-EQUAL <CMD 1> TORCH>
        <SET-VAR <CMD 1> LIT 0>
        <TELL "The torch goes out." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "Cool." CR>
    )>
>

<WRITE (HAS-C HAS-P)
    ;"must have charcoal and paper in player or room"
    <COND (
        <AND 
            <OR <IS-IN C-ROOM CHARCOAL> <IS-IN PLAYER CHARCOAL>>
            <OR <IS-IN C-ROOM BOOK-PAGE> <IS-IN PLAYER BOOK-PAGE>>
        >
        <COPY-MOVE NOTE C-ROOM>
    )(
        <IS-EQUAL 1 1>
        <TELL "You need paper and charcoal." CR>
    )>
>

<DROP ()
    <COND (
        <IS-IN PLAYER  <CMD 1>>
        <MOVE <CMD 1> C-ROOM>
    )>
>

<SWIM ()
    <TELL "The water looks a little chilly, it would be better if we had a boat, or maybe built one?" CR>
>

<JUMP ()
    <TELL "No thank you" CR>
>