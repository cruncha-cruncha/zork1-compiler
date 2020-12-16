let BOARD = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["BOARDS", "BOARD"],
  DESC: () => "board",
  FLAGS: { NDESCBIT: true },
};

let TEETH = {
  IN: () => "GLOBAL_OBJECTS",
  SYNONYM: () => ["OVERBOARD", "TEETH"],
  DESC: () => "set of teeth",
  FLAGS: { NDESCBIT: true },
};

let WALL = {
  IN: () => "GLOBAL_OBJECTS",
  SYNONYM: () => ["WALL", "WALLS"],
  ADJECTIVE: () => ["SURROUNDING"],
  DESC: () => "surrounding wall",
};

let GRANITE_WALL = {
  IN: () => "GLOBAL_OBJECTS",
  SYNONYM: () => ["WALL"],
  ADJECTIVE: () => ["GRANITE"],
  DESC: () => "granite wall",
};

let SONGBIRD = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["BIRD", "SONGBIRD"],
  ADJECTIVE: () => ["SONG"],
  DESC: () => "songbird",
  FLAGS: { NDESCBIT: true },
};

let WHITE_HOUSE = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["HOUSE"],
  ADJECTIVE: () => ["WHITE", "BEAUTI", "COLONI"],
  DESC: () => "white house",
  FLAGS: { NDESCBIT: true },
};

let FOREST = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["FOREST", "TREES", "PINES", "HEMLOCKS"],
  DESC: () => "forest",
  FLAGS: { NDESCBIT: true },
};

let TREE = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["TREE", "BRANCH"],
  ADJECTIVE: () => ["LARGE", "STORM"],
  DESC: () => "tree",
  FLAGS: { NDESCBIT: true, CLIMBBIT: true },
};

let MOUNTAIN_RANGE = {
  IN: () => "MOUNTAINS",
  DESC: () => "mountain range",
  SYNONYM: () => ["MOUNTAIN", "RANGE"],
  ADJECTIVE: () => ["IMPASSABLE", "FLATHEAD"],
  FLAGS: { NDESCBIT: true, CLIMBBIT: true },
};

let GLOBAL_WATER = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["WATER", "QUANTITY"],
  DESC: () => "water",
  FLAGS: { DRINKBIT: true },
};

let WATER = {
  IN: () => "BOTTLE",
  SYNONYM: () => ["WATER", "QUANTITY", "LIQUID", "H2O"],
  DESC: () => "quantity of water",
  FLAGS: { TRYTAKEBIT: true, TAKEBIT: true, DRINKBIT: true },
  SIZE: () => 4,
};

let KITCHEN_WINDOW = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["WINDOW"],
  ADJECTIVE: () => ["KITCHEN", "SMALL"],
  DESC: () => "kitchen window",
  FLAGS: { DOORBIT: true, NDESCBIT: true },
};

let CHIMNEY = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["CHIMNEY"],
  ADJECTIVE: () => ["DARK", "NARROW"],
  DESC: () => "chimney",
  FLAGS: { CLIMBBIT: true, NDESCBIT: true },
};

let GHOSTS = {
  IN: () => "ENTRANCE_TO_HADES",
  SYNONYM: () => ["GHOSTS", "SPIRITS", "FIENDS", "FORCE"],
  ADJECTIVE: () => ["INVISIBLE", "EVIL"],
  DESC: () => "number of ghosts",
  FLAGS: { ACTORBIT: true, NDESCBIT: true },
};

let SKULL = {
  IN: () => "LAND_OF_LIVING_DEAD",
  SYNONYM: () => ["SKULL", "HEAD", "TREASURE"],
  ADJECTIVE: () => ["CRYSTAL"],
  DESC: () => "crystal skull",
  FDESC: () => "Lying in one corner of the room is a beautifully carved crystal skull. It appears to be grinning at you rather nastily.",
  FLAGS: { TAKEBIT: true },
  VALUE: () => 10,
  TVALUE: () => 10,
};

let LOWERED_BASKET = {
  IN: () => "LOWER_SHAFT",
  SYNONYM: () => ["CAGE", "DUMBWAITER", "BASKET"],
  ADJECTIVE: () => ["LOWERED"],
  LDESC: () => "From the chain is suspended a basket.",
  DESC: () => "basket",
  FLAGS: { TRYTAKEBIT: true },
};

let RAISED_BASKET = {
  IN: () => "SHAFT_ROOM",
  SYNONYM: () => ["CAGE", "DUMBWAITER", "BASKET"],
  DESC: () => "basket",
  FLAGS: { TRANSBIT: true, TRYTAKEBIT: true, CONTBIT: true, OPENBIT: true },
  LDESC: () => "At the end of the chain is a basket.",
  CAPACITY: () => 50,
};

let LUNCH = {
  IN: () => "SANDWICH_BAG",
  SYNONYM: () => ["FOOD", "SANDWICH", "LUNCH", "DINNER"],
  ADJECTIVE: () => ["HOT", "PEPPER"],
  DESC: () => "lunch",
  FLAGS: { TAKEBIT: true, FOODBIT: true },
  LDESC: () => "A hot pepper sandwich is here.",
};

let BAT = {
  IN: () => "BAT_ROOM",
  SYNONYM: () => ["BAT", "VAMPIRE"],
  ADJECTIVE: () => ["VAMPIRE", "DERANGED"],
  DESC: () => "bat",
  FLAGS: { ACTORBIT: true, TRYTAKEBIT: true },
};

let BELL = {
  IN: () => "NORTH_TEMPLE",
  SYNONYM: () => ["BELL"],
  ADJECTIVE: () => ["SMALL", "BRASS"],
  DESC: () => "brass bell",
  FLAGS: { TAKEBIT: true },
};

let HOT_BELL = {
  SYNONYM: () => ["BELL"],
  ADJECTIVE: () => ["BRASS", "HOT", "RED", "SMALL"],
  DESC: () => "red hot brass bell",
  FLAGS: { TRYTAKEBIT: true },
  LDESC: () => "On the ground is a red hot bell.",
};

