export const rooms = {
  southOfHouse: {
    in: "rooms",
    desc: "South of House",
    ldesc: "You are facing the south side of a white house. There is no door here, and all the windows are boarded.",
    flags: ["rlandbit", "sacredbit", "onbit"],
    globals: ["whiteHouse", "board", "boardedWindow", "forest"],
  },
  westOfHouse: {
    in: "rooms",
    action: "westHouse",
    desc: "West of House",
    flags: ["sacredbit", "rlandbit", "onbit"],
    globals: ["forest", "whiteHouse", "board"],
  },
  stoneBarrow: {
    in: "rooms",
    action: "stoneBarrowFcn",
    desc: "Stone Barrow",
    ldesc: "You are standing in front of a massive barrow of stone. In the east face is a huge stone door which is open. You cannot see into the dark of the tomb.",
    flags: ["rlandbit", "onbit", "sacredbit"],
  },
  northOfHouse: {
    in: "rooms",
    desc: "North of House",
    ldesc: "You are facing the north side of a white house. There is no door here, and all the windows are boarded up. To the north a narrow path winds through the trees.",
    flags: ["onbit", "sacredbit", "rlandbit"],
    globals: ["board", "whiteHouse", "boardedWindow", "forest"],
  },
};
