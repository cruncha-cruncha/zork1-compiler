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
        <COND (
            <IS-ASC <GET-VAR PLAYER HEALTH> 1>
            <TELL "You've fatally injured yourself while using the implement." CR>
        )(
            <IS-EQUAL 1 1>
            <TELL "Your implement was dull, causing you to injure yourself while using it." CR> 
            <TELL "You have " <GET-VAR PLAYER HEALTH> " health left." CR>
        )>
        <SET-VAR DMG 0>
    )>

    <RETURN DMG>
>


