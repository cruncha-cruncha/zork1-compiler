<BUZZ A OF MY ALL SOME THE AGAIN>

<SYNTAX WHERE AM I>
<SYNTAX WHERE CAN I GO>
<SYNTAX WHAT IS HERE>
<SYNTAX WHAT CAN I DO>
<SYNTAX TIME>
<SYNTAX LOOK AROUND>
<SYNTAX INVENTORY>
<SYNTAX WEATHER>
<SYNTAX WEATHER REPORT>
<SYNTAX SLEEP>
<SYNTAX CHEAT OBJECT>
<SYNTAX DEBUG>

<SYNTAX EXAMINE OBJECT>
<SYNONYM EXAMINE INSPECT READ INVESTIGATE>
<SYNTAX TAKE OBJECT>
<SYNONYM TAKE GATHER GET>
<SYNTAX DROP OBJECT>
<SYNTAX EMPTY OBJECT>
<SYNONYM EMPTY UNPACK>
<SYNTAX ADD OBJECT TO OBJECT>
<SYNTAX PUT OBJECT IN OBJECT>
<SYNTAX HIT OBJECT WITH OBJECT>
<SYNONYM HIT SMASH>
<SYNTAX WORK OBJECT WITH OBJECT>

<SYNTAX EAT OBJECT>
<SYNONYM EAT TASTE LICK DRINK IMBIBE>
<SYNTAX OPEN OBJECT>
<SYNTAX SPARK OBJECT AT OBJECT>
<SYNTAX TALK TO OBJECT>
<SYNTAX PEE ON OBJECT>
<SYNTAX WRITE NOTE>

<SYNTAX ENTER CABIN>
<SYNTAX EXIT CABIN>
<SYNTAX SWIM>
<SYNTAX JUMP IN>
<SYNTAX JUMP DOWN>

<SYNTAX GREAT BALL FIRE> ;"bat"
<SYNTAX EUNICE BROKE HEART> ;"owl"
<SYNTAX SCHOOL IS FOR ME> ;"fish"
<SYNTAX BUG THAT BIG> ;"frog"
<SYNTAX EVER PLAY LOTTERY> ;"rabbit"
<SYNTAX LISTENING TO RIVER> ;"snake"
<SYNTAX GROWING UP BACK HOME> ;"crow"
<SYNTAX YES>
<SYNTAX NO>

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

;"Explain more the first time we take something"
<GLOBAL FIRST-TAKE 1>

<ROUTINE V-DESC-ROOM ()
    <DESC C-ROOM>
>

<WEATHER ()
    <WEATHER-REPORT>
>

<LOOK ()
    <COND (
        <IS-EQUAL FIRST-LOOK-AROUND 1>
        <TELL "LOOK AROUND runs two commands:" CR>
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
        Interactions may not be obvious. Objects nested inside other objects are not listed,
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
    <TELL "You have " <GET-VAR PLAYER HEALTH> " health (of a maximum " <GET-VAR PLAYER MAX-HEALTH> ")">

    <EACH-OBJ PLAYER (OBJ)
        <SET-VAR COUNT <ADD COUNT 1>>
    >

    <COND (
        <IS-EQUAL COUNT 0>
        <TELL ", and you're not carrying anything." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL ", and you're carrying:" CR>
        <EACH-OBJ PLAYER (OBJ)
            <SET-VAR COUNT <ADD COUNT 1>>
            <DESC OBJ>
            <TELL CR>
        >
    )>
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
        <RETURN 1>
    )>

    <TELL "items inside:" CR>
    <EACH-OBJ <CMD 1> (OBJ) 
        <DESC OBJ>
        <TELL CR>
    >
>

