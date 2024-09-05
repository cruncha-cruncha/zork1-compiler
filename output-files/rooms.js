export const rooms = {
  southOfHouse: {
    isRoom: "southOfHouse",
    desc: "You're in a field, south of a house",
    move: {
      NORTH: { room: "house" },
    },
  },
  house: {
    isRoom: "house",
    desc: "You're in a house",
    move: {
      SOUTH: { room: "southOfHouse" },
    },
  },
};