let AXE = {
  IN: () => "TROLL",
  SYNONYM: () => ["AXE", "AX"],
  ADJECTIVE: () => ["BLOODY"],
  DESC: () => "bloody axe",
  FLAGS: { WEAPONBIT: true, TRYTAKEBIT: true, TAKEBIT: true, NDESCBIT: true },
  SIZE: () => 25,
};

let BOLT = {
  IN: () => "DAM_ROOM",
  SYNONYM: () => ["BOLT", "NUT"],
  ADJECTIVE: () => ["METAL", "LARGE"],
  DESC: () => "bolt",
  FLAGS: { NDESCBIT: true, TURNBIT: true, TRYTAKEBIT: true },
};

let BUBBLE = {
  IN: () => "DAM_ROOM",
  SYNONYM: () => ["BUBBLE"],
  ADJECTIVE: () => ["SMALL", "GREEN", "PLASTIC"],
  DESC: () => "green bubble",
  FLAGS: { NDESCBIT: true, TRYTAKEBIT: true },
};

let ALTAR = {
  IN: () => "SOUTH_TEMPLE",
  SYNONYM: () => ["ALTAR"],
  DESC: () => "altar",
  FLAGS: { NDESCBIT: true, SURFACEBIT: true, CONTBIT: true, OPENBIT: true },
  CAPACITY: () => 50,
};

let BOOK = {
  IN: () => "ALTAR",
  SYNONYM: () => ["BOOK", "PRAYER", "PAGE", "BOOKS"],
  ADJECTIVE: () => ["LARGE", "BLACK"],
  DESC: () => "black book",
  FLAGS: { READBIT: true, TAKEBIT: true, CONTBIT: true, BURNBIT: true, TURNBIT: true },
  FDESC: () => "On the altar is a large black book, open to page 569.",
  SIZE: () => 10,
  TEXT: () => "Commandment #12592| | Oh ye who go about saying unto each:  \"Hello sailor\":| Dost thou know the magnitude of thy sin before the gods?| Yea, verily, thou shalt be ground between two stones.| Shall the angry gods cast thy body into the whirlpool?| Surely, thy eye shall be put out with a sharp stick!| Even unto the ends of the earth shalt thou wander and| Unto the land of the dead shalt thou be sent at last.| Surely thou shalt repent of thy cunning.",
};

let BROKEN_LAMP = {
  SYNONYM: () => ["LAMP", "LANTERN"],
  ADJECTIVE: () => ["BROKEN"],
  DESC: () => "broken lantern",
  FLAGS: { TAKEBIT: true },
};

let SCEPTRE = {
  IN: () => "COFFIN",
  SYNONYM: () => ["SCEPTRE", "SCEPTER", "TREASURE"],
  ADJECTIVE: () => ["SHARP", "EGYPTIAN", "ANCIENT", "ENAMELED"],
  DESC: () => "sceptre",
  FLAGS: { TAKEBIT: true, WEAPONBIT: true },
  LDESC: () => "An ornamented sceptre, tapering to a sharp point, is here.",
  FDESC: () => "A sceptre, possibly that of ancient Egypt itself, is in the coffin. The sceptre is ornamented with colored enamel, and tapers to a sharp point.",
  SIZE: () => 3,
  VALUE: () => 4,
  TVALUE: () => 6,
};

let TIMBERS = {
  IN: () => "TIMBER_ROOM",
  SYNONYM: () => ["TIMBERS", "PILE"],
  ADJECTIVE: () => ["WOODEN", "BROKEN"],
  DESC: () => "broken timber",
  FLAGS: { TAKEBIT: true },
  SIZE: () => 50,
};

let SLIDE = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["CHUTE", "RAMP", "SLIDE"],
  ADJECTIVE: () => ["STEEP", "METAL", "TWISTING"],
  DESC: () => "chute",
  FLAGS: { CLIMBBIT: true },
};

let KITCHEN_TABLE = {
  IN: () => "KITCHEN",
  SYNONYM: () => ["TABLE"],
  ADJECTIVE: () => ["KITCHEN"],
  DESC: () => "kitchen table",
  FLAGS: { NDESCBIT: true, CONTBIT: true, OPENBIT: true, SURFACEBIT: true },
  CAPACITY: () => 50,
};

let ATTIC_TABLE = {
  IN: () => "ATTIC",
  SYNONYM: () => ["TABLE"],
  DESC: () => "table",
  FLAGS: { NDESCBIT: true, CONTBIT: true, OPENBIT: true, SURFACEBIT: true },
  CAPACITY: () => 40,
};

let SANDWICH_BAG = {
  IN: () => "KITCHEN_TABLE",
  SYNONYM: () => ["BAG", "SACK"],
  ADJECTIVE: () => ["BROWN", "ELONGATED", "SMELLY"],
  DESC: () => "brown sack",
  FLAGS: { TAKEBIT: true, CONTBIT: true, BURNBIT: true },
  FDESC: () => "On the table is an elongated brown sack, smelling of hot peppers.",
  CAPACITY: () => 9,
  SIZE: () => 9,
};

let TOOL_CHEST = {
  IN: () => "MAINTENANCE_ROOM",
  SYNONYM: () => ["CHEST", "CHESTS", "GROUP", "TOOLCHESTS"],
  ADJECTIVE: () => ["TOOL"],
  DESC: () => "group of tool chests",
  FLAGS: { CONTBIT: true, OPENBIT: true, TRYTAKEBIT: true, SACREDBIT: true },
};

let YELLOW_BUTTON = {
  IN: () => "MAINTENANCE_ROOM",
  SYNONYM: () => ["BUTTON", "SWITCH"],
  ADJECTIVE: () => ["YELLOW"],
  DESC: () => "yellow button",
  FLAGS: { NDESCBIT: true },
};

let BROWN_BUTTON = {
  IN: () => "MAINTENANCE_ROOM",
  SYNONYM: () => ["BUTTON", "SWITCH"],
  ADJECTIVE: () => ["BROWN"],
  DESC: () => "brown button",
  FLAGS: { NDESCBIT: true },
};

let RED_BUTTON = {
  IN: () => "MAINTENANCE_ROOM",
  SYNONYM: () => ["BUTTON", "SWITCH"],
  ADJECTIVE: () => ["RED"],
  DESC: () => "red button",
  FLAGS: { NDESCBIT: true },
};

