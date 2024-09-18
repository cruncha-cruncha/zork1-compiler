export const player = {
  isPlayer: true,
  startRoom: 'forest1',
  vars: {
    health: 40,
    maxHealth: 50,
  },
  objects: {
    axe: ['inst_14',],
    cloak: ['inst_15',],
    cup: ['inst_17',],
    flint: ['inst_16',],
    kettle: ['inst_18',],
    knife: ['inst_19',],
  },
  hooks: {
    enter: 'vDescRoom',
    always: 'timePasses',
  },
};

export const globals = {
  weather: 3,
  dry: 2,
  moonPhase: 2,
  day: 1,
  time: 1,
  pCapacity: 10,
  totalSleeps: 0,
  totalNaps: 0,
  totalFood: 0,
  detailedDesc: 0,
  firstLookAround: 1,
  firstExamine: 1,
};
