// ---------------- ACTIONS ----------------

/*
<ROUTINE WEST-HOUSE (RARG)
	 <COND (<EQUAL? .RARG ,M-LOOK>
		<TELL
"You are standing in an open field west of a white house, with a boarded
front door.">
		<COND (,WON-FLAG
		       <TELL
" A secret path leads southwest into the forest.">)>
		<CRLF>)>>
*/

const westHouse = newLocalScope((rarg) => {
	set("rarg", rarg);
  if (equal_q(local(rarg), global(mLook))) {
    tell("You are standing in an open field west of a white house, with a boarded\nfront door.");
    if (global(wonFlag)) {
      tell(" A secret path leads southwest into the forest.");
    }
    crlf();
  }
});
addRoutine(westHouse);

/*
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
*/

const batDrops = [
  mine1,
  mine2,
  mine3,
  mine4,
  ladderTop,
  ladderBottom,
  squeekyRoom,
  mineEntrance
];

/*
<ROUTINE FLY-ME ()
	 <FWEEP 4>
	 <TELL
"The bat grabs you by the scruff of your neck and lifts you away...." CR CR>
	 <GOTO <PICK-ONE ,BAT-DROPS> <>>
	 <COND (<NOT <EQUAL? ,HERE ,ENTRANCE-TO-HADES>>
		<V-FIRST-LOOK>)>
	 T>
*/

// don't need to create new local scope if no params
const flyMe = () => {
  fweep(4);
  tell("The bat grabs you by the scruff of your neck and lifts you away...." + "\n" + "\n");
  gotoRoom(pickOne(global(batDrops)), false);
  if (not(equal_q(global(here), global(entranceToHades)))) {
    vFirstLook();
  }
}
addRoutine(flyMe);

/*
<GLOBAL MIRROR-MUNG <>>
*/

const mirrorMung = false;

/*
<GLOBAL LUCKY T>
*/

const lucky = true;

/*
<ROUTINE I-XB ()
	<OR ,XC
	  <AND 
			<EQUAL? ,HERE ,ENTRANCE-TO-HADES>
		  <TELL
"The tension of this ceremony is broken, and the wraiths, amused but
shaken at your clumsy attempt, resume their hideous jeering." CR>
		>
	>
	<SETG XB <>>
>
*/

const iXb = () => {
	or(global("xc"), and(
		equal_q(global("here"), global("entranceToHades")),
		tell("The tension of this ceremony is broken, and the wraiths, amused but\nshaken at your clumsy attempt, resume their hideous jeering." + "\n")
	))
	setg("xb", false)
}
addRoutine(iXb);

/*
<ROUTINE I-XC ()
	 <SETG XC <>>
	 <I-XB>>
*/

// does <> mean false?? yes!
const iXc = () => {
	setg("xc", false)
	iXb()
}
addRoutine(iXc);

/*
<ROUTINE I-XBH ()
	 <REMOVE-CAREFULLY ,HOT-BELL>
	 <MOVE ,BELL ,ENTRANCE-TO-HADES>
	 <COND (<EQUAL? ,HERE ,ENTRANCE-TO-HADES>
		<TELL "The bell appears to have cooled down." CR>)>>
*/

const iXbh = () => {
	removeCarefully(global("hotBell"))
	move(global("bell"), global("entranceToHades"))
	if (equal_q(global("here"), global("entranceToHades"))) {
		tell("The bell appears to have cooled down." + "\n")
	}
}
addRoutine(iXbh);

/*
<ROUTINE MOVE-ALL (FROM TO "AUX" X N)
	 <COND (<SET X <FIRST? .FROM>>
		<REPEAT ()
			<COND (<NOT .X> <RETURN>)>
			<SET N <NEXT? .X>>
			<FCLEAR .X ,INVISIBLE>
			<MOVE .X .TO>
			<SET X .N>>)>>
*/
 
