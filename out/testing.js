let MOUNTAINS = {
IN: "ROOMS",
LDESC: "The forest thins out, revealing impassable mountains.",
DESC: "Forest",
UP_TO: () => 
TELL("The mountains are impassable.")
,
NORTH_TO: () => 
"FOREST_2",
EAST_TO: () => 
TELL("The mountains are impassable.")
,
SOUTH_TO: () => 
"FOREST_2",
WEST_TO: () => 
"FOREST_2",
FLAGS: { RLANDBIT: true, ONBIT: true, SACREDBIT: true },
GLOBAL: ["TREE", "WHITE_HOUSE"],
};
