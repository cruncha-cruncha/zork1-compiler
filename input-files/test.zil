<DIRECTIONS UP EAST WEST>

<GLOBAL G-VAR 1>

<PLAYER 
      (ROOM FOREST-1)
      (VARS DISTANCE 0)>

<ROOM FOREST-1
      (DESC "You're in a forest. The smell of pine hangs thick in the air.")
      (EAST TO FOREST-2)
      (UP "There is no side-to-side")>

<ROOM FOREST-2
      (DESC "You're in a forest. A dark sky shows overhead between gaps in the branches, far above you.")
      (WEST TO FOREST-1)
      (UP "There is no up there is no down")>

<OBJECT JOURNAL
      (AKA JOURNAL)
      (IN PLAYER)
      (DESC JOURNAL-DESC-FCN)
      (VARS PAGES 0)>

<OBJECT TREE
      (AKA TREE)
      (IN FOREST-2)
      (DESC "A TREE")
      (VARS HEIGHT 20)>

<OBJECT MONSTER
      (AKA MONSTER)
      (IN FOREST-2)
      (DESC "A hideous five-eyed MONSTER")
      (VARS CAN-BE-ATTACKED 1 CAN-ATTACK 1 HEALTH 5)>

<ROUTINE JOURNAL-DESC-FCN ()
      <TELL "A journal">>

<ROUTINE V-DESC-ROOM ()
      <DESC CURRENT-ROOM>
      <TELL CR>>

<ROUTINE V-LIST-MY-VARS ()
      <TELL "Player variables:" CR>
      <EACH-VAR PLAYER (NAME VAL)
            <TELL NAME ": " VAL CR>
      >>

<ROUTINE V-LIST-MY-OBJECTS ()
      <TELL "Objects in player:" CR>
      <EACH-OBJ PLAYER (OBJ)
            <DESC OBJ>
            <TELL CR>
      >>

<ROUTINE V-LIST-OBJECTS () 
      <EACH-OBJ CURRENT-ROOM (OBJ)
            <DESC OBJ>
            <TELL CR>
      >
>

<ROUTINE V-LIST-OBJECT-VARS (COUNT)
      <EACH-VAR CMD-PRSO (NAME VAL)
            <TELL NAME ": " VAL CR>
            <SET-VAR COUNT <ADD COUNT 1>>
      >
      <COND
            (<IS-ASC COUNT 1> <TELL "No vars" CR>)
      >
>

<SYNTAX DESC ROOM = V-DESC-ROOM>
<SYNTAX LIST VARS IN OBJECT = V-LIST-OBJECT-VARS>
<SYNTAX LIST MY VARS = V-LIST-MY-VARS>
<SYNTAX LIST MY OBJECTS = V-LIST-MY-OBJECTS>
<SYNTAX LIST ROOM OBJECTS = V-LIST-OBJECTS>
;<SYNTAX ATTACK OBJECT (CAN-BE-ATTACKED) = V-ATTACK>

