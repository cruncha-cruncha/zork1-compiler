export const objects = {
  cabinDoor: {
    isObject: 'cabinDoor',
    desc: { text: "the cabin DOOR" },
    copies: {
      inst_1: {
        isObject: 'cabinDoor',
        isInst: 'inst_1',
        loc: { scope: 'room', name: 'cabinExterior' },
        vars: {
          hasBoards: 3,
          health: 8,
          isLocked: 1,
          noTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      hasBoards: 3,
      health: 8,
      isLocked: 1,
      noTake: 1,
    },
    hooks: {
    },
  },
  cabinWindow: {
    isObject: 'cabinWindow',
    desc: { routine: 'descCabinWindow' },
    copies: {
      inst_2: {
        isObject: 'cabinWindow',
        isInst: 'inst_2',
        loc: { scope: 'room', name: 'cabinExterior' },
        vars: {
          isBroken: 0,
          noTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      isBroken: 0,
      noTake: 1,
    },
    hooks: {
    },
  },
  cabinDoorKey: {
    isObject: 'cabinDoorKey',
    desc: { text: "a KEY" },
    copies: {
      inst_3: {
        isObject: 'cabinDoorKey',
        isInst: 'inst_3',
        loc: { scope: 'object', name: 'detritus', inst: 'inst_46' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  book: {
    isObject: 'book',
    desc: { routine: 'descBook' },
    copies: {
      inst_4: {
        isObject: 'book',
        isInst: 'inst_4',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
          health: 2,
          tinder: 1,
        },
        objects: {
          bookPage: ['inst_5',],
        },
      },
    },
    vars: {
      health: 2,
      tinder: 1,
    },
    hooks: {
    },
  },
  bookPage: {
    isObject: 'bookPage',
    desc: { text: "some PAPER" },
    copies: {
      inst_5: {
        isObject: 'bookPage',
        isInst: 'inst_5',
        loc: { scope: 'object', name: 'book', inst: 'inst_4' },
        vars: {
          tinder: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      tinder: 1,
    },
    hooks: {
    },
  },
  note: {
    isObject: 'note',
    desc: { text: "a NOTE" },
    copies: {
    },
    vars: {
      tinder: 1,
    },
    hooks: {
    },
  },
  table: {
    isObject: 'table',
    desc: { text: "a wooden TABLE" },
    copies: {
      inst_6: {
        isObject: 'table',
        isInst: 'inst_6',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
          hasBoards: 2,
          health: 4,
        },
        objects: {
        },
      },
    },
    vars: {
      hasBoards: 2,
      health: 4,
    },
    hooks: {
    },
  },
  chair: {
    isObject: 'chair',
    desc: { text: "a wooden CHAIR" },
    copies: {
      inst_7: {
        isObject: 'chair',
        isInst: 'inst_7',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
          hasBoards: 1,
          health: 6,
        },
        objects: {
        },
      },
      inst_8: {
        isObject: 'chair',
        isInst: 'inst_8',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
          hasBoards: 1,
          health: 6,
        },
        objects: {
        },
      },
    },
    vars: {
      hasBoards: 1,
      health: 6,
    },
    hooks: {
    },
  },
  bedFrame: {
    isObject: 'bedFrame',
    desc: { text: "a wooden BED frame" },
    copies: {
      inst_9: {
        isObject: 'bedFrame',
        isInst: 'inst_9',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  bucket: {
    isObject: 'bucket',
    desc: { text: "a BUCKET" },
    copies: {
      inst_10: {
        isObject: 'bucket',
        isInst: 'inst_10',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  nails: {
    isObject: 'nails',
    desc: { text: "some NAILS" },
    copies: {
      inst_11: {
        isObject: 'nails',
        isInst: 'inst_11',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  glassShard: {
    isObject: 'glassShard',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  soup: {
    isObject: 'soup',
    desc: { text: "SOUP" },
    copies: {
    },
    vars: {
      edible: 1,
    },
    hooks: {
    },
  },
  tea: {
    isObject: 'tea',
    desc: { text: "TEA" },
    copies: {
    },
    vars: {
      edible: 1,
    },
    hooks: {
    },
  },
  fire: {
    isObject: 'fire',
    desc: { text: "a FIRE" },
    copies: {
    },
    vars: {
      fuel: 2,
      noTake: 1,
    },
    hooks: {
    },
  },
  charcoal: {
    isObject: 'charcoal',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  magicRing: {
    isObject: 'magicRing',
    desc: { routine: 'descMagicRing' },
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  caveSpider: {
    isObject: 'caveSpider',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  monsterTooth: {
    isObject: 'monsterTooth',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  monster: {
    isObject: 'monster',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  wire: {
    isObject: 'wire',
    desc: { text: "some WIRE" },
    copies: {
      inst_12: {
        isObject: 'wire',
        isInst: 'inst_12',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  pickAxe: {
    isObject: 'pickAxe',
    desc: { text: "a PICK-AXE" },
    copies: {
      inst_13: {
        isObject: 'pickAxe',
        isInst: 'inst_13',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  sword: {
    isObject: 'sword',
    desc: { text: "a SWORD" },
    copies: {
      inst_14: {
        isObject: 'sword',
        isInst: 'inst_14',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  rock: {
    isObject: 'rock',
    desc: { text: "a ROCK" },
    copies: {
      inst_15: {
        isObject: 'rock',
        isInst: 'inst_15',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
        },
        objects: {
        },
      },
      inst_16: {
        isObject: 'rock',
        isInst: 'inst_16',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
        },
        objects: {
        },
      },
      inst_17: {
        isObject: 'rock',
        isInst: 'inst_17',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
        },
        objects: {
        },
      },
      inst_18: {
        isObject: 'rock',
        isInst: 'inst_18',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
        },
        objects: {
        },
      },
      inst_19: {
        isObject: 'rock',
        isInst: 'inst_19',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
        },
        objects: {
        },
      },
      inst_20: {
        isObject: 'rock',
        isInst: 'inst_20',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
        },
        objects: {
        },
      },
      inst_21: {
        isObject: 'rock',
        isInst: 'inst_21',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
        },
        objects: {
        },
      },
      inst_22: {
        isObject: 'rock',
        isInst: 'inst_22',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
        },
        objects: {
        },
      },
      inst_23: {
        isObject: 'rock',
        isInst: 'inst_23',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
        },
        objects: {
        },
      },
      inst_24: {
        isObject: 'rock',
        isInst: 'inst_24',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
        },
        objects: {
        },
      },
      inst_25: {
        isObject: 'rock',
        isInst: 'inst_25',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
        },
        objects: {
        },
      },
      inst_26: {
        isObject: 'rock',
        isInst: 'inst_26',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
        },
        objects: {
        },
      },
      inst_27: {
        isObject: 'rock',
        isInst: 'inst_27',
        loc: { scope: 'room', name: 'den2' },
        vars: {
        },
        objects: {
        },
      },
      inst_28: {
        isObject: 'rock',
        isInst: 'inst_28',
        loc: { scope: 'room', name: 'den3' },
        vars: {
        },
        objects: {
        },
      },
      inst_29: {
        isObject: 'rock',
        isInst: 'inst_29',
        loc: { scope: 'room', name: 'caveLake' },
        vars: {
        },
        objects: {
        },
      },
      inst_30: {
        isObject: 'rock',
        isInst: 'inst_30',
        loc: { scope: 'room', name: 'caveLake' },
        vars: {
        },
        objects: {
        },
      },
      inst_31: {
        isObject: 'rock',
        isInst: 'inst_31',
        loc: { scope: 'room', name: 'caveLake' },
        vars: {
        },
        objects: {
        },
      },
      inst_32: {
        isObject: 'rock',
        isInst: 'inst_32',
        loc: { scope: 'room', name: 'crypt' },
        vars: {
        },
        objects: {
        },
      },
      inst_33: {
        isObject: 'rock',
        isInst: 'inst_33',
        loc: { scope: 'room', name: 'caveEntrance1' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  bones: {
    isObject: 'bones',
    desc: { text: "some BONES" },
    copies: {
      inst_34: {
        isObject: 'bones',
        isInst: 'inst_34',
        loc: { scope: 'room', name: 'den1' },
        vars: {
          edible: 1,
        },
        objects: {
        },
      },
      inst_35: {
        isObject: 'bones',
        isInst: 'inst_35',
        loc: { scope: 'room', name: 'den1' },
        vars: {
          edible: 1,
        },
        objects: {
        },
      },
      inst_36: {
        isObject: 'bones',
        isInst: 'inst_36',
        loc: { scope: 'room', name: 'den1' },
        vars: {
          edible: 1,
        },
        objects: {
        },
      },
      inst_37: {
        isObject: 'bones',
        isInst: 'inst_37',
        loc: { scope: 'room', name: 'den2' },
        vars: {
          edible: 1,
        },
        objects: {
        },
      },
      inst_38: {
        isObject: 'bones',
        isInst: 'inst_38',
        loc: { scope: 'room', name: 'den3' },
        vars: {
          edible: 1,
        },
        objects: {
        },
      },
      inst_39: {
        isObject: 'bones',
        isInst: 'inst_39',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
          edible: 1,
        },
        objects: {
        },
      },
      inst_40: {
        isObject: 'bones',
        isInst: 'inst_40',
        loc: { scope: 'room', name: 'caveLake' },
        vars: {
          edible: 1,
        },
        objects: {
        },
      },
      inst_41: {
        isObject: 'bones',
        isInst: 'inst_41',
        loc: { scope: 'object', name: 'coffin', inst: 'inst_43' },
        vars: {
          edible: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      edible: 1,
    },
    hooks: {
    },
  },
  masterKey: {
    isObject: 'masterKey',
    desc: { text: "a MASTER-KEY" },
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  bat: {
    isObject: 'bat',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  goldLump: {
    isObject: 'goldLump',
    desc: { text: "a GOLD lump" },
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  stoneDoor: {
    isObject: 'stoneDoor',
    desc: { text: "a stone DOOR" },
    copies: {
      inst_42: {
        isObject: 'stoneDoor',
        isInst: 'inst_42',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
          noTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      noTake: 1,
    },
    hooks: {
    },
  },
  coffin: {
    isObject: 'coffin',
    desc: { text: "a COFFIN" },
    copies: {
      inst_43: {
        isObject: 'coffin',
        isInst: 'inst_43',
        loc: { scope: 'room', name: 'crypt' },
        vars: {
        },
        objects: {
          bones: ['inst_41',],
          cursedSkull: ['inst_44',],
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  cursedSkull: {
    isObject: 'cursedSkull',
    desc: { text: "a cursed SKULL" },
    copies: {
      inst_44: {
        isObject: 'cursedSkull',
        isInst: 'inst_44',
        loc: { scope: 'object', name: 'coffin', inst: 'inst_43' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  obsidianShard: {
    isObject: 'obsidianShard',
    desc: { routine: 'descObsidian' },
    copies: {
      inst_45: {
        isObject: 'obsidianShard',
        isInst: 'inst_45',
        loc: { scope: 'room', name: 'subGrotto7' },
        vars: {
          damage: 60,
          maxDamage: 60,
        },
        objects: {
        },
      },
    },
    vars: {
      damage: 60,
      maxDamage: 60,
    },
    hooks: {
    },
  },
  boatFrame: {
    isObject: 'boatFrame',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  boat: {
    isObject: 'boat',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  tree: {
    isObject: 'tree',
    desc: { text: "a TREE" },
    copies: {
    },
    vars: {
      health: 20,
    },
    hooks: {
    },
  },
  log: {
    isObject: 'log',
    desc: { text: "a LOG" },
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  roughBoard: {
    isObject: 'roughBoard',
    desc: { text: "a rough wooden BOARD" },
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  detritus: {
    isObject: 'detritus',
    desc: { text: "leafy DETRITUS" },
    copies: {
      inst_46: {
        isObject: 'detritus',
        isInst: 'inst_46',
        loc: { scope: 'room', name: 'cabinExterior' },
        vars: {
          tinder: 1,
        },
        objects: {
          cabinDoorKey: ['inst_3',],
        },
      },
    },
    vars: {
      tinder: 1,
    },
    hooks: {
    },
  },
  stick: {
    isObject: 'stick',
    desc: { text: "a STICK" },
    copies: {
    },
    vars: {
      tinder: 1,
    },
    hooks: {
    },
  },
  torch: {
    isObject: 'torch',
    desc: { text: "a makeshift TORCH" },
    copies: {
    },
    vars: {
      fuel: 8,
      lit: 1,
      tinder: 1,
    },
    hooks: {
    },
  },
  bulrush: {
    isObject: 'bulrush',
    desc: { text: "some REEDS" },
    copies: {
    },
    vars: {
      tinder: 1,
    },
    hooks: {
    },
  },
  strap: {
    isObject: 'strap',
    desc: { text: "a strong cloth-like STRAP" },
    copies: {
    },
    vars: {
      tinder: 1,
    },
    hooks: {
    },
  },
  sap: {
    isObject: 'sap',
    desc: { text: "tree SAP" },
    copies: {
      inst_47: {
        isObject: 'sap',
        isInst: 'inst_47',
        loc: { scope: 'room', name: 'storage' },
        vars: {
          tinder: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      tinder: 1,
    },
    hooks: {
    },
  },
  boiledSap: {
    isObject: 'boiledSap',
    desc: { text: "BOILED-SAP" },
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  berries: {
    isObject: 'berries',
    desc: { text: "some BERRIES" },
    copies: {
    },
    vars: {
      edible: 1,
    },
    hooks: {
    },
  },
  herbs: {
    isObject: 'herbs',
    desc: { text: "some HERBS" },
    copies: {
    },
    vars: {
      edible: 1,
    },
    hooks: {
    },
  },
  nuts: {
    isObject: 'nuts',
    desc: { text: "some NUTS" },
    copies: {
    },
    vars: {
      edible: 1,
    },
    hooks: {
    },
  },
  mushroom: {
    isObject: 'mushroom',
    desc: { text: "a MUSHROOM" },
    copies: {
    },
    vars: {
      edible: 1,
    },
    hooks: {
    },
  },
  roots: {
    isObject: 'roots',
    desc: { text: "a ROOT" },
    copies: {
    },
    vars: {
      edible: 1,
    },
    hooks: {
    },
  },
  ferns: {
    isObject: 'ferns',
    desc: { text: "a FERN" },
    copies: {
    },
    vars: {
      edible: 1,
    },
    hooks: {
    },
  },
  riverStone: {
    isObject: 'riverStone',
    desc: { routine: 'descRiverStone' },
    copies: {
      inst_48: {
        isObject: 'riverStone',
        isInst: 'inst_48',
        loc: { scope: 'room', name: 'forest4' },
        vars: {
          isWet: 0,
        },
        objects: {
        },
      },
    },
    vars: {
      isWet: 0,
    },
    hooks: {
    },
  },
  treeHollow: {
    isObject: 'treeHollow',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  gem: {
    isObject: 'gem',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  owl: {
    isObject: 'owl',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  crow: {
    isObject: 'crow',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  fish: {
    isObject: 'fish',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  frog: {
    isObject: 'frog',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  beetle: {
    isObject: 'beetle',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  rabbit: {
    isObject: 'rabbit',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  snake: {
    isObject: 'snake',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  bear: {
    isObject: 'bear',
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  axe: {
    isObject: 'axe',
    desc: { text: "an AXE" },
    copies: {
      inst_49: {
        isObject: 'axe',
        isInst: 'inst_49',
        loc: { scope: 'player' },
        vars: {
          damage: 3,
          health: 10,
          maxDamage: 5,
          maxHealth: 10,
        },
        objects: {
        },
      },
    },
    vars: {
      damage: 3,
      health: 10,
      maxDamage: 5,
      maxHealth: 10,
    },
    hooks: {
    },
  },
  cloak: {
    isObject: 'cloak',
    desc: { text: "a CLOAK" },
    copies: {
      inst_50: {
        isObject: 'cloak',
        isInst: 'inst_50',
        loc: { scope: 'player' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  flint: {
    isObject: 'flint',
    desc: { text: "FLINT" },
    copies: {
      inst_51: {
        isObject: 'flint',
        isInst: 'inst_51',
        loc: { scope: 'player' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  cup: {
    isObject: 'cup',
    desc: { text: "a CUP" },
    copies: {
      inst_52: {
        isObject: 'cup',
        isInst: 'inst_52',
        loc: { scope: 'player' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  kettle: {
    isObject: 'kettle',
    desc: { text: "a KETTLE" },
    copies: {
      inst_53: {
        isObject: 'kettle',
        isInst: 'inst_53',
        loc: { scope: 'player' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
    },
  },
  knife: {
    isObject: 'knife',
    desc: { text: "a KNIFE" },
    copies: {
      inst_54: {
        isObject: 'knife',
        isInst: 'inst_54',
        loc: { scope: 'player' },
        vars: {
          damage: 2,
          health: 10,
          maxDamage: 2,
          maxHealth: 10,
        },
        objects: {
        },
      },
    },
    vars: {
      damage: 2,
      health: 10,
      maxDamage: 2,
      maxHealth: 10,
    },
    hooks: {
    },
  },
  water: {
    isObject: 'water',
    desc: { text: "water" },
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
};

export const translateSynonym = (word) => {
  switch (word) {
  case 'AX':
  case 'AXE':
    return ['axe'];
  case 'BED':
  case 'BED-FRAME':
    return ['bedFrame'];
  case 'BERRIES':
  case 'BERRY':
    return ['berries'];
  case 'BOILED-SAP':
    return ['boiledSap'];
  case 'BONE':
  case 'BONES':
    return ['bones'];
  case 'BOOK':
    return ['book'];
  case 'BOOK-PAGE':
  case 'PAGE':
  case 'PAPER':
    return ['bookPage'];
  case 'BUCKET':
    return ['bucket'];
  case 'BULRUSH':
  case 'BULRUSHES':
  case 'CATTAIL':
  case 'CATTAILS':
  case 'REED':
  case 'REEDS':
  case 'TYPHA':
    return ['bulrush'];
  case 'CABIN-DOOR':
    return ['cabinDoor'];
  case 'DOOR':
    return ['cabinDoor', 'stoneDoor'];
  case 'CABIN-KEY':
  case 'DOOR-KEY':
  case 'KEY':
    return ['cabinDoorKey'];
  case 'WINDOW':
    return ['cabinWindow'];
  case 'CHAIR':
    return ['chair'];
  case 'CLOAK':
  case 'COAT':
    return ['cloak'];
  case 'COFFIN':
    return ['coffin'];
  case 'CUP':
    return ['cup'];
  case 'CURSED-SKULL':
  case 'SKULL':
    return ['cursedSkull'];
  case 'BRUSH':
  case 'DETRITUS':
    return ['detritus'];
  case 'FERN':
  case 'FERNS':
    return ['ferns'];
  case 'FIRE':
    return ['fire'];
  case 'FLINT':
    return ['flint'];
  case 'GOLD':
  case 'GOLD-LUMP':
    return ['goldLump'];
  case 'HERB':
  case 'HERBS':
    return ['herbs'];
  case 'KETTLE':
  case 'POT':
    return ['kettle'];
  case 'KNIFE':
    return ['knife'];
  case 'LOG':
  case 'LOGS':
    return ['log'];
  case 'MAGIC-RING':
  case 'RING':
    return ['magicRing'];
  case 'MASTER-KEY':
    return ['masterKey'];
  case 'MUSHROOM':
  case 'MUSHROOMS':
    return ['mushroom'];
  case 'NAILS':
    return ['nails'];
  case 'NOTE':
    return ['note'];
  case 'NUT':
  case 'NUTS':
    return ['nuts'];
  case 'OBSIDIAN':
  case 'OBSIDIAN-SHARD':
    return ['obsidianShard'];
  case 'PICK':
  case 'PICK-AXE':
    return ['pickAxe'];
  case 'RIVER-STONE':
  case 'STONE':
  case 'WHETSTONE':
    return ['riverStone'];
  case 'ROCK':
  case 'ROCKS':
    return ['rock'];
  case 'ROOT':
  case 'ROOTS':
    return ['roots'];
  case 'BOARD':
  case 'ROUGH-BOARD':
    return ['roughBoard'];
  case 'SAP':
    return ['sap'];
  case 'SOUP':
    return ['soup'];
  case 'BRANCH':
  case 'STICK':
    return ['stick'];
  case 'STONE-DOOR':
    return ['stoneDoor'];
  case 'STRAP':
    return ['strap'];
  case 'SWORD':
    return ['sword'];
  case 'TABLE':
    return ['table'];
  case 'TEA':
    return ['tea'];
  case 'TORCH':
    return ['torch'];
  case 'TREE':
    return ['tree'];
  case 'WIRE':
  case 'WIRES':
    return ['wire'];
  default:
    return null;
  }
}
