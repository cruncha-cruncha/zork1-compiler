;"chance to high-roll. Hit is guaranteed, but damage is always high or low roll"
<GLOBAL COMBAT-HIGH-CHANCE 50>

;"inputs"
<GLOBAL COMBAT-DAMAGE 0>
<GLOBAL COMBAT-MAX-DAMAGE 0>

;"DAMAGE x (1 + ((health - (max-health / 2)) / max-health)) +/- (MAX-DAMAGE / 4)"
;"or DAMAGE +/- MAX-DAMAGE when the moon is full"

<ROUTINE ROLL-DMG (DMG COEF)
    <COND (
        <IS-ASC <RAND> COMBAT-HIGH-CHANCE>
        <SET-VAR COEF 1>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR COEF -1>
    )>

    <COND (
        <IS-EQUAL MOON-PHASE 4>
        <SET-VAR DMG <ADD COMBAT-DAMAGE <MULTIPLY COMBAT-MAX-DAMAGE COEF>>>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR DMG <ADD
            <MULTIPLY COMBAT-DAMAGE
                <ADD 1 <DIVIDE <SUBTRACT <GET-VAR PLAYER HEALTH> <DIVIDE <GET-VAR PLAYER MAX-HEALTH> 2>> <GET-VAR PLAYER MAX-HEALTH>>>
            >
            <MULTIPLY COEF <DIVIDE COMBAT-MAX-DAMAGE 4>>
        >>
    )>

    <COND (
        ;"Negative damage injures the player"
        <IS-ASC DMG 0>
        <SET-VAR PLAYER HEALTH <ADD <GET-VAR PLAYER HEALTH> DMG>>
        <TELL "Your weapon is dull, causing you to injure yourself." CR> 
        <SET-VAR DMG 0>
    )>

    <RETURN DMG>
>

<HIT () AXE
    <COND (
        <NOT <IS-EQUAL AXE <CMD 2>>>
        ;"don't handle that here"
    )>

    <COND (
        <IS-EQUAL <GET-VAR PLAYER T-CROW> 2>
        ;"edge never goes dull from a hit after invoking the crow's story" 
    )(
        <OR
            <IS-ASC 0 <GET-VAR <CMD 1> MAX-DAMAGE>>
            <IS-ASC 0 <GET-VAR <CMD 1> IS-HARD>>
        >
        <SET-VAR <CMD 2> DAMAGE <SUBTRACT <GET-VAR <CMD 2> DAMAGE> 2>>
        <TELL "Your axe takes 2 points of damage" CR>
    )(
        <IS-EQUAL <GET-VAR <CMD 1> IS-SOFT> 1>
        ;"hitting something soft will not dull the edge of the axe"
    )(
        <IS-EQUAL 1 1>
        <SET-VAR <CMD 2> DAMAGE <SUBTRACT <GET-VAR <CMD 2> DAMAGE> 1>>
        <TELL "Your axe takes 1 point of damage" CR>
    )>

    <COND (
        <IS-DES 0 <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR <CMD 2> DAMAGE 0>
    )>
>

<HIT () KNIFE
    <COND (
        <NOT <IS-EQUAL KNIFE <CMD 2>>>
        ;"don't handle that here"
    )>

    <COND (
        <IS-EQUAL <GET-VAR PLAYER T-CROW> 2>
        ;"edge never goes dull from a hit after invoking the crow's story" 
    )(
        <AND 
            <IS-EQUAL <GET-VAR <CMD 1> IS-SOFT> 1>
            <NOT <IS-EQUAL <GET-VAR <CMD 1> IS-HARD> 1>>
        >
        ;"hitting something soft will not dull the edge of the knife"
    )(
        <IS-EQUAL 1 1>
        <SET-VAR <CMD 2> DAMAGE <SUBTRACT <GET-VAR <CMD 2> DAMAGE> 1>>
        <TELL "Your knife takes 1 point of damage" CR>
        <COND (
            <IS-DES 0 <GET-VAR <CMD 2> DAMAGE>>
            <SET-VAR <CMD 2> DAMAGE 0>
        )>
    )>
>

<HIT () OBSIDIAN-SHARD
    <COND (
        <NOT <IS-EQUAL OBSIDIAN-SHARD <CMD 2>>>
        ;"don't handle that here"
    )>

    ;"hitting anything hard will shatter it"
    <COND (
        <IS-EQUAL <GET-VAR <CMD 1> IS-HARD> 1>
        <TELL "The obsidian shatters" CR>
        <MOVE <CMD 2>>
    )>

    <SET-VAR <CMD 2> DAMAGE <SUBTRACT <GET-VAR <CMD 2> DAMAGE> 28>>
    <TELL "The obsidian's edge chips. It takes 28 points of damage" CR>
    <COND (
        <IS-DES 0 <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR <CMD 2> DAMAGE 0>
    )>
>

<HIT () PICK-AXE
    ;"the pick-axe never goes dull"
>

<HIT () SWORD
    <COND (
        <NOT <IS-EQUAL AXE <CMD 2>>>
        ;"don't handle that here"
    )>

    <COND (
        <IS-EQUAL <GET-VAR PLAYER T-CROW> 2>
        ;"edge never goes dull from a hit after invoking the crow's story" 
    )(
        <OR
            <IS-ASC 0 <GET-VAR <CMD 1> MAX-DAMAGE>>
            <IS-ASC 0 <GET-VAR <CMD 1> IS-HARD>>
        >
        <SET-VAR <CMD 2> DAMAGE <SUBTRACT <GET-VAR <CMD 2> DAMAGE> 5>>
        <TELL "Your sword takes 5 points of damage" CR>
    )(
        <IS-EQUAL <GET-VAR <CMD 1> IS-SOFT> 1>
        ;"hitting something soft will secretly dull the edge by 1"
        <SET-VAR <CMD 2> DAMAGE <SUBTRACT <GET-VAR <CMD 2> DAMAGE> 1>>
    )(
        <IS-EQUAL 1 1>
        <SET-VAR <CMD 2> DAMAGE <SUBTRACT <GET-VAR <CMD 2> DAMAGE> 2>>
        <TELL "Your sword takes 2 points of damage" CR>
    )>

    <COND (
        <IS-DES 0 <GET-VAR <CMD 2> DAMAGE>>
        <SET-VAR <CMD 2> DAMAGE 0>
    )>
>

<ROUTINE DESC-SWORD ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "The intricately-carved handle on this sword warns that only those worthy may wield it." CR>
        <DESC-WEAPON-SHARPNESS> 
    )(
        <IS-EQUAL 1 1>
        <TELL "a SWORD">
    )>
>

<ROUTINE DESC-PICK-AXE ()
    <COND (
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "This deft tool doesn't look like it will lose it's edge any time soon. It's a work horse." CR>
        <COND (
            <IS-IN <CMD 0> STRAP>
            <TELL "It's got a strap tied to it." CR>
        )(
            <IS-DES 50 <RAND>>
            <TELL "It could be used as a grappling hook if you tied some rope to it." CR>
        )(
            <IS-EQUAL 1 1>
            <TELL "It's clearly been used to mine for gold, tirelessly hitting rocks." CR>
        )>
    )(
        <IS-EQUAL 1 1>
        <TELL "a PICK-AXE">
    )>
>

<HIT () 
    ;"didn't answer the question"
    <EACH-OBJ Q-STORAGE (OBJ)
        <MOVE OBJ>
    >
>
