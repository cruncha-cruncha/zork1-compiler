export const objects = {
  axe: {
    isObject: 'axe',
    desc: { text: "an AXE" },
    objects: [
    ],
    loc: 'player',
    vars: {
      health: 10,
      maxSharp: 5,
      sharp: 3,
    },
    hooks: {
    },
  },
  boots: {
    isObject: 'boots',
    desc: { text: "BOOTS" },
    objects: [
    ],
    loc: 'player',
    vars: {
    },
    hooks: {
    },
  },
  cloak: {
    isObject: 'cloak',
    desc: { text: "a CLOAK" },
    objects: [
    ],
    loc: 'player',
    vars: {
    },
    hooks: {
    },
  },
  flint: {
    isObject: 'flint',
    desc: { text: "FLINT" },
    objects: [
    ],
    loc: 'player',
    vars: {
    },
    hooks: {
    },
  },
  bowl: {
    isObject: 'bowl',
    desc: { text: "a BOWL" },
    objects: [
    ],
    loc: 'player',
    vars: {
    },
    hooks: {
    },
  },
  cup: {
    isObject: 'cup',
    desc: { text: "a CUP" },
    objects: [
    ],
    loc: 'player',
    vars: {
    },
    hooks: {
    },
  },
  kettle: {
    isObject: 'kettle',
    desc: { text: "a KETTLE" },
    objects: [
    ],
    loc: 'player',
    vars: {
    },
    hooks: {
    },
  },
  cutlery: {
    isObject: 'cutlery',
    desc: { text: "some CUTLERY" },
    objects: [
    ],
    loc: 'player',
    vars: {
    },
    hooks: {
    },
  },
  knife: {
    isObject: 'knife',
    desc: { text: "a KNIFE" },
    objects: [
    ],
    loc: 'player',
    vars: {
    },
    hooks: {
    },
  },
  cabinDoor: {
    isObject: 'cabinDoor',
    desc: { text: "the cabin DOOR" },
    objects: [
    ],
    loc: 'cabinExterior',
    vars: {
      canOpen: 1,
      isLocked: 1,
      noTake: 1,
    },
    hooks: {
      prso: 'fCabinDoorPrso',
    },
  },
  cabinWindow: {
    isObject: 'cabinWindow',
    desc: { text: "a cabin WINDOW" },
    objects: [
    ],
    loc: 'cabinExterior',
    vars: {
      noTake: 1,
    },
    hooks: {
    },
  },
  cabinDoorKey: {
    isObject: 'cabinDoorKey',
    desc: { text: "a KEY" },
    objects: [
    ],
    loc: 'detritus1',
    vars: {
    },
    hooks: {
    },
  },
  knottedRoot1: {
    isObject: 'knottedRoot1',
    desc: { text: "a knotted ROOT" },
    objects: [
    ],
    loc: 'detritus1',
    vars: {
      noTake: 1,
    },
    hooks: {
    },
  },
  detritus1: {
    isObject: 'detritus1',
    desc: { text: "leafy DETRITUS on the ground" },
    objects: [
      'cabinDoorKey',
      'knottedRoot1',
    ],
    loc: 'cabinExterior',
    vars: {
      canUnpack: 1,
    },
    hooks: {
      prso: 'vDetritus1Prso',
    },
  },
};

export const translateSynonym = (word) => {
  switch (word) {
  case 'AX':
  case 'AXE':
    return ['axe'];
  case 'BOOT':
  case 'BOOTS':
    return ['boots'];
  case 'BOWL':
    return ['bowl'];
  case 'CABIN-DOOR':
  case 'DOOR':
    return ['cabinDoor'];
  case 'CABIN-DOOR-KEY':
  case 'DOOR-KEY':
  case 'KEY':
    return ['cabinDoorKey'];
  case 'WINDOW':
    return ['cabinWindow'];
  case 'CLOAK':
  case 'COAT':
    return ['cloak'];
  case 'CUP':
    return ['cup'];
  case 'CUTLERY':
    return ['cutlery'];
  case 'BRUSH':
  case 'DETRITUS':
    return ['detritus1'];
  case 'FLINT':
    return ['flint'];
  case 'KETTLE':
  case 'POT':
    return ['kettle'];
  case 'KNIFE':
    return ['knife'];
  case 'KNOTTED-ROOT':
  case 'ROOT':
    return ['knottedRoot1'];
  default:
    return null;
  }
}