let BLUE_BUTTON = {
  IN: () => "MAINTENANCE_ROOM",
  SYNONYM: () => ["BUTTON", "SWITCH"],
  ADJECTIVE: () => ["BLUE"],
  DESC: () => "blue button",
  FLAGS: { NDESCBIT: true },
};

let TROPHY_CASE = {
  IN: () => "LIVING_ROOM",
  SYNONYM: () => ["CASE"],
  ADJECTIVE: () => ["TROPHY"],
  DESC: () => "trophy case",
  FLAGS: { TRANSBIT: true, CONTBIT: true, NDESCBIT: true, TRYTAKEBIT: true, SEARCHBIT: true },
  CAPACITY: () => 10000,
};

let RUG = {
  IN: () => "LIVING_ROOM",
  SYNONYM: () => ["RUG", "CARPET"],
  ADJECTIVE: () => ["LARGE", "ORIENTAL"],
  DESC: () => "carpet",
  FLAGS: { NDESCBIT: true, TRYTAKEBIT: true },
};

let CHALICE = {
  IN: () => "TREASURE_ROOM",
  SYNONYM: () => ["CHALICE", "CUP", "SILVER", "TREASURE"],
  ADJECTIVE: () => ["SILVER", "ENGRAVINGS"],
  DESC: () => "chalice",
  FLAGS: { TAKEBIT: true, TRYTAKEBIT: true, CONTBIT: true },
  LDESC: () => "There is a silver chalice, intricately engraved, here.",
  CAPACITY: () => 5,
  SIZE: () => 10,
  VALUE: () => 10,
  TVALUE: () => 5,
};

let GARLIC = {
  IN: () => "SANDWICH_BAG",
  SYNONYM: () => ["GARLIC", "CLOVE"],
  DESC: () => "clove of garlic",
  FLAGS: { TAKEBIT: true, FOODBIT: true },
  SIZE: () => 4,
};

let TRIDENT = {
  IN: () => "ATLANTIS_ROOM",
  SYNONYM: () => ["TRIDENT", "FORK", "TREASURE"],
  ADJECTIVE: () => ["POSEIDON", "OWN", "CRYSTAL"],
  DESC: () => "crystal trident",
  FLAGS: { TAKEBIT: true },
  FDESC: () => "On the shore lies Poseidon's own crystal trident.",
  SIZE: () => 20,
  VALUE: () => 4,
  TVALUE: () => 11,
};

let CYCLOPS = {
  IN: () => "CYCLOPS_ROOM",
  SYNONYM: () => ["CYCLOPS", "MONSTER", "EYE"],
  ADJECTIVE: () => ["HUNGRY", "GIANT"],
  DESC: () => "cyclops",
  FLAGS: { ACTORBIT: true, NDESCBIT: true, TRYTAKEBIT: true },
  STRENGTH: () => 10000,
};

let DAM = {
  IN: () => "DAM_ROOM",
  SYNONYM: () => ["DAM", "GATE", "GATES", "FCD\#3"],
  DESC: () => "dam",
  FLAGS: { NDESCBIT: true, TRYTAKEBIT: true },
};

let TRAP_DOOR = {
  IN: () => "LIVING_ROOM",
  SYNONYM: () => ["DOOR", "TRAPDOOR", "TRAP_DOOR", "COVER"],
  ADJECTIVE: () => ["TRAP", "DUSTY"],
  DESC: () => "trap door",
  FLAGS: { DOORBIT: true, NDESCBIT: true, INVISIBLE: true },
};

let BOARDED_WINDOW = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["WINDOW"],
  ADJECTIVE: () => ["BOARDED"],
  DESC: () => "boarded window",
  FLAGS: { NDESCBIT: true },
};

let FRONT_DOOR = {
  IN: () => "WEST_OF_HOUSE",
  SYNONYM: () => ["DOOR"],
  ADJECTIVE: () => ["FRONT", "BOARDED"],
  DESC: () => "door",
  FLAGS: { DOORBIT: true, NDESCBIT: true },
};

let BARROW_DOOR = {
  IN: () => "STONE_BARROW",
  SYNONYM: () => ["DOOR"],
  ADJECTIVE: () => ["HUGE", "STONE"],
  DESC: () => "stone door",
  FLAGS: { DOORBIT: true, NDESCBIT: true, OPENBIT: true },
};

let BARROW = {
  IN: () => "STONE_BARROW",
  SYNONYM: () => ["BARROW", "TOMB"],
  ADJECTIVE: () => ["MASSIVE", "STONE"],
  DESC: () => "stone barrow",
  FLAGS: { NDESCBIT: true },
};

let BOTTLE = {
  IN: () => "KITCHEN_TABLE",
  SYNONYM: () => ["BOTTLE", "CONTAINER"],
  ADJECTIVE: () => ["CLEAR", "GLASS"],
  DESC: () => "glass bottle",
  FLAGS: { TAKEBIT: true, TRANSBIT: true, CONTBIT: true },
  FDESC: () => "A bottle is sitting on the table.",
  CAPACITY: () => 4,
};

let CRACK = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["CRACK"],
  ADJECTIVE: () => ["NARROW"],
  DESC: () => "crack",
  FLAGS: { NDESCBIT: true },
};

let COFFIN = {
  IN: () => "EGYPT_ROOM",
  SYNONYM: () => ["COFFIN", "CASKET", "TREASURE"],
  ADJECTIVE: () => ["SOLID", "GOLD"],
  DESC: () => "gold coffin",
  FLAGS: { TAKEBIT: true, CONTBIT: true, SACREDBIT: true, SEARCHBIT: true },
  LDESC: () => "The solid-gold coffin used for the burial of Ramses II is here.",
  CAPACITY: () => 35,
  SIZE: () => 55,
  VALUE: () => 10,
  TVALUE: () => 15,
};

let GRATE = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["GRATE", "GRATING"],
  DESC: () => "grating",
  FLAGS: { DOORBIT: true, NDESCBIT: true, INVISIBLE: true },
};