// from is an OBJECT, to is a ROOM
const moveAll = newLocalScope((from, to) => {
	set("from", from);
	set("to", to);
	set("x", null);
	set("n", null);
  // get the first OBJECT IN the from OBJECT
  if (set("x", first_q(local("from")))) {
    while (true) {
			// pass to a NOT func
      if (not(local("x"))) {
				// is this right? Or should we just break?
        return;
      }
      // get the next OBJECT IN the from OBJECT
      set("n", next_q(local("x")));
      // fclear has something to do with FLAGS on an OBJECT
      fclear(local("x"), global("invisible"));
      // 
      move(local("x"), local("to"));
      set("x", local("n"));
    }
  }
});
addRoutine(moveAll);

/*
<ROUTINE KILL-INTERRUPTS ()
	 <DISABLE <INT I-XB>>
	 <DISABLE <INT I-XC>>
	 <DISABLE <INT I-CYCLOPS>>
	 <DISABLE <INT I-LANTERN>>
	 <DISABLE <INT I-CANDLES>>
	 <DISABLE <INT I-SWORD>>
	 <DISABLE <INT I-FOREST-ROOM>>
	 <DISABLE <INT I-MATCH>>
	 <FCLEAR ,MATCH ,ONBIT>
	 <RTRUE>>
*/

// https://archive.org/details/Learning_ZIL_Steven_Eric_Meretzky_1995/page/n17/mode/2up
// https://apps.dtic.mil/sti/citations/ADA070930 or https://mdl-language.readthedocs.io/en/latest/01-basic-introduction/
// https://eblong.com/infocom/

// Random Claude Shannon
// http://people.math.harvard.edu/~ctm/home/text/others/shannon/entropy/entropy.pdf

// symbol table
// shunting-yard algorithm

// <SET .X ...> and <SET ,X ...> are never used, same for SETG
// possible meanings of dot, comma:
// dereference (value at address of pointer)
// address (the address the pointer refers to)
// WRONG! https://mdl-language.readthedocs.io/en/latest/04-values-of-atoms/#431-set-1
// dot = local value
// comma = global value
// also, this is weird:
// <SET FOO <SET BAR 500>>$
// 500
// <SET BAR FOO>$
// FOO
// .BAR$
// FOO
// ..BAR$
// 500

/*
<ROUTINE OTVAL-FROB ("OPTIONAL" (O ,TROPHY-CASE) "AUX" F (SCORE 0))
	<SET F <FIRST? .O>>
	<REPEAT ()
		<COND (<NOT .F> <RETURN .SCORE>)>
		<SET SCORE <+ .SCORE <GETP .F ,P?TVALUE>>>
		<COND (<FIRST? .F> <OTVAL-FROB .F>)>
		<SET F <NEXT? .F>>
	>
>
*/

const otvalFrob = newLocalScope((o=global("trophyCase")) => {
	set("o", o);
	set("f", null);
	set("score", 0);
	set("f", first_q(local("o")));
	while (true) {
		if (not(local("f"))) {
			return local("score")
		}
		set("score", plus(local("score"), getp(local("f"), property("tvalue"))))
		// is this conditional going to work?
		if (first_q(local("f"))) {
			otvalFrob(local("f"))
		}
		set("f", next_q(local("f")))
	}
});
addRoutine(otvalFrob);

// ,P?XXX refers to property XXX of an OBJECT (or ROOM?). Either way, is always global

// ---------------- DUNGEON ----------------

// ??
// every object has: in, synonym, adjective, desc, flags
// can also have: size, action, fdesc, capacity, strength, ldesc, tvalue
// OBJECT actions cannot have any required params, but may have optional params?
// ROOM actions always take one param?

// in room direction conditionals, ELSE is always followed by a string
// OBJECTs can be in ROOMs or OBJECTs, what does GLOBAL-OBJECTS and LOCAL-GLOBALS mean?
// ROOMs are always in "room"
// action must always be a routine

