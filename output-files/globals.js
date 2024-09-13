export const player = {
  isPlayer: true,
  startRoom: 'forest1',
  vars: {
    health: 40,
  },
  objects: [
    'axe',
    'boots',
    'cloak',
    'flint',
    'bowl',
    'cup',
    'kettle',
    'cutlery',
    'knife',
  ],
  hooks: {
    enter: 'vDescRoom',
    always: 'timePasses',
  },
};

export const globals = {
  weather: 3,
  dry: 2,
  moon: 2,
  day: 1,
  time: 1,
  detailedDesc: 0,
  firstLookAround: 1,
  firstExamine: 1,
};
