export const player = {
  isPlayer: true,
  startRoom: 'forest1',
  vars: {
    health: 45,
    maxHealth: 50,
  },
  objects: {
    axe: ['inst_60',],
    cloak: ['inst_61',],
    cup: ['inst_63',],
    flint: ['inst_62',],
    kettle: ['inst_64',],
    knife: ['inst_65',],
  },
  hooks: {
    enter: 'playerActEnter',
    always: 'playerAlways',
  },
};

export const globals = {
  firstSoup: 1,
  fireInDraftyCabin: 0,
  foundMagicRing: 0,
  parentMonsterHealthStep: 8,
  parentMonsterAtkDmg: 40,
  childMonsterAtkDmg: 3,
  parentMonsterFirstEncounter: 1,
  parentMonsterWillChase: 0,
  childMonsterWillChase: 0,
  combatHighChance: 50,
  combatDamage: 0,
  combatMaxDamage: 0,
  forestBurnedDown: 0,
  firstForestPath: 0,
  talkPrompts: 0,
  killedCrow: 0,
  weather: 3,
  dry: 2,
  moonPhase: 1,
  day: 1,
  dayM3: 0,
  time: 1,
  totalSleeps: 0,
  napsToday: 0,
  totalFood: 0,
  maxMaxHealth: 100,
  firstInventoryLimit: 1,
  winEnterCabin: 0,
  winLightFire: 0,
  winCookMeal: 0,
  winFindGem: 0,
  winEnterCave: 0,
  winKillMonster: 0,
  winBuildBoat: 0,
  detailedDesc: 0,
  firstLookAround: 1,
  firstWhereAmI: 1,
  firstWhatIsHere: 1,
  firstExamine: 1,
  firstTake: 1,
};
