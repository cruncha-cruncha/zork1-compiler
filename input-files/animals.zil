<GLOBAL TALK-PROMPTS 0>

<GLOBAL KILLED-CROW 0>

;"used by the bear to ask a question and get a response"
<ROOM Q-STORAGE>

<OBJECT BAT
    (AKA BAT)
    (DESC "a BAT")
    (VARS IS-ANIMAL 1 HEALTH 1 OWN-TAKE 1 IS-SOFT 1)>

<OBJECT OWL
    (AKA OWL)
    (DESC "an OWL")
    (VARS IS-ANIMAL 1 HEALTH 1 OWN-TAKE 1 IS-SOFT 1)>

<OBJECT CROW
    (AKA CROW)
    (DESC "a CROW")
    (VARS IS-ANIMAL 1 HEALTH 1 OWN-TAKE 1 IS-SOFT 1)>

<OBJECT FISH
    (AKA FISH)
    (DESC "a FISH")
    (COPY <WATER 1>)
    (VARS IS-ANIMAL 1 HEALTH 1 OWN-TAKE 1 IS-SOFT 1)
    (ACT-IN-ROOM <DROWN-FISH>)
    (ACT-IN-PLAYER <DROWN-FISH>)>

<OBJECT FROG
    (AKA FROG)
    (DESC "a FROG")
    (VARS IS-ANIMAL 1 HEALTH 1 OWN-TAKE 1 IS-SOFT 1)>

;"not actually an animal, just some set decoration"
<OBJECT BEETLE
    (AKA BEETLE)
    (DESC "a BEETLE")>

<OBJECT RABBIT
    (AKA RABBIT)
    (DESC "a RABBIT")
    (COPY <ROOM CAVE-ENTRANCE-1>)
    (VARS IS-ANIMAL 1 HEALTH 1 OWN-TAKE 1 IS-SOFT 1)>

<OBJECT SNAKE
    (AKA SNAKE)
    (DESC "a SNAKE")
    (VARS IS-ANIMAL 1 HEALTH 1 OWN-TAKE 1 IS-SOFT 1)
    (ACT-IN-PLAYER <SNAKE-IN-PLAYER>)>

<OBJECT BEAR
    (AKA BEAR)
    (DESC "a BEAR")
    (VARS IS-ANIMAL 1 HEALTH 20 OWN-TAKE 1 IS-SOFT 1 ASKED 0)
    (COPY <ROOM STORAGE>
          <ROOM STORAGE>)
    (ACT-IN-ROOM <BEAR-IN-ROOM>)>

;"a question from the bear or crow"
<OBJECT YN-Q
    (VARS EXP 1)>

<TALK (N) BEETLE
    <SET-VAR N <RAND>>
    <COND (
        <IS-DES 20 N>
        <TELL "It flaps it's wings" CR>
    )(
        <IS-DES 40 N>
        <TELL "It rolls away" CR>
    )(
        <IS-DES 60 N>
        <TELL "It points urgently upwards. Or maybe it's just stretching." CR>
    )(
        <IS-DES 80 N>
        <TELL "It opens and closes it's carapace." CR>
    )(
        <IS-DES 100 N>
        <TELL "It doesn't listen." CR>
    )>
>

<TALK () OWL
    <COND (
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"can't talk to a dead animal"
        <RETURN 0>
    )>
    <COND (
        <OR
            <IS-EQUAL <GET-VAR PLAYER T-OWL> 0> 
            <IS-EQUAL DAY-M-3 0>
        >
        <TELL "'Would you like to hear how EUNICE BROKE MY HEART? She left one night without saying goodbye.'" CR>
    )(
        <IS-EQUAL DAY-M-3 1>
        <TELL "'Would you like to hear how EUNICE BROKE MY HEART? He left one night without saying goodbye.'" CR>
    )(
        <IS-EQUAL DAY-M-3 2>
        <TELL "'Would you like to hear how EUNICE BROKE MY HEART? They left one night without saying goodbye.'" CR>
    )>
    <TELL "The owl trails off, lost in thought." CR>
    <COND (
        <IS-ASC <GET-VAR PLAYER T-OWL> 1>
        <SET-VAR PLAYER T-OWL 1>
    )>
>