let PUMP = {
  IN: () => "RESERVOIR_NORTH",
  SYNONYM: () => ["PUMP", "AIR_PUMP", "TOOL", "TOOLS"],
  ADJECTIVE: () => ["SMALL", "HAND_HELD"],
  DESC: () => "hand-held air pump",
  FLAGS: { TAKEBIT: true, TOOLBIT: true },
};

let DIAMOND = {
  SYNONYM: () => ["DIAMOND", "TREASURE"],
  ADJECTIVE: () => ["HUGE", "ENORMOUS"],
  DESC: () => "huge diamond",
  FLAGS: { TAKEBIT: true },
  LDESC: () => "There is an enormous diamond (perfectly cut) here.",
  VALUE: () => 10,
  TVALUE: () => 10,
};

let JADE = {
  IN: () => "BAT_ROOM",
  SYNONYM: () => ["FIGURINE", "TREASURE"],
  ADJECTIVE: () => ["EXQUISITE", "JADE"],
  DESC: () => "jade figurine",
  FLAGS: { TAKEBIT: true },
  LDESC: () => "There is an exquisite jade figurine here.",
  SIZE: () => 10,
  VALUE: () => 5,
  TVALUE: () => 5,
};

let KNIFE = {
  IN: () => "ATTIC_TABLE",
  SYNONYM: () => ["KNIVES", "KNIFE", "BLADE"],
  ADJECTIVE: () => ["NASTY", "UNRUSTY"],
  DESC: () => "nasty knife",
  FLAGS: { TAKEBIT: true, WEAPONBIT: true, TRYTAKEBIT: true },
  FDESC: () => "On a table is a nasty-looking knife.",
};

let BONES = {
  IN: () => "MAZE_5",
  SYNONYM: () => ["BONES", "SKELETON", "BODY"],
  DESC: () => "skeleton",
  FLAGS: { TRYTAKEBIT: true, NDESCBIT: true },
};

let BURNED_OUT_LANTERN = {
  IN: () => "MAZE_5",
  SYNONYM: () => ["LANTERN", "LAMP"],
  ADJECTIVE: () => ["RUSTY", "BURNED", "DEAD", "USELESS"],
  DESC: () => "burned-out lantern",
  FLAGS: { TAKEBIT: true },
  FDESC: () => "The deceased adventurer's useless lantern is here.",
  SIZE: () => 20,
};

let BAG_OF_COINS = {
  IN: () => "MAZE_5",
  SYNONYM: () => ["BAG", "COINS", "TREASURE"],
  ADJECTIVE: () => ["OLD", "LEATHER"],
  DESC: () => "leather bag of coins",
  FLAGS: { TAKEBIT: true },
  LDESC: () => "An old leather bag, bulging with coins, is here.",
  SIZE: () => 15,
  VALUE: () => 10,
  TVALUE: () => 5,
};

let LAMP = {
  IN: () => "LIVING_ROOM",
  SYNONYM: () => ["LAMP", "LANTERN", "LIGHT"],
  ADJECTIVE: () => ["BRASS"],
  DESC: () => "brass lantern",
  FLAGS: { TAKEBIT: true, LIGHTBIT: true },
  FDESC: () => "A battery-powered brass lantern is on the trophy case.",
  LDESC: () => "There is a brass lantern (battery-powered) here.",
  SIZE: () => 15,
};

let EMERALD = {
  IN: () => "BUOY",
  SYNONYM: () => ["EMERALD", "TREASURE"],
  ADJECTIVE: () => ["LARGE"],
  DESC: () => "large emerald",
  FLAGS: { TAKEBIT: true },
  VALUE: () => 5,
  TVALUE: () => 10,
};

let ADVERTISEMENT = {
  IN: () => "MAILBOX",
  SYNONYM: () => ["ADVERTISEMENT", "LEAFLET", "BOOKLET", "MAIL"],
  ADJECTIVE: () => ["SMALL"],
  DESC: () => "leaflet",
  FLAGS: { READBIT: true, TAKEBIT: true, BURNBIT: true },
  LDESC: () => "A small leaflet is on the ground.",
  TEXT: () => "\"WELCOME TO ZORK!| | ZORK is a game of adventure, danger, and low cunning. In it you will explore some of the most amazing territory ever seen by mortals. No computer should be without one!\"",
  SIZE: () => 2,
};

let LEAK = {
  IN: () => "MAINTENANCE_ROOM",
  SYNONYM: () => ["LEAK", "DRIP", "PIPE"],
  DESC: () => "leak",
  FLAGS: { NDESCBIT: true, INVISIBLE: true },
};

let MACHINE = {
  IN: () => "MACHINE_ROOM",
  SYNONYM: () => ["MACHINE", "PDP10", "DRYER", "LID"],
  DESC: () => "machine",
  FLAGS: { CONTBIT: true, NDESCBIT: true, TRYTAKEBIT: true },
  CAPACITY: () => 50,
};

let INFLATED_BOAT = {
  SYNONYM: () => ["BOAT", "RAFT"],
  ADJECTIVE: () => ["INFLAT", "MAGIC", "PLASTIC", "SEAWORTHY"],
  DESC: () => "magic boat",
  FLAGS: { TAKEBIT: true, BURNBIT: true, VEHBIT: true, OPENBIT: true, SEARCHBIT: true },
  CAPACITY: () => 100,
  SIZE: () => 20,
};

let MAILBOX = {
  IN: () => "WEST_OF_HOUSE",
  SYNONYM: () => ["MAILBOX", "BOX"],
  ADJECTIVE: () => ["SMALL"],
  DESC: () => "small mailbox",
  FLAGS: { CONTBIT: true, TRYTAKEBIT: true },
  CAPACITY: () => 10,
};

let MATCH = {
  IN: () => "DAM_LOBBY",
  SYNONYM: () => ["MATCH", "MATCHES", "MATCHBOOK"],
  ADJECTIVE: () => ["MATCH"],
  DESC: () => "matchbook",
  FLAGS: { READBIT: true, TAKEBIT: true },
  LDESC: () => "There is a matchbook whose cover says \"Visit Beautiful FCD#3\" here.",
  SIZE: () => 2,
  TEXT: () => "| (Close cover before striking)| | YOU too can make BIG MONEY in the exciting field of PAPER SHUFFLING!| | Mr. Anderson of Muddle, Mass. says: \"Before I took this course I was a lowly bit twiddler. Now with what I learned at GUE Tech I feel really important and can obfuscate and confuse with the best.\"| | Dr. Blank had this to say: \"Ten short days ago all I could look forward to was a dead-end job as a doctor. Now I have a promising future and make really big Zorkmids.\"| | GUE Tech can't promise these fantastic results to everyone. But when you earn your degree from GUE Tech, your future will be brighter.",
};

