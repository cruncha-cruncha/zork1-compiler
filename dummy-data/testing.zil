"1ACTIONS for
	        Zork I: The Great Underground Empire
	(c) Copyright 1983 Infocom, Inc. All Rights Reserved."

"SUBTITLE THE WHITE HOUSE"

<ROUTINE WEST-HOUSE (RARG)
	 <COND (<EQUAL? .RARG ,M-LOOK>
		<TELL
"You are standing in an open field west of a white house, with a boarded
front door.">
		<COND (,WON-FLAG
		       <TELL
" A secret path leads southwest into the forest.">)>
		<CRLF>)>>

<ROUTINE EAST-HOUSE (RARG)
	 <COND (<EQUAL? .RARG ,M-LOOK>
		<TELL
"You are behind the white house. A path leads into the forest
to the east. In one corner of the house there is a small window
which is ">
		<COND (<FSET? ,KITCHEN-WINDOW ,OPENBIT>
		       <TELL "open.">)
		      (T <TELL "slightly ajar.">)>
		<CRLF>)>>

<ROUTINE OPEN-CLOSE (OBJ STROPN STRCLS)
	 <COND (<VERB? OPEN>
		<COND (<FSET? .OBJ ,OPENBIT>
		       <TELL <PICK-ONE ,DUMMY>>)
		      (T
		       <TELL .STROPN>
		       <FSET .OBJ ,OPENBIT>)>
		<CRLF>)
	       (<VERB? CLOSE>
		<COND (<FSET? .OBJ ,OPENBIT>
		       <TELL .STRCLS>
		       <FCLEAR .OBJ ,OPENBIT>
		       T)
		      (T <TELL <PICK-ONE ,DUMMY>>)>
		<CRLF>)>>

<ROUTINE BOARD-F ()
	 <COND (<VERB? TAKE EXAMINE>
		<TELL "The boards are securely fastened." CR>)>>