<TALK (N) CROW
    <SET-VAR N <RAND>>
    <COND (
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"can't talk to a dead animal"
        <RETURN 0>
    )>
    <COND (
        <AND
            <IS-IN Q-STORAGE YN-Q>
            <IS-EQUAL <GET-VAR <INST Q-STORAGE YN-Q> FROM-CROW> 1>
        >
        <SET-VAR <INST Q-STORAGE YN-Q> EXP 1>
        <COND (
            <IS-DES 34 N>
            <TELL "'Are you deaf? Or just ugly?'" CR>
        )(
            <IS-DES 67 N>
            <TELL "'I said WOULD YOU LIKE TO FLY'" CR>
        )(
            <IS-DES 100 N>
            <TELL "The crow says a rude word" CR>
        )>
    )(
        <IS-DES 34 N>
        <TELL "'GROWING UP BACK HOME, my sister always used to tease me, so I'd try to steal her toys.'" CR>
    )(
        <IS-DES 67 N>
        <TELL "'GROWING UP BACK HOME, I'd try to steal my sisters toys. She'd always tease me for it.'" CR>
    )(
        <IS-DES 100 N>
        <TELL "'Don't you wish you could fly?'" CR>
        <COPY-MOVE YN-Q Q-STORAGE>
        <SET-VAR <INST Q-STORAGE YN-Q> FROM-CROW 1>
    )>
    <COND (
        <IS-ASC <GET-VAR PLAYER T-CROW> 1>
        <SET-VAR PLAYER T-CROW 1>
    )>
>

<TALK () FISH
    <COND (
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"can't talk to a dead animal"
        <RETURN 0>
    )>

    <COND (
        <OR
            <AND
                <IS-EQUAL WATER <LOC <CMD 1>>>
                <OR
                    <IS-IN <LOC <CMD 1>> BERRIES>
                    <IS-IN <LOC <CMD 1>> NUTS>
                >
            >
            <IS-EQUAL C-ROOM <LOC <CMD 1>>>
            <IS-EQUAL PLAYER <LOC <CMD 1>>>
        >
        <COND (
            <IS-DES 40 <RAND>>
            <TELL "'You know, I don't think SCHOOL IS FOR ME. Oh sure I got a degree, but I haven't
            kept in touch with anyone from those days, and I don't miss it. I just felt out of place the whole time,
            my scales were crawling. It was nice to have some structure, but I prefer learning at my own pace.'" CR>
            <COND (
                <IS-ASC <GET-VAR PLAYER T-FISH> 1>
                <SET-VAR PLAYER T-FISH 1>
            )>
        )(
            <IS-EQUAL 1 1>
            <TELL "You know what they say about fishing" CR>
        )>
    )(
        <AND
            <IS-EQUAL WATER <LOC <CMD 1>>>
            <NOT <OR
                <IS-IN <LOC <CMD 1>> BERRIES>
                <IS-IN <LOC <CMD 1>> NUTS>
            >>
        >
        <TELL "Try adding some BERRIES or NUTS to the water" CR>
    )>
>

<TALK () FROG
    <COND (
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"can't talk to a dead animal"
        <RETURN 0>
    )>
    <COND (
        <OR
                <IS-EQUAL <GET-VAR PLAYER T-FROG> 0> 
                <IS-EQUAL DAY-M-3 0>
        >
        <TELL "'You ain't never seen A BUG THAT BIG. Beefy sucker, huge wings,
        beady eyes, flying real low to the ground. I nabbed it from right here,
        sitting just like this, and it knew it was right away licked. Had an earthy aftertaste,
        notes of herb, kept me fed for the whole day.'" CR>
    )(
        <IS-EQUAL DAY-M-3 1>
        <TELL "'I ain't ever caught A BUG THAT BIG before. Beefy sucker, huge wings,
        diamond eyes, flapping away, darting through the trees. I surprised it from over there,
        had to open pretty wide. Bit of a wormy aftertaste, and crunchier than I'm used to,
        but it kept me fed for a whole week.'" CR>
    )(
        <IS-EQUAL DAY-M-3 2>
        <TELL "'Nobody's ever seen A BUG THAT BIG again. Beefy sucker, huge wings,
        a hundred eyes, faster than stink, and flying higher than any crow I've ever
        seen. I had to leap up in order to grab it, and it fought me all the way down, burnt
        my tongue something fierce. Bit of a metallic aftertaste, and the screams were so loud,
        but it kept me fed for a whole month.'" CR>
    )>
    <COND (
        <IS-ASC <GET-VAR PLAYER T-FROG> 1>
        <SET-VAR PLAYER T-FROG 1>
    )>
>

