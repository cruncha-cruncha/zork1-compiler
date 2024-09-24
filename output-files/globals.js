export const player = {
  isPlayer: true,
  startRoom: 'forest1',
  vars: {
    health: 40,
    maxHealth: 50,
  },
  objects: {
    axe: ['inst_49',],
    cloak: ['inst_50',],
    cup: ['inst_52',],
    flint: ['inst_51',],
    kettle: ['inst_53',],
    knife: ['inst_54',],
  },
  hooks: {
    enter: 'playerActEnter',
    always: 'playerAlways',
  },
};

export const globals = {
  firstSoup: 1,
  foundMagicRing: 0,
  combatHighChance: 50,
  combatDamage: 0,
  combatMaxDamage: 0,
  forestBurnedDown: 0,
  weather: 3,
  dry: 2,
  moonPhase: 1,
  day: 1,
  time: 1,
  pCapacity: 10,
  totalSleeps: 0,
  napsToday: 0,
  totalFood: 0,
  detailedDesc: 0,
  firstLookAround: 1,
  firstWhereAmI: 1,
  firstWhatIsHere: 1,
  firstExamine: 1,
};
