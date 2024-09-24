export const rooms = {
  cabin: {
    isRoom: 'cabin',
    desc: { text: "You're inside a log cabin. It's rustic, but has a lovely fireplace.\n" },
    objects: {
      bedFrame: ['inst_9',],
      book: ['inst_4',],
      bucket: ['inst_10',],
      chair: ['inst_7','inst_8',],
      nails: ['inst_11',],
      table: ['inst_6',],
    },
    vars: {
      aboveGround: 1,
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
      detritus: ['inst_46',],
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
  longHall1: {
    isRoom: 'longHall1',
    desc: { text: "You're at the south end of a large hall.\n" },
    objects: {
      rock: ['inst_15','inst_16','inst_17','inst_18','inst_19','inst_20','inst_21','inst_22','inst_23','inst_24',],
    },
    vars: {
    },
    move: {
      NORTH: { room: 'longHall2' },
      UP: { room: 'caveEntrance1' },
    },
    hooks: {
    },
  },
  longHall2: {
    isRoom: 'longHall2',
    desc: { text: "You're still in a large hall.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'grotto2' },
      NORTH: { room: 'longHall3' },
      SOUTH: { room: 'longHall1' },
      WEST: { room: 'grotto1' },
    },
    hooks: {
    },
  },
  longHall3: {
    isRoom: 'longHall3',
    desc: { text: "You're at the north end of a large hall.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'corridor1' },
      SOUTH: { room: 'longHall2' },
      WEST: { room: 'grotto3' },
    },
    hooks: {
    },
  },
  grotto1: {
    isRoom: 'grotto1',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'longHall2' },
    },
    hooks: {
    },
  },
  grotto2: {
    isRoom: 'grotto2',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      WEST: { room: 'longHall2' },
    },
    hooks: {
    },
  },
  grotto3: {
    isRoom: 'grotto3',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'longHall3' },
    },
    hooks: {
    },
  },
  corridor1: {
    isRoom: 'corridor1',
    desc: { text: "You're in a long corridor.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'hole1' },
      NORTH: { room: 'corridor2' },
      WEST: { room: 'longHall3' },
    },
    hooks: {
    },
  },
  corridor2: {
    isRoom: 'corridor2',
    desc: { text: "You're in a long corridor.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'grotto5' },
      NORTH: { room: 'corridor3' },
      SOUTH: { room: 'corridor1' },
    },
    hooks: {
    },
  },
  corridor3: {
    isRoom: 'corridor3',
    desc: { text: "You're in a long corridor.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'longHall4' },
      NORTH: { room: 'corridor4' },
      SOUTH: { room: 'corridor2' },
      WEST: { room: 'hole2' },
    },
    hooks: {
    },
  },
  corridor4: {
    isRoom: 'corridor4',
    desc: { text: "You're in a long corridor.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      SOUTH: { room: 'corridor3' },
      WEST: { room: 'grotto7' },
    },
    hooks: {
    },
  },
  hole1: {
    isRoom: 'hole1',
    desc: { text: "You're in a small grotto. There's a massive hole in the ground.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      WEST: { room: 'corridor1' },
    },
    hooks: {
      exit: 'exitHoleRm',
    },
  },
  grotto5: {
    isRoom: 'grotto5',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      WEST: { room: 'corridor2' },
    },
    hooks: {
    },
  },
  hole2: {
    isRoom: 'hole2',
    desc: { text: "You're in a small grotto. There's a massive hole in the ground.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'corridor3' },
    },
    hooks: {
      exit: 'exitHoleRm',
    },
  },
  grotto7: {
    isRoom: 'grotto7',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'corridor4' },
      WEST: { room: 'subGrotto7' },
    },
    hooks: {
    },
  },
  subGrotto7: {
    isRoom: 'subGrotto7',
    desc: { text: "You're in a smaller grotto.\n" },
    objects: {
      obsidianShard: ['inst_45',],
    },
    vars: {
    },
    move: {
      EAST: { room: 'grotto7' },
    },
    hooks: {
    },
  },
  longHall4: {
    isRoom: 'longHall4',
    desc: { text: "You're in a large hall." },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'longHall5' },
      NORTH: { room: 'grotto10' },
      SOUTH: { room: 'grotto9' },
      WEST: { room: 'corridor3' },
    },
    hooks: {
    },
  },
  longHall5: {
    isRoom: 'longHall5',
    desc: { text: "You're in a large hall." },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'grotto12' },
      SOUTH: { room: 'hole3' },
      WEST: { room: 'longHall4' },
    },
    hooks: {
    },
  },
  grotto9: {
    isRoom: 'grotto9',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'longHall4' },
    },
    hooks: {
    },
  },
  grotto10: {
    isRoom: 'grotto10',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'mazeRest1' },
      SOUTH: { room: 'longHall4' },
    },
    hooks: {
    },
  },
  hole3: {
    isRoom: 'hole3',
    desc: { text: "You're in a small grotto. There's a massive hole in the ground.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'longHall5' },
    },
    hooks: {
      exit: 'exitHoleRm',
    },
  },
  grotto12: {
    isRoom: 'grotto12',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      SOUTH: { room: 'longHall5' },
    },
    hooks: {
    },
  },
  mazeRest1: {
    isRoom: 'mazeRest1',
    desc: { text: "You're in a lovely rocky cave.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'btnRm1' },
      SOUTH: { room: 'grotto10' },
      UP: { room: 'passage1' },
    },
    hooks: {
    },
  },
  btnRm1: {
    isRoom: 'btnRm1',
    desc: { routine: 'descBtnRm1' },
    objects: {
    },
    vars: {
      start: 0,
      step: 0,
    },
    move: {
      EAST: { routine: 'btnRm1East' },
      NORTH: { routine: 'btnRm1North' },
      SOUTH: { routine: 'btnRm1South' },
      WEST: { routine: 'btnRm1West' },
    },
    hooks: {
    },
  },
  mazeRest2: {
    isRoom: 'mazeRest2',
    desc: { text: "You're in a lovely rocky cave.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'btnRm2' },
      SOUTH: { room: 'btnRm1' },
    },
    hooks: {
    },
  },
  btnRm2: {
    isRoom: 'btnRm2',
    desc: { routine: 'descBtnRm2' },
    objects: {
    },
    vars: {
      start: 0,
      step: 0,
    },
    move: {
      EAST: { routine: 'btnRm2East' },
      SOUTH: { routine: 'btnRm2South' },
      WEST: { routine: 'btnRm2West' },
    },
    hooks: {
    },
  },
  passage1: {
    isRoom: 'passage1',
    desc: { text: "A passageway in the rock\n" },
    objects: {
    },
    vars: {
    },
    move: {
      DOWN: { room: 'mazeRest1' },
      EAST: { room: 'cavern1' },
      UP: { room: 'caveEntrance2' },
      WEST: { room: 'den1' },
    },
    hooks: {
    },
  },
  cavern1: {
    isRoom: 'cavern1',
    desc: { text: "A massive cavern\n" },
    objects: {
      bones: ['inst_39',],
      pickAxe: ['inst_13',],
      rock: ['inst_25','inst_26',],
      stoneDoor: ['inst_42',],
      sword: ['inst_14',],
      wire: ['inst_12',],
    },
    vars: {
    },
    move: {
      NORTH: { routine: 'cavern1North' },
      SOUTH: { room: 'caveLake' },
      WEST: { room: 'passage1' },
    },
    hooks: {
    },
  },
  crypt: {
    isRoom: 'crypt',
    desc: { text: "a crypt\n" },
    objects: {
      coffin: ['inst_43',],
      rock: ['inst_32',],
    },
    vars: {
    },
    move: {
      SOUTH: { room: 'cavern1' },
    },
    hooks: {
    },
  },
  caveLake: {
    isRoom: 'caveLake',
    desc: { text: "an underground lake" },
    objects: {
      bones: ['inst_40',],
      rock: ['inst_29','inst_30','inst_31',],
    },
    vars: {
    },
    move: {
      NORTH: { room: 'cavern1' },
    },
    hooks: {
    },
  },
  den1: {
    isRoom: 'den1',
    desc: { text: "the den of a monster" },
    objects: {
      bones: ['inst_34','inst_35','inst_36',],
    },
    vars: {
    },
    move: {
      EAST: { room: 'passage1' },
      NORTH: { room: 'den2' },
      SOUTH: { room: 'den4' },
    },
    hooks: {
    },
  },
  den2: {
    isRoom: 'den2',
    desc: { text: "the den of a monster" },
    objects: {
      bones: ['inst_37',],
      rock: ['inst_27',],
    },
    vars: {
    },
    move: {
      EAST: { room: 'den1' },
      WEST: { room: 'den3' },
    },
    hooks: {
    },
  },
  den3: {
    isRoom: 'den3',
    desc: { text: "the den of a monster" },
    objects: {
      bones: ['inst_38',],
      rock: ['inst_28',],
    },
    vars: {
    },
    move: {
      NORTH: { room: 'den2' },
      SOUTH: { room: 'den4' },
    },
    hooks: {
    },
  },
  den4: {
    isRoom: 'den4',
    desc: { text: "the den of a monster" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'den1' },
      WEST: { room: 'den3' },
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
      riverStone: ['inst_48',],
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
    desc: { text: "Amidst boulders and tree trunks a black space beckons you forward. A cave. You could GO DOWN, but there is no telling whether you would come back up. Some sort of light source might help.\n" },
    objects: {
      rock: ['inst_33',],
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      DOWN: { room: 'longHall1' },
      EAST: { routine: 'caveEntrance1East' },
      NORTH: { routine: 'caveEntrance1North' },
      SOUTH: { routine: 'caveEntrance1South' },
      WEST: { routine: 'caveEntrance1West' },
    },
    hooks: {
    },
  },
  caveEntrance2: {
    isRoom: 'caveEntrance2',
    desc: { text: "Out of the forest appears a large, rocky hole in the ground. A cave. You could GO DOWN. Some sort of light source might help you navigate." },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      DOWN: { room: 'passage1' },
      EAST: { routine: 'caveEntrance2East' },
      NORTH: { text: "There's nothing but dense forest" },
      SOUTH: { routine: 'caveEntrance2South' },
      WEST: { routine: 'caveEntrance2West' },
    },
    hooks: {
    },
  },
  storage: {
    isRoom: 'storage',
    objects: {
      sap: ['inst_47',],
    },
    vars: {
    },
    move: {
    },
    hooks: {
    },
  },
};