<TALK () RABBIT
    <COND (
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"can't talk to a dead animal"
        <RETURN 0>
    )>
    <COND (
        <OR
                <IS-EQUAL <GET-VAR PLAYER T-RABBIT> 0> 
                <IS-EQUAL DAY-M-3 0>
        >
        <TELL "'I don't know if you EVER PLAY THE LOTTERY. Last week, I won half a dozen carrots. It's
        all thanks to my lucky pebble: I always keep it close, and it helps me roll. Yes, the dice
        was in my favour that day.'" CR>
    )(
        <IS-EQUAL DAY-M-3 1>
        <TELL "'Be careful if you EVER PLAY THE LOTTERY. Two years ago, I won half a dozen carrots. It
        was all thanks to my lucky pebble; still keep it close, but it hasn't been helping me roll lately. The die
        are no longer in my favour.'" CR>
    )(
        <IS-EQUAL DAY-M-3 2>
        <TELL "'Got my lucky pebble if i EVER PLAY THE LOTTERY. Next week, I'm going to win half a dozen carrots. I'll
        keep the pebble right here under my paw, and it'll help me roll. I've already found it, so I know it's lucky. The die
        will surely be in my favour.'" CR>
    )>
    <COND (
        <IS-ASC <GET-VAR PLAYER T-RABBIT> 1>
        <SET-VAR PLAYER T-RABBIT 1>
    )>
>

<TALK () SNAKE
    <COND (
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"can't talk to a dead animal"
        <RETURN 0>
    )>
    <COND (
        <OR
                <IS-EQUAL <GET-VAR PLAYER T-SNAKE> 0> 
                <IS-EQUAL DAY-M-3 0>
        >
        <TELL "'Every night I fall asleep LISTENING TO THE RIVER. I dream of flying,
        curving through the sky, over the trees and through the clouds. My body grows
        larger and larger until I'm size of the river. When I land, my body dissolves 
        into water.'" CR>
    )(
        <IS-EQUAL DAY-M-3 1>
        <TELL "'Most nights I fall asleep LISTENING TO THE RIVER. I dream of flying,
        sliding around the sky, over spiky trees and through soft clouds. My body balloons
        to the size of the river. When I'm done flying, I crash back to the earth and
        my body becomes water.'" CR>
    )(
        <IS-EQUAL DAY-M-3 2>
        <TELL "'Some nights I fall asleep LISTENING TO THE RIVER. I dream of flying,
        hurtling through the air, the clouds cold around me, trees far below. My body bloats
        until I'm the size of the river. I crash back to earth, exploding into water.'" CR>
    )>
    <COND (
        <IS-ASC <GET-VAR PLAYER T-SNAKE> 1>
        <SET-VAR PLAYER T-SNAKE 1>
    )>
>

<TALK () BAT
    <COND (
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"can't talk to a dead animal"
        <RETURN 0>
    )>
    <COND (
        <IS-DES 40 <RAND>>
        <TELL "'I once saw a GREAT BALL OF FIRE in the sky. Took me 15 minutes, but it was there.'" CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "It doesn't hear you." CR>
    )>
    <COND (
        <IS-ASC <GET-VAR PLAYER T-BAT> 1>
        <SET-VAR PLAYER T-BAT 1>
    )>
>

<TALK () BEAR
    <COND (
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"can't talk to a dead animal"
        <RETURN 0>
    )>
    <TELL "The bear will talk when it's ready" CR>
>

<ROUTINE SNAKE-IN-PLAYER ()
    <COND (
        <IS-DES <GET-VAR <CMD 0> HEALTH> 0>
        <TELL "The snake bites you for 2 damage" CR>
        <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> 2>>
    )>
>

<TAKE () FROG
    <COND (
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        <TELL "Picked up" CR>
        <MOVE <CMD 1> PLAYER>
    )(
        <IS-DES 75 <RAND>>
        <TELL "You grab it." CR>
        <MOVE <CMD 1> PLAYER>
    )(
        <IS-EQUAL 1 1>
        <TELL "It slips away." CR>
    )>
>

