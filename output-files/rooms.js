export const rooms = {
  cabin: {
    isRoom: 'cabin',
    desc: { routine: 'descCabin' },
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
      enter: 'cabinEnter',
    },
  },
  cabinExterior: {
    isRoom: 'cabinExterior',
    desc: { routine: 'descCabinExterior' },
    objects: {
      cabinDoor: ['inst_1',],
      cabinWindow: ['inst_2',],
      detritus: ['inst_41',],
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
      enter: 'cabinExteriorEnter',
    },
  },
  longHall1: {
    isRoom: 'longHall1',
    desc: { text: "You're at the south end of a large hall.\n" },
    objects: {
      rock: ['inst_17','inst_18','inst_19','inst_20',],
    },
    vars: {
    },
    move: {
      NORTH: { room: 'longHall2' },
      UP: { room: 'caveEntrance1' },
    },
    hooks: {
      enter: 'longHall1Enter',
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
    desc: { text: "You're in a small grotto. There's a massive hole in the ground.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      WEST: { room: 'longHall2' },
    },
    hooks: {
      enter: 'enterHoleRm',
    },
  },
  grotto3: {
    isRoom: 'grotto3',
    desc: { text: "You're in a small grotto. There's a massive hole in the ground.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'longHall3' },
    },
    hooks: {
      enter: 'enterHoleRm',
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
      EAST: { room: 'grotto5' },
      NORTH: { room: 'corridor2' },
      SOUTH: { room: 'grotto4' },
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
      EAST: { room: 'grotto6' },
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
      WEST: { room: 'grotto7' },
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
      NORTH: { room: 'grotto9' },
      SOUTH: { room: 'corridor3' },
      WEST: { room: 'grotto8' },
    },
    hooks: {
    },
  },
  grotto4: {
    isRoom: 'grotto4',
    desc: { text: "You're in a small grotto. There's a massive hole in the ground.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'corridor1' },
    },
    hooks: {
      enter: 'enterHoleRm',
    },
  },
  grotto5: {
    isRoom: 'grotto5',
    desc: { text: "You're in a small grotto. There's a massive hole in the ground.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      WEST: { room: 'corridor1' },
    },
    hooks: {
      enter: 'enterHoleRm',
    },
  },
  grotto6: {
    isRoom: 'grotto6',
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
  grotto7: {
    isRoom: 'grotto7',
    desc: { text: "You're in a small grotto. There's a massive hole in the ground.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      WEST: { room: 'corridor2' },
    },
    hooks: {
      enter: 'enterHoleRm',
    },
  },
  grotto8: {
    isRoom: 'grotto8',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'corridor4' },
      WEST: { room: 'subGrotto8' },
    },
    hooks: {
    },
  },
  subGrotto8: {
    isRoom: 'subGrotto8',
    desc: { text: "You're in a smaller grotto.\n" },
    objects: {
      obsidianShard: ['inst_38',],
    },
    vars: {
    },
    move: {
      EAST: { room: 'grotto7' },
    },
    hooks: {
    },
  },
  grotto9: {
    isRoom: 'grotto9',
    desc: { text: "You're in a small grotto. There's a massive hole in the ground.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      SOUTH: { room: 'corridor4' },
    },
    hooks: {
      enter: 'enterHoleRm',
    },
  },
  longHall4: {
    isRoom: 'longHall4',
    desc: { text: "You're in a large hall.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'longHall5' },
      NORTH: { room: 'grotto10' },
      SOUTH: { room: 'grotto11' },
      WEST: { room: 'longHall5' },
    },
    hooks: {
    },
  },
  longHall5: {
    isRoom: 'longHall5',
    desc: { text: "You're in a large hall.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'longHall4' },
      NORTH: { room: 'grotto12' },
      SOUTH: { room: 'grotto13' },
      WEST: { room: 'longHall4' },
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
      SOUTH: { room: 'longHall4' },
    },
    hooks: {
    },
  },
  grotto11: {
    isRoom: 'grotto11',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'longHall4' },
      WEST: { room: 'corridor3' },
    },
    hooks: {
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
      NORTH: { room: 'mazeRest1' },
      SOUTH: { room: 'longHall5' },
    },
    hooks: {
    },
  },
  grotto13: {
    isRoom: 'grotto13',
    desc: { text: "You're in a small grotto.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'longHall5' },
    },
    hooks: {
    },
  },
  mazeRest1: {
    isRoom: 'mazeRest1',
    desc: { routine: 'descMazeRest1' },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'btnRm1' },
      SOUTH: { room: 'grotto12' },
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
    desc: { text: "You're in a lovely cave, with a huge pile of rocks.\n" },
    objects: {
      rockPile: ['inst_12',],
    },
    vars: {
    },
    move: {
      NORTH: { room: 'mazeGridPink' },
      SOUTH: { room: 'btnRm1' },
    },
    hooks: {
    },
  },
  mazeGridRed: {
    isRoom: 'mazeGridRed',
    desc: { text: "a red room\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'mazeGridBlue' },
      NORTH: { room: 'mazeGridBlue' },
      SOUTH: { room: 'mazeGridBlue' },
      WEST: { room: 'mazeGridBlue' },
    },
    hooks: {
    },
  },
  mazeGridBlue: {
    isRoom: 'mazeGridBlue',
    desc: { text: "a blue room\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'mazeGridRed' },
      NORTH: { room: 'mazeGridRed' },
      SOUTH: { room: 'mazeGridPink' },
      WEST: { room: 'mazeGridRed' },
    },
    hooks: {
    },
  },
  mazeGridPink: {
    isRoom: 'mazeGridPink',
    desc: { text: "a red room\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'mazeGridBlue' },
      NORTH: { room: 'mazeGridBlue' },
      SOUTH: { room: 'mazeGridBlue' },
      WEST: { room: 'mazeGridTeal' },
    },
    hooks: {
    },
  },
  mazeGridTeal: {
    isRoom: 'mazeGridTeal',
    desc: { text: "a blue room\n" },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { room: 'mazeGridRed' },
      NORTH: { room: 'mazeRest3' },
      SOUTH: { room: 'mazeGridRed' },
      WEST: { room: 'mazeGridRed' },
    },
    hooks: {
    },
  },
  mazeRest3: {
    isRoom: 'mazeRest3',
    desc: { text: "You're in a lovely rocky cave.\n" },
    objects: {
    },
    vars: {
    },
    move: {
      NORTH: { room: 'btnRm2' },
      SOUTH: { room: 'mazeRest2' },
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
  den1: {
    isRoom: 'den1',
    desc: { text: "You're in a monster's den\n" },
    objects: {
      bones: ['inst_27','inst_28','inst_29',],
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
    desc: { text: "You're in a monster's den\n" },
    objects: {
      bones: ['inst_30',],
      rock: ['inst_22',],
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
    desc: { text: "You're in a monster's den\n" },
    objects: {
      bones: ['inst_31',],
      rock: ['inst_23',],
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
    desc: { text: "You're in a monster's den\n" },
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
  passage1: {
    isRoom: 'passage1',
    desc: { text: "You're in a series of tunnels.\n" },
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
      enter: 'passage1Enter',
    },
  },
  cavern1: {
    isRoom: 'cavern1',
    desc: { routine: 'descCavern1' },
    objects: {
      bones: ['inst_32',],
      pickAxe: ['inst_15',],
      rock: ['inst_21',],
      stoneDoor: ['inst_35',],
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
    desc: { routine: 'descCrypt' },
    objects: {
      coffin: ['inst_36',],
      rock: ['inst_25',],
      sword: ['inst_16',],
    },
    vars: {
    },
    move: {
      EAST: { room: 'waterfallPassage' },
      SOUTH: { routine: 'cryptSouth' },
    },
    hooks: {
    },
  },
  waterfallPassage: {
    isRoom: 'waterfallPassage',
    desc: { routine: 'descWaterfallPassage' },
    objects: {
    },
    vars: {
    },
    move: {
      EAST: { routine: 'waterfallPassageUp' },
      UP: { routine: 'waterfallPassageUp' },
      WEST: { room: 'crypt' },
    },
    hooks: {
      enter: 'waterfallPassageEnter',
    },
  },
  caveLake: {
    isRoom: 'caveLake',
    desc: { routine: 'descCaveLake' },
    objects: {
      bones: ['inst_33',],
      rock: ['inst_24',],
      water: ['inst_67',],
    },
    vars: {
    },
    move: {
      NORTH: { room: 'cavern1' },
    },
    hooks: {
      always: 'caveLakeAlways',
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
      enter: 'forest1Enter',
    },
  },
  forest2: {
    isRoom: 'forest2',
    desc: { routine: 'descForest2' },
    objects: {
      stick: ['inst_44',],
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      NORTH: { room: 'cabinExterior' },
      SOUTH: { room: 'forest1' },
    },
    hooks: {
      enter: 'forest2Enter',
    },
  },
  forest3: {
    isRoom: 'forest3',
    desc: { routine: 'descForest3' },
    objects: {
      fern: ['inst_52',],
      riverStone: ['inst_53',],
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'cabinExterior' },
      NORTH: { room: 'forest4' },
      SOUTH: { room: 'forest5' },
      WEST: { room: 'lake1' },
    },
    hooks: {
      enter: 'forest3Enter',
    },
  },
  forest4: {
    isRoom: 'forest4',
    desc: { routine: 'descForest4' },
    objects: {
      stick: ['inst_45',],
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'field2' },
      NORTH: { room: 'caveEntrance2' },
      SOUTH: { room: 'forest3' },
      WEST: { room: 'forest6' },
    },
    hooks: {
      enter: 'forest4Enter',
    },
  },
  forest5: {
    isRoom: 'forest5',
    desc: { routine: 'descForest5' },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'forest2' },
      NORTH: { room: 'lake1' },
      SOUTH: { room: 'caveEntrance1' },
      WEST: { text: "Think again" },
    },
    hooks: {
      enter: 'forest5Enter',
      exit: 'forest5Exit',
    },
  },
  forest6: {
    isRoom: 'forest6',
    desc: { routine: 'descForest6' },
    objects: {
      nuts: ['inst_51',],
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
      enter: 'forest6Enter',
      exit: 'forest6Exit',
    },
  },
  lake1: {
    isRoom: 'lake1',
    desc: { routine: 'descLake1' },
    objects: {
      boatFrame: ['inst_40',],
      water: ['inst_66',],
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      EAST: { room: 'forest3' },
      NORTH: { room: 'forest6' },
      SOUTH: { room: 'forest5' },
      WEST: { routine: 'lake1West' },
    },
    hooks: {
      enter: 'lake1Enter',
      always: 'lake1Always',
    },
  },
  field1: {
    isRoom: 'field1',
    desc: { routine: 'descField1' },
    objects: {
      herbs: ['inst_49',],
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
      enter: 'field1Enter',
    },
  },
  field2: {
    isRoom: 'field2',
    desc: { routine: 'descField2' },
    objects: {
      berries: ['inst_48',],
      herbs: ['inst_50',],
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
      enter: 'field2Enter',
    },
  },
  cliff1: {
    isRoom: 'cliff1',
    desc: { routine: 'descCliff1' },
    objects: {
    },
    vars: {
      aboveGround: 1,
    },
    move: {
      DOWN: { routine: 'cliff1Down' },
      EAST: { routine: 'cliff1East' },
      SOUTH: { room: 'field1' },
      WEST: { room: 'field1' },
    },
    hooks: {
      enter: 'cliff1Enter',
    },
  },
  caveEntrance1: {
    isRoom: 'caveEntrance1',
    desc: { routine: 'descCaveEntrance1' },
    objects: {
      detritus: ['inst_42',],
      rabbit: ['inst_57',],
      rock: ['inst_26',],
      stick: ['inst_46',],
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
    desc: { routine: 'descCaveEntrance2' },
    objects: {
      detritus: ['inst_43',],
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
  qStorage: {
    isRoom: 'qStorage',
    objects: {
    },
    vars: {
    },
    move: {
    },
    hooks: {
    },
  },
  storage: {
    isRoom: 'storage',
    objects: {
      bear: ['inst_58','inst_59',],
      childMonster: ['inst_13',],
      parentMonster: ['inst_14',],
      sap: ['inst_47',],
      treeHollow: ['inst_54',],
    },
    vars: {
    },
    move: {
    },
    hooks: {
    },
  },
};