let MIRROR_2 = {
  IN: () => "MIRROR_ROOM_2",
  SYNONYM: () => ["REFLECTION", "MIRROR", "ENORMOUS"],
  DESC: () => "mirror",
  FLAGS: { TRYTAKEBIT: true, NDESCBIT: true },
};

let MIRROR_1 = {
  IN: () => "MIRROR_ROOM_1",
  SYNONYM: () => ["REFLECTION", "MIRROR", "ENORMOUS"],
  DESC: () => "mirror",
  FLAGS: { TRYTAKEBIT: true, NDESCBIT: true },
};

let PAINTING = {
  IN: () => "GALLERY",
  SYNONYM: () => ["PAINTING", "ART", "CANVAS", "TREASURE"],
  ADJECTIVE: () => ["BEAUTI"],
  DESC: () => "painting",
  FLAGS: { TAKEBIT: true, BURNBIT: true },
  FDESC: () => "Fortunately, there is still one chance for you to be a vandal, for on the far wall is a painting of unparalleled beauty.",
  LDESC: () => "A painting by a neglected genius is here.",
  SIZE: () => 15,
  VALUE: () => 4,
  TVALUE: () => 6,
};

let CANDLES = {
  IN: () => "SOUTH_TEMPLE",
  SYNONYM: () => ["CANDLES", "PAIR"],
  ADJECTIVE: () => ["BURNING"],
  DESC: () => "pair of candles",
  FLAGS: { TAKEBIT: true, FLAMEBIT: true, ONBIT: true, LIGHTBIT: true },
  FDESC: () => "On the two ends of the altar are burning candles.",
  SIZE: () => 10,
};

let GUNK = {
  SYNONYM: () => ["GUNK", "PIECE", "SLAG"],
  ADJECTIVE: () => ["SMALL", "VITREOUS"],
  DESC: () => "small piece of vitreous slag",
  FLAGS: { TAKEBIT: true, TRYTAKEBIT: true },
  SIZE: () => 10,
};

let BODIES = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["BODIES", "BODY", "REMAINS", "PILE"],
  ADJECTIVE: () => ["MANGLED"],
  DESC: () => "pile of bodies",
  FLAGS: { NDESCBIT: true, TRYTAKEBIT: true },
};

let LEAVES = {
  IN: () => "GRATING_CLEARING",
  SYNONYM: () => ["LEAVES", "LEAF", "PILE"],
  DESC: () => "pile of leaves",
  FLAGS: { TAKEBIT: true, BURNBIT: true, TRYTAKEBIT: true },
  LDESC: () => "On the ground is a pile of leaves.",
  SIZE: () => 25,
};

let PUNCTURED_BOAT = {
  SYNONYM: () => ["BOAT", "PILE", "PLASTIC"],
  ADJECTIVE: () => ["PLASTIC", "PUNCTURE", "LARGE"],
  DESC: () => "punctured boat",
  FLAGS: { TAKEBIT: true, BURNBIT: true },
  SIZE: () => 20,
};

let INFLATABLE_BOAT = {
  IN: () => "DAM_BASE",
  SYNONYM: () => ["BOAT", "PILE", "PLASTIC", "VALVE"],
  ADJECTIVE: () => ["PLASTIC", "INFLAT"],
  DESC: () => "pile of plastic",
  FLAGS: { TAKEBIT: true, BURNBIT: true },
  LDESC: () => "There is a folded pile of plastic here which has a small valve attached.",
  SIZE: () => 20,
};

let BAR = {
  IN: () => "LOUD_ROOM",
  SYNONYM: () => ["BAR", "PLATINUM", "TREASURE"],
  ADJECTIVE: () => ["PLATINUM", "LARGE"],
  DESC: () => "platinum bar",
  FLAGS: { TAKEBIT: true, SACREDBIT: true },
  LDESC: () => "On the ground is a large platinum bar.",
  SIZE: () => 20,
  VALUE: () => 10,
  TVALUE: () => 5,
};

let POT_OF_GOLD = {
  IN: () => "END_OF_RAINBOW",
  SYNONYM: () => ["POT", "GOLD", "TREASURE"],
  ADJECTIVE: () => ["GOLD"],
  DESC: () => "pot of gold",
  FLAGS: { TAKEBIT: true, INVISIBLE: true },
  FDESC: () => "At the end of the rainbow is a pot of gold.",
  SIZE: () => 15,
  VALUE: () => 10,
  TVALUE: () => 10,
};

let PRAYER = {
  IN: () => "NORTH_TEMPLE",
  SYNONYM: () => ["PRAYER", "INSCRIPTION"],
  ADJECTIVE: () => ["ANCIENT", "OLD"],
  DESC: () => "prayer",
  FLAGS: { READBIT: true, SACREDBIT: true, NDESCBIT: true },
  TEXT: () => "The prayer is inscribed in an ancient script, rarely used today. It seems to be a philippic against small insects, absent-mindedness, and the picking up and dropping of small objects. The final verse consigns trespassers to the land of the dead. All evidence indicates that the beliefs of the ancient Zorkers were obscure.",
};

let RAILING = {
  IN: () => "DOME_ROOM",
  SYNONYM: () => ["RAILING", "RAIL"],
  ADJECTIVE: () => ["WOODEN"],
  DESC: () => "wooden railing",
  FLAGS: { NDESCBIT: true },
};

let RAINBOW = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["RAINBOW"],
  DESC: () => "rainbow",
  FLAGS: { NDESCBIT: true, CLIMBBIT: true },
};

let RIVER = {
  IN: () => "LOCAL_GLOBALS",
  DESC: () => "river",
  SYNONYM: () => ["RIVER"],
  ADJECTIVE: () => ["FRIGID"],
  FLAGS: { NDESCBIT: true },
};