<TAKE () FISH
    <COND (
        <AND 
            <IS-EQUAL WATER <LOC <CMD 1>>>
            <IS-DES <GET-VAR <CMD 1> HEALTH> 0>
        >
        <COND (
            <AND
                <OR
                    <IS-IN <LOC <CMD 1>> BERRIES>
                    <IS-IN <LOC <CMD 1>> NUTS>
                >
                <IS-DES 40 <RAND>>
            >
            <TELL "You caught it" CR>
            <MOVE <CMD 1> PLAYER>
        )(
            <NOT <OR
                <IS-IN <LOC <CMD 1>> BERRIES>
                <IS-IN <LOC <CMD 1>> NUTS>
            >>
            <TELL "It helps if there's some good bait in the water." CR>
        )(
            <IS-EQUAL 1 1>
            <TELL "Try again, it takes patience to catch a fish." CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "Picked up" CR>
        <MOVE <CMD 1> PLAYER>
    )>
>

<ROUTINE DROWN-FISH ()
    <COND (
        <NOT <IS-EQUAL WATER <LOC <CMD 0>>>>
        <COND (
            <NOT <IS-DES <GET-VAR <CMD 0> HEALTH> 0>>
            <TELL "The fish dies" CR>
        )>
        <SET-VAR <CMD 0> HEALTH 0>
    )>
>

<TAKE () RABBIT
    <COND (
        <IS-DES <GET-VAR <CMD 1> HEALTH> 0>
        <COND (
            <OR 
                <IS-IN PLAYER BERRIES>
                <IS-IN PLAYER HERBS>
                <IS-IN PLAYER FERN>
            >
            <TELL "Picked up" CR>
        )(
            <IS-EQUAL 1 1>
            <TELL "It darts away, you're not carrying any food it likes." CR>
        )>            
    )(
        <IS-EQUAL 1 1>
        <TELL "Picked up" CR>
        <MOVE <CMD 1> PLAYER>
    )>
>

<TAKE () SNAKE
    <COND (
        <IS-DES <GET-VAR <CMD 1> HEALTH> 0>
        <TELL "The snake doesn't like this" CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "Picked up" CR>
    )>
    <MOVE <CMD 1> PLAYER>
>

<TAKE () BAT
    <TELL "The bat is too nimble." CR>
>

<TAKE () BEAR
    <TELL "This bear doesn't want a hug; it snaps your neck." CR>
    <SET-VAR PLAYER HEALTH 0>
>

<ROUTINE BEAR-IN-ROOM (QN DMG) 
    <COND (
        <IS-DES <GET-VAR <CMD 0> HEALTH> 0>
        <COND (
            <IS-EQUAL <GET-VAR <CMD 0> ENRAGED> 1>
            <TELL "The bear swings at you">            
            <SET-VAR <CMD 0> ENRAGED 0>
            <SET-VAR DMG <DIVIDE <RAND> 5>>
            <COND (
                <IS-ASC DMG 5>
                <TELL ", but you manage to dodge it" CR>
            )(
                <IS-EQUAL 1 1>
                <TELL "." CR>
                <TELL "You take " DMG " damage" CR>
                <SET-VAR PLAYER HEALTH <SUBTRACT <GET-VAR PLAYER HEALTH> DMG>>
            )>
        )(
            <IS-IN PLAYER GEM>
            <COND (
                <AND 
                    <IS-DES 50 <RAND>>
                    <NOT <IS-EQUAL <GET-VAR <CMD 0> ASKED> 1>>
                >
                <TELL "A bear asks 'How's your boat coming along?'" CR>
                <SET-VAR <CMD 0> ASKED 1>
            )>
            <RETURN 1>
        )(
            <AND 
                <NOT <IS-IN Q-STORAGE YN-Q>>
                <NOT <IS-EQUAL <GET-VAR <CMD 0> ASKED> 1>>
            >
            <TELL "A bear asks you ">
            <SET-VAR QN <RAND>>
            <COND (
                <IS-DES 20 QN>
                <TELL "'Do you know the owl?'" CR>
                <COPY-MOVE YN-Q Q-STORAGE>
                <SET-VAR <INST Q-STORAGE YN-Q> K-OWL 1>
            )(
                <IS-DES 40 QN>
                <TELL "'Do you know the fish?'" CR>
                <COPY-MOVE YN-Q Q-STORAGE>
                <SET-VAR <INST Q-STORAGE YN-Q> K-FISH 1>
            )(
                <IS-DES 60 QN>
                <TELL "'Do you know the frog?'" CR>
                <COPY-MOVE YN-Q Q-STORAGE>
                <SET-VAR <INST Q-STORAGE YN-Q> K-FROG 1>
            )(
                <IS-DES 80 QN>
                <TELL "'Do you know the rabbit?'" CR>
                <COPY-MOVE YN-Q Q-STORAGE>
                <SET-VAR <INST Q-STORAGE YN-Q> K-RABBIT 1>
            )(
                <IS-DES 100 QN>
                <TELL "'Do you know the snake?'" CR>
                <COPY-MOVE YN-Q Q-STORAGE>
                <SET-VAR <INST Q-STORAGE YN-Q> K-SNAKE 1>
            )>
            <TELL " (it expects a YES or NO)" CR>
            <SET-VAR <INST Q-STORAGE YN-Q> FROM-BEAR 1>
        )>
    )>
>

<YES (TF)
    <COND (
        <IS-EQUAL <GET-VAR <INST Q-STORAGE YN-Q> FROM-CROW> 1>
        <COND (
            <AND 
                <IS-IN C-ROOM CROW>
                <IS-DES <GET-VAR <INST C-ROOM CROW> HEALTH> 0>
            >
            <TELL "'Not me'" CR>
            <TELL "The crow flies off" CR>
            <MOVE <INST C-ROOM CROW>> 
        )>
        <MOVE <INST Q-STORAGE YN-Q>>
        <RETURN 1>
    )>

    <COND (
        <IS-EQUAL <GET-VAR <INST Q-STORAGE YN-Q> K-OWL> 1>
        <COND (
            <IS-EQUAL <GET-VAR PLAYER T-OWL> 0>
            <SET-VAR TF -1>
        )(
            <IS-EQUAL 1 1>
            <SET-VAR TF 1>
        )>
    )(
        <IS-EQUAL <GET-VAR <INST Q-STORAGE YN-Q> K-FISH> 1>
        <COND (
            <IS-EQUAL <GET-VAR PLAYER T-FISH> 0>
            <SET-VAR TF -1>
        )(
            <IS-EQUAL 1 1>
            <SET-VAR TF 1>
        )>
    )(
        <IS-EQUAL <GET-VAR <INST Q-STORAGE YN-Q> K-FROG> 1>
        <COND (
            <IS-EQUAL <GET-VAR PLAYER T-FROG> 0>
            <SET-VAR TF 0>
        )(
            <IS-EQUAL 1 1>
            <SET-VAR TF 1>
        )>
    )(
        <IS-EQUAL <GET-VAR <INST Q-STORAGE YN-Q> K-RABBIT> 1>
        <COND (
            <IS-EQUAL <GET-VAR PLAYER T-RABBIT> 0>
            <SET-VAR TF -1>
        )(
            <IS-EQUAL 1 1>
            <SET-VAR TF 1>
        )>
    )(
        <IS-EQUAL <GET-VAR <INST Q-STORAGE YN-Q> K-SNAKE> 1>
        <COND (
            <IS-EQUAL <GET-VAR PLAYER T-SNAKE> 0>
            <SET-VAR TF -1>
        )(
            <IS-EQUAL 1 1>
            <SET-VAR TF 1>
        )>
    )>

    <COND (
        ;"if you do know the animal, bear will reveal location of the gem"
        <AND
            <IS-EQUAL TF 1>
            <IS-IN C-ROOM BEAR>
        >
        <TELL "'Good, they are my friend.'" CR>
        <COND (
            <NOT <IS-IN FOREST-3 TREE-HOLLOW>>
            <TELL "'The crow is always stealing things. It hides them in a tree hollow, east of the lake.'" CR>
            <MOVE <INST STORAGE TREE-HOLLOW> FOREST-3>
        )>
        <TELL "Conversation over, the bear sits down to forage" CR>
        <SET-VAR <INST C-ROOM BEAR> ASKED 1>
        <MOVE <INST Q-STORAGE YN-Q>>
    )(
        ;"if you lie, bear is enraged and attacks"
        <AND
            <IS-EQUAL TF -1>
            <IS-IN C-ROOM BEAR>
        >
        <TELL "'Liar'" CR>
        <MOVE <INST Q-STORAGE YN-Q>>
        <SET-VAR <INST C-ROOM BEAR> ENRAGED 1>
    )>    
>

<NO ()
    <COND (
        <IS-EQUAL <GET-VAR <INST Q-STORAGE YN-Q> FROM-CROW> 1>
        <TELL "'Shame'" CR>
        <MOVE <INST Q-STORAGE YN-Q>>
    )(
        <IS-EQUAL <GET-VAR <INST Q-STORAGE YN-Q> FROM-BEAR> 1>
        <TELL "The bear shrugs, then sits down to forage" CR>
        <SET-VAR <INST C-ROOM BEAR> ASKED 1>
        <MOVE <INST Q-STORAGE YN-Q>>
    )>
>

<HIT FROG ()
    <COND (
        <AND
                <IS-EQUAL <CMD 1> FROG>
                <IS-EQUAL <CMD 2> FROG>
        >
        <TELL "They bounce off eachother" CR>
    )(
        <IS-EQUAL <CMD 2> FROG>
        <COND (
                <NOT <IS-EQUAL <GET-VAR <CMD 2> HEALTH> 0>>
                <TELL "The frog dies" CR>
                <SET-VAR <CMD 2> HEALTH 0>
        )>
    )(
        <AND
                <IS-EQUAL <CMD 1> FROG>
                <IS-EQUAL <GET-VAR <CMD 1> HEALTH> 0>
        >
        <TELL "The frog is already dead" CR>
    )(
        <AND
                <IS-EQUAL <CMD 1> FROG>
                <IS-IN PLAYER <CMD 1>>
        >
        <TELL "The frog dies" CR>
        <SET-VAR <CMD 1> HEALTH 0>
    )(
        <IS-EQUAL 1 1>
        <COND (
                <IS-DES 80 <RAND>>
                <TELL "You hit the frog, it dies" CR>
                <SET-VAR <CMD 1> HEALTH 0>
        )(
                <IS-EQUAL 1 1>
                <TELL "The frog is too quick for you, better luck next time" CR>
                <MOVE <CMD 1>>
        )>
    )>
>

<HIT BEAR (DMG)
    <COND (
        <IS-EQUAL BEAR <CMD 2>>
        ;"hitting something with a bear does nothing"
    )(
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"hitting a dead bear does nothing"
    )>

    ;"the bear will hit back after any attempted hit"
    <SET-VAR <CMD 1> ENRAGED 1>

    <COND (
        ;"weapons deal damage with their damage"
        <IS-DES <GET-VAR <CMD 2> MAX-DAMAGE> 0>
        <SET-VAR COMBAT-DAMAGE <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR COMBAT-MAX-DAMAGE <GET-VAR <CMD 2> MAX-DAMAGE>>
    )(
        ;"can only hit with a weapon"
        <IS-EQUAL 1 1>
        <TELL "You can't hit it with that" CR>
        <RETURN 0>
    )>
                
    <SET-VAR DMG <ROLL-DMG>>
    <COND (
        <IS-EQUAL DMG 0>
        <TELL "The bear takes no damage" CR>
        <RETURN 1>
    )>

    <SET-VAR <CMD 1> HEALTH <SUBTRACT <GET-VAR <CMD 1> HEALTH> DMG>>
    <TELL "You hit the bear for " DMG " damage." CR>

    <COND (
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"the bear is dead"
        <TELL "The bear is dead" CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "It lumbers away" CR>
        <MOVE <CMD 1> STORAGE>
    )>
>

<HIT SNAKE ()
    <COND (
        <IS-EQUAL SNAKE <CMD 2>>
        <COND (
            <IS-DES <GET-VAR <CMD 2> HEALTH> 0>
            <SET-VAR <CMD 2> HEALTH 0>
            <TELL "The snake dies" CR>
        )>
    )(
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"hitting a dead snake does nothing"
    )>

    <COND (
        <IS-IN PLAYER <CMD 1>>
        <TELL "You kill it." CR>
        <SET-VAR <CMD 1> HEALTH 0>
    )(
        <IS-EQUAL 1 1>
        <COND (
                <IS-DES 67 <RAND>>
                <TELL "You hit it, and it dies" CR>
                <SET-VAR <CMD 1> HEALTH 0>
        )(
                <IS-EQUAL 1 1>
                <TELL "The snake is too quick for you, better luck next time" CR>
                <MOVE <CMD 1>>
        )>
    )>
>

<HIT RABBIT ()
    <COND (
        <IS-EQUAL RABBIT <CMD 2>>
        <COND (
            <IS-DES <GET-VAR <CMD 2> HEALTH> 0>
            <SET-VAR <CMD 2> HEALTH 0>
            <TELL "The rabbit dies" CR>
        )>
    )(
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"hitting a dead rabbit does nothing"
    )>

    <COND (
        <IS-IN PLAYER <CMD 1>>
        <TELL "You kill it." CR>
        <SET-VAR <CMD 1> HEALTH 0>
    )(
        <IS-EQUAL 1 1>
        <COND (
                <IS-DES 40 <RAND>>
                <TELL "You hit it, and it dies" CR>
                <SET-VAR <CMD 1> HEALTH 0>
        )(
                <IS-EQUAL 1 1>
                <TELL "The rabbit is too quick for you, better luck next time" CR>
                <MOVE <CMD 1>>
        )>
    )>
>

<HIT CROW ()
    <COND (
        <IS-EQUAL CROW <CMD 2>>
        <COND (
            <IS-DES <GET-VAR <CMD 2> HEALTH> 0>
            <SET-VAR <CMD 2> HEALTH 0>
            <TELL "The crow dies" CR>
        )>
    )(
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"hitting a dead crow does nothing"
    )>

    <COND (
        <IS-IN PLAYER <CMD 1>>
        <TELL "You kill it." CR>
        <SET-VAR <CMD 1> HEALTH 0>
    )(
        <IS-EQUAL KNIFE <CMD 2>>
        <TELL "You throw your knife at the crow." CR>
        <COND (
            <AND
                <IS-DES 5 <RAND>>
                <IS-EQUAL KILLED-CROW 0>
            >
            <TELL "You hit it, and it dies." CR>
            <SET-VAR <CMD 1> HEALTH 0>
            <MOVE <INST PLAYER KNIFE> C-ROOM>
            <SET-VAR KILLED-CROW 1>
        )(
            <IS-EQUAL 1 1>
            <TELL "You miss, and lose your knife. The crow laughs at you." CR>
            <MOVE <INST C-ROOM KNIFE>>
            <MOVE <INST PLAYER KNIFE>>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "Try hitting it with your knife" CR>
    )>
>

<HIT OWL ()
    <COND (
        <IS-EQUAL OWL <CMD 2>>
        <COND (
            <IS-DES <GET-VAR <CMD 2> HEALTH> 0>
            <SET-VAR <CMD 2> HEALTH 0>
            <TELL "The owl dies" CR>
        )>
    )(
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"hitting a dead owl does nothing"
    )>

    <COND (
        <IS-IN PLAYER <CMD 1>>
        <TELL "You kill it." CR>
        <SET-VAR <CMD 1> HEALTH 0>
    )(
        <IS-EQUAL 1 1>
        <TELL "You miss, and the owl flies away." CR>
        <MOVE <CMD 1>>
    )>
>

<HIT BAT () 
    <COND (
        <IS-EQUAL BAT <CMD 2>>
        <COND (
            <IS-DES <GET-VAR <CMD 2> HEALTH> 0>
            <SET-VAR <CMD 2> HEALTH 0>
            <TELL "The bat dies" CR>
        )>
    )(
        <NOT <IS-DES <GET-VAR <CMD 1> HEALTH> 0>>
        ;"hitting a dead bat does nothing"
    )>

    <COND (
        <IS-IN PLAYER <CMD 1>>
        <TELL "You kill it." CR>
        <SET-VAR <CMD 1> HEALTH 0>
    )(
        <IS-EQUAL 1 1>
        <COND (
                <IS-DES 50 <RAND>>
                <TELL "You hit it, and it dies" CR>
                <SET-VAR <CMD 1> HEALTH 0>
        )(
                <IS-EQUAL 1 1>
                <TELL "The bat just keeps flapping around" CR>
        )>
    )>
>

;"bat, GREAT BALL FIRE"
<GREAT ()
    <COND (
        <IS-EQUAL C-ROOM CAVERN-1>
        <COND (
            <IS-EQUAL <GET-VAR <INST CAVERN-1 STONE-DOOR> IS-LOCKED> 1>
            <SET-VAR <INST CAVERN-1 STONE-DOOR> IS-LOCKED 0>
            <TELL "The stone door clicks, and slides open" CR>
            <MOVE PLAYER CRYPT>
        )(
            <IS-EQUAL 1 1>
            <TELL "The stone door rumbles, but it can't open any further" CR>
        )>
        <SET-VAR PLAYER T-BAT 2>
    )(
        <IS-EQUAL <GET-VAR <INST CAVERN-1 STONE-DOOR> IS-LOCKED> 1>
        <TELL "You hear a deep but faint rumble of stone on stone; something is trying to move. Maybe you're not in the right place." CR>
    )(
        <IS-EQUAL 1 1>
        <TELL "Far away, you hear the stone door rumbling." CR>
    )>
>

;"fish, SCHOOL IS FOR ME"
<SCHOOL ()
    <COND (
        <IS-EQUAL COMBAT-HIGH-CHANCE 50>
        <SET-VAR COMBAT-HIGH-CHANCE 75>
        <TELL "Your weapons suddenly feel lighter, nimbler. Water can crash, or it can flow." CR>
        <SET-VAR PLAYER T-FISH 2>
    )>
>

;"frog, BUG THAT BIG"
<BUG ()
    <COND (
        <IS-EQUAL MAX-MAX-HEALTH 100>
        <SET-VAR MAX-MAX-HEALTH 150>
        <SET-VAR PLAYER MAX-HEALTH <ADD <GET-VAR PLAYER MAX-HEALTH> 50>>
        <TELL "Your suddenly see the potential for growth, and a powerful thirst consumes you." CR>
        <SET-VAR PLAYER T-FROG 2>
    )>
>

;"owl, EUNICE BROKE HEART"
<EUNICE (DIFF)
    <EACH-OBJ SWORD (INST)
        <COND (
            <IS-EQUAL <GET-VAR INST MAX-DAMAGE> 30>
            <SET-VAR INST MAX-DAMAGE 45>
            <SET-VAR DIFF 1>
        )>
    >
    <EACH-OBJ AXE (INST)
        <COND (
            <IS-EQUAL <GET-VAR INST MAX-DAMAGE> 10>
            <SET-VAR INST MAX-DAMAGE 16>
            <SET-VAR DIFF 1>
        )>
    >
    <EACH-OBJ KNIFE (INST)
        <COND (
            <IS-EQUAL <GET-VAR INST MAX-DAMAGE> 4>
            <SET-VAR INST MAX-DAMAGE 6>
            <SET-VAR DIFF 1>
        )>
    >
    <EACH-OBJ OBSIDIAN-SHARD (INST)
        <COND (
            <IS-EQUAL <GET-VAR INST MAX-DAMAGE> 50>
            <SET-VAR INST MAX-DAMAGE 70>
            <COND (
                <IS-EQUAL <GET-VAR INST DAMAGE> 50>
                <SET-VAR INST DAMAGE 70>
            )>
            <SET-VAR DIFF 1>
        )>
    >

    <COND (
        <IS-EQUAL DIFF 1>
        <TELL "You suddenly understand how the owl keeps it's talons so sharp. Your weapons could be deadlier." CR>
        <SET-VAR PLAYER T-OWL 2>
    )>
>

;"snake, LISTENING TO RIVER"
<LISTENING ()
    <COND (
        <IS-EQUAL PARENT-MONSTER-HEALTH-STEP 8>
        <SET-VAR PARENT-MONSTER-HEALTH-STEP 5>
        ;"we don't know exactly where the monsters are, so this is the safest way of setting their variables"
        <EACH-OBJ PARENT-MONSTER (M)
            <SET-VAR M MAX-HEALTH 
                <SUBTRACT <GET-VAR M MAX-HEALTH> 10>>
        >
        <EACH-OBJ CHILD-MONSTER (M)
            <SET-VAR M MAX-HEALTH 90>
        >
        <TELL "You can suddenly picture the path ahead, enemies weaken at your step." CR>
        <SET-VAR PLAYER T-SNAKE 2>
    )>
>

;"rabbit, EVER PLAY LOTTERY"
<EVER ()
    <COND (
        <IS-EQUAL PARENT-MONSTER-ATK-DMG 40>
        <SET-VAR PARENT-MONSTER-ATK-DMG 28>
        <SET-VAR CHILD-MONSTER-ATK-DMG 2>
        <TELL "You realize that sometimes you can't change the die, only the wager. Enemies can't hit you nearly as hard when you've got less exposure." CR>
        <SET-VAR PLAYER T-RABBIT 2>
    )>
>

;"crow GROWING UP BACK HOME"
<GROWING ()
    <COND (
        <NOT <IS-IN FOREST-3 TREE-HOLLOW>>
        <MOVE <INST STORAGE TREE-HOLLOW> FOREST-3>
        <COND (
            <IS-IN FOREST-3 PLAYER>
            <TELL "You notice a hollow in that tree over there, looks like a good place to hide things." CR>
        )(
            <IS-EQUAL 1 1>
            <TELL "A voice on the breeze whispers 'You'll never find my tree hollow'" CR>
        )>
    )>
    <COND (
        <NOT <IS-EQUAL <GET-VAR PLAYER T-CROW> 2>>
        <SET-VAR PLAYER T-CROW 2>
        ;"edges will stay sharp, excluding obsidian and effects of the cursed skull"
        <TELL "You realize that all things are one, and finding a way through is not so difficult." CR>
    )>
>