// ROOM directionals: bit tricky to explain, can look like
// (IN "The dam blocks your way.")
// (IN TO SQUEEKY-ROOM)
// (IN PER GRATING-EXIT)
// (IN TO STONE-BARROW IF WON-FLAG)
// (IN TO KITCHEN IF KITCHEN-WINDOW IS OPEN)
// (IN TO RESERVOIR IF LOW-TIDE ELSE "You would drown.")
// (IN TO X IF Y IS Z ELSE "Text")

// "IN" | "NORTH" | "NE" | "EAST" | "SE" | "SOUTH" | "SW" | "WEST" | "NW" | "LAND" | "UP" | "DOWN" | "OUT"

// "GLOBAL" => {
// at least 2 children
// all must be words
// objects that are in this room?

// "PSEUDO" => {
// at least 3 children
// alternating word, text, word, text, etc.
// like (PSEUDO "DOOR" DOOR-PSEUDO "PAINT" PAINT-PSEUDO)
// fake objects (actually routines) that are in this room that you can interact with? In other ways than move?

/*
<OBJECT SWORD
	(IN LIVING-ROOM)
	(SYNONYM SWORD ORCRIST GLAMDRING BLADE)
	(ADJECTIVE ELVISH OLD ANTIQUE)
	(DESC "sword")
	(FLAGS TAKEBIT WEAPONBIT TRYTAKEBIT)
	(ACTION SWORD-FCN)
	(FDESC
"Above the trophy case hangs an elvish sword of great antiquity.")
	(SIZE 30)
	(TVALUE 0)>
*/

const sword = {
	baseType: BaseTypes.OBJECT,
	in: livingRoom, // a room
	synonym: ["sword", "orcrist", 'glamdring', 'blade'],
	adjective: ['elvish', 'old', 'antique'],
	desc: "sword",
	flags: ["takebit", "weaponbit", "trytakebit"],
	action: swordFcn, // a routine
	fdesc: "Above the trophy case hangs an elvish sword of great antiquity.",
	size: 30,
	tvalue: 0
}
addObject(sword);

/*
<OBJECT THIEF
	(IN ROUND-ROOM)
	(SYNONYM THIEF ROBBER MAN PERSON)
	(ADJECTIVE SHADY SUSPICIOUS SEEDY)
	(DESC "thief")
	(FLAGS ACTORBIT INVISIBLE CONTBIT OPENBIT TRYTAKEBIT)
	(ACTION ROBBER-FUNCTION)
	(LDESC
"There is a suspicious-looking individual, holding a large bag, leaning
against one wall. He is armed with a deadly stiletto.")
	(STRENGTH 5)>
*/

const thief = {
	baseType: BaseTypes.OBJECT,
	in: roundRoom, // a room
	synonym: ["thief", "robber", "man", "person"],
	adjective: ["shady", "suspicious", "seedy"],
	desc: "thief",
	flags: ["actorbit", "invisible", "contbit", "openbit", "trytakebit"],
	action: robberFunction, // a routine
	ldesc: "There is a suspicious-looking individual, holding a large bag, leaning\nagainst one wall. He is armed with a deadly stiletto.",
	strength: 5
}
addObject(thief);

/*
<OBJECT STILETTO
	(IN THIEF)
	(SYNONYM STILETTO)
	(ADJECTIVE VICIOUS)
	(DESC "stiletto")
	(ACTION STILETTO-FUNCTION)
	(FLAGS WEAPONBIT TRYTAKEBIT TAKEBIT NDESCBIT)
	(SIZE 10)>
*/

// does NDESCBIT mean this object has no LDESC? No! Maybe?
const stiletto = {
	baseType: BaseTypes.OBJECT,
	in: thief, // an object
	synonym: ["stiletto"],
	adjective: ["vicious"],
	desc: "stiletto",
	action: stilletoFunction, // a routine
	flags: ["weaponbit", "trytakebit", "takebit", "ndescbit"],
	size: 10
}
addObject(stilleto);