<ROUTINE TEETH-F ()
	 <COND (<AND <VERB? BRUSH>
		     <EQUAL? ,PRSO ,TEETH>>
		<COND (<AND <EQUAL? ,PRSI ,PUTTY>
			    <IN? ,PRSI ,WINNER>>
		       <JIGS-UP
"Well, you seem to have been brushing your teeth with some sort of
glue. As a result, your mouth gets glued together (with your nose)
and you die of respiratory failure.">)
		      (<NOT ,PRSI>
		       <TELL
"Dental hygiene is highly recommended, but I'm not sure what you want
to brush them with." CR>)
		      (T
		       <TELL "A nice idea, but with a " D ,PRSI "?" CR>)>)>>

<ROUTINE GRANITE-WALL-F ()
	 <COND (<EQUAL? ,HERE ,NORTH-TEMPLE>
		<COND (<VERB? FIND>
		       <TELL "The west wall is solid granite here." CR>)
		      (<VERB? TAKE RAISE LOWER>
		       <TELL "It's solid granite." CR>)>)
	       (<EQUAL? ,HERE ,TREASURE-ROOM>
		<COND (<VERB? FIND>
		       <TELL "The east wall is solid granite here." CR>)
		      (<VERB? TAKE RAISE LOWER>
		       <TELL "It's solid granite." CR>)>)
	       (<EQUAL? ,HERE ,SLIDE-ROOM>
		<COND (<VERB? FIND READ>
		       <TELL "It only SAYS \"Granite Wall\"." CR>)
		      (T <TELL "The wall isn't granite." CR>)>)
	       (T
		<TELL "There is no granite wall here." CR>)>>

<ROUTINE SONGBIRD-F ()
	 <COND (<VERB? FIND TAKE>
		<TELL "The songbird is not here but is probably nearby." CR>)
	       (<VERB? LISTEN>
		<TELL "You can't hear the songbird now." CR>)
	       (<VERB? FOLLOW>
		<TELL "It can't be followed." CR>)
	       (T
		<TELL "You can't see any songbird here." CR>)>>

<ROUTINE WHITE-HOUSE-F ()
    <COND (<EQUAL? ,HERE ,KITCHEN ,LIVING-ROOM ,ATTIC>
	   <COND (<VERB? FIND>
		  <TELL "Why not find your brains?" CR>)
		 (<VERB? WALK-AROUND>
		  <GO-NEXT ,IN-HOUSE-AROUND>
		  T)>)
	  (<NOT <OR <EQUAL? ,HERE ,EAST-OF-HOUSE ,WEST-OF-HOUSE>
		    <EQUAL? ,HERE ,NORTH-OF-HOUSE ,SOUTH-OF-HOUSE>>>
	   <COND (<VERB? FIND>
		  <COND (<EQUAL? ,HERE ,CLEARING>
			 <TELL "It seems to be to the west." CR>)
			(T
			 <TELL "It was here just a minute ago...." CR>)>)
		 (T <TELL "You're not at the house." CR>)>)
	  (<VERB? FIND>
	   <TELL
"It's right here! Are you blind or something?" CR>)
	  (<VERB? WALK-AROUND>
	   <GO-NEXT ,HOUSE-AROUND>
	   T)
	  (<VERB? EXAMINE>
	   <TELL
"The house is a beautiful colonial house which is painted white.
It is clear that the owners must have been extremely wealthy." CR>)
	  (<VERB? THROUGH OPEN>
	   <COND (<EQUAL? ,HERE ,EAST-OF-HOUSE>
		  <COND (<FSET? ,KITCHEN-WINDOW ,OPENBIT>
			 <GOTO ,KITCHEN>)
			(T
			 <TELL "The window is closed." CR>
			 <THIS-IS-IT ,KITCHEN-WINDOW>)>)
		 (T
		  <TELL "I can't see how to get in from here." CR>)>)
	  (<VERB? BURN>
	   <TELL "You must be joking." CR>)>>

;"0 -> no next, 1 -> success, 2 -> failed move"

<ROUTINE GO-NEXT (TBL "AUX" VAL)
	 <COND (<SET VAL <LKP ,HERE .TBL>>
		<COND (<NOT <GOTO .VAL>> 2)
		      (T 1)>)>>

<ROUTINE FOREST-F ()
	 <COND (<VERB? WALK-AROUND>
		<COND (<OR <EQUAL? ,HERE
			       ,WEST-OF-HOUSE ,NORTH-OF-HOUSE
			       ,SOUTH-OF-HOUSE>
			   <EQUAL? ,HERE ,EAST-OF-HOUSE>>
		       <TELL "You aren't even in the forest." CR>)>
		<GO-NEXT ,FOREST-AROUND>)
	       (<VERB? DISEMBARK>
		<TELL "You will have to specify a direction." CR>)
	       (<VERB? FIND>
		<TELL "You cannot see the forest for the trees." CR>)
	       (<VERB? LISTEN>
		<TELL "The pines and the hemlocks seem to be murmuring."
		      CR>)>>

<ROUTINE MOUNTAIN-RANGE-F ()
	 <COND (<VERB? CLIMB-UP CLIMB-DOWN CLIMB-FOO>
		<TELL "Don't you believe me? The mountains are impassable!"
		      CR>)>>

<ROUTINE WATER-F ("AUX" AV W PI?)
	 <COND (<VERB? SGIVE> <RFALSE>)
	       (<VERB? THROUGH BOARD>
		<TELL <PICK-ONE ,SWIMYUKS> CR>
		<RTRUE>)
	       (<VERB? FILL>	;"fill bottle with water =>"
		<SET W ,PRSI>	   ;"put water in bottle"
		<SETG PRSA ,V?PUT>
		<SETG PRSI ,PRSO>
		<SETG PRSO .W>
		<SET PI? <>>)
	       (<OR <EQUAL? ,PRSO ,GLOBAL-WATER>
		    <EQUAL? ,PRSO ,WATER>>
		<SET W ,PRSO>
		<SET PI? <>>)
	       (ELSE
		<SET W ,PRSI>
		<COND (.W <SET PI? T>)>)>
	 <COND (<EQUAL? .W ,GLOBAL-WATER>
		<SET W ,WATER>
		<COND (<VERB? TAKE PUT> <REMOVE-CAREFULLY .W>)>)>
	 <COND (.PI? <SETG PRSI .W>)
	       (T <SETG PRSO .W>)>
	 <SET AV <LOC ,WINNER>>
	 <COND (<NOT <FSET? .AV ,VEHBIT>> <SET AV <>>)>
	 <COND (<AND <VERB? TAKE PUT> <NOT .PI?>>
		<COND (<AND .AV
			    <OR <EQUAL? .AV ,PRSI>
				<AND <NOT ,PRSI>
				     <NOT <IN? .W .AV>>>>>
		       <TELL "There is now a puddle in the bottom of the "
			     D .AV "." CR>
		       <REMOVE-CAREFULLY ,PRSO>
		       <MOVE ,PRSO .AV>)
		      (<AND ,PRSI <NOT <EQUAL? ,PRSI ,BOTTLE>>>
		       <TELL "The water leaks out of the " D ,PRSI
			     " and evaporates immediately." CR>
		       <REMOVE-CAREFULLY .W>)
		      (<IN? ,BOTTLE ,WINNER>
		       <COND (<NOT <FSET? ,BOTTLE ,OPENBIT>>
			      <TELL "The bottle is closed." CR>
			      <THIS-IS-IT ,BOTTLE>)
			     (<NOT <FIRST? ,BOTTLE>>
			      <MOVE ,WATER ,BOTTLE>
			      <TELL "The bottle is now full of water." CR>)
			     (T
			      <TELL "The water slips through your fingers." CR>
			      <RTRUE>)>)
		      (<AND <IN? ,PRSO ,BOTTLE>
			    <VERB? TAKE>
			    <NOT ,PRSI>>
		       <TELL
"It's in the bottle. Perhaps you should take that instead." CR>)
		      (T
		       <TELL "The water slips through your fingers." CR>)>)
	       (.PI?
		<COND (<AND <VERB? PUT>
			    <GLOBAL-IN? ,RIVER ,HERE>>
		       <PERFORM ,V?PUT ,PRSO ,RIVER>)
		      (ELSE
		       <TELL "Nice try." CR>)>
		<RTRUE>)
	       (<VERB? DROP GIVE>
		<COND (<AND <VERB? DROP>
			    <IN? ,WATER ,BOTTLE>
			    <NOT <FSET? ,BOTTLE ,OPENBIT>>>
		       <TELL "The bottle is closed." CR>
		       <RTRUE>)>
		<REMOVE-CAREFULLY ,WATER>
		<COND (.AV
		       <TELL "There is now a puddle in the bottom of the "
			     D .AV "." CR>
		       <MOVE ,WATER .AV>)
		      (T
		       <TELL
"The water spills to the floor and evaporates immediately." CR>
		       <REMOVE-CAREFULLY ,WATER>)>)
	       (<VERB? THROW>
		<TELL
"The water splashes on the walls and evaporates immediately." CR>
		<REMOVE-CAREFULLY ,WATER>)>>

<GLOBAL KITCHEN-WINDOW-FLAG <>>

<ROUTINE KITCHEN-WINDOW-F ()
	 <COND (<VERB? OPEN CLOSE>
		<SETG KITCHEN-WINDOW-FLAG T>
		<OPEN-CLOSE ,KITCHEN-WINDOW
"With great effort, you open the window far enough to allow entry."
"The window closes (more easily than it opened).">)
	       (<AND <VERB? EXAMINE>
		     <NOT ,KITCHEN-WINDOW-FLAG>>
		<TELL
"The window is slightly ajar, but not enough to allow entry." CR>)
	       (<VERB? WALK BOARD THROUGH>
		<COND (<EQUAL? ,HERE ,KITCHEN>
		       <DO-WALK ,P?EAST>)
		      (T
		       <DO-WALK ,P?WEST>)>
		<RTRUE>)
	       (<VERB? LOOK-INSIDE>
		<TELL "You can see ">
		<COND (<EQUAL? ,HERE ,KITCHEN>
		       <TELL "a clear area leading towards a forest." CR>)
		      (T
		       <TELL "what appears to be a kitchen." CR>)>)>>

<ROUTINE GHOSTS-F ()
	 <COND (<VERB? TELL>
		<TELL "The spirits jeer loudly and ignore you." CR>
		<SETG P-CONT <>>)
	       (<VERB? EXORCISE>
		<TELL "Only the ceremony itself has any effect." CR>)
	       (<AND <VERB? ATTACK MUNG> <EQUAL? ,PRSO ,GHOSTS>>
		<TELL "How can you attack a spirit with material objects?" CR>)
	       (T
		<TELL "You seem unable to interact with these spirits." CR>)>>

<GLOBAL CAGE-TOP T>

<ROUTINE BASKET-F ()
	 <COND (<VERB? RAISE>
		<COND (,CAGE-TOP
		       <TELL <PICK-ONE ,DUMMY> CR>)
		      (T
		       <MOVE ,RAISED-BASKET ,SHAFT-ROOM>
		       <MOVE ,LOWERED-BASKET ,LOWER-SHAFT>
		       <SETG CAGE-TOP T>
		       <THIS-IS-IT ,RAISED-BASKET>
		       <TELL
"The basket is raised to the top of the shaft." CR>)>)
	       (<VERB? LOWER>
		<COND (<NOT ,CAGE-TOP>
		       <TELL <PICK-ONE ,DUMMY> CR>)
		      (T
		       <MOVE ,RAISED-BASKET ,LOWER-SHAFT>
		       <MOVE ,LOWERED-BASKET ,SHAFT-ROOM>
		       <THIS-IS-IT ,LOWERED-BASKET>
		       <TELL
"The basket is lowered to the bottom of the shaft." CR>
		       <SETG CAGE-TOP <>>
		       <COND (<AND ,LIT <NOT <SETG LIT <LIT? ,HERE>>>>
			      <TELL "It is now pitch black." CR>)>
		       T)>)
	       (<OR <EQUAL? ,PRSO ,LOWERED-BASKET>
		    <EQUAL? ,PRSI ,LOWERED-BASKET>>
		<TELL "The basket is at the other end of the chain." CR>)
	       (<AND <VERB? TAKE>
		     <EQUAL? ,PRSO ,RAISED-BASKET ,LOWERED-BASKET>>
		<TELL "The cage is securely fastened to the iron chain." CR>)>>

<ROUTINE BAT-F ()
	 <COND (<VERB? TELL>
		<FWEEP 6>
		<SETG P-CONT <>>)
	       (<VERB? TAKE ATTACK MUNG>
		<COND (<EQUAL? <LOC ,GARLIC> ,WINNER ,HERE>
		       <TELL "You can't reach him; he's on the ceiling." CR>)
		      (T <FLY-ME>)>)>>

<ROUTINE FLY-ME ()
	 <FWEEP 4>
	 <TELL
"The bat grabs you by the scruff of your neck and lifts you away...." CR CR>
	 <GOTO <PICK-ONE ,BAT-DROPS> <>>
	 <COND (<NOT <EQUAL? ,HERE ,ENTRANCE-TO-HADES>>
		<V-FIRST-LOOK>)>
	 T>

<ROUTINE FWEEP (N)
	 <REPEAT ()
		 <COND (<L? <SET N <- .N 1>> 1> <RETURN>)
		       (T <TELL "    Fweep!" CR>)>>
	 <CRLF>>

<GLOBAL BAT-DROPS
      <LTABLE 0
	      MINE-1
	      MINE-2
	      MINE-3
	      MINE-4
	      LADDER-TOP
	      LADDER-BOTTOM
	      SQUEEKY-ROOM
	      MINE-ENTRANCE>>

<ROUTINE BELL-F ()
	 <COND (<VERB? RING>
		<COND (<AND <EQUAL? ,HERE ,LLD-ROOM>
			    <NOT ,LLD-FLAG>>
		       <RFALSE>)
		      (T
		       <TELL "Ding, dong." CR>)>)>>

<ROUTINE HOT-BELL-F ()
	 <COND (<VERB? TAKE>
		<TELL "The bell is very hot and cannot be taken." CR>)
	       (<OR <VERB? RUB> <AND <VERB? RING> ,PRSI>>
		<COND (<FSET? ,PRSI ,BURNBIT>
		       <TELL "The " D ,PRSI " burns and is consumed." CR>
		       <REMOVE-CAREFULLY ,PRSI>)
		      (<EQUAL? ,PRSI ,HANDS>
		       <TELL "The bell is too hot to touch." CR>)
		      (T
		       <TELL "The heat from the bell is too intense." CR>)>)
	       (<VERB? POUR-ON>
		<REMOVE-CAREFULLY ,PRSO>
		<TELL "The water cools the bell and is evaporated." CR>
		<QUEUE I-XBH 0>
		<I-XBH>)
	       (<VERB? RING>
		<TELL "The bell is too hot to reach." CR>)>>

<ROUTINE BOARDED-WINDOW-FCN ()
	 <COND (<VERB? OPEN>
		<TELL "The windows are boarded and can't be opened." CR>)
	       (<VERB? MUNG>
		<TELL "You can't break the windows open." CR>)>>

<ROUTINE NAILS-PSEUDO ()
	 <COND (<VERB? TAKE>
		<TELL
"The nails, deeply imbedded in the door, cannot be removed." CR>)>>

<ROUTINE CRACK-FCN ()
	 <COND (<VERB? THROUGH>
		<TELL "You can't fit through the crack." CR>)>>

<ROUTINE KITCHEN-FCN (RARG)
	<COND (<EQUAL? .RARG ,M-LOOK>
	       <TELL
"You are in the kitchen of the white house. A table seems to
have been used recently for the preparation of food. A passage
leads to the west and a dark staircase can be seen leading
upward. A dark chimney leads down and to the east is a small
window which is ">
	       <COND (<FSET? ,KITCHEN-WINDOW ,OPENBIT>
		      <TELL "open." CR>)
		     (T
		      <TELL "slightly ajar." CR>)>)
	      (<==? .RARG ,M-BEG>
	       <COND (<AND <VERB? CLIMB-UP> <EQUAL? ,PRSO ,STAIRS>>
		      <DO-WALK ,P?UP>)
		     (<AND <VERB? CLIMB-UP> <EQUAL? ,PRSO ,STAIRS>>
		      <TELL "There are no stairs leading down." CR>)>)>>

<ROUTINE STONE-BARROW-FCN (RARG)
	 <COND (<AND <EQUAL? .RARG ,M-BEG>
		     <OR <VERB? ENTER>
			 <AND <VERB? WALK>
			      <EQUAL? ,PRSO ,P?WEST ,P?IN>>
			 <AND <VERB? THROUGH>
			      <EQUAL? ,PRSO ,BARROW>>>>
		<TELL
"Inside the Barrow|
As you enter the barrow, the door closes inexorably behind you. Around
you it is dark, but ahead is an enormous cavern, brightly lit. Through
its center runs a wide stream. Spanning the stream is a small wooden
footbridge, and beyond a path leads into a dark tunnel. Above the
bridge, floating in the air, is a large sign. It reads:  All ye who
stand before this bridge have completed a great and perilous adventure
which has tested your wit and courage. You have mastered">
		<COND (<EQUAL? <BAND <GETB 0 1> 8> 0>
		       <TELL "
the first part of the ZORK trilogy. Those who pass over this bridge must be
prepared to undertake an even greater adventure that will severely test your
skill and bravery!|
|
The ZORK trilogy continues with \"ZORK II: The Wizard of Frobozz\" and
is completed in \"ZORK III: The Dungeon Master.\"" CR>)
		      (T
		       <TELL "
ZORK: The Great Underground Empire.|" CR>)>
		<FINISH>)>>

<ROUTINE BARROW-DOOR-FCN ()
	 <COND (<VERB? OPEN CLOSE>
		<TELL "The door is too heavy." CR>)>>

<ROUTINE BARROW-FCN ()
	 <COND (<VERB? THROUGH>
		<DO-WALK ,P?WEST>)>>

\

<ROUTINE TROPHY-CASE-FCN ()
    <COND (<AND <VERB? TAKE> <EQUAL? ,PRSO ,TROPHY-CASE>>
	   <TELL
"The trophy case is securely fastened to the wall." CR>)>>
	
<GLOBAL RUG-MOVED <>>

<ROUTINE LIVING-ROOM-FCN (RARG "AUX" RUG? TC)
	<COND (<EQUAL? .RARG ,M-LOOK>
	       <TELL
"You are in the living room. There is a doorway to the east">
	       <COND (,MAGIC-FLAG
		      <TELL
". To the
west is a cyclops-shaped opening in an old wooden door, above which is
some strange gothic lettering, ">)
		     (T
		      <TELL
", a wooden
door with strange gothic lettering to the west, which appears to be
nailed shut, ">)>
	       <TELL "a trophy case, ">
	       <SET RUG? ,RUG-MOVED>
	       <COND (<AND .RUG? <FSET? ,TRAP-DOOR ,OPENBIT>>
		      <TELL
		       "and a rug lying beside an open trap door.">)
		     (.RUG?
		      <TELL "and a closed trap door at your feet.">)
		     (<FSET? ,TRAP-DOOR ,OPENBIT>
		      <TELL "and an open trap door at your feet.">)
		     (T
		      <TELL
		       "and a large oriental rug in the center of the room.">)>
	       <CRLF>
	       T)
	      (<EQUAL? .RARG ,M-END>
	       <COND (<OR <VERB? TAKE>
			  <AND <VERB? PUT>
			       <EQUAL? ,PRSI ,TROPHY-CASE>>>
		      <COND (<IN? ,PRSO ,TROPHY-CASE>
			     <TOUCH-ALL ,PRSO>)>
		      <SETG SCORE <+ ,BASE-SCORE <OTVAL-FROB>>>
		      <SCORE-UPD 0>
		      <RFALSE>)>)>>

<ROUTINE TOUCH-ALL (OBJ "AUX" F)
	 <SET F <FIRST? .OBJ>>
	 <REPEAT ()
		 <COND (<NOT .F> <RETURN>)
		       (T
			<FSET .F ,TOUCHBIT>
			<COND (<FIRST? .F> <TOUCH-ALL .F>)>)>
		 <SET F <NEXT? .F>>>>

<ROUTINE OTVAL-FROB ("OPTIONAL" (O ,TROPHY-CASE) "AUX" F (SCORE 0))
	 <SET F <FIRST? .O>>
	 <REPEAT ()
		 <COND (<NOT .F> <RETURN .SCORE>)>
		 <SET SCORE <+ .SCORE <GETP .F ,P?TVALUE>>>
		 <COND (<FIRST? .F> <OTVAL-FROB .F>)>
		 <SET F <NEXT? .F>>>>

<ROUTINE TRAP-DOOR-FCN ()
    <COND (<VERB? RAISE>
	   <PERFORM ,V?OPEN ,TRAP-DOOR>
	   <RTRUE>)
	  (<AND <VERB? OPEN CLOSE>
		<EQUAL? ,HERE ,LIVING-ROOM>>
	   <OPEN-CLOSE ,PRSO
"The door reluctantly opens to reveal a rickety staircase descending into
darkness."
"The door swings shut and closes.">)
	  (<AND <VERB? LOOK-UNDER> <EQUAL? ,HERE LIVING-ROOM>>
	   <COND (<FSET? ,TRAP-DOOR ,OPENBIT>
		  <TELL
"You see a rickety staircase descending into darkness." CR>)
		 (T <TELL "It's closed." CR>)>)
	  (<EQUAL? ,HERE ,CELLAR>
	   <COND (<AND <VERB? OPEN UNLOCK>
		       <NOT <FSET? ,TRAP-DOOR ,OPENBIT>>>
		  <TELL
"The door is locked from above." CR>)
		 (<AND <VERB? CLOSE> <NOT <FSET? ,TRAP-DOOR ,OPENBIT>>>
		  <FCLEAR ,TRAP-DOOR ,TOUCHBIT>
		  <FCLEAR ,TRAP-DOOR ,OPENBIT>
		  <TELL "The door closes and locks." CR>)
		 (<VERB? OPEN CLOSE>
		  <TELL <PICK-ONE ,DUMMY> CR>)>)>>

<ROUTINE CELLAR-FCN (RARG)
  <COND (<EQUAL? .RARG ,M-LOOK>
	 <TELL
"You are in a dark and damp cellar with a narrow passageway leading
north, and a crawlway to the south. On the west is the bottom of a
steep metal ramp which is unclimbable." CR>)
	(<EQUAL? .RARG ,M-ENTER>
	 <COND (<AND <FSET? ,TRAP-DOOR ,OPENBIT>
		     <NOT <FSET? ,TRAP-DOOR ,TOUCHBIT>>>
		<FCLEAR ,TRAP-DOOR ,OPENBIT>
		<FSET ,TRAP-DOOR ,TOUCHBIT>
		<TELL
"The trap door crashes shut, and you hear someone barring it." CR CR>)>)>>

<ROUTINE CHIMNEY-F ()
	 <COND (<VERB? EXAMINE>
		<TELL "The chimney leads ">
		<COND (<==? ,HERE ,KITCHEN>
		       <TELL "down">)
		      (T <TELL "up">)>
		<TELL "ward, and looks climbable." CR>)>>

<ROUTINE UP-CHIMNEY-FUNCTION ("AUX" F)
  <COND (<NOT <SET F <FIRST? ,WINNER>>>
	 <TELL "Going up empty-handed is a bad idea." CR>
	 <RFALSE>)
	(<AND <OR <NOT <SET F <NEXT? .F>>>
		  <NOT <NEXT? .F>>>
	      <IN? ,LAMP ,WINNER>>
	 <COND (<NOT <FSET? ,TRAP-DOOR ,OPENBIT>>
		<FCLEAR ,TRAP-DOOR ,TOUCHBIT>)>
	 <RETURN ,KITCHEN>)
	(T
	 <TELL "You can't get up there with what you're carrying." CR>
	 <RFALSE>)>>

<ROUTINE TRAP-DOOR-EXIT ()
	 <COND (,RUG-MOVED
		<COND (<FSET? ,TRAP-DOOR ,OPENBIT>
		       <RETURN ,CELLAR>)
		      (T
		       <TELL "The trap door is closed." CR>
		       <THIS-IS-IT ,TRAP-DOOR>
		       <RFALSE>)>)
	       (T
		<TELL "You can't go that way." CR>
		<RFALSE>)>>