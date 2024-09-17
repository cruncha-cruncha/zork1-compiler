<ROOM FOREST-1
      (DESC <DESC-FOREST-1>)
      (VARS FIRST-TIME 1 ABOVE-GROUND 1)
      (NORTH TO FOREST-2)
      (EAST TO FOREST-2)
      (WEST TO CAVE-ENTRANCE-1)>

<ROOM FOREST-2
      (DESC "You're in a forest, the trees are thinner here. There's a trail heading NORTH (and back SOUTH)." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO CABIN-EXTERIOR)
      (SOUTH TO FOREST-1)>

<ROOM FOREST-3
      (DESC "You're in a lightly-populated forest, and a small stream burbles nearby." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO FOREST-4)
      (EAST TO CABIN-EXTERIOR)
      (SOUTH TO FOREST-2)
      (WEST TO FOREST-5)>

<ROOM FOREST-4
      (DESC "The forest is thicker again here, but the trail still looks good, and the you can hear a stream." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO CAVE-ENTRANCE-2)
      (EAST TO FIELD-2)
      (SOUTH TO FOREST-3)
      (WEST TO FOREST-5)>

<ROOM FOREST-5
      (DESC "You're in the forest. The trail is faint here, and barely-worn." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO LAKE-1)
      (EAST TO CAVE-ENTRANCE-1)
      (SOUTH TO CAVE-ENTRANCE-1)
      (WEST "Think again")>

<ROOM FOREST-6
      (DESC "This is deep forest, but there is trail running SOUTH and EAST" CR)
      (VARS ABOVE-GROUND 1)
      (NORTH "The forest gets thicker, closing in authoritatively. No chance of getting through.")
      (EAST TO CAVE-ENTRANCE-2)
      (SOUTH TO LAKE-1)
      (WEST "Not a chance.")>

<ROOM LAKE-1
      (DESC "You find yourself at the edge a large, calm lake" CR)
      (VARS ABOVE-GROUND 1)
      (NORTH TO FOREST-6)
      (EAST TO FOREST-4)
      (SOUTH TO FOREST-5)
      (WEST TO FOREST-5)>

<ROOM FIELD-1
      (DESC "A massive field stretches out all around you, with wild grass and sage.")
      (VARS ABOVE-GROUND 1)
      (NORTH TO FIELD-2)
      (EAST TO CLIFF-1)
      (SOUTH TO FOREST-1)
      (WEST TO CABIN-EXTERIOR)>

<ROOM FIELD-2
      (DESC "This field seems to go on forever, gently rolling past the horizon." CR)
      (VARS ABOVE-GROUND 1)
      (NORTH "Nothing but endless field lies north. Best kept for another time.")
      (EAST TO CLIFF-1)
      (SOUTH TO CABIN-EXTERIOR)
      (WEST TO FOREST-4)>

<ROOM CLIFF-1
      (DESC "Grassy fields abruptly stop at the edge of a cliff, a stream hurtling over. You can taste more adventure awaiting in the land below.")
      (VARS ABOVE-GROUND 1)
      (NORTH TO FIELD-2)
      (EAST "You're not yet prepared for climbing down this sheer drop.")
      (SOUTH TO FIELD-1)
      (WEST TO FIELD-1)>

<ROOM CAVE-ENTRANCE-1
      (DESC "Amidst boulders and tree trunks a black space beckons you forward. A cave. You could GO DOWN, but there is no telling whether you would come back up.")
      (VARS ABOVE-GROUND 1)
      (NORTH TO FOREST-5)
      (EAST TO FOREST-1)
      (SOUTH TO FOREST-1)
      (WEST TO FOREST-5)>

<ROOM CAVE-ENTRANCE-2
      (DESC "Out of the forest appears a large, rocky hole in the ground. A cave. You could GO DOWN.")
      (NORTH "There's nothing but dense forest")
      (EAST TO FOREST-4)
      (SOUTH TO FOREST-4)
      (WEST TO FOREST-6)>

<OBJECT BOAT-FRAME>

<ROUTINE DESC-FOREST-1 ()
      <COND (
            <IS-EQUAL <GET-VAR FOREST-1 FIRST-TIME> 1>
            <SET-VAR FOREST-1 FIRST-TIME 0>
            <TELL "It's the end of summer, and you're in a dense forest. Birds
            and other small creatures can be heard rustling, buzzing, and chirping
            through the undergrowth. The smell of pine hangs thick in the air. Light
            filters down through needles and leaves." CR>
            <WEATHER-REPORT>
            <TELL "There's a trail up ahead, it looks like you could GO NORTH" CR>
      )(
            <IS-EQUAL DETAILED-DESC 1>
            <TELL "You're in dense forest. The undergrowth is thick with dead
            branches, leaves, and other detritus. Vines, bushes, and brush all
            vie for a spot in the sunlight. Even during a thunderstorm, the wind
            barely penetrates this far down, leaving the air hot and thick with
            the smell of pine." CR>
            <TELL "Despite the foliage, there's a well-defined trail heading
            NORTH and WEST" CR>
      )(
            <IS-EQUAL 1 1>
            <TELL "You're in a dense forest. There's a trail heading NORTH and WEST." CR>
      )>
>