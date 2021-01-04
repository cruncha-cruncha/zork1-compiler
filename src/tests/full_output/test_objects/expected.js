let INFLATED_BOAT = {
  SYNONYM: () => ["BOAT", "RAFT"],
  ADJECTIVE: () => ["INFLAT", "MAGIC", "PLASTIC", "SEAWORTHY"],
  DESC: () => "magic boat",
  FLAGS: { TAKEBIT: true, BURNBIT: true, VEHBIT: true, OPENBIT: true, SEARCHBIT: true },
  ACTION: () => RBOAT_FUNCTION,
  CAPACITY: () => 100,
  SIZE: () => 20,
  VTYPE: { NONLANDBIT: true },
};

let BAT = {
  IN: BAT_ROOM,
  SYNONYM: () => ["BAT", "VAMPIRE"],
  ADJECTIVE: () => ["VAMPIRE", "DERANGED"],
  DESC: () => "bat",
  FLAGS: { ACTORBIT: true, TRYTAKEBIT: true },
  DESCFCN: () => "BAT_D",
  ACTION: () => BAT_F,
};

let WOODEN_DOOR = {
  IN: LIVING_ROOM,
  SYNONYM: () => ["DOOR", "LETTERING", "WRITING"],
  ADJECTIVE: () => ["WOODEN", "GOTHIC", "STRANGE", "WEST"],
  DESC: () => "wooden door",
  FLAGS: { READBIT: true, DOORBIT: true, NDESCBIT: true, TRANSBIT: true },
  ACTION: () => FRONT_DOOR_FCN,
  TEXT: () => "The engravings translate to \"This space intentionally left blank.\"",
};

let THIEF = {
  IN: ROUND_ROOM,
  SYNONYM: () => ["THIEF", "ROBBER", "MAN", "PERSON"],
  ADJECTIVE: () => ["SHADY", "SUSPICIOUS", "SEEDY"],
  DESC: () => "thief",
  FLAGS: { ACTORBIT: true, INVISIBLE: true, CONTBIT: true, OPENBIT: true, TRYTAKEBIT: true },
  ACTION: () => ROBBER_FUNCTION,
  LDESC: () => "There is a suspicious-looking individual, holding a large bag, leaning\nagainst one wall. He is armed with a deadly stiletto.",
  STRENGTH: 5,
};

let TORCH = {
  IN: PEDESTAL,
  SYNONYM: () => ["TORCH", "IVORY", "TREASURE"],
  ADJECTIVE: () => ["FLAMING", "IVORY"],
  DESC: () => "torch",
  FLAGS: { TAKEBIT: true, FLAMEBIT: true, ONBIT: true, LIGHTBIT: true },
  ACTION: () => TORCH_OBJECT,
  FDESC: () => "Sitting on the pedestal is a flaming torch, made of ivory.",
  SIZE: () => 20,
  VALUE: () => 14,
  TVALUE: () => 6,
};

