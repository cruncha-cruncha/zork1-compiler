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
          hasBoards: 2,
          health: 6,
          isLocked: 1,
          isSoft: 1,
          noTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      hasBoards: 2,
      health: 6,
      isLocked: 1,
      isSoft: 1,
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
        loc: { scope: 'object', name: 'detritus', inst: 'inst_41' },
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
          isSoft: 1,
          tinder: 1,
        },
        objects: {
          bookPage: ['inst_5',],
        },
      },
    },
    vars: {
      health: 2,
      isSoft: 1,
      tinder: 1,
    },
    hooks: {
    },
  },
  bookPage: {
    isObject: 'bookPage',
    desc: { routine: 'descBookPage' },
    copies: {
      inst_5: {
        isObject: 'bookPage',
        isInst: 'inst_5',
        loc: { scope: 'object', name: 'book', inst: 'inst_4' },
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
  note: {
    isObject: 'note',
    desc: { routine: 'descNote' },
    copies: {
    },
    vars: {
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
          isSoft: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      hasBoards: 2,
      health: 4,
      isSoft: 1,
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
          isSoft: 1,
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
          isSoft: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      hasBoards: 1,
      health: 6,
      isSoft: 1,
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
          hasBoards: 2,
          health: 6,
          isSoft: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      hasBoards: 2,
      health: 6,
      isSoft: 1,
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
          isHard: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      isHard: 1,
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
  soup: {
    isObject: 'soup',
    desc: { text: "SOUP" },
    copies: {
    },
    vars: {
      isEdible: 1,
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
      isEdible: 1,
    },
    hooks: {
    },
  },
  fire: {
    isObject: 'fire',
    desc: { routine: 'descFire' },
    copies: {
    },
    vars: {
      fuel: 3,
      noTake: 1,
    },
    hooks: {
    },
  },
  charcoal: {
    isObject: 'charcoal',
    desc: { text: "some CHARCOAL" },
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
  rockPile: {
    isObject: 'rockPile',
    desc: { text: "a large rock pile" },
    copies: {
      inst_12: {
        isObject: 'rockPile',
        isInst: 'inst_12',
        loc: { scope: 'room', name: 'mazeRest2' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
      inRoom: 'rockPileInRoom',
    },
  },
  caveSpider: {
    isObject: 'caveSpider',
    desc: { text: "a SPIDER" },
    copies: {
    },
    vars: {
      health: 2,
      ownTake: 1,
    },
    hooks: {
      inRoom: 'caveSpiderInRoom',
    },
  },
  childMonster: {
    isObject: 'childMonster',
    desc: { routine: 'descChildMonster' },
    copies: {
      inst_13: {
        isObject: 'childMonster',
        isInst: 'inst_13',
        loc: { scope: 'room', name: 'storage' },
        vars: {
          health: 100,
          isSoft: 1,
          maxHealth: 100,
          noTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      health: 100,
      isSoft: 1,
      maxHealth: 100,
      noTake: 1,
    },
    hooks: {
      inRoom: 'childMonsterInRoom',
    },
  },
  parentMonster: {
    isObject: 'parentMonster',
    desc: { routine: 'descParentMonster' },
    copies: {
      inst_14: {
        isObject: 'parentMonster',
        isInst: 'inst_14',
        loc: { scope: 'room', name: 'storage' },
        vars: {
          health: 60,
          isSoft: 1,
          maxHealth: 70,
          noTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      health: 60,
      isSoft: 1,
      maxHealth: 70,
      noTake: 1,
    },
    hooks: {
      inRoom: 'parentMonsterInRoom',
    },
  },
  pickAxe: {
    isObject: 'pickAxe',
    desc: { routine: 'descPickAxe' },
    copies: {
      inst_15: {
        isObject: 'pickAxe',
        isInst: 'inst_15',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
          damage: 10,
          maxDamage: 4,
        },
        objects: {
        },
      },
    },
    vars: {
      damage: 10,
      maxDamage: 4,
    },
    hooks: {
    },
  },
  sword: {
    isObject: 'sword',
    desc: { routine: 'descSword' },
    copies: {
      inst_16: {
        isObject: 'sword',
        isInst: 'inst_16',
        loc: { scope: 'room', name: 'crypt' },
        vars: {
          damage: 30,
          maxDamage: 30,
          ownTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      damage: 30,
      maxDamage: 30,
      ownTake: 1,
    },
    hooks: {
    },
  },
  rock: {
    isObject: 'rock',
    desc: { text: "a ROCK" },
    copies: {
      inst_17: {
        isObject: 'rock',
        isInst: 'inst_17',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
          damage: 2,
          isHard: 1,
          maxDamage: 2,
        },
        objects: {
        },
      },
      inst_18: {
        isObject: 'rock',
        isInst: 'inst_18',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
          damage: 2,
          isHard: 1,
          maxDamage: 2,
        },
        objects: {
        },
      },
      inst_19: {
        isObject: 'rock',
        isInst: 'inst_19',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
          damage: 2,
          isHard: 1,
          maxDamage: 2,
        },
        objects: {
        },
      },
      inst_20: {
        isObject: 'rock',
        isInst: 'inst_20',
        loc: { scope: 'room', name: 'longHall1' },
        vars: {
          damage: 2,
          isHard: 1,
          maxDamage: 2,
        },
        objects: {
        },
      },
      inst_21: {
        isObject: 'rock',
        isInst: 'inst_21',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
          damage: 2,
          isHard: 1,
          maxDamage: 2,
        },
        objects: {
        },
      },
      inst_22: {
        isObject: 'rock',
        isInst: 'inst_22',
        loc: { scope: 'room', name: 'den2' },
        vars: {
          damage: 2,
          isHard: 1,
          maxDamage: 2,
        },
        objects: {
        },
      },
      inst_23: {
        isObject: 'rock',
        isInst: 'inst_23',
        loc: { scope: 'room', name: 'den3' },
        vars: {
          damage: 2,
          isHard: 1,
          maxDamage: 2,
        },
        objects: {
        },
      },
      inst_24: {
        isObject: 'rock',
        isInst: 'inst_24',
        loc: { scope: 'room', name: 'caveLake' },
        vars: {
          damage: 2,
          isHard: 1,
          maxDamage: 2,
        },
        objects: {
        },
      },
      inst_25: {
        isObject: 'rock',
        isInst: 'inst_25',
        loc: { scope: 'room', name: 'crypt' },
        vars: {
          damage: 2,
          isHard: 1,
          maxDamage: 2,
        },
        objects: {
        },
      },
      inst_26: {
        isObject: 'rock',
        isInst: 'inst_26',
        loc: { scope: 'room', name: 'caveEntrance1' },
        vars: {
          damage: 2,
          isHard: 1,
          maxDamage: 2,
        },
        objects: {
        },
      },
    },
    vars: {
      damage: 2,
      isHard: 1,
      maxDamage: 2,
    },
    hooks: {
    },
  },
  bones: {
    isObject: 'bones',
    desc: { routine: 'descBones' },
    copies: {
      inst_27: {
        isObject: 'bones',
        isInst: 'inst_27',
        loc: { scope: 'room', name: 'den1' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
      inst_28: {
        isObject: 'bones',
        isInst: 'inst_28',
        loc: { scope: 'room', name: 'den1' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
      inst_29: {
        isObject: 'bones',
        isInst: 'inst_29',
        loc: { scope: 'room', name: 'den1' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
      inst_30: {
        isObject: 'bones',
        isInst: 'inst_30',
        loc: { scope: 'room', name: 'den2' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
      inst_31: {
        isObject: 'bones',
        isInst: 'inst_31',
        loc: { scope: 'room', name: 'den3' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
      inst_32: {
        isObject: 'bones',
        isInst: 'inst_32',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
      inst_33: {
        isObject: 'bones',
        isInst: 'inst_33',
        loc: { scope: 'room', name: 'caveLake' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
      inst_34: {
        isObject: 'bones',
        isInst: 'inst_34',
        loc: { scope: 'object', name: 'coffin', inst: 'inst_36' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      isEdible: 1,
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
    desc: { routine: 'descStoneDoor' },
    copies: {
      inst_35: {
        isObject: 'stoneDoor',
        isInst: 'inst_35',
        loc: { scope: 'room', name: 'cavern1' },
        vars: {
          health: 10,
          isHard: 1,
          isLocked: 1,
          noTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      health: 10,
      isHard: 1,
      isLocked: 1,
      noTake: 1,
    },
    hooks: {
    },
  },
  coffin: {
    isObject: 'coffin',
    desc: { text: "a COFFIN" },
    copies: {
      inst_36: {
        isObject: 'coffin',
        isInst: 'inst_36',
        loc: { scope: 'room', name: 'crypt' },
        vars: {
          isHard: 1,
        },
        objects: {
          bones: ['inst_34',],
          cursedSkull: ['inst_37',],
        },
      },
    },
    vars: {
      isHard: 1,
    },
    hooks: {
    },
  },
  cursedSkull: {
    isObject: 'cursedSkull',
    desc: { routine: 'descCursedSkull' },
    copies: {
      inst_37: {
        isObject: 'cursedSkull',
        isInst: 'inst_37',
        loc: { scope: 'object', name: 'coffin', inst: 'inst_36' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
      inPlayer: 'cursedSkullInPlayer',
      exitPlayer: 'cursedSkullExitPlayer',
    },
  },
  obsidianShard: {
    isObject: 'obsidianShard',
    desc: { routine: 'descObsidian' },
    copies: {
      inst_38: {
        isObject: 'obsidianShard',
        isInst: 'inst_38',
        loc: { scope: 'room', name: 'subGrotto8' },
        vars: {
          damage: 50,
          maxDamage: 50,
        },
        objects: {
        },
      },
      inst_39: {
        isObject: 'obsidianShard',
        isInst: 'inst_39',
        loc: { scope: 'object', name: 'water', inst: 'inst_67' },
        vars: {
          damage: 50,
          maxDamage: 50,
        },
        objects: {
        },
      },
    },
    vars: {
      damage: 50,
      maxDamage: 50,
    },
    hooks: {
    },
  },
  boatFrame: {
    isObject: 'boatFrame',
    desc: { routine: 'descBoatFrame' },
    copies: {
      inst_40: {
        isObject: 'boatFrame',
        isInst: 'inst_40',
        loc: { scope: 'room', name: 'lake1' },
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
  boat: {
    isObject: 'boat',
    desc: { text: "a BOAT" },
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
      isSoft: 1,
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
      health: 10,
      isSoft: 1,
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
      isSoft: 1,
    },
    hooks: {
    },
  },
  detritus: {
    isObject: 'detritus',
    desc: { routine: 'descDetritus' },
    copies: {
      inst_41: {
        isObject: 'detritus',
        isInst: 'inst_41',
        loc: { scope: 'room', name: 'cabinExterior' },
        vars: {
          ownTake: 1,
        },
        objects: {
          cabinDoorKey: ['inst_3',],
        },
      },
      inst_42: {
        isObject: 'detritus',
        isInst: 'inst_42',
        loc: { scope: 'room', name: 'caveEntrance1' },
        vars: {
          ownTake: 1,
        },
        objects: {
        },
      },
      inst_43: {
        isObject: 'detritus',
        isInst: 'inst_43',
        loc: { scope: 'room', name: 'caveEntrance2' },
        vars: {
          ownTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      ownTake: 1,
    },
    hooks: {
    },
  },
  stick: {
    isObject: 'stick',
    desc: { text: "a STICK" },
    copies: {
      inst_44: {
        isObject: 'stick',
        isInst: 'inst_44',
        loc: { scope: 'room', name: 'forest2' },
        vars: {
          isSoft: 1,
        },
        objects: {
        },
      },
      inst_45: {
        isObject: 'stick',
        isInst: 'inst_45',
        loc: { scope: 'room', name: 'forest4' },
        vars: {
          isSoft: 1,
        },
        objects: {
        },
      },
      inst_46: {
        isObject: 'stick',
        isInst: 'inst_46',
        loc: { scope: 'room', name: 'caveEntrance1' },
        vars: {
          isSoft: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      isSoft: 1,
    },
    hooks: {
    },
  },
  torch: {
    isObject: 'torch',
    desc: { routine: 'descTorch' },
    copies: {
    },
    vars: {
      fuel: 8,
      isLit: 0,
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
      inst_48: {
        isObject: 'berries',
        isInst: 'inst_48',
        loc: { scope: 'room', name: 'field2' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      isEdible: 1,
    },
    hooks: {
    },
  },
  herbs: {
    isObject: 'herbs',
    desc: { text: "some HERBS" },
    copies: {
      inst_49: {
        isObject: 'herbs',
        isInst: 'inst_49',
        loc: { scope: 'room', name: 'field1' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
      inst_50: {
        isObject: 'herbs',
        isInst: 'inst_50',
        loc: { scope: 'room', name: 'field2' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      isEdible: 1,
    },
    hooks: {
    },
  },
  nuts: {
    isObject: 'nuts',
    desc: { text: "some NUTS" },
    copies: {
      inst_51: {
        isObject: 'nuts',
        isInst: 'inst_51',
        loc: { scope: 'room', name: 'forest6' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      isEdible: 1,
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
      isEdible: 1,
    },
    hooks: {
    },
  },
  root: {
    isObject: 'root',
    desc: { text: "a ROOT" },
    copies: {
    },
    vars: {
      isEdible: 1,
    },
    hooks: {
    },
  },
  fern: {
    isObject: 'fern',
    desc: { text: "a FERN" },
    copies: {
      inst_52: {
        isObject: 'fern',
        isInst: 'inst_52',
        loc: { scope: 'room', name: 'forest3' },
        vars: {
          isEdible: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      isEdible: 1,
    },
    hooks: {
    },
  },
  riverStone: {
    isObject: 'riverStone',
    desc: { routine: 'descRiverStone' },
    copies: {
      inst_53: {
        isObject: 'riverStone',
        isInst: 'inst_53',
        loc: { scope: 'room', name: 'forest3' },
        vars: {
          isHard: 1,
          wetness: 0,
        },
        objects: {
        },
      },
    },
    vars: {
      isHard: 1,
      wetness: 0,
    },
    hooks: {
    },
  },
  treeHollow: {
    isObject: 'treeHollow',
    desc: { text: "a tree HOLLOW" },
    copies: {
      inst_54: {
        isObject: 'treeHollow',
        isInst: 'inst_54',
        loc: { scope: 'room', name: 'storage' },
        vars: {
          noTake: 1,
        },
        objects: {
          gem: ['inst_55',],
        },
      },
    },
    vars: {
      noTake: 1,
    },
    hooks: {
    },
  },
  gem: {
    isObject: 'gem',
    desc: { routine: 'descGem' },
    copies: {
      inst_55: {
        isObject: 'gem',
        isInst: 'inst_55',
        loc: { scope: 'object', name: 'treeHollow', inst: 'inst_54' },
        vars: {
        },
        objects: {
        },
      },
    },
    vars: {
    },
    hooks: {
      enterPlayer: 'gemEnterPlayer',
    },
  },
  bat: {
    isObject: 'bat',
    desc: { text: "a BAT" },
    copies: {
    },
    vars: {
      health: 1,
      isAnimal: 1,
      isSoft: 1,
      ownTake: 1,
    },
    hooks: {
    },
  },
  owl: {
    isObject: 'owl',
    desc: { text: "an OWL" },
    copies: {
    },
    vars: {
      health: 1,
      isAnimal: 1,
      isSoft: 1,
      ownTake: 1,
    },
    hooks: {
    },
  },
  crow: {
    isObject: 'crow',
    desc: { text: "a CROW" },
    copies: {
    },
    vars: {
      health: 1,
      isAnimal: 1,
      isSoft: 1,
      ownTake: 1,
    },
    hooks: {
    },
  },
  fish: {
    isObject: 'fish',
    desc: { text: "a FISH" },
    copies: {
      inst_56: {
        isObject: 'fish',
        isInst: 'inst_56',
        loc: { scope: 'object', name: 'water', inst: 'inst_66' },
        vars: {
          health: 1,
          isAnimal: 1,
          isSoft: 1,
          ownTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      health: 1,
      isAnimal: 1,
      isSoft: 1,
      ownTake: 1,
    },
    hooks: {
      inRoom: 'drownFish',
      inPlayer: 'drownFish',
    },
  },
  frog: {
    isObject: 'frog',
    desc: { text: "a FROG" },
    copies: {
    },
    vars: {
      health: 1,
      isAnimal: 1,
      isSoft: 1,
      ownTake: 1,
    },
    hooks: {
    },
  },
  beetle: {
    isObject: 'beetle',
    desc: { text: "a BEETLE" },
    copies: {
    },
    vars: {
    },
    hooks: {
    },
  },
  rabbit: {
    isObject: 'rabbit',
    desc: { text: "a RABBIT" },
    copies: {
      inst_57: {
        isObject: 'rabbit',
        isInst: 'inst_57',
        loc: { scope: 'room', name: 'caveEntrance1' },
        vars: {
          health: 1,
          isAnimal: 1,
          isSoft: 1,
          ownTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      health: 1,
      isAnimal: 1,
      isSoft: 1,
      ownTake: 1,
    },
    hooks: {
    },
  },
  snake: {
    isObject: 'snake',
    desc: { text: "a SNAKE" },
    copies: {
    },
    vars: {
      health: 1,
      isAnimal: 1,
      isSoft: 1,
      ownTake: 1,
    },
    hooks: {
      inPlayer: 'snakeInPlayer',
    },
  },
  bear: {
    isObject: 'bear',
    desc: { text: "a BEAR" },
    copies: {
      inst_58: {
        isObject: 'bear',
        isInst: 'inst_58',
        loc: { scope: 'room', name: 'storage' },
        vars: {
          asked: 0,
          health: 20,
          isAnimal: 1,
          isSoft: 1,
          ownTake: 1,
        },
        objects: {
        },
      },
      inst_59: {
        isObject: 'bear',
        isInst: 'inst_59',
        loc: { scope: 'room', name: 'storage' },
        vars: {
          asked: 0,
          health: 20,
          isAnimal: 1,
          isSoft: 1,
          ownTake: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      asked: 0,
      health: 20,
      isAnimal: 1,
      isSoft: 1,
      ownTake: 1,
    },
    hooks: {
      inRoom: 'bearInRoom',
    },
  },
  ynQ: {
    isObject: 'ynQ',
    copies: {
    },
    vars: {
      exp: 1,
    },
    hooks: {
    },
  },
  axe: {
    isObject: 'axe',
    desc: { routine: 'descAxe' },
    copies: {
      inst_60: {
        isObject: 'axe',
        isInst: 'inst_60',
        loc: { scope: 'player' },
        vars: {
          damage: 8,
          health: 10,
          maxDamage: 10,
          maxHealth: 10,
        },
        objects: {
        },
      },
    },
    vars: {
      damage: 8,
      health: 10,
      maxDamage: 10,
      maxHealth: 10,
    },
    hooks: {
    },
  },
  cloak: {
    isObject: 'cloak',
    desc: { routine: 'descCloak' },
    copies: {
      inst_61: {
        isObject: 'cloak',
        isInst: 'inst_61',
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
    desc: { routine: 'descFlint' },
    copies: {
      inst_62: {
        isObject: 'flint',
        isInst: 'inst_62',
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
    desc: { routine: 'descCup' },
    copies: {
      inst_63: {
        isObject: 'cup',
        isInst: 'inst_63',
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
    desc: { routine: 'descKettle' },
    copies: {
      inst_64: {
        isObject: 'kettle',
        isInst: 'inst_64',
        loc: { scope: 'player' },
        vars: {
          isHard: 1,
        },
        objects: {
        },
      },
    },
    vars: {
      isHard: 1,
    },
    hooks: {
    },
  },
  knife: {
    isObject: 'knife',
    desc: { routine: 'descKnife' },
    copies: {
      inst_65: {
        isObject: 'knife',
        isInst: 'inst_65',
        loc: { scope: 'player' },
        vars: {
          damage: 4,
          health: 10,
          maxDamage: 4,
          maxHealth: 10,
        },
        objects: {
        },
      },
    },
    vars: {
      damage: 4,
      health: 10,
      maxDamage: 4,
      maxHealth: 10,
    },
    hooks: {
    },
  },
  water: {
    isObject: 'water',
    desc: { text: "WATER" },
    copies: {
      inst_66: {
        isObject: 'water',
        isInst: 'inst_66',
        loc: { scope: 'room', name: 'lake1' },
        vars: {
          ownTake: 1,
        },
        objects: {
          fish: ['inst_56',],
        },
      },
      inst_67: {
        isObject: 'water',
        isInst: 'inst_67',
        loc: { scope: 'room', name: 'caveLake' },
        vars: {
          ownTake: 1,
        },
        objects: {
          obsidianShard: ['inst_39',],
        },
      },
    },
    vars: {
      ownTake: 1,
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
  case 'BAT':
    return ['bat'];
  case 'BEAR':
    return ['bear'];
  case 'BED':
  case 'BED-FRAME':
    return ['bedFrame'];
  case 'BEETLE':
    return ['beetle'];
  case 'BERRIES':
  case 'BERRY':
    return ['berries'];
  case 'BOAT':
  case 'BOAT-FRAME':
  case 'FRAME':
    return ['boatFrame'];
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
  case 'CAVE-SPIDER':
  case 'SPIDER':
    return ['caveSpider'];
  case 'CHAIR':
    return ['chair'];
  case 'CHARCOAL':
  case 'COAL':
    return ['charcoal'];
  case 'MONSTER':
    return ['childMonster', 'parentMonster'];
  case 'CLOAK':
  case 'COAT':
    return ['cloak'];
  case 'COFFIN':
    return ['coffin'];
  case 'CROW':
    return ['crow'];
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
    return ['fern'];
  case 'FIRE':
    return ['fire'];
  case 'FISH':
    return ['fish'];
  case 'FLINT':
    return ['flint'];
  case 'FROG':
    return ['frog'];
  case 'GEM':
    return ['gem'];
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
  case 'MESSAGE':
  case 'NOTE':
    return ['note'];
  case 'NUT':
  case 'NUTS':
    return ['nuts'];
  case 'OBSIDIAN':
  case 'OBSIDIAN-SHARD':
    return ['obsidianShard'];
  case 'OWL':
    return ['owl'];
  case 'PICK':
  case 'PICK-AXE':
    return ['pickAxe'];
  case 'RABBIT':
    return ['rabbit'];
  case 'RIVER-STONE':
  case 'STONE':
  case 'WHETSTONE':
    return ['riverStone'];
  case 'ROCK':
  case 'ROCKS':
    return ['rock', 'rockPile'];
  case 'ROOT':
  case 'ROOTS':
    return ['root'];
  case 'BOARD':
  case 'ROUGH-BOARD':
    return ['roughBoard'];
  case 'SAP':
    return ['sap'];
  case 'SNAKE':
    return ['snake'];
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
  case 'HOLE':
  case 'HOLLOW':
  case 'TREE-HOLLOW':
    return ['treeHollow'];
  case 'WATER':
    return ['water'];
  default:
    return null;
  }
}