let BUOY = {
  IN: () => "RIVER_4",
  SYNONYM: () => ["BUOY"],
  ADJECTIVE: () => ["RED"],
  DESC: () => "red buoy",
  FLAGS: { TAKEBIT: true, CONTBIT: true },
  FDESC: () => "There is a red buoy here (probably a warning).",
  CAPACITY: () => 20,
  SIZE: () => 10,
};

function TREASURE_INSIDE() {
  if (VERB_4(OPEN)) {
    SCORE_OBJ(_3EMERALD)
    RFALSE()
  }

}
let ROPE = {
  IN: () => "ATTIC",
  SYNONYM: () => ["ROPE", "HEMP", "COIL"],
  ADJECTIVE: () => ["LARGE"],
  DESC: () => "rope",
  FLAGS: { TAKEBIT: true, SACREDBIT: true, TRYTAKEBIT: true },
  FDESC: () => "A large coil of rope is lying in the corner.",
  SIZE: () => 10,
};

let RUSTY_KNIFE = {
  IN: () => "MAZE_5",
  SYNONYM: () => ["KNIVES", "KNIFE"],
  ADJECTIVE: () => ["RUSTY"],
  DESC: () => "rusty knife",
  FLAGS: { TAKEBIT: true, TRYTAKEBIT: true, WEAPONBIT: true, TOOLBIT: true },
  FDESC: () => "Beside the skeleton is a rusty knife.",
  SIZE: () => 20,
};

let SAND = {
  IN: () => "SANDY_CAVE",
  SYNONYM: () => ["SAND"],
  DESC: () => "sand",
  FLAGS: { NDESCBIT: true },
};

let BRACELET = {
  IN: () => "GAS_ROOM",
  SYNONYM: () => ["BRACELET", "JEWEL", "SAPPHIRE", "TREASURE"],
  ADJECTIVE: () => ["SAPPHIRE"],
  DESC: () => "sapphire-encrusted bracelet",
  FLAGS: { TAKEBIT: true },
  SIZE: () => 10,
  VALUE: () => 5,
  TVALUE: () => 5,
};

let SCREWDRIVER = {
  IN: () => "MAINTENANCE_ROOM",
  SYNONYM: () => ["SCREWDRIVER", "TOOL", "TOOLS", "DRIVER"],
  ADJECTIVE: () => ["SCREW"],
  DESC: () => "screwdriver",
  FLAGS: { TAKEBIT: true, TOOLBIT: true },
};

let KEYS = {
  IN: () => "MAZE_5",
  SYNONYM: () => ["KEY"],
  ADJECTIVE: () => ["SKELETON"],
  DESC: () => "skeleton key",
  FLAGS: { TAKEBIT: true, TOOLBIT: true },
  SIZE: () => 10,
};

let SHOVEL = {
  IN: () => "SANDY_BEACH",
  SYNONYM: () => ["SHOVEL", "TOOL", "TOOLS"],
  DESC: () => "shovel",
  FLAGS: { TAKEBIT: true, TOOLBIT: true },
  SIZE: () => 15,
};

let COAL = {
  IN: () => "DEAD_END_5",
  SYNONYM: () => ["COAL", "PILE", "HEAP"],
  ADJECTIVE: () => ["SMALL"],
  DESC: () => "small pile of coal",
  FLAGS: { TAKEBIT: true, BURNBIT: true },
  SIZE: () => 20,
};

let LADDER = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["LADDER"],
  ADJECTIVE: () => ["WOODEN", "RICKETY", "NARROW"],
  DESC: () => "wooden ladder",
  FLAGS: { NDESCBIT: true, CLIMBBIT: true },
};

let SCARAB = {
  IN: () => "SANDY_CAVE",
  SYNONYM: () => ["SCARAB", "BUG", "BEETLE", "TREASURE"],
  ADJECTIVE: () => ["BEAUTI", "CARVED", "JEWELED"],
  DESC: () => "beautiful jeweled scarab",
  FLAGS: { TAKEBIT: true, INVISIBLE: true },
  SIZE: () => 8,
  VALUE: () => 5,
  TVALUE: () => 5,
};

let LARGE_BAG = {
  IN: () => "THIEF",
  SYNONYM: () => ["BAG"],
  ADJECTIVE: () => ["LARGE", "THIEFS"],
  DESC: () => "large bag",
  FLAGS: { TRYTAKEBIT: true, NDESCBIT: true },
};

let STILETTO = {
  IN: () => "THIEF",
  SYNONYM: () => ["STILETTO"],
  ADJECTIVE: () => ["VICIOUS"],
  DESC: () => "stiletto",
  FLAGS: { WEAPONBIT: true, TRYTAKEBIT: true, TAKEBIT: true, NDESCBIT: true },
  SIZE: () => 10,
};

let MACHINE_SWITCH = {
  IN: () => "MACHINE_ROOM",
  SYNONYM: () => ["SWITCH"],
  DESC: () => "switch",
  FLAGS: { NDESCBIT: true, TURNBIT: true },
};

let WOODEN_DOOR = {
  IN: () => "LIVING_ROOM",
  SYNONYM: () => ["DOOR", "LETTERING", "WRITING"],
  ADJECTIVE: () => ["WOODEN", "GOTHIC", "STRANGE", "WEST"],
  DESC: () => "wooden door",
  FLAGS: { READBIT: true, DOORBIT: true, NDESCBIT: true, TRANSBIT: true },
  TEXT: () => "The engravings translate to \"This space intentionally left blank.\"",
};

let SWORD = {
  IN: () => "LIVING_ROOM",
  SYNONYM: () => ["SWORD", "ORCRIST", "GLAMDRING", "BLADE"],
  ADJECTIVE: () => ["ELVISH", "OLD", "ANTIQUE"],
  DESC: () => "sword",
  FLAGS: { TAKEBIT: true, WEAPONBIT: true, TRYTAKEBIT: true },
  FDESC: () => "Above the trophy case hangs an elvish sword of great antiquity.",
  SIZE: () => 30,
  TVALUE: () => 0,
};

