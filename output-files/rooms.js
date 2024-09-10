export const rooms = {
  forest1: {
    isRoom: 'forest1',
    desc: { text: "You're in a forest. The smell of pine hangs thick in the air." },
    objects: [
    ],
    vars: {
    },
    move: {
      EAST: { room: 'forest2' },
      UP: { text: "There is no side-to-side" },
    },
    hooks: {
    },
  },
  forest2: {
    isRoom: 'forest2',
    desc: { text: "You're in a forest. A dark sky shows overhead between gaps in the branches, far above you." },
    objects: [
      'tree',
      'monster',
    ],
    vars: {
    },
    move: {
      UP: { text: "There is no up there is no down" },
      WEST: { room: 'forest1' },
    },
    hooks: {
    },
  },
};
