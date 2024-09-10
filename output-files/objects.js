export const objects = {
  journal: {
    isObject: 'journal',
    desc: { routine: 'journalDescFcn' },
    objects: [
    ],
    loc: 'player',
    vars: {
      pages: 0,
    },
    hooks: {
    },
  },
  tree: {
    isObject: 'tree',
    desc: { text: "A TREE" },
    objects: [
    ],
    loc: 'forest2',
    vars: {
      height: 20,
    },
    hooks: {
    },
  },
  monster: {
    isObject: 'monster',
    desc: { text: "A hideous five-eyed MONSTER" },
    objects: [
    ],
    loc: 'forest2',
    vars: {
      canBeAttacked: 1,
      canAttack: 1,
      health: 5,
    },
    hooks: {
    },
  },
};

export const lookupBySynonym = (word) => {
  switch (word) {
  case 'JOURNAL':
    return objects['journal'];
  case 'TREE':
    return objects['tree'];
  case 'MONSTER':
    return objects['monster'];
  default:
    return null;
  }
}