let MAP = {
  IN: () => "TROPHY_CASE",
  SYNONYM: () => ["PARCHMENT", "MAP"],
  ADJECTIVE: () => ["ANTIQUE", "OLD", "ANCIENT"],
  DESC: () => "ancient map",
  FLAGS: { INVISIBLE: true, READBIT: true, TAKEBIT: true },
  FDESC: () => "In the trophy case is an ancient parchment which appears to be a map.",
  SIZE: () => 2,
  TEXT: () => "The map shows a forest with three clearings. The largest clearing contains a house. Three paths leave the large clearing. One of these paths, leading southwest, is marked \"To Stone Barrow\".",
};

let BOAT_LABEL = {
  IN: () => "INFLATED_BOAT",
  SYNONYM: () => ["LABEL", "FINEPRINT", "PRINT"],
  ADJECTIVE: () => ["TAN", "FINE"],
  DESC: () => "tan label",
  FLAGS: { READBIT: true, TAKEBIT: true, BURNBIT: true },
  SIZE: () => 2,
  TEXT: () => "  !!!!FROBOZZ MAGIC BOAT COMPANY!!!!| | Hello, Sailor!| | Instructions for use:| | To get into a body of water, say \"Launch\".| To get to shore, say \"Land\" or the direction in which you want to maneuver the boat.| | Warranty:| | This boat is guaranteed against all defects for a period of 76 milliseconds from date of purchase or until first used, whichever comes first.| | Warning:| This boat is made of thin plastic.| Good Luck!",
};

let THIEF = {
  IN: () => "ROUND_ROOM",
  SYNONYM: () => ["THIEF", "ROBBER", "MAN", "PERSON"],
  ADJECTIVE: () => ["SHADY", "SUSPICIOUS", "SEEDY"],
  DESC: () => "thief",
  FLAGS: { ACTORBIT: true, INVISIBLE: true, CONTBIT: true, OPENBIT: true, TRYTAKEBIT: true },
  LDESC: () => "There is a suspicious-looking individual, holding a large bag, leaning against one wall. He is armed with a deadly stiletto.",
  STRENGTH: () => 5,
};

let PEDESTAL = {
  IN: () => "TORCH_ROOM",
  SYNONYM: () => ["PEDESTAL"],
  ADJECTIVE: () => ["WHITE", "MARBLE"],
  DESC: () => "pedestal",
  FLAGS: { NDESCBIT: true, CONTBIT: true, OPENBIT: true, SURFACEBIT: true },
  CAPACITY: () => 30,
};

let TORCH = {
  IN: () => "PEDESTAL",
  SYNONYM: () => ["TORCH", "IVORY", "TREASURE"],
  ADJECTIVE: () => ["FLAMING", "IVORY"],
  DESC: () => "torch",
  FLAGS: { TAKEBIT: true, FLAMEBIT: true, ONBIT: true, LIGHTBIT: true },
  FDESC: () => "Sitting on the pedestal is a flaming torch, made of ivory.",
  SIZE: () => 20,
  VALUE: () => 14,
  TVALUE: () => 6,
};

let GUIDE = {
  IN: () => "DAM_LOBBY",
  SYNONYM: () => ["GUIDE", "BOOK", "BOOKS", "GUIDEBOOKS"],
  ADJECTIVE: () => ["TOUR", "GUIDE"],
  DESC: () => "tour guidebook",
  FLAGS: { READBIT: true, TAKEBIT: true, BURNBIT: true },
  FDESC: () => "Some guidebooks entitled \"Flood Control Dam #3\" are on the reception desk.",
  TEXT: () => "\" Flood Control Dam #3| | FCD#3 was constructed in year 783 of the Great Underground Empire to harness the mighty Frigid River. This work was supported by a grant of 37 million zorkmids from your omnipotent local tyrant Lord Dimwit Flathead the Excessive. This impressive structure is composed of 370,000 cubic feet of concrete, is 256 feet tall at the center, and 193 feet wide at the top. The lake created behind the dam has a volume of 1.7 billion cubic feet, an area of 12 million square feet, and a shore line of 36 thousand feet.| | The construction of FCD#3 took 112 days from ground breaking to the dedication. It required a work force of 384 slaves, 34 slave drivers, 12 engineers, 2 turtle doves, and a partridge in a pear tree. The work was managed by a command team composed of 2345 bureaucrats, 2347 secretaries (at least two of whom could type), 12,256 paper shufflers, 52,469 rubber stampers, 245,193 red tape processors, and nearly one million dead trees.| | We will now point out some of the more interesting features of FCD#3 as we conduct you on a guided tour of the facilities:| | 1) You start your tour here in the Dam Lobby. You will notice on your right that....",
};

let TROLL = {
  IN: () => "TROLL_ROOM",
  SYNONYM: () => ["TROLL"],
  ADJECTIVE: () => ["NASTY"],
  DESC: () => "troll",
  FLAGS: { ACTORBIT: true, OPENBIT: true, TRYTAKEBIT: true },
  LDESC: () => "A nasty-looking troll, brandishing a bloody axe, blocks all passages out of the room.",
  STRENGTH: () => 2,
};

let TRUNK = {
  IN: () => "RESERVOIR",
  SYNONYM: () => ["TRUNK", "CHEST", "JEWELS", "TREASURE"],
  ADJECTIVE: () => ["OLD"],
  DESC: () => "trunk of jewels",
  FLAGS: { TAKEBIT: true, INVISIBLE: true },
  FDESC: () => "Lying half buried in the mud is an old trunk, bulging with jewels.",
  LDESC: () => "There is an old trunk here, bulging with assorted jewels.",
  SIZE: () => 35,
  VALUE: () => 15,
  TVALUE: () => 5,
};

let TUBE = {
  IN: () => "MAINTENANCE_ROOM",
  SYNONYM: () => ["TUBE", "TOOTH", "PASTE"],
  DESC: () => "tube",
  FLAGS: { TAKEBIT: true, CONTBIT: true, READBIT: true },
  LDESC: () => "There is an object which looks like a tube of toothpaste here.",
  CAPACITY: () => 7,
  SIZE: () => 5,
  TEXT: () => "---> Frobozz Magic Gunk Company <---| All-Purpose Gunk",
};

