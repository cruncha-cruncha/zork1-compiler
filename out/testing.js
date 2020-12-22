let RESERVOIR_SOUTH = {
  IN: () => "ROOMS",
  DESC: () => "Reservoir South",
  ACTION: () => "RESERVOIR_SOUTH_FCN",
  FLAGS: { RLANDBIT: true },
  GLOBAL: { GLOBAL_WATER: true },
  PSEUDO: () => { return { LAKE: "LAKE_PSEUDO", CHASM: "CHASM_PSEUDO" }},
};