/*
<OBJECT PUTTY
	(IN TUBE)
	(SYNONYM MATERIAL GUNK)
	(ADJECTIVE VISCOUS)
	(DESC "viscous material")
	(FLAGS TAKEBIT TOOLBIT)
	(SIZE 6)
	(ACTION PUTTY-FCN)>
*/

const putty = {
	baseType: BaseTypes.OBJECT,
	in: tube, // an object
	synonym: ["material", "gunk"],
	adjective: ["viscous"],
	desc: "viscous material",
	flags: ["takebit", "toolbit"],
	size: 6,
	action: puttyFcn // a routine
}
addObject(putty);

/*
<OBJECT NEST
	(IN UP-A-TREE)
	(SYNONYM NEST)
	(ADJECTIVE BIRDS)
	(DESC "bird's nest")
	(FLAGS TAKEBIT BURNBIT CONTBIT OPENBIT SEARCHBIT)
	(FDESC "Beside you on the branch is a small bird's nest.")
	(CAPACITY 20)>
*/

const nest = {
	baseType: BaseTypes.OBJECT,
	in: upATree, // a room
	synonym: "nest",
	adjective: "birds",
	desc: "bird's nest",
	flags: ["takebit", "burnbit", "contbit", "openbit", "searchbit"],
	fdesc: "Beside you on the branch is a small bird's nest.",
	capacity: 20
}
addObject(nest);

/*
<ROOM WEST-OF-HOUSE
      (IN ROOMS)
      (DESC "West of House")
      (NORTH TO NORTH-OF-HOUSE)
      (SOUTH TO SOUTH-OF-HOUSE)
      (NE TO NORTH-OF-HOUSE)
      (SE TO SOUTH-OF-HOUSE)
      (WEST TO FOREST-1)
      (EAST "The door is boarded and you can't remove the boards.")
      (SW TO STONE-BARROW IF WON-FLAG)
      (IN TO STONE-BARROW IF WON-FLAG)
      (ACTION WEST-HOUSE)
      (FLAGS RLANDBIT ONBIT SACREDBIT)
      (GLOBAL WHITE-HOUSE BOARD FOREST)>
*/

const westOfHouse = {
	baseType: BaseTypes.ROOM,
	in: rooms, // ???
	desc: "West of House",
	// ...
	action: westHouse, // a routine
	flags: ["rlandbit", "onbit", "sacredbit"],
	global: ["whiteHouse", "board", "forest"] // ???
}
addRoom(westOfHouse);

/*
<ROOM ATTIC
      (IN ROOMS)
      (LDESC "This is the attic. The only exit is a stairway leading down.")
      (DESC "Attic")
      (DOWN TO KITCHEN)
      (FLAGS RLANDBIT SACREDBIT)
      (GLOBAL STAIRS)>
*/

const attic = {
	baseType: BaseTypes.ROOM,
	in: rooms, // ???
	ldesc: "This is the attic. The only exit is a stairway leading down.",
	desc: "Attic",
	// ...
	flags: ["rlandbit", "sacredbit"],
	global: ["stairs"] // ???
}

/*
<ROOM CELLAR
      (IN ROOMS)
      (DESC "Cellar")
      (NORTH TO TROLL-ROOM)
      (SOUTH TO EAST-OF-CHASM)
      (UP TO LIVING-ROOM IF TRAP-DOOR IS OPEN)
      (WEST
"You try to ascend the ramp, but it is impossible, and you slide back down.")
      (ACTION CELLAR-FCN)
      (FLAGS RLANDBIT)
      (VALUE 25)
      (GLOBAL TRAP-DOOR SLIDE STAIRS)>
*/

/*
<ROOM LOWER-SHAFT	;"was BSHAF"
      (IN ROOMS)
      (LDESC
"This is a small drafty room in which is the bottom of a long
shaft. To the south is a passageway and to the east a very narrow
passage. In the shaft can be seen a heavy iron chain.")
      (DESC "Drafty Room")
      (SOUTH TO MACHINE-ROOM)
      (OUT TO TIMBER-ROOM
       IF EMPTY-HANDED
       ELSE "You cannot fit through this passage with that load.")
      (EAST TO TIMBER-ROOM
       IF EMPTY-HANDED
       ELSE "You cannot fit through this passage with that load.")
      (ACTION NO-OBJS)
      (FLAGS RLANDBIT SACREDBIT)
      (PSEUDO "CHAIN" CHAIN-PSEUDO)>
*/

