<DIRECTIONS NORTH EAST SOUTH WEST>

<PLAYER 
      (ROOM SOUTH-OF-HOUSE)>

<ROOM SOUTH-OF-HOUSE
      (DESC "You're in a field, south of a house")
      (NORTH TO HOUSE)>

<ROOM HOUSE
      (DESC "You're in a house")
      (SOUTH TO SOUTH-OF-HOUSE)>

<ROUTINE V-DESC-ROOM () 
      <TELL "Tell something">>

<SYNTAX WHERE AM I = V-DESC-ROOM>