let PUTTY = {
  IN: () => "TUBE",
  SYNONYM: () => ["MATERIAL", "GUNK"],
  ADJECTIVE: () => ["VISCOUS"],
  DESC: () => "viscous material",
  FLAGS: { TAKEBIT: true, TOOLBIT: true },
  SIZE: () => 6,
};

let ENGRAVINGS = {
  IN: () => "ENGRAVINGS_CAVE",
  SYNONYM: () => ["WALL", "ENGRAVINGS", "INSCRIPTION"],
  ADJECTIVE: () => ["OLD", "ANCIENT"],
  DESC: () => "wall with engravings",
  FLAGS: { READBIT: true, SACREDBIT: true },
  LDESC: () => "There are old engravings on the walls here.",
  TEXT: () => "The engravings were incised in the living rock of the cave wall by an unknown hand. They depict, in symbolic form, the beliefs of the ancient Zorkers. Skillfully interwoven with the bas reliefs are excerpts illustrating the major religious tenets of that time. Unfortunately, a later age seems to have considered them blasphemous and just as skillfully excised them.",
};

let OWNERS_MANUAL = {
  IN: () => "STUDIO",
  SYNONYM: () => ["MANUAL", "PIECE", "PAPER"],
  ADJECTIVE: () => ["ZORK", "OWNERS", "SMALL"],
  DESC: () => "ZORK owner's manual",
  FLAGS: { READBIT: true, TAKEBIT: true },
  FDESC: () => "Loosely attached to a wall is a small piece of paper.",
  TEXT: () => "Congratulations!| | You are the privileged owner of ZORK I: The Great Underground Empire, a self-contained and self-maintaining universe. If used and maintained in accordance with normal operating practices for small universes, ZORK will provide many months of trouble-free operation.",
};

let CLIMBABLE_CLIFF = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["WALL", "CLIFF", "WALLS", "LEDGE"],
  ADJECTIVE: () => ["ROCKY", "SHEER"],
  DESC: () => "cliff",
  FLAGS: { NDESCBIT: true, CLIMBBIT: true },
};

let WHITE_CLIFF = {
  IN: () => "LOCAL_GLOBALS",
  SYNONYM: () => ["CLIFF", "CLIFFS"],
  ADJECTIVE: () => ["WHITE"],
  DESC: () => "white cliffs",
  FLAGS: { NDESCBIT: true, CLIMBBIT: true },
};

let WRENCH = {
  IN: () => "MAINTENANCE_ROOM",
  SYNONYM: () => ["WRENCH", "TOOL", "TOOLS"],
  DESC: () => "wrench",
  FLAGS: { TAKEBIT: true, TOOLBIT: true },
  SIZE: () => 10,
};

let CONTROL_PANEL = {
  IN: () => "DAM_ROOM",
  SYNONYM: () => ["PANEL"],
  ADJECTIVE: () => ["CONTROL"],
  DESC: () => "control panel",
  FLAGS: { NDESCBIT: true },
};

let NEST = {
  IN: () => "UP_A_TREE",
  SYNONYM: () => ["NEST"],
  ADJECTIVE: () => ["BIRDS"],
  DESC: () => "bird's nest",
  FLAGS: { TAKEBIT: true, BURNBIT: true, CONTBIT: true, OPENBIT: true, SEARCHBIT: true },
  FDESC: () => "Beside you on the branch is a small bird's nest.",
  CAPACITY: () => 20,
};

let EGG = {
  IN: () => "NEST",
  SYNONYM: () => ["EGG", "TREASURE"],
  ADJECTIVE: () => ["BIRDS", "ENCRUSTED", "JEWELED"],
  DESC: () => "jewel-encrusted egg",
  FLAGS: { TAKEBIT: true, CONTBIT: true, SEARCHBIT: true },
  VALUE: () => 5,
  TVALUE: () => 5,
  CAPACITY: () => 6,
  FDESC: () => "In the bird's nest is a large egg encrusted with precious jewels, apparently scavenged by a childless songbird. The egg is covered with fine gold inlay, and ornamented in lapis lazuli and mother-of-pearl. Unlike most eggs, this one is hinged and closed with a delicate looking clasp. The egg appears extremely fragile.",
};

let BROKEN_EGG = {
  SYNONYM: () => ["EGG", "TREASURE"],
  ADJECTIVE: () => ["BROKEN", "BIRDS", "ENCRUSTED", "JEWEL"],
  DESC: () => "broken jewel-encrusted egg",
  FLAGS: { TAKEBIT: true, CONTBIT: true, OPENBIT: true },
  CAPACITY: () => 6,
  TVALUE: () => 2,
  LDESC: () => "There is a somewhat ruined egg here.",
};

let BAUBLE = {
  SYNONYM: () => ["BAUBLE", "TREASURE"],
  ADJECTIVE: () => ["BRASS", "BEAUTI"],
  DESC: () => "beautiful brass bauble",
  FLAGS: { TAKEBIT: true },
  VALUE: () => 1,
  TVALUE: () => 1,
};

let CANARY = {
  IN: () => "EGG",
  SYNONYM: () => ["CANARY", "TREASURE"],
  ADJECTIVE: () => ["CLOCKWORK", "GOLD", "GOLDEN"],
  DESC: () => "golden clockwork canary",
  FLAGS: { TAKEBIT: true, SEARCHBIT: true },
  VALUE: () => 6,
  TVALUE: () => 4,
  FDESC: () => "There is a golden clockwork canary nestled in the egg. It has ruby eyes and a silver beak. Through a crystal window below its left wing you can see intricate machinery inside. It appears to have wound down.",
};

let BROKEN_CANARY = {
  IN: () => "BROKEN_EGG",
  SYNONYM: () => ["CANARY", "TREASURE"],
  ADJECTIVE: () => ["BROKEN", "CLOCKWORK", "GOLD", "GOLDEN"],
  DESC: () => "broken clockwork canary",
  FLAGS: { TAKEBIT: true },
  TVALUE: () => 1,
  FDESC: () => "There is a golden clockwork canary nestled in the egg. It seems to have recently had a bad experience. The mountings for its jewel-like eyes are empty, and its silver beak is crumpled. Through a cracked crystal window below its left wing you can see the remains of intricate machinery. It is not clear what result winding it would have, as the mainspring seems sprung.",
};

