export const rooms = {
  cabin: {
    isRoom: 'cabin',
    desc: { text: "You're inside a log cabin. It's rustic, but has a lovely fireplace.\n" },
    objects: {
      bedFrame: ['inst_9',],
      book: ['inst_4',],
      bucket: ['inst_11',],
      chair: ['inst_7','inst_8',],
      firePlace: ['inst_10',],
      nails: ['inst_12',],
      table: ['inst_6',],
    },
    vars: {
      aboveGround: 1,
      isLocked: 1,
    },
    move: {
      EAST: { room: 'cabinExterior' },
      NORTH: { room: 'cabinExterior' },
      OUT: { room: 'cabinExterior' },
      SOUTH: { room: 'cabinExterior' },
      WEST: { room: 'cabinExterior' },
    },
    hooks: {
    },
  },
  cabinExterior: {
    isRoom: 'cabinExterior',
    desc: { routine: 'descCabinExterior' },
    objects: {
      cabinDoor: ['inst_1',],
      cabinWindow: ['inst_2',],
      detritus: ['inst_13',],
    },
    vars: {
      aboveGround: 1,
      firstTime: 1,
    },
    move: {
      EAST: { room: 'field1' },
      NORTH: { room: 'field2' },
      SOUTH: { room: 'forest2' },
      WEST: { room: 'forest3' },
    },
    hooks: {
    },
  },
  storage: {
    isRoom: 'storage',
    objects: {
    },
    vars: {
    },
    move: {
    },
    hooks: {
    },
  },
  forest1: {
    isRoom: 'forest1',
    desc: { routine: 'descForest1' },
    objects: {
    },
    vars: {
      aboveGround: 1,
      firstTime: 1,
    },
    move: {
      EAST: { room: 'forest2' },
      NORTH: { room: 'forest2' },
      WEST: { room: 'caveEntrance1' },
    },
    hooks: {
    },
  },
  forest2: {
    isRoom: 'forest2',
    desc: { text: "You're in a forest, the trees are thinner here. There's a trail heading NORTH (and back SOUTH).\n" },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      NORTH: { room: 'cabinExterior' },
      SOUTH: { room: 'forest1' },
    },
    hooks: {
    },
  },
  forest3: {
    isRoom: 'forest3',
    desc: { text: "You're in a lightly-populated forest, and a small stream burbles nearby.\n" },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'cabinExterior' },
      NORTH: { room: 'forest4' },
      SOUTH: { room: 'forest2' },
      WEST: { room: 'forest5' },
    },
    hooks: {
    },
  },
  forest4: {
    isRoom: 'forest4',
    desc: { text: "The forest is thicker again here, but the trail still looks good, and the you can hear a stream.\n" },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'field2' },
      NORTH: { room: 'caveEntrance2' },
      SOUTH: { room: 'forest3' },
      WEST: { room: 'forest5' },
    },
    hooks: {
    },
  },
  forest5: {
    isRoom: 'forest5',
    desc: { text: "You're in the forest. The trail is faint here, and barely-worn.\n" },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'caveEntrance1' },
      NORTH: { room: 'lake1' },
      SOUTH: { room: 'caveEntrance1' },
      WEST: { text: "Think again" },
    },
    hooks: {
    },
  },
  forest6: {
    isRoom: 'forest6',
    desc: { text: "This is deep forest, but there is trail running SOUTH and EAST\n" },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'caveEntrance2' },
      NORTH: { text: "The forest gets thicker, closing in authoritatively. No chance of getting through." },
      SOUTH: { room: 'lake1' },
      WEST: { text: "Not a chance." },
    },
    hooks: {
    },
  },
  lake1: {
    isRoom: 'lake1',
    desc: { text: "You find yourself at the edge a large, calm lake\n" },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'forest4' },
      NORTH: { room: 'forest6' },
      SOUTH: { room: 'forest5' },
      WEST: { room: 'forest5' },
    },
    hooks: {
    },
  },
  field1: {
    isRoom: 'field1',
    desc: { text: "A massive field stretches out all around you, with wild grass and sage." },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'cliff1' },
      NORTH: { room: 'field2' },
      SOUTH: { room: 'forest1' },
      WEST: { room: 'cabinExterior' },
    },
    hooks: {
    },
  },
  field2: {
    isRoom: 'field2',
    desc: { text: "This field seems to go on forever, gently rolling past the horizon.\n" },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'cliff1' },
      NORTH: { text: "Nothing but endless field lies north. Best kept for another time." },
      SOUTH: { room: 'cabinExterior' },
      WEST: { room: 'forest4' },
    },
    hooks: {
    },
  },
  cliff1: {
    isRoom: 'cliff1',
    desc: { text: "Grassy fields abruptly stop at the edge of a cliff, a stream hurtling over. You can taste more adventure awaiting in the land below." },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { text: "You're not yet prepared for climbing down this sheer drop." },
      NORTH: { room: 'field2' },
      SOUTH: { room: 'field1' },
      WEST: { room: 'field1' },
    },
    hooks: {
    },
  },
  caveEntrance1: {
    isRoom: 'caveEntrance1',
    desc: { text: "Amidst boulders and tree trunks a black space beckons you forward. A cave. You could GO DOWN, but there is no telling whether you would come back up." },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'forest1' },
      NORTH: { room: 'forest5' },
      SOUTH: { room: 'forest1' },
      WEST: { room: 'forest5' },
    },
    hooks: {
    },
  },
  caveEntrance2: {
    isRoom: 'caveEntrance2',
    desc: { text: "Out of the forest appears a large, rocky hole in the ground. A cave. You could GO DOWN." },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'forest4' },
      NORTH: { text: "There's nothing but dense forest" },
      SOUTH: { room: 'forest4' },
      WEST: { room: 'forest6' },
    },
    hooks: {
    },
  },
};