<EAT ()
    <COND (
        <NOT <IS-EQUAL <GET-VAR <CMD 1> IS-EDIBLE> 1>>
        <TELL "You can't eat that." CR>
        <COND (
            <IS-DES 50 <RAND>>
            <TELL "But you try anyway. Take 2 damage" CR>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 2>>
        )>
    )(
        <OR
            <IS-EQUAL SOUP <CMD 1>>
            <IS-EQUAL TEA <CMD 1>>
        >
        <SET-VAR WIN-COOK-MEAL 1>
        <TELL "Yum. You gain 40 health">
        <COND (
            <AND
                <IS-EQUAL TEA <CMD 1>>
                <IS-ASC <GET-VAR PLAYER MAX-HEALTH> MAX-MAX-HEALTH>
            >
            <TELL ", and feel yourself growing stronger" CR>
            <SET-VAR PLAYER MAX-HEALTH <ADD <GET-VAR PLAYER MAX-HEALTH> 25>>
        )(
            <IS-EQUAL 1 1>
            <TELL CR>
        )>

        <SET-VAR PLAYER HEALTH <ADD <GET-VAR PLAYER HEALTH> 40>>
        
        <COND (
            <AND 
                <IS-EQUAL <GET-VAR <CMD 1> HAS-MUSHROOM> 1>
                <NOT <IS-IN FOREST-3 TREE-HOLLOW>>
                <OR
                    <IS-IN <INST FOREST-3 TREE-HOLLOW> GEM>
                    <IS-IN <INST STORAGE TREE-HOLLOW> GEM>
                >
            >
            <TELL "You have a vision of a sparkling gem, hidden in the hollow of a tree, east of the forest lake." CR>
            <MOVE <INST STORAGE TREE-HOLLOW> FOREST-3>
        )>
        <MOVE <CMD 1>>
    )(
        <OR
            <IS-EQUAL BONES <CMD 1>>
            <IS-EQUAL MUSHROOM <CMD 1>>
            <IS-EQUAL ROOT <CMD 1>>
        >
        <MOVE <CMD 1>>
        <COND (
            <IS-DES 50 <RAND>>
            <TELL "It's a little chewy, but ok. You gain 5 health." CR>
            <SET-VAR PLAYER HEALTH <ADD <GET-VAR PLAYER HEALTH> 5>>
        )(
            <IS-EQUAL 1 1>
            <TELL "It doesn't agree with you. Take 5 damage" CR>
            <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 5>>
        )>
    )(
        <IS-EQUAL 1 1>
        <MOVE <CMD 1>>
        <TELL "You gain 10 health.">
        <COND (
            <IS-EQUAL WIN-COOK-MEAL 0>
            <TELL " Cook some soup for a heartier meal.">
        )>
        <TELL CR>
        <SET-VAR PLAYER HEALTH <ADD <GET-VAR PLAYER HEALTH> 10>>
    )>
    <COND (
        <IS-DES <GET-VAR PLAYER HEALTH> <GET-VAR PLAYER MAX-HEALTH>>
        <SET-VAR PLAYER HEALTH <GET-VAR PLAYER MAX-HEALTH>>
    )>
>

