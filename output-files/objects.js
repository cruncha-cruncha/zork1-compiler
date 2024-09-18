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
          canOpen: 1,
          noTake: 1,
        },
        objects: {
        },
      },
    },
    hooks: {
      prso: 'prsoCabinDoor',
    },
  },
  cabinWindow: {
    isObject: 'cabinWindow',
    desc: { text: "a cabin WINDOW" },
    copies: {
      inst_2: {
        isObject: 'cabinWindow',
        isInst: 'inst_2',
        loc: { scope: 'room', name: 'cabinExterior' },
        vars: {
          noTake: 1,
        },
        objects: {
        },
      },
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
        loc: { scope: 'object', name: 'detritus', inst: 'inst_13' },
        vars: {
        },
        objects: {
        },
      },
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
        },
        objects: {
          bookPages: ['inst_5',],
        },
      },
    },
    hooks: {
      prso: 'prsoBook',
    },
  },
  bookPages: {
    isObject: 'bookPages',
    desc: { text: "some PAPER" },
    copies: {
      inst_5: {
        isObject: 'bookPages',
        isInst: 'inst_5',
        loc: { scope: 'object', name: 'book', inst: 'inst_4' },
        vars: {
        },
        objects: {
        },
      },
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
        },
        objects: {
        },
      },
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
        },
        objects: {
        },
      },
      inst_8: {
        isObject: 'chair',
        isInst: 'inst_8',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
        },
        objects: {
        },
      },
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
    hooks: {
    },
  },
  firePlace: {
    isObject: 'firePlace',
    desc: { text: "a FIREPLACE" },
    copies: {
      inst_10: {
        isObject: 'firePlace',
        isInst: 'inst_10',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
          noTake: 1,
        },
        objects: {
        },
      },
    },
    hooks: {
    },
  },
  bucket: {
    isObject: 'bucket',
    desc: { text: "a BUCKET" },
    copies: {
      inst_11: {
        isObject: 'bucket',
        isInst: 'inst_11',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
        },
        objects: {
        },
      },
    },
    hooks: {
    },
  },
  nails: {
    isObject: 'nails',
    desc: { text: "some NAILS" },
    copies: {
      inst_12: {
        isObject: 'nails',
        isInst: 'inst_12',
        loc: { scope: 'room', name: 'cabin' },
        vars: {
        },
        objects: {
        },
      },
    },
    hooks: {
    },
  },
  fire: {
    isObject: 'fire',
    desc: { text: "a FIRE" },
    copies: {
    },
    hooks: {
    },
  },
  log: {
    isObject: 'log',
    desc: { text: "a LOG" },
    copies: {
    },
    hooks: {
    },
  },
  soup: {
    isObject: 'soup',
    desc: { text: "SOUP" },
    copies: {
    },
    hooks: {
    },
  },
  tea: {
    isObject: 'tea',
    desc: { text: "TEA" },
    copies: {
    },
    hooks: {
    },
  },
  driedGrass1: {
    isObject: 'driedGrass1',
    copies: {
    },
    hooks: {
    },
  },
  stick1: {
    isObject: 'stick1',
    copies: {
    },
    hooks: {
    },
  },
  sap1: {
    isObject: 'sap1',
    copies: {
    },
    hooks: {
    },
  },
  detritus2: {
    isObject: 'detritus2',
    copies: {
    },
    hooks: {
    },
  },
  water1: {
    isObject: 'water1',
    copies: {
    },
    hooks: {
    },
  },
  wire1: {
    isObject: 'wire1',
    copies: {
    },
    hooks: {
    },
  },
  charcoal: {
    isObject: 'charcoal',
    copies: {
    },
    hooks: {
    },
  },
  masterKey: {
    isObject: 'masterKey',
    copies: {
    },
    hooks: {
    },
  },
  roughBoard: {
    isObject: 'roughBoard',
    copies: {
    },
    hooks: {
    },
  },
  rock: {
    isObject: 'rock',
    copies: {
    },
    hooks: {
    },
  },
  bullrushes: {
    isObject: 'bullrushes',
    copies: {
    },
    hooks: {
    },
  },
  bones: {
    isObject: 'bones',
    copies: {
    },
    hooks: {
    },
  },
  berries: {
    isObject: 'berries',
    copies: {
    },
    hooks: {
    },
  },
  herbs: {
    isObject: 'herbs',
    copies: {
    },
    hooks: {
    },
  },
  nuts: {
    isObject: 'nuts',
    copies: {
    },
    hooks: {
    },
  },
  mushroom: {
    isObject: 'mushroom',
    copies: {
    },
    hooks: {
    },
  },
  roots: {
    isObject: 'roots',
    copies: {
    },
    hooks: {
    },
  },
  ferns: {
    isObject: 'ferns',
    copies: {
    },
    hooks: {
    },
  },
  glassShard: {
    isObject: 'glassShard',
    copies: {
    },
    hooks: {
    },
  },
  boat: {
    isObject: 'boat',
    copies: {
    },
    hooks: {
    },
  },
  treeHollow: {
    isObject: 'treeHollow',
    copies: {
    },
    hooks: {
    },
  },
  riverStone: {
    isObject: 'riverStone',
    copies: {
    },
    hooks: {
    },
  },
  gem: {
    isObject: 'gem',
    copies: {
    },
    hooks: {
    },
  },
  owl: {
    isObject: 'owl',
    copies: {
    },
    hooks: {
    },
  },
  crow: {
    isObject: 'crow',
    copies: {
    },
    hooks: {
    },
  },
  fish: {
    isObject: 'fish',
    copies: {
    },
    hooks: {
    },
  },
  frog: {
    isObject: 'frog',
    copies: {
    },
    hooks: {
    },
  },
  beetle: {
    isObject: 'beetle',
    copies: {
    },
    hooks: {
    },
  },
  rabbit: {
    isObject: 'rabbit',
    copies: {
    },
    hooks: {
    },
  },
  snake: {
    isObject: 'snake',
    copies: {
    },
    hooks: {
    },
  },
  bat: {
    isObject: 'bat',
    copies: {
    },
    hooks: {
    },
  },
  bear: {
    isObject: 'bear',
    copies: {
    },
    hooks: {
    },
  },
  spiderWeb: {
    isObject: 'spiderWeb',
    copies: {
    },
    hooks: {
    },
  },
  monster: {
    isObject: 'monster',
    copies: {
    },
    hooks: {
    },
  },
  goldLump: {
    isObject: 'goldLump',
    copies: {
    },
    hooks: {
    },
  },
  stoneDoor: {
    isObject: 'stoneDoor',
    copies: {
    },
    hooks: {
    },
  },
  coffin: {
    isObject: 'coffin',
    copies: {
    },
    hooks: {
    },
  },
  cursedSkull: {
    isObject: 'cursedSkull',
    copies: {
    },
    hooks: {
    },
  },
  magicRing: {
    isObject: 'magicRing',
    copies: {
    },
    hooks: {
    },
  },
  obsidian: {
    isObject: 'obsidian',
    copies: {
    },
    hooks: {
    },
  },
  sword: {
    isObject: 'sword',
    copies: {
    },
    hooks: {
    },
  },
  pickAxe: {
    isObject: 'pickAxe',
    copies: {
    },
    hooks: {
    },
  },
  boatFrame: {
    isObject: 'boatFrame',
    copies: {
    },
    hooks: {
    },
  },
  detritus: {
    isObject: 'detritus',
    desc: { text: "leafy DETRITUS" },
    copies: {
      inst_13: {
        isObject: 'detritus',
        isInst: 'inst_13',
        loc: { scope: 'room', name: 'cabinExterior' },
        vars: {
        },
        objects: {
          cabinDoorKey: ['inst_3',],
        },
      },
    },
    hooks: {
      prso: 'prsoDetritus',
    },
  },
  axe: {
    isObject: 'axe',
    desc: { text: "an AXE" },
    copies: {
      inst_14: {
        isObject: 'axe',
        isInst: 'inst_14',
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
    hooks: {
    },
  },
  cloak: {
    isObject: 'cloak',
    desc: { text: "a CLOAK" },
    copies: {
      inst_15: {
        isObject: 'cloak',
        isInst: 'inst_15',
        loc: { scope: 'player' },
        vars: {
        },
        objects: {
        },
      },
    },
    hooks: {
    },
  },
  flint: {
    isObject: 'flint',
    desc: { text: "FLINT" },
    copies: {
      inst_16: {
        isObject: 'flint',
        isInst: 'inst_16',
        loc: { scope: 'player' },
        vars: {
        },
        objects: {
        },
      },
    },
    hooks: {
    },
  },
  cup: {
    isObject: 'cup',
    desc: { text: "a CUP" },
    copies: {
      inst_17: {
        isObject: 'cup',
        isInst: 'inst_17',
        loc: { scope: 'player' },
        vars: {
        },
        objects: {
        },
      },
    },
    hooks: {
    },
  },
  kettle: {
    isObject: 'kettle',
    desc: { text: "a KETTLE" },
    copies: {
      inst_18: {
        isObject: 'kettle',
        isInst: 'inst_18',
        loc: { scope: 'player' },
        vars: {
        },
        objects: {
        },
      },
    },
    hooks: {
    },
  },
  knife: {
    isObject: 'knife',
    desc: { text: "a KNIFE" },
    copies: {
      inst_19: {
        isObject: 'knife',
        isInst: 'inst_19',
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
  case 'BOOK':
    return ['book'];
  case 'PAPER':
    return ['bookPages'];
  case 'BUCKET':
    return ['bucket'];
  case 'CABIN-DOOR':
  case 'DOOR':
    return ['cabinDoor'];
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
  case 'CUP':
    return ['cup'];
  case 'BRUSH':
  case 'DETRITUS':
    return ['detritus'];
  case 'FIRE':
    return ['fire'];
  case 'FIRE-PLACE':
  case 'FIREPLACE':
    return ['firePlace'];
  case 'FLINT':
    return ['flint'];
  case 'KETTLE':
  case 'POT':
    return ['kettle'];
  case 'KNIFE':
    return ['knife'];
  case 'LOG':
  case 'LOGS':
    return ['log'];
  case 'NAILS':
    return ['nails'];
  case 'SOUP':
    return ['soup'];
  case 'TABLE':
    return ['table'];
  case 'TEA':
    return ['tea'];
  default:
    return null;
  }
}