/*
<ROOM GRATING-ROOM
      (IN ROOMS)
      (DESC "Grating Room")
      (SW TO MAZE-11)
      (UP TO GRATING-CLEARING
       IF GRATE IS OPEN ELSE "The grating is closed.")
      (ACTION MAZE-11-FCN)
      (GLOBAL GRATE)
      (FLAGS RLANDBIT)>
*/

/*
<ROOM ENTRANCE-TO-HADES
      (IN ROOMS)
      (DESC "Entrance to Hades")
      (UP TO TINY-CAVE)
      (IN TO LAND-OF-LIVING-DEAD IF LLD-FLAG
       ELSE "Some invisible force prevents you from passing through the gate.")
      (SOUTH TO LAND-OF-LIVING-DEAD IF LLD-FLAG
       ELSE "Some invisible force prevents you from passing through the gate.")
      (ACTION LLD-ROOM)
      (FLAGS RLANDBIT ONBIT)
      (GLOBAL BODIES)
      (PSEUDO "GATE" GATE-PSEUDO "GATES" GATE-PSEUDO)>
*/

/*
<ROOM LIVING-ROOM
      (IN ROOMS)
      (DESC "Living Room")
      (EAST TO KITCHEN)
      (WEST TO STRANGE-PASSAGE IF MAGIC-FLAG ELSE "The door is nailed shut.")
      (DOWN PER TRAP-DOOR-EXIT)
      (ACTION LIVING-ROOM-FCN)
      (FLAGS RLANDBIT ONBIT SACREDBIT)
      (GLOBAL STAIRS)
      (PSEUDO "NAILS" NAILS-PSEUDO "NAIL" NAILS-PSEUDO)>
*/

// ---------------- SYNTAX ----------------

/*
<SYNONYM NW NORTHWEST>
*/

/*
<SYNONYM CHOMP LOSE BARF>
*/

/*
<SYNTAX TURN OBJECT (FIND TURNBIT) (HELD CARRIED ON-GROUND IN-ROOM)
	WITH OBJECT (FIND RMUNGBIT) = V-TURN PRE-TURN>
*/

/*
<SYNTAX TAKE OBJECT (FIND TAKEBIT) (IN-ROOM CARRIED MANY)
	FROM OBJECT = V-TAKE PRE-TAKE>
*/

/*
<SYNTAX CLIMB UP OBJECT (FIND RMUNGBIT) = V-CLIMB-UP>
*/

/*
<SYNTAX CLIMB UP OBJECT (FIND CLIMBBIT) (ON-GROUND IN-ROOM) = V-CLIMB-UP>
*/

// ---------------- VERBS ----------------

/*
<GLOBAL LUCKY 1>
*/

/*
<ROUTINE V-SWING ()
	 <COND (<NOT ,PRSI>
		<TELL "Whoosh!" CR>)
	       (T
		<PERFORM ,V?ATTACK ,PRSI ,PRSO>)>>
*/

/*
<GLOBAL INDENTS
	<TABLE (PURE)
	       ""
	       "  "
	       "    "
	       "      "
	       "        "
	       "          ">>
*/

/*
<ROUTINE DO-WALK (DIR)
	 <SETG P-WALK-DIR .DIR>
	 <PERFORM ,V?WALK .DIR>>
*/

/*
<ROUTINE GLOBAL-IN? (OBJ1 OBJ2 "AUX" TX)
	 <COND (<SET TX <GETPT .OBJ2 ,P?GLOBAL>>
		<ZMEMQB .OBJ1 .TX <- <PTSIZE .TX> 1>>)>> 
*/