<TAKE (PU)
    ;"didn't answer the question"
    <EACH-OBJ Q-STORAGE (OBJ)
        <MOVE OBJ>
    >

    <COND (
        <IS-EQUAL <CMD 1> <CMD 2>>
        <TELL "There's nothing like that here." CR>
    )(
        <IS-EQUAL <GET-VAR <CMD 1> NO-TAKE> 1>
        <TELL "This can't be picked up" CR>
    )(
        <IS-EQUAL <GET-VAR <CMD 1> OWN-TAKE> 1>
        ;"this object has it's own take handler"
    )(
        <IS-IN PLAYER <CMD 1>>
        <EACH-OBJ C-ROOM (OBJ)
            <COND (
                <AND 
                    <IS-EQUAL OBJ <CMD 1>>
                    <NOT <IS-EQUAL PU 1>>
                >
                <TELL "Picked up" CR>
                <MOVE OBJ PLAYER>
                <SET-VAR PU 1>
            )>
        >
        <COND (
            <NOT <IS-EQUAL PU 1>>
            <TELL "There's none in here" CR>
        )>
    )(
        <IS-IN C-ROOM <CMD 1>>
        <MOVE <CMD 1> PLAYER>
        <TELL "Picked up" CR>
        <COND (
            <IS-EQUAL FIRST-TAKE 1>
            <SET-VAR FIRST-TAKE 0>
            <TELL "List items you're carrying using INVENTORY." CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "Huh?" CR>
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
    <COND (
        <IS-EQUAL <CMD 1> FIRE>
        <MOVE <CMD 1>>
        <TELL "The fire goes out." CR>
        <COPY-MOVE CHARCOAL C-ROOM>
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
        <TELL "You need both paper and charcoal." CR>
    )>
>

<DROP ()
    <COND (
        <IS-IN PLAYER <CMD 1>>
        <MOVE <CMD 1> C-ROOM>
        <TELL "Dropped " CR>
        <COND (
            <IS-EQUAL WATER <CMD 1>>
            <MOVE <CMD 1>>
        )>
    )>
>

<SWIM ()
    <TELL "The water looks a little chilly, it would be better if we had a boat, or maybe built one?" CR>
>

<JUMP ()
    <TELL "No thank you" CR>
>

<ADD () <ADD-TO>>
<PUT () <ADD-TO>>

<ROUTINE ADD-TO ()
    <COND (
        <IS-EQUAL FIRE <CMD 1>>
        <TELL "You burn yourself, and take 2 damage." CR>
        <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 2>>
    )(
        <IS-EQUAL <GET-VAR <CMD 1> NO-TAKE> 1>
        <TELL "Nice try" CR> 
    )(
        <OR
            <IS-EQUAL RIVER-STONE <CMD 1>>
            <IS-EQUAL RIVER-STONE <CMD 2>>
        >
        <ADD-TO-STONE>
    )(
        <AND
            <IS-EQUAL DETRITUS <CMD 2>>
            <NOT <IS-EQUAL WATER <CMD 1>>>
            ;"cannot be nested"
            <OR
                <IS-IN PLAYER <CMD 2>>
                <IS-IN C-ROOM <CMD 2>>
            >
        >
        <MOVE <CMD 1> <CMD 2>>
        <TELL "Added" CR>
    )(
        <IS-EQUAL WATER <CMD 2>>
        <COND (
            <IS-EQUAL WATER <CMD 1>>
            <TELL "The water gets wetter." CR>
        )(
            <IS-EQUAL 1 1>
            <TELL "splash" CR>
            <MOVE <CMD 1> <CMD 2>>
        )>
    )(
        <IS-EQUAL FIRE <CMD 2>>
        <ADD-TO-FIRE>
    )(
        <OR
            <IS-EQUAL KETTLE <CMD 2>>
            <IS-EQUAL BUCKET <CMD 2>>
            <IS-EQUAL CUP <CMD 2>>
        >

        <COND (
            <IS-EQUAL <CMD 1> <CMD 3>>
            <TELL "Where is that?" CR>
        )(
            <OR
                <IS-EQUAL WATER <CMD 1>>
                <IS-EQUAL SAP <CMD 1>>
                <IS-EQUAL <GET-VAR <CMD 1> IS-EDIBLE> 1>
            >
            <COND (
                <AND
                    <IS-EQUAL WATER <CMD 1>>
                    <IS-IN <CMD 2> WATER>
                >
                ;"water + water = water"
            )(
                <IS-EQUAL 1 1>
                <MOVE <CMD 1> <CMD 2>>
            )>
            <TELL "Added." CR>
        )(
            <IS-EQUAL 1 1>
            <TELL "You can't put that in there" CR>
        )>
    )(
        <IS-EQUAL STICK <CMD 2>>
        <COND (
            <OR
                <IS-EQUAL SAP <CMD 1>>
                <IS-EQUAL BOILED-SAP <CMD 1>>
            >
            <COND (
                <OR <IS-IN <CMD 2> SAP> <IS-IN <CMD 2> BOILED-SAP>>
                <TELL "This stick already has some sticky stuff on it." CR>
                <RETURN 0>
            )>
            <MOVE <CMD 1> <CMD 2>>
        )(
            <OR 
                <IS-EQUAL DETRITUS <CMD 1>>
                <IS-EQUAL BULRUSH <CMD 1>>
            >
            <COND (
                <OR <IS-IN <CMD 2> DETRITUS> <IS-IN <CMD 2> BULRUSH>>
                <TELL "This stick already has some tinder." CR>
                <RETURN 0>
            )>
            <MOVE <CMD 1> <CMD 2>>
        )(
            <IS-EQUAL 1 1>
            <TELL "To make a torch, add SAP or BOILED-SAP, then DETRITUS or BULRUSH." CR>
            <RETURN 0>
        )>
        <COND (
            <NOT <OR <IS-IN <CMD 2> SAP> <IS-IN <CMD 2> BOILED-SAP>>>
            <TELL "Now add some SAP or BOILED-SAP" CR>
        )(    
            <NOT <OR <IS-IN <CMD 2> DETRITUS> <IS-IN <CMD 2> BULRUSH>>>
            <TELL "Now add some DETRITUS or BULRUSH" CR>
        )(
            <IS-EQUAL 1 1>
            <TELL "You've made yourself a TORCH. SPARK FLINT AT TORCH to light it." CR>
            <COPY-MOVE TORCH PLAYER>
            <MOVE <CMD 2>>
        )>
    )(
        <IS-EQUAL PICK-AXE <CMD 2>>
        <COND (
            <IS-EQUAL STRAP <CMD 1>>
            <COND (
                <IS-IN <CMD 2> STRAP>
                <TELL "Already has a strap on it" CR>
            )(
                <IS-EQUAL 1 1>
                <TELL "You tie the strap to the pick-axe." CR>
                <MOVE <CMD 1> <CMD 2>>
            )>
        )(
            <IS-EQUAL 1 1>
            <TELL "You can tie a strap to the pick-axe using ADD, but that's about it" CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "Can't put that in there" CR>
    )>
>

<ROUTINE ADD-TO-FIRE (HAS-EDBL ONLY-EDBL HAS-SAP HAS-SHM)
    <COND (
        <OR
            <IS-EQUAL KETTLE <CMD 1>>
            <IS-EQUAL BUCKET <CMD 1>>
            <IS-EQUAL CUP <CMD 1>>
        >

        <SET-VAR ONLY-EDBL 1>
        <EACH-OBJ <CMD 1> (OBJ)
            <COND (
                <IS-EQUAL <GET-VAR OBJ IS-EDIBLE> 1>
                <SET-VAR HAS-EDBL 1>
            )(
                <OR
                    <IS-EQUAL SAP OBJ>
                    <IS-EQUAL BOILED-SAP OBJ>
                >
                <SET-VAR HAS-SAP 1>
                <SET-VAR ONLY-EDBL 0>
            )(
                <NOT <IS-EQUAL WATER OBJ>>
                <SET-VAR ONLY-EDBL 0>
            )>
        >

        <COND (
            <IS-EQUAL HAS-SAP 1>
            <TELL "Everything in there becomes BOILED-SAP" CR>
            <EACH-OBJ <CMD 1> (OBJ)
                <MOVE OBJ>
            >
            <COPY-MOVE BOILED-SAP <CMD 1>>
        )(
            <AND
                <IS-EQUAL ONLY-EDBL 1>
                <IS-EQUAL HAS-EDBL 1>
                <IS-IN <CMD 1> WATER> 
            >
            <COND (
                <IS-IN <CMD 1> MUSHROOM>
                <SET-VAR HAS-SHM 1>
            )>
            <COND (
                <IS-EQUAL CUP <CMD 1>>
                <TELL "You boil up some tea. Can DRINK it." CR>
                <EACH-OBJ <CMD 1> (OBJ)
                    <MOVE OBJ>
                >
                <COPY-MOVE TEA <CMD 1>>
                <COND (
                    <IS-EQUAL HAS-SHM 1>
                    <SET-VAR <INST <CMD 1> TEA> HAS-MUSHROOM 1>
                )>
            )(
                <IS-EQUAL 1 1>
                <TELL "You boil up some soup. Can EAT it." CR>
                <EACH-OBJ <CMD 1> (OBJ)
                    <MOVE OBJ>
                >
                <COPY-MOVE SOUP <CMD 1>>
                <COND (
                    <IS-EQUAL HAS-SHM 1>
                    <SET-VAR <INST <CMD 1> SOUP> HAS-MUSHROOM 1>
                )>
            )>
        )(
            <IS-EQUAL 1 1>
            <TELL "You need water and something edible to make soup: ADD BERRIES TO KETTLE." CR>
        )>
    )(
        <IS-EQUAL OBSIDIAN-SHARD <CMD 1>>
        <TELL "It sits there in the fire, looking black." CR>
    )(
        <IS-DES <GET-VAR <CMD 1> MAX-DAMAGE> 0>
        <TELL "The edge goes dull" CR>
        <SET-VAR <CMD 1> DAMAGE 0>
    )(
        <IS-EQUAL LOG <CMD 1>>
        <SET-VAR WIN-LIGHT-FIRE 1>
        <TELL "The fire will continue to burn for another hour or two." CR>
        <SET-VAR <CMD 2> FUEL <ADD <GET-VAR <CMD 2> FUEL> 12>>
        <MOVE <CMD 1>>
    )(
        <IS-EQUAL ROUGH-BOARD <CMD 1>>
        <SET-VAR WIN-LIGHT-FIRE 1>
        <TELL "The fire will continue to burn for a little longer." CR>
        <SET-VAR <CMD 2> FUEL <ADD <GET-VAR <CMD 2> FUEL> 5>>
        <MOVE <CMD 1>>
    )(
        <IS-EQUAL WATER <CMD 1>>
        <TELL "The fire dies with a sputter." CR>
        <MOVE <CMD 2>>
        <MOVE <CMD 1>>
        <COPY-MOVE CHARCOAL C-ROOM>
    )(
        <IS-EQUAL <GET-VAR <CMD 1> IS-ANIMAL> 1>
        <TELL "It stinks, but goes up in smoke." CR>
        <MOVE <CMD 1>>
    )(
        <IS-EQUAL 1 1>
        <TELL "It goes up in smoke." CR>
        <MOVE <CMD 1>>
    )>
>

<ROUTINE ADD-TO-STONE ()
    <COND (
        <IS-EQUAL WATER <CMD 1>>
        <TELL "The stone is damp and ready to sharpen weapons. It will dry out eventually though." CR>
        <SET-VAR <CMD 2> WETNESS 20>
    )(
        <IS-EQUAL WATER <CMD 2>>
        <TELL "The stone is damp and ready to sharpen weapons. It will dry out eventually though." CR>
        <SET-VAR <CMD 2> WETNESS 20>
    )(
        <IS-EQUAL 1 1>
        <TELL "Are you sure there's WATER around?" CR>
    )>
>

<TIME ()
    <TELL "It's day " DAY ", time is " TIME CR>
>

<CHEAT ()
    <COND (
        <NOT <IS-EQUAL <CMD 1> <CMD 2>>>
        <COPY-MOVE <CMD 1> PLAYER>
        <RETURN 1>
    )>

    ;"talk to all the animals"
    <SET-VAR PLAYER T-CROW 2>
    <SET-VAR PLAYER T-OWL 2>
    <SET-VAR PLAYER T-FISH 2>
    <SET-VAR PLAYER T-FROG 2>
    <SET-VAR PLAYER T-RABBIT 2>
    <SET-VAR PLAYER T-SNAKE 2>

    ;"increase weapon max-damage (owl buff)"
    <EACH-OBJ SWORD (INST)
        <SET-VAR INST MAX-DAMAGE 45>
    >
    <EACH-OBJ AXE (INST)
       <SET-VAR INST MAX-DAMAGE 16>
    >
    <EACH-OBJ KNIFE (INST)
        <SET-VAR INST MAX-DAMAGE 6>
    >
    <EACH-OBJ OBSIDIAN-SHARD (INST)
        <COND (
            <IS-EQUAL <GET-VAR INST MAX-DAMAGE> 50>
            <SET-VAR INST MAX-DAMAGE 70>
            <COND (
                <IS-EQUAL <GET-VAR INST DAMAGE> 50>
                <SET-VAR INST DAMAGE 70>
            )>
        )>
    >

    ;"increase high-roll chance (fish buff)"
    <SET-VAR COMBAT-HIGH-CHANCE 75>

    ;"increase player max-health (frog buff)"
    <SET-VAR MAX-MAX-HEALTH 150>

    ;"decrease parent and child monster attack damage (rabbit buff)"
    <SET-VAR PARENT-MONSTER-ATK-DMG 28>
    <SET-VAR CHILD-MONSTER-ATK-DMG 2>

    ;"decrease parent and child monster health, scaling (snake buff)"
    <SET-VAR PARENT-MONSTER-HEALTH-STEP 5>
    <EACH-OBJ PARENT-MONSTER (M)
        <SET-VAR M MAX-HEALTH 
            <SUBTRACT <GET-VAR M MAX-HEALTH> 10>>
    >
    <EACH-OBJ CHILD-MONSTER (M)
        <SET-VAR M MAX-HEALTH 90>
    >

    ;"reveal tree-hollow"
    <COND (
        <NOT <IS-IN FOREST-3 TREE-HOLLOW>>
        <MOVE <INST STORAGE TREE-HOLLOW> FOREST-3>
    )>

    ;"open the cabin-door"
    <COND (
        <IS-ASC 0 <GET-VAR <INST CABIN-EXTERIOR CABIN-DOOR> HEALTH>>
        <SET-VAR <INST CABIN-EXTERIOR CABIN-DOOR> HEALTH 0>
    )>

    ;"open the stone-door"
    <COND (
        <IS-ASC 0 <GET-VAR <INST CAVERN-1 STONE-DOOR> HEALTH>>
        <SET-VAR <INST CAVERN-1 STONE-DOOR> HEALTH 0>
    )>

    ;"kill the child monster"
    <EACH-OBJ CHILD-MONSTER (M)
        <COND (
            <IS-ASC 0 <GET-VAR M HEALTH>>
            <SET-VAR M HEALTH 0>
            <SET-VAR WIN-KILL-MONSTER <ADD WIN-KILL-MONSTER 1>>
            <MOVE M DEN-1>
        )>
    >

    ;"get magic-ring"
    <SET-VAR PLAYER HAS-MAGIC-RING 1>
    <SET-VAR FOUND-MAGIC-RING 1>
    
    ;"get sword"
    <EACH-OBJ SWORD (S)
        <MOVE S PLAYER>
    >

    ;"sharpen all weapons, except obsidian"
    <EACH-OBJ SWORD (INST)
        <SET-VAR INST DAMAGE 45>
    >
    <EACH-OBJ AXE (INST)
       <SET-VAR INST DAMAGE 16>
    >
    <EACH-OBJ KNIFE (INST)
        <SET-VAR INST DAMAGE 6>
    >

    ;"max player health and max-health"
    <SET-VAR PLAYER MAX-HEALTH 150>
    <SET-VAR PLAYER HEALTH 150>

    ;"teleport to den-1"
    <MOVE PLAYER DEN-1>

    ;"pass first encounter with parent monster"
    <SET-VAR PARENT-MONSTER-FIRST-ENCOUNTER 0>

    ;"spawn the monster"
    <EACH-OBJ PARENT-MONSTER (M)
        <MOVE M DEN-1>
    >
>

<DEBUG ()
    <TELL "Player vars" CR>
    <EACH-VAL PLAYER (K V)
        <TELL K ": " V CR>
    >

    <TELL "Random vars" CR>
    <TELL "KILLED-CROW: " KILLED-CROW CR>
    <TELL "COMBAT-HIGH-CHANCE: " COMBAT-HIGH-CHANCE CR>
    <TELL "COMBAT-DAMAGE: " COMBAT-DAMAGE CR>
    <TELL "COMBAT-MAX-DAMAGE: " COMBAT-MAX-DAMAGE CR>
    <TELL "FIRE-IN-DRAFTY-CABIN: " FIRE-IN-DRAFTY-CABIN CR>
    <TELL "FOREST-BURNED-DOWN: " FOREST-BURNED-DOWN CR>
    <TELL "TOTAL-SLEEPS: " TOTAL-SLEEPS CR>
    <TELL "NAPS-TODAY: " NAPS-TODAY CR>
    <TELL "TOTAL-FOOD: " TOTAL-FOOD CR>
    <TELL "FOUND-MAGIC-RING: " FOUND-MAGIC-RING CR>

    <TELL "Monster vars" CR>
    <TELL "PARENT-MONSTER-HEALTH-STEP: " PARENT-MONSTER-HEALTH-STEP CR>
    <TELL "PARENT-MONSTER-ATK-DMG: " PARENT-MONSTER-ATK-DMG CR>
    <TELL "CHILD-MONSTER-ATK-DMG: " CHILD-MONSTER-ATK-DMG CR>
    <TELL "PARENT-MONSTER-FIRST-ENCOUNTER: " PARENT-MONSTER-FIRST-ENCOUNTER CR>
    <TELL "PARENT-MONSTER-WILL-CHASE: " PARENT-MONSTER-WILL-CHASE CR>
    <TELL "CHILD-MONSTER-WILL-CHASE: " CHILD-MONSTER-WILL-CHASE CR>

    <TELL "Win vars" CR>
    <TELL "WIN-ENTER-CABIN: " WIN-ENTER-CABIN CR>
    <TELL "WIN-LIGHT-FIRE: " WIN-LIGHT-FIRE CR>
    <TELL "WIN-COOK-MEAL: " WIN-COOK-MEAL CR>
    <TELL "WIN-FIND-GEM: " WIN-FIND-GEM CR>
    <TELL "WIN-ENTER-CAVE: " WIN-ENTER-CAVE CR>
    <TELL "WIN-KILL-MONSTER: " WIN-KILL-MONSTER CR>
    <TELL "WIN-WRITE-NOTE: (later)" CR>
    <TELL "WIN-BUILD-BOAT: " WIN-BUILD-BOAT CR>

    <TELL "Conditions" CR>
    <COND (
        <IS-EQUAL <GET-VAR C-ROOM ABOVE-GROUND> 1>
        <TELL "ABOVE-GROUND: 1" CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "ABOVE-GROUND: !1" CR>
    )>
    <COND (
        <IS-EQUAL <ROOM-HAS-LIGHT> 1>
        <TELL "ROOM-HAS-LIGHT: 1" CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "ROOM-HAS-LIGHT: !1" CR>
    )>
>