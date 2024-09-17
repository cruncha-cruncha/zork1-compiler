<ROOM CABIN
      (DESC "You're inside a log cabin. It's rustic, but has a lovely fireplace." CR)
      (VARS ABOVE-GROUND 1 IS-LOCKED 1)
      (NORTH TO CABIN-EXTERIOR)
      (EAST TO CABIN-EXTERIOR)
      (SOUTH TO CABIN-EXTERIOR)
      (WEST TO CABIN-EXTERIOR)
      (OUT TO CABIN-EXTERIOR)>

<ROOM CABIN-EXTERIOR
      (DESC <DESC-CABIN-EXTERIOR>)
      (VARS ABOVE-GROUND 1 FIRST-TIME 1)
      (NORTH TO FIELD-2)
      (EAST TO FIELD-1)
      (SOUTH TO FOREST-2)
      (WEST TO FOREST-3)>

<OBJECT CABIN-DOOR
      (AKA DOOR CABIN-DOOR)
      (DESC "the cabin DOOR")
      (COPY <ROOM CABIN-EXTERIOR>)
      (VARS NO-TAKE 1 CAN-OPEN 1)
      (ACT-PRSO <PRSO-CABIN-DOOR>)>

<OBJECT CABIN-WINDOW
      (AKA WINDOW)
      (DESC "a cabin WINDOW")
      (VARS NO-TAKE 1)
      (COPY <ROOM CABIN-EXTERIOR>)>

<OBJECT CABIN-DOOR-KEY
      (AKA KEY DOOR-KEY CABIN-KEY)
      (DESC "a KEY")
      (COPY <ROOM CABIN-EXTERIOR>)>

<OBJECT BOOK
    (AKA BOOK)
    (COPY <ROOM CABIN>)
    (VARS HEALTH 2)
    (DESC <DESC-BOOK>)
    (ACT-PRSO <PRSO-BOOK>)>

;"can turn into a note"
<OBJECT BOOK-PAGES
    (AKA PAPER)
    (COPY <BOOK 1>)
    (DESC "some PAPER")>

<OBJECT TABLE
    (AKA TABLE)
    (COPY <ROOM CABIN>)
    (DESC "a wooden TABLE")>

<OBJECT CHAIR
    (AKA CHAIR)
    (COPY <ROOM CABIN>
          <ROOM CABIN>)
    (DESC "a wooden CHAIR")>

<OBJECT BED-FRAME
    (AKA BED BED-FRAME)
    (COPY <ROOM CABIN>)
    (DESC "a wooden BED frame")>

<OBJECT FIRE-PLACE
    (AKA FIREPLACE FIRE-PLACE)
    (COPY <ROOM CABIN>)
    (VARS NO-TAKE 1)
    (DESC "a FIREPLACE")>

<OBJECT BUCKET
    (AKA BUCKET)
    (COPY <ROOM CABIN>)
    (DESC "a BUCKET")>

<OBJECT NAILS
    (AKA NAILS)
    (COPY <ROOM CABIN>)
    (DESC "some NAILS")>

<ROUTINE DESC-CABIN-EXTERIOR ()
      <COND (
            <IS-EQUAL <GET-VAR PRSO FIRST-TIME> 1>
            <SET-VAR PRSO FIRST-TIME 0>
            <TELL "You're at the transition between a forest and a field.
            There are trails in all directions, but at the center a cabin!
            It's got a scenic window facing the fields. You could try to
            OPEN DOOR" CR>
            <RETURN 1>
      )>
      <TELL "You're outside a cabin on the edge of a forest, overlooking some fields." CR>
      <COND (
            <IS-EQUAL <GET-VAR CABIN IS-LOCKED> 1>
            <TELL "The cabin door is locked. Where's the key?" CR>
      )>
>

<ROUTINE PRSO-CABIN-DOOR ()
      <COND (
            <IS-EQUAL CMD "OPEN">
            <COND (
                  <AND <IS-IN PLAYER CABIN-DOOR-KEY> <IS-EQUAL <GET-VAR CABIN IS-LOCKED> 1>>
                  <SET-VAR PRSO IS-LOCKED 0>
                  <TELL "The key works!" CR>
                  <MOVE PLAYER CABIN>
            )(
                  <IS-EQUAL <GET-VAR CABIN IS-LOCKED> 0>
                  <MOVE PLAYER CABIN>
            )(
                  <IS-EQUAL 1 1>
                  <TELL "The door is locked. You should LOOK AROUND for a key." CR> 
            )>
      )>
>

<ROUTINE DESC-BOOK () 
    <COND (
        <IS-EQUAL <GET-VAR PRSO HEALTH> 0>
        <TELL "a torn-up book">
        <RETURN 1>
    )(
        <IS-EQUAL DETAILED-DESC 1>
        <TELL "This leather-bound journal's yellowing pages are covered in
        scratchy handwriting, probably done with charcoal. You can make out a
        few passages:" CR>
        <TELL "...boat is coming along, should be ready in..." CR>
        <TELL "...beast is hungry, what does it eat down there? No use mining any more..." CR>
        <TELL "...lost the gem, a crow must have taken it..." CR>
        <RETURN 1>
    )>
    <TELL "a BOOK">
>

<ROUTINE PRSO-BOOK ()
    <COND (
        <AND <IS-DES <GET-VAR PRSO HEALTH> 0> <IS-EQUAL CMD "UNPACK"> <IS-IN PRSO BOOK-PAGES>>
        <MOVE PRSO BOOK-PAGES PLAYER>
        <TELL "You've ripped out some blank pages from the book" CR>
    )(
        <IS-EQUAL CMD "HIT">
        <COND (
            <IS-DES <GET-VAR PRSO HEALTH> 0>
            <TELL "Why are you hitting this book?" CR>
            <COND (
                <IS-IN PRSO BOOK-PAGES>
                <MOVE PRSO BOOK-PAGES CURRENT-ROOM>
                <TELL "You've cut out some pages onto the floor." CR>
            )>
            <SET-VAR PRSO HEALTH <ADD <GET-VAR PRSO HEALTH> -1>>
        )>
    )>
>

<ROUTINE PRSO-TABLE ()>

