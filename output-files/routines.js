import { game, getEmptyResource } from './engine.js';
import { player, globals } from './globals.js';
import { objects } from './objects.js';
import { rooms } from './rooms.js';

export const routines = {
  descCabin: descCabin,
  descCabinExterior: descCabinExterior,
  descBook: descBook,
  descCabinWindow: descCabinWindow,
  descAxe: descAxe,
  descKnife: descKnife,
  descFlint: descFlint,
  descCloak: descCloak,
  descKettle: descKettle,
  cabinEnter: cabinEnter,
  descNote: descNote,
  descBookPage: descBookPage,
  descCup: descCup,
  descTorch: descTorch,
  descFire: descFire,
  descBtnRm1: descBtnRm1,
  btnRm1North: btnRm1North,
  btnRm1East: btnRm1East,
  btnRm1South: btnRm1South,
  btnRm1West: btnRm1West,
  rockPileInRoom: rockPileInRoom,
  descBtnRm2: descBtnRm2,
  btnRm2East: btnRm2East,
  btnRm2South: btnRm2South,
  btnRm2West: btnRm2West,
  descMagicRing: descMagicRing,
  enterHoleRm: enterHoleRm,
  longHall1Enter: longHall1Enter,
  descMazeRest1: descMazeRest1,
  caveSpiderInRoom: caveSpiderInRoom,
  childMonsterInRoom: childMonsterInRoom,
  parentMonsterInRoom: parentMonsterInRoom,
  descChildMonster: descChildMonster,
  descParentMonster: descParentMonster,
  descObsidian: descObsidian,
  cavern1North: cavern1North,
  cryptSouth: cryptSouth,
  descCursedSkull: descCursedSkull,
  cursedSkullInPlayer: cursedSkullInPlayer,
  cursedSkullExitPlayer: cursedSkullExitPlayer,
  descWeaponSharpness: descWeaponSharpness,
  waterfallPassageUp: waterfallPassageUp,
  descStoneDoor: descStoneDoor,
  descBones: descBones,
  passage1Enter: passage1Enter,
  waterfallPassageEnter: waterfallPassageEnter,
  descCaveLake: descCaveLake,
  descCavern1: descCavern1,
  descCrypt: descCrypt,
  descWaterfallPassage: descWaterfallPassage,
  caveLakeAlways: caveLakeAlways,
  rollDmg: rollDmg,
  descSword: descSword,
  descPickAxe: descPickAxe,
  descForest1: descForest1,
  descRiverStone: descRiverStone,
  caveEntrance1North: caveEntrance1North,
  caveEntrance1East: caveEntrance1East,
  caveEntrance1South: caveEntrance1South,
  caveEntrance1West: caveEntrance1West,
  caveEntrance2East: caveEntrance2East,
  caveEntrance2South: caveEntrance2South,
  cliff1East: cliff1East,
  cliff1Down: cliff1Down,
  lake1West: lake1West,
  caveEntrance2West: caveEntrance2West,
  forest1Enter: forest1Enter,
  forest2Enter: forest2Enter,
  lake1Enter: lake1Enter,
  forest3Enter: forest3Enter,
  forest4Enter: forest4Enter,
  forest5Enter: forest5Enter,
  forest6Enter: forest6Enter,
  field1Enter: field1Enter,
  field2Enter: field2Enter,
  cabinExteriorEnter: cabinExteriorEnter,
  forest5Exit: forest5Exit,
  forest6Exit: forest6Exit,
  cliff1Enter: cliff1Enter,
  descDetritus: descDetritus,
  descGem: descGem,
  gemEnterPlayer: gemEnterPlayer,
  descBoatFrame: descBoatFrame,
  descCaveEntrance1: descCaveEntrance1,
  descCaveEntrance2: descCaveEntrance2,
  descCliff1: descCliff1,
  descLake1: descLake1,
  descForest2: descForest2,
  descForest3: descForest3,
  descForest4: descForest4,
  descForest5: descForest5,
  descForest6: descForest6,
  descField1: descField1,
  descField2: descField2,
  lake1Always: lake1Always,
  snakeInPlayer: snakeInPlayer,
  drownFish: drownFish,
  bearInRoom: bearInRoom,
  playerAlways: playerAlways,
  checkPulse: checkPulse,
  inventoryLimit: inventoryLimit,
  clearQuestions: clearQuestions,
  burnFire: burnFire,
  timePasses: timePasses,
  weatherReport: weatherReport,
  checkFood: checkFood,
  checkSleep: checkSleep,
  playerActEnter: playerActEnter,
  recap: recap,
  roomHasLight: roomHasLight,
  monsterHealth: monsterHealth,
  burnForestDown: burnForestDown,
  vDescRoom: vDescRoom,
  vRoomDetail: vRoomDetail,
  vDescObjectsInRoom: vDescObjectsInRoom,
  addToFire: addToFire,
  addToStone: addToStone,
};

export const handlers = {
  add: {
    func: add,
    objHandlers: {
    },
  },
  bug: {
    func: bug,
    objHandlers: {
    },
  },
  cheat: {
    func: cheat,
    objHandlers: {
    },
  },
  debug: {
    func: debug,
    objHandlers: {
    },
  },
  drop: {
    func: drop,
    objHandlers: {
    },
  },
  eat: {
    func: eat,
    objHandlers: {
    },
  },
  empty: {
    func: empty,
    objHandlers: {
      book: {
        before: book_empty,
      },
    },
  },
  enter: {
    func: enter,
    objHandlers: {
    },
  },
  eunice: {
    func: eunice,
    objHandlers: {
    },
  },
  ever: {
    func: ever,
    objHandlers: {
    },
  },
  examine: {
    func: examine,
    objHandlers: {
    },
  },
  exit: {
    func: exit,
    objHandlers: {
    },
  },
  great: {
    func: great,
    objHandlers: {
    },
  },
  growing: {
    func: growing,
    objHandlers: {
    },
  },
  hit: {
    func: hit,
    objHandlers: {
      axe: {
        after: hit_axe,
      },
      bat: {
        before: bat_hit,
      },
      bear: {
        before: bear_hit,
      },
      bedFrame: {
        before: bedFrame_hit,
      },
      book: {
        before: book_hit,
      },
      cabinDoor: {
        before: cabinDoor_hit,
      },
      cabinWindow: {
        before: cabinWindow_hit,
      },
      caveSpider: {
        before: caveSpider_hit,
      },
      chair: {
        before: chair_hit,
      },
      childMonster: {
        before: childMonster_hit,
      },
      crow: {
        before: crow_hit,
      },
      cursedSkull: {
        before: cursedSkull_hit,
      },
      frog: {
        before: frog_hit,
      },
      knife: {
        after: hit_knife,
      },
      log: {
        before: log_hit,
      },
      mushroom: {
        before: mushroom_hit,
      },
      obsidianShard: {
        after: hit_obsidianShard,
      },
      owl: {
        before: owl_hit,
      },
      parentMonster: {
        before: parentMonster_hit,
      },
      pickAxe: {
        after: hit_pickAxe,
      },
      rabbit: {
        before: rabbit_hit,
      },
      rock: {
        before: rock_hit,
      },
      snake: {
        before: snake_hit,
      },
      stoneDoor: {
        before: stoneDoor_hit,
      },
      sword: {
        after: hit_sword,
      },
      table: {
        before: table_hit,
      },
      tree: {
        before: tree_hit,
      },
    },
  },
  inventory: {
    func: inventory,
    objHandlers: {
    },
  },
  jump: {
    func: jump,
    objHandlers: {
    },
  },
  listening: {
    func: listening,
    objHandlers: {
    },
  },
  look: {
    func: look,
    objHandlers: {
    },
  },
  no: {
    func: no,
    objHandlers: {
    },
  },
  open: {
    func: open,
    objHandlers: {
      cabinDoor: {
        before: cabinDoor_open,
      },
      stoneDoor: {
        before: stoneDoor_open,
      },
    },
  },
  pee: {
    func: pee,
    objHandlers: {
    },
  },
  school: {
    func: school,
    objHandlers: {
    },
  },
  sleep: {
    func: sleep,
    objHandlers: {
    },
  },
  spark: {
    objHandlers: {
      flint: {
        before: flint_spark,
      },
    },
  },
  swim: {
    func: swim,
    objHandlers: {
    },
  },
  take: {
    func: take,
    objHandlers: {
      bat: {
        after: take_bat,
      },
      bear: {
        after: take_bear,
      },
      caveSpider: {
        after: take_caveSpider,
      },
      detritus: {
        after: take_detritus,
      },
      fish: {
        after: take_fish,
      },
      frog: {
        after: take_frog,
      },
      magicRing: {
        after: take_magicRing,
      },
      rabbit: {
        after: take_rabbit,
      },
      rock: {
        after: take_rock,
      },
      rockPile: {
        after: take_rockPile,
      },
      snake: {
        after: take_snake,
      },
      sword: {
        before: sword_take,
      },
      water: {
        before: water_take,
      },
    },
  },
  talk: {
    objHandlers: {
      bat: {
        after: talk_bat,
      },
      bear: {
        after: talk_bear,
      },
      beetle: {
        after: talk_beetle,
      },
      crow: {
        after: talk_crow,
      },
      fish: {
        after: talk_fish,
      },
      frog: {
        after: talk_frog,
      },
      owl: {
        after: talk_owl,
      },
      rabbit: {
        after: talk_rabbit,
      },
      snake: {
        after: talk_snake,
      },
    },
  },
  time: {
    func: time,
    objHandlers: {
    },
  },
  weather: {
    func: weather,
    objHandlers: {
    },
  },
  what: {
    func: what,
    objHandlers: {
    },
  },
  where: {
    func: where,
    objHandlers: {
    },
  },
  work: {
    objHandlers: {
      bedFrame: {
        before: bedFrame_work,
      },
      boatFrame: {
        before: boatFrame_work,
      },
      bones: {
        before: bones_work,
      },
      cabinDoor: {
        before: cabinDoor_work,
      },
      cabinWindow: {
        before: cabinWindow_work,
      },
      chair: {
        before: chair_work,
      },
      detritus: {
        before: detritus_work,
      },
      pickAxe: {
        before: pickAxe_work,
      },
      riverStone: {
        before: riverStone_work,
      },
      table: {
        before: table_work,
      },
    },
  },
  write: {
    func: write,
    objHandlers: {
    },
  },
  yes: {
    func: yes,
    objHandlers: {
    },
  },
};

function descCabin(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("You're inside a rustic log cabin. It's well built. There's a window looking out over the fields. The furnishings are spartan, but it's clearly been lived in. The smell of smoke lingers in the air.", '\n');
  } else if (globals['winEnterCabin'] < 1) {
    game.log("You're inside a log cabin. There's signs of a previous occupant. It's safe to make a fire in here.", '\n');
  } else if (1 === 1) {
    game.log("You're inside a log cabin.", '\n');
  };
  return 0;
}

function descCabinExterior(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((rooms['cabinExterior'].vars['firstTime'] || 0) === 1) {
    rooms['cabinExterior'].vars['firstTime'] = 0;
    game.log("You're outside a cabin. You could try to OPEN DOOR", '\n');
    return 1;
  };
  game.log("You're outside a cabin on the edge of a forest, overlooking some fields.", '\n');
  if ((game.getInst(rooms['cabinExterior'], 'cabinDoor', false).vars['isLocked'] || 0) === 1) {
    game.log("The cabin door is locked. Where's the key? It's probably hidden under some leaves.", '\n');
  };
  return 0;
}

function descBook(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0) < 1) {
    game.log("a torn-up book");
    return 1;
  } else if (globals['detailedDesc'] === 1) {
    game.log("The delicate pages of this leather-bound journal are covered in scratchy handwriting, probably done with charcoal. You can make out a few passages:", '\n');
    game.log("...boat is coming along, should be ready...", '\n');
    game.log("...lost the gem, a crow must have...", '\n');
    game.log("...beast is hungry, what does it eat down there? No use mining any more...", '\n');
    return 1;
  };
  game.log("a BOOK");
  return 0;
}

function descCabinWindow(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (((locals['cmd']?.[0] ?? getEmptyResource()).vars['isBroken'] || 0) === 1) {
    game.log("a broken WINDOW", '\n');
  } else if (1 === 1) {
    game.log("a WINDOW", '\n');
  };
  return 0;
}

function descAxe(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("Your trusty axe. If you find one, you might be able to HIT TREE WITH AXE, get some logs to keep the fire going. But be careful, tools dull with use.", '\n');
    routines['descWeaponSharpness'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  } else if (1 === 1) {
    game.log("an AXE");
  };
  return 0;
}

function descKnife(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("A long knife. When in danger, you can HIT <thing> WITH KNIFE. But be careful, weapons dull with use.", '\n');
    routines['descWeaponSharpness'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  } else if (1 === 1) {
    game.log("a KNIFE");
  };
  return 0;
}

function descFlint(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("Can try to SPARK FLINT AT <something> to start a fire. The fire won't last very long without more substantial fuel though.", '\n');
  } else if (1 === 1) {
    game.log("FLINT");
  };
  return 0;
}

function descCloak(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("This cloak keeps you warm, and is very comfortable for sleeping.", '\n');
  } else if (1 === 1) {
    game.log("a CLOAK");
  };
  return 0;
}

function descKettle(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("You've cooked many a meal in this old, dented kettle. Start with ADD WATER TO KETTLE, then something edible.", '\n');
  } else if (1 === 1) {
    game.log("a KETTLE");
  };
  return 0;
}

function cabinEnter(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  globals['winEnterCabin'] = 1;
  game.move(locals, game.getInst(rooms['cabinExterior'], 'cabinWindow', false), locals['cRoom']);
  return 0;
}

function descNote(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("I'm not sure what you wrote down, hopefully something meaningful. You could leave it in the cabin for the next person who comes through here.", '\n');
  } else if (1 === 1) {
    game.log("a NOTE");
  };
  return 0;
}

function descBookPage(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("You could try to WRITE NOTE with this paper.", '\n');
  } else if (1 === 1) {
    game.log("some PAPER");
  };
  return 0;
}

function descCup(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("This cup can hold tea, and tea is just soup, but it's known to provide strength beyond that of it's contents.", '\n');
  } else if (1 === 1) {
    game.log("a CUP");
  };
  return 0;
}

function descTorch(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    if (((locals['cmd']?.[0] ?? getEmptyResource()).vars['isLit'] || 0) === 1) {
      game.log("This torch has ", ((locals['cmd']?.[0] ?? getEmptyResource()).vars['fuel'] || 0).toString(), " fuel, and is burning.", '\n');
    } else if (1 === 1) {
      game.log("This unlit torch has ", ((locals['cmd']?.[0] ?? getEmptyResource()).vars['fuel'] || 0).toString(), " fuel.", '\n');
    };
  } else if (((locals['cmd']?.[0] ?? getEmptyResource()).vars['isLit'] || 0) === 1) {
    game.log("a TORCH", '\n');
  } else if (1 === 1) {
    game.log("an unlit TORCH", '\n');
  };
  return 0;
}

function descFire(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("This fire has ", ((locals['cmd']?.[0] ?? getEmptyResource()).vars['fuel'] || 0).toString(), " fuel left.", '\n');
  } else if (1 === 1) {
    game.log("a FIRE", '\n');
  };
  return 0;
}

function descBtnRm1(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("You're in a room. There's a symbol on the floor:", '\n');
    game.log("a small circle, with a line going east", '\n');
    game.log("at the end of that line, a second small circle", '\n');
    game.log("coming out of the second small circle, a line going north", '\n');
    game.log("at the end of that line, an X", '\n');
    if ((rooms['btnRm1'].vars['step'] || 0) === 0) {
      game.log("The first circle is glowing blue", '\n');
    } else if ((rooms['btnRm1'].vars['step'] || 0) === 1) {
      game.log("The second circle is glowing blue", '\n');
    } else if ((rooms['btnRm1'].vars['step'] || 0) === 2) {
      game.log("The X is glowing blue", '\n');
    };
  } else if (1 === 1) {
    game.log("You're in a room. There's a symbol on the floor.", '\n');
  };
  return 0;
}

function btnRm1North(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((rooms['btnRm1'].vars['step'] || 0) === 1) {
    rooms['btnRm1'].vars['step'] = 2;
    if ((rooms['btnRm1'].vars['start'] || 0) === 0) {
      rooms['btnRm1'].vars['start'] = 2;
      game.move(locals, player, rooms['mazeRest2']);
      return 1;
    };
    game.log("You appear to be in the same room, but something has changed.", '\n');
  } else if (1 === 1) {
    rooms['btnRm1'].vars['step'] = (rooms['btnRm1'].vars['start'] || 0);
    game.log("You appear to be in the same room that you started from.", '\n');
  };
  return 0;
}

function btnRm1East(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((rooms['btnRm1'].vars['step'] || 0) === 0) {
    rooms['btnRm1'].vars['step'] = 1;
    game.log("You appear to be in the same room, but something has changed.", '\n');
  } else if (1 === 1) {
    rooms['btnRm1'].vars['step'] = (rooms['btnRm1'].vars['start'] || 0);
    game.log("You appear to be in the same room that you started from.", '\n');
  };
  return 0;
}

function btnRm1South(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((rooms['btnRm1'].vars['step'] || 0) === 2) {
    rooms['btnRm1'].vars['step'] = 1;
    game.log("You appear to be in the same room, but something has changed.", '\n');
  } else if (1 === 1) {
    rooms['btnRm1'].vars['step'] = (rooms['btnRm1'].vars['start'] || 0);
    game.log("You appear to be in the same room that you started from.", '\n');
  };
  return 0;
}

function btnRm1West(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((rooms['btnRm1'].vars['step'] || 0) === 1) {
    rooms['btnRm1'].vars['step'] = 0;
    if ((rooms['btnRm1'].vars['start'] || 0) === 2) {
      rooms['btnRm1'].vars['start'] = 0;
      game.move(locals, player, rooms['mazeRest1']);
      return 1;
    };
    game.log("You appear to be in the same room, but something has changed.", '\n');
  } else if (1 === 1) {
    rooms['btnRm1'].vars['step'] = (rooms['btnRm1'].vars['start'] || 0);
    game.log("You appear to be in the same room that you started from.", '\n');
  };
  return 0;
}

function rockPileInRoom(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (game.isEqual(locals['obj'], objects['rock'])) {
      game.move(locals, locals['obj'], );
    };
  };
  return 0;
}

function descBtnRm2(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("You're in a room. There are some symbols on the floor:", '\n');
    game.log("two small circles, one west and east, connected by a line", '\n');
    game.log("the number 3 is written just above the line", '\n');
    if ((rooms['btnRm2'].vars['step'] || 0) === 0) {
      game.log("The east circle glows faintly blue", '\n');
    } else if ((rooms['btnRm2'].vars['step'] || 0) === 1) {
      game.log("The west circle glows faintly blue", '\n');
    } else if ((rooms['btnRm2'].vars['step'] || 0) === 2) {
      game.log("The east circle glows a strong blue", '\n');
    } else if ((rooms['btnRm2'].vars['step'] || 0) === 3) {
      game.log("The west circle glows a strong blue", '\n');
    };
  } else if (1 === 1) {
    game.log("You're in a room. There are some symbols on the floor.", '\n');
  };
  return 0;
}

function btnRm2East(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((rooms['btnRm2'].vars['step'] || 0) === 1) {
    rooms['btnRm2'].vars['step'] = 2;
    game.log("You appear to be in the same room, but something has changed.", '\n');
  } else if (1 === 1) {
    rooms['btnRm2'].vars['step'] = 0;
    game.log("You appear to be in the same room that you started from.", '\n');
  };
  return 0;
}

function btnRm2South(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  rooms['btnRm2'].vars['step'] = 0;
  game.move(locals, player, rooms['mazeRest2']);
  return 0;
}

function btnRm2West(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((rooms['btnRm2'].vars['step'] || 0) === 0) {
    rooms['btnRm2'].vars['step'] = 1;
    game.log("You appear to be in the same room, but something has changed.", '\n');
  } else if ((rooms['btnRm2'].vars['step'] || 0) === 2) {
    rooms['btnRm2'].vars['step'] = 3;
    game.log("You appear to be in the same room, but something has changed.", '\n');
    if (globals['foundMagicRing'] === 0) {
      globals['foundMagicRing'] = 1;
      game.copyMove(objects['magicRing'], locals['cRoom']);
      game.log("And something shiny appeared on the ground.", '\n');
    } else if (Math.floor(Math.random() * 100) < 50) {
      game.copyMove(game.getInst(rooms['storage'], 'obsidianShard', false), locals['cRoom']);
      game.log("And something shiny appeared on the ground.", '\n');
    };
  } else if (1 === 1) {
    rooms['btnRm2'].vars['step'] = 0;
    game.log("You appear to be in the same room that you started from.", '\n');
  };
  return 0;
}

function descMagicRing(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("This intricately-carved silver ring weighs much more than it looks like it should. It may be magic.");
  } else if (1 === 1) {
    game.log("a RING");
  };
  return 0;
}

function enterHoleRm(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((!(game.isInLocation(locals['cRoom'], player, objects['torch'], false))) && (!(game.isInLocation(locals['cRoom'], player, objects['gem'], false)))) {
    game.log("Without light to see where you're going, you fall into the hole.", '\n');
    player.vars['health'] = -1;
  };
  return 0;
}

function longHall1Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  globals['winEnterCave'] = 1;
  return 0;
}

function descMazeRest1(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("You're in a lovely rocky cave. There's another space UP.", '\n');
  } else if (1 === 1) {
    game.log("You're in a lovely rocky cave.", '\n');
  };
  return 0;
}

function caveSpiderInRoom(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((50 > Math.floor(Math.random() * 100)) && (0 < (player.vars['health'] || 0))) {
    game.log("A spider attacks, you take one damage", '\n');
    player.vars['health'] = ((player.vars['health'] || 0) - 1);
  } else if (!(((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    game.log("There's a spider in here", '\n');
  };
  return 0;
}

function childMonsterInRoom(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['parentMonster'], false)) {
    globals['childMonsterWillChase'] = 0;
    game.move(locals, (locals['cmd']?.[0] ?? getEmptyResource()), rooms['storage']);
    return 0;
  } else if (!(((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    return 0;
  };
  if (!((game.isEqual(locals['cRoom'], rooms['den1'])) || (game.isEqual(locals['cRoom'], rooms['den2'])) || (game.isEqual(locals['cRoom'], rooms['den3'])) || (game.isEqual(locals['cRoom'], rooms['den4'])))) {
    game.log("The monster runs back to it's den", '\n');
    globals['childMonsterWillChase'] = 0;
    game.move(locals, (locals['cmd']?.[0] ?? getEmptyResource()), rooms['storage']);
    return 1;
  };
  if (80 > Math.floor(Math.random() * 100)) {
    game.log("A monster hits you for ", globals['childMonsterAtkDmg'].toString(), " health", '\n');
    player.vars['health'] = ((player.vars['health'] || 0) - globals['childMonsterAtkDmg']);
  } else if (1 === 1) {
    game.log("A monster attacks, but misses.", '\n');
  };
  globals['childMonsterWillChase'] = 1;
  return 0;
}

function parentMonsterInRoom(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (!(((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    return 0;
  } else if (globals['parentMonsterFirstEncounter'] === 1) {
    game.log("There's a monster here. It hits you for ");
    if (globals['parentMonsterAtkDmg'] < (player.vars['health'] || 0)) {
      game.log(globals['parentMonsterAtkDmg'].toString());
      player.vars['health'] = ((player.vars['health'] || 0) - globals['parentMonsterAtkDmg']);
    } else if (1 === 1) {
      locals['dmg'] = ((player.vars['health'] || 0) - 1);
      game.log(locals['dmg'].toString());
      player.vars['health'] = 1;
    };
    game.log(" health, then runs away. It must be scared of the light.", '\n');
    globals['parentMonsterFirstEncounter'] = 0;
    game.move(locals, (locals['cmd']?.[0] ?? getEmptyResource()), rooms['storage']);
    return 1;
  } else if (
  routines['roomHasLight'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']) === 1) {
    game.log("A monster runs from the room. It looks larger than it did last time.", '\n');
    globals['parentMonsterWillChase'] = -1;
  } else if (1 === 1) {
    globals['parentMonsterWillChase'] = 1;
  };
  if (50 > Math.floor(Math.random() * 100)) {
    game.log("A monster hits you for ", globals['parentMonsterAtkDmg'].toString(), " health", '\n');
    player.vars['health'] = ((player.vars['health'] || 0) - globals['parentMonsterAtkDmg']);
  } else if (1 === 1) {
    game.log("A monster attacks, but misses.", '\n');
  };
  return 0;
}

function descChildMonster(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("Health: ", ((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0).toString(), ", Max: ", ((locals['cmd']?.[0] ?? getEmptyResource()).vars['maxHealth'] || 0).toString(), ", Damage: ", globals['childMonsterAtkDmg'].toString(), '\n');
    if (!(((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
      game.log("This monster was small but well fed.", '\n');
    };
  } else if (1 === 1) {
    game.log("a MONSTER");
  };
  return 0;
}

function descParentMonster(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("Health: ", ((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0).toString(), ", Max: ", ((locals['cmd']?.[0] ?? getEmptyResource()).vars['maxHealth'] || 0).toString(), ", Damage: ", globals['parentMonsterAtkDmg'].toString(), '\n');
    if (!(((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
      game.log("This monster was experienced.", '\n');
    };
  } else if (1 === 1) {
    game.log("a MONSTER");
  };
  return 0;
}

function descObsidian(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("This obsidian shard is extremely sharp but very brittle; you might only get one good hit with it.");
  } else if (1 === 1) {
    game.log("an OBSIDIAN shard");
  };
  return 0;
}

function cavern1North(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((1 > (game.getInst(locals['cRoom'], 'stoneDoor', false).vars['health'] || 0)) || ((game.getInst(locals['cRoom'], 'stoneDoor', false).vars['isLocked'] || 0) === 0)) {
    game.move(locals, player, rooms['crypt']);
  } else if (game.isInLocation(locals['cRoom'], player, objects['masterKey'], false)) {
    game.log("You unlock the stone door.", '\n');
    game.move(locals, player, rooms['crypt']);
  } else if (1 === 1) {
    game.log("The stone door blocks your path.", '\n');
  };
  return 0;
}

function cryptSouth(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (((game.getInst(rooms['crypt'], 'stoneDoor', false).vars['isLocked'] || 0) === 0) || ((game.getInst(rooms['crypt'], 'stoneDoor', false).vars['health'] || 0) < 1)) {
    game.move(locals, player, rooms['cavern1']);
  } else if (1 === 1) {
    game.log("You press a small panel in the wall. A stone door slides open.", '\n');
    game.getInst(rooms['crypt'], 'stoneDoor', false).vars['isLocked'] = 0;
    game.move(locals, player, rooms['cavern1']);
  };
  return 0;
}

function descCursedSkull(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("The ancient king this skull belongs to was known to be jealous. Taking it can't be good.", '\n');
  } else if (1 === 1) {
    game.log("a SKULL");
  };
  return 0;
}

function cursedSkullInPlayer(cRoom, cmd) {
  const locals = {cRoom, cmd, af: 0};
  for (let object of game.getObjectsIn(player)) {
    locals['obj'] = object;
    if ((locals['obj'].vars['maxDamage'] || 0) > 0) {
      locals['obj'].vars['damage'] = 0;
      locals['af'] = 1;
    };
  };
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if ((locals['obj'].vars['maxDamage'] || 0) > 0) {
      locals['obj'].vars['damage'] = 0;
      locals['af'] = 1;
    };
  };
  if (locals['af'] === 1) {
    game.log("The skull makes all your weapons go dull.", '\n');
  };
  return 0;
}

function cursedSkullExitPlayer(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((game.isInLocation(locals['cRoom'], player, objects['sword'], false)) && ((player.vars['hasMagicRing'] || 0) === 0)) {
    game.log("The sword crumbles from your hand. You're left clenching an empty fist.", '\n');
    game.move(locals, game.getInst(player, 'sword', false), );
  };
  return 0;
}

function descWeaponSharpness(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[0] ?? getEmptyResource()), false)) {
    game.log(" Damage: ", ((locals['cmd']?.[0] ?? getEmptyResource()).vars['damage'] || 0).toString(), ", Max: ", ((locals['cmd']?.[0] ?? getEmptyResource()).vars['maxDamage'] || 0).toString(), '\n');
  };
  return 0;
}

function waterfallPassageUp(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], game.getInst(player, 'pickAxe', false), objects['strap'], false)) {
    game.log("Using the improvised grappling hook, you climb up the waterfall.", '\n');
    game.move(locals, player, rooms['cliff1']);
  } else if (1 === 1) {
    game.log("You're behind a waterfall, with no way to climb up. Perhaps you could jury-rig some sort of grappling hook?", '\n');
  };
  return 0;
}

function descStoneDoor(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((globals['detailedDesc'] === 1) && ((player.vars['tBat'] || 0) === 2)) {
    game.log("This heavy door is carved with jagged runes. It tells of an old king, jealous of all those around him. He died unfulfilled.", '\n');
  } else if (globals['detailedDesc'] === 1) {
    game.log("This heavy door is carved with runes. It tells of an old king.", '\n');
  } else if (1 === 1) {
    game.log("a stone DOOR");
  };
  return 0;
}

function descBones(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("These bones could be human, but some of them look larger. You could use one of the chips as a needle, to help you weave some straps (WORK). You could make a nice bone broth. Or, with a sharp enough tool, whittle a key.", '\n');
  } else if (1 === 1) {
    game.log("some BONES");
  };
  return 0;
}

function passage1Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  globals['winEnterCave'] = 1;
  return 0;
}

function waterfallPassageEnter(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  globals['winEnterCave'] = 1;
  return 0;
}

function descCaveLake(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("You're on the shore of a large underground lake. The ceiling is low. Water drips from stalactites. Some hang down into the surface of the water, forming pillars with their reflection.", '\n');
  } else if (1 === 1) {
    game.log("You're on the shore of an underground lake.", '\n');
  };
  return 0;
}

function descCavern1(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("You're in a massive cavern, partially natural, partially carved by human hand. It must have taken a lot of work. Is that the ceiling? Or is it just darkness? You voice echoes indeterminately.", '\n');
  } else if (1 === 1) {
    game.log("You're in a massive cavern.", '\n');
  };
  return 0;
}

function descCrypt(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("You're in a crypt with a stone coffin in the center. The walls are plain. Significant effort must have gone into making them so.", '\n');
  } else if (1 === 1) {
    game.log("You're in a crypt.", '\n');
  };
  return 0;
}

function descWaterfallPassage(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("You're in a narrow passage. To the west, a crypt. To the east, a wall of water. You must be behind a waterfall. There's no way UP unless you can jury-rig some climbing equipment. Have you found anything hook-like down here?", '\n');
  } else if (1 === 1) {
    game.log("You're in a narrow passage.", '\n');
  };
  return 0;
}

function caveLakeAlways(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['water'], false))) {
    game.copyMove(objects['water'], locals['cRoom']);
  };
  return 0;
}

function rollDmg(cRoom, cmd) {
  const locals = {cRoom, cmd, coef: 0,dmg: 0};
  if (Math.floor(Math.random() * 100) < globals['combatHighChance']) {
    locals['coef'] = 1;
  } else if (1 === 1) {
    locals['coef'] = -1;
  };
  if (globals['moonPhase'] === 4) {
    locals['dmg'] = (globals['combatDamage'] + (globals['combatMaxDamage'] * locals['coef']));
  } else if (1 === 1) {
    locals['dmg'] = ((globals['combatDamage'] * (1 + Math.floor(((player.vars['health'] || 0) - Math.floor((player.vars['maxHealth'] || 0) / 2)) / (player.vars['maxHealth'] || 0)))) + (locals['coef'] * Math.floor(globals['combatMaxDamage'] / 4)));
  };
  if (locals['dmg'] < 0) {
    player.vars['health'] = ((player.vars['health'] || 0) + locals['dmg']);
    game.log("Your weapon is dull, causing you to injure yourself.", '\n');
    locals['dmg'] = 0;
  };
  return locals['dmg'];
  return 0;
}

function descSword(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("The intricately-carved handle on this sword warns that only those worthy may wield it.", '\n');
    routines['descWeaponSharpness'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  } else if (1 === 1) {
    game.log("a SWORD");
  };
  return 0;
}

function descPickAxe(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("This deft tool doesn't look like it will lose it's edge any time soon. It's a work horse.", '\n');
    if (game.isInLocation(locals['cRoom'], (locals['cmd']?.[0] ?? getEmptyResource()), objects['strap'], false)) {
      game.log("It's got a strap tied to it.", '\n');
    } else if (50 > Math.floor(Math.random() * 100)) {
      game.log("It could be used as a grappling hook if you tied some rope to it.", '\n');
    } else if (1 === 1) {
      game.log("It's clearly been used to mine for gold, tirelessly hitting rocks.", '\n');
    };
  } else if (1 === 1) {
    game.log("a PICK-AXE");
  };
  return 0;
}

function descForest1(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((rooms['forest1'].vars['firstTime'] || 0) === 1) {
    rooms['forest1'].vars['firstTime'] = 0;
    game.log("It's the end of summer. Birds and other small creatures are rustling through the forest. Light filters down through long green needles, and the smell of pine hangs thick in the air.", '\n');
    routines['weatherReport'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    game.log("There's a trail up ahead, it looks like you could GO NORTH", '\n');
  } else if ((globals['detailedDesc'] === 1) || (10 > Math.floor(Math.random() * 100))) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("You've followed a trail deep into the forest. You crouch down, and see nothing but the trunks of black spruce and jack pine. Even during a thunderstorm, the wind will barely penetrate this far down, leaving the air hot and thick with the smell of sap.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("You're deep in the forest, following a well-defined trail. The ground is matted in a solid layer of orange needles, long and slender, dotted with small pine cones and broken by the occasional outcropping of white limestone. Black spruce and jack pine sway their branches overhead. You spot a clump of dark brown hair caught on a branch.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("You've walked far into the forest. It's been nothing but black, white, and red spruce for awhile now. The trail is firm and well-packed under your feet. The terrain here is fairly level, which makes for easy walking.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("You're in a dense forest. Well, the canopy is dense, and you often have to duck under branches. The undergrowth is surprisingly spare. Most small plants are unable to push there way through the thick matt of pine needles, long and orange. You accidentally step on a snail.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're in a dense forest", '\n');
  };
  return 0;
}

function descRiverStone(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("A flat and smooth river stone. Perfect for sharpening weapons: WORK <weapon> WITH STONE. But make sure it's wet first: ADD WATER TO STONE.", '\n');
  } else if (1 === 1) {
    game.log("a smooth STONE");
  };
  return 0;
}

function caveEntrance1North(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['forestBurnedDown'] === 1) {
    game.log("There's nothing but charred debris as far as the eye can see.", '\n');
  } else if (1 === 1) {
    game.move(locals, player, rooms['forest5']);
  };
  return 0;
}

function caveEntrance1East(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['forestBurnedDown'] === 1) {
    game.log("There's nothing but charred debris as far as the eye can see.", '\n');
  } else if (1 === 1) {
    game.move(locals, player, rooms['forest1']);
  };
  return 0;
}

function caveEntrance1South(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['forestBurnedDown'] === 1) {
    game.log("Charred debris blocks your way.", '\n');
  } else if (1 === 1) {
    game.move(locals, player, rooms['forest1']);
  };
  return 0;
}

function caveEntrance1West(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['forestBurnedDown'] === 1) {
    game.log("Charred debris blocks your way.", '\n');
  } else if (1 === 1) {
    game.move(locals, player, rooms['forest5']);
  };
  return 0;
}

function caveEntrance2East(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['forestBurnedDown'] === 1) {
    game.log("Charred debris blocks your way.", '\n');
  } else if (1 === 1) {
    game.move(locals, player, rooms['forest4']);
  };
  return 0;
}

function caveEntrance2South(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['forestBurnedDown'] === 1) {
    game.log("There's nothing but charred debris as far as the eye can see.", '\n');
  } else if (1 === 1) {
    game.move(locals, player, rooms['forest4']);
  };
  return 0;
}

function cliff1East(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("You leap off the cliff.", '\n');
  player.vars['health'] = 0;
  return 0;
}

function cliff1Down(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], game.getInst(player, 'pickAxe', false), objects['strap'], false)) {
    game.log("Using the improvised grappling hook, you climb down the waterfall.", '\n');
    game.move(locals, player, rooms['waterfallPassage']);
  } else if (1 === 1) {
    game.log("It's too dangerous to go down without some sort of climbing equipment. Perhaps you could improvise a grappling hook with some things?", '\n');
  };
  return 0;
}

function lake1West(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("You can't swim that far", '\n');
  return 0;
}

function caveEntrance2West(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['forestBurnedDown'] === 1) {
    game.log("Charred debris blocks your way.", '\n');
  } else if (1 === 1) {
    game.move(locals, player, rooms['forest6']);
  };
  return 0;
}

function forest1Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, dc: 0};
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (game.isEqual(locals['obj'], objects['detritus'])) {
      locals['dc'] = (locals['dc'] + 1);
    };
  };
  if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['tree'], false))) {
    game.copyMove(objects['tree'], locals['cRoom']);
  };
  if ((locals['dc'] * 45) < Math.floor(Math.random() * 100)) {
    game.copyMove(objects['detritus'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['stick'], false))) && (34 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['stick'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['root'], false))) && (12 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['root'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['mushroom'], false))) && (10 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['mushroom'], locals['cRoom']);
  };
  if ((1 < globals['firstForestPath']) && (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['rabbit'], false))) && (25 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['rabbit'], locals['cRoom']);
  } else if (75 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'rabbit', false), );
  };
  if ((0 > globals['time']) && (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['owl'], false))) && (20 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['owl'], locals['cRoom']);
  } else if (25 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'owl', false), );
  };
  globals['firstForestPath'] = (globals['firstForestPath'] + 1);
  return 0;
}

function forest2Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, dc: 0,hasDead: 0};
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (game.isEqual(locals['obj'], objects['detritus'])) {
      locals['dc'] = (locals['dc'] + 1);
    } else if (((locals['obj'].vars['isAnimal'] || 0) === 1) && ((locals['obj'].vars['health'] || 0) < 1)) {
      locals['hasDead'] = 1;
    };
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['stick'], false))) && (20 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['stick'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['fern'], false))) && (34 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['fern'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nuts'], false))) && (25 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['nuts'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['mushroom'], false))) && (15 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['mushroom'], locals['cRoom']);
  };
  if ((1 < globals['firstForestPath']) && (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['rabbit'], false))) && (40 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['rabbit'], locals['cRoom']);
    if (globals['talkPrompts'] < 2) {
      globals['talkPrompts'] = (globals['talkPrompts'] + 1);
      game.log("There's a rabbit here, bouncing up and down like it has something to say. You could try to TALK TO it", '\n');
    };
  } else if (60 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'rabbit', false), );
  };
  if ((0 > globals['time']) && (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['owl'], false))) && (!(globals['weather'] === 0)) && ((80 > Math.floor(Math.random() * 100)) || (locals['hasDead'] === 1))) {
    game.copyMove(objects['owl'], locals['cRoom']);
    if (20 > Math.floor(Math.random() * 100)) {
      game.log("An owl hoots in the trees.", '\n');
    };
  } else if (25 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'owl', false), );
  };
  if ((1 < globals['firstForestPath']) && (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['crow'], false))) && (!(globals['killedCrow'] === 1))) {
    if ((locals['hasDead'] === 1) && (75 > Math.floor(Math.random() * 100))) {
      game.copyMove(objects['crow'], locals['cRoom']);
      if (50 > Math.floor(Math.random() * 100)) {
        game.log("A crow squawks at you", '\n');
      };
    } else if (10 > Math.floor(Math.random() * 100)) {
      game.copyMove(objects['crow'], locals['cRoom']);
    };
  } else if ((90 > Math.floor(Math.random() * 100)) || (globals['killedCrow'] === 1)) {
    game.move(locals, game.getInst(locals['cRoom'], 'crow', false), );
  };
  globals['firstForestPath'] = (globals['firstForestPath'] + 1);
  return 0;
}

function lake1Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, blc: 0,hasDead: 0};
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (game.isEqual(locals['obj'], objects['bulrush'])) {
      locals['blc'] = (locals['blc'] + 1);
    } else if (((locals['obj'].vars['isAnimal'] || 0) === 1) && ((locals['obj'].vars['health'] || 0) < 1)) {
      locals['hasDead'] = 1;
    };
  };
  for (let i of Array.from(Array(Math.max(0, (2 - locals['blc']))).keys())) {
    locals['val'] = i;
    game.copyMove(objects['bulrush'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['frog'], false))) && (70 > Math.floor(Math.random() * 100)) && (!(globals['weather'] === 0))) {
    game.copyMove(objects['frog'], locals['cRoom']);
    if (globals['talkPrompts'] < 2) {
      globals['talkPrompts'] = (globals['talkPrompts'] + 1);
      game.log("There's a frog here, opening and closing it's mouth as if trying to say something. You could try to TALK TO it", '\n');
    };
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['rabbit'], false))) && (25 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['rabbit'], locals['cRoom']);
  } else if (1 === 1) {
    game.move(locals, game.getInst(locals['cRoom'], 'rabbit', false), );
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['snake'], false))) && (!(globals['weather'] === 0)) && (25 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['snake'], locals['cRoom']);
  } else if (75 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'snake', false), );
  };
  if ((0 > globals['time']) && (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['owl'], false))) && (!(globals['weather'] === 0)) && ((50 > Math.floor(Math.random() * 100)) || (locals['hasDead'] === 1))) {
    game.copyMove(objects['owl'], locals['cRoom']);
    if (25 > Math.floor(Math.random() * 100)) {
      game.log("An owl blinks in the moonlight.", '\n');
    };
  } else if (25 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'owl', false), );
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['crow'], false))) && (!(globals['weather'] === 0)) && (!(globals['killedCrow'] === 1))) {
    if ((locals['hasDead'] === 1) && (75 > Math.floor(Math.random() * 100))) {
      game.copyMove(objects['crow'], locals['cRoom']);
      if (50 > Math.floor(Math.random() * 100)) {
        game.log("A crow squawks at you", '\n');
      };
    } else if (10 > Math.floor(Math.random() * 100)) {
      game.copyMove(objects['crow'], locals['cRoom']);
    };
  } else if ((90 > Math.floor(Math.random() * 100)) || (globals['killedCrow'] === 1)) {
    game.move(locals, game.getInst(locals['cRoom'], 'crow', false), );
  };
  return 0;
}

function forest3Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, hasDead: 0};
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (((locals['obj'].vars['isAnimal'] || 0) === 1) && ((locals['obj'].vars['health'] || 0) < 1)) {
      locals['hasDead'] = 1;
    };
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['stick'], false))) && (34 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['stick'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['fern'], false))) && (25 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['fern'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['herbs'], false))) && (25 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['herbs'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['mushroom'], false))) && (10 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['mushroom'], locals['cRoom']);
  };
  if ((0 > globals['time']) && (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['owl'], false))) && (!(globals['weather'] === 0)) && ((80 > Math.floor(Math.random() * 100)) || (locals['hasDead'] === 1))) {
    game.copyMove(objects['owl'], locals['cRoom']);
    if (20 > Math.floor(Math.random() * 100)) {
      game.log("An owl hoots in the trees.", '\n');
    };
  } else if (50 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'owl', false), );
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['crow'], false))) && (!(globals['killedCrow'] === 1))) {
    if ((locals['hasDead'] === 1) && (75 > Math.floor(Math.random() * 100))) {
      game.copyMove(objects['crow'], locals['cRoom']);
      if (50 > Math.floor(Math.random() * 100)) {
        game.log("A crow squawks at you", '\n');
      };
    } else if (10 > Math.floor(Math.random() * 100)) {
      game.copyMove(objects['crow'], locals['cRoom']);
    };
  } else if ((75 > Math.floor(Math.random() * 100)) || (globals['killedCrow'] === 1)) {
    game.move(locals, game.getInst(locals['cRoom'], 'crow', false), );
  };
  if (game.isInLocation(locals['cRoom'], game.getInst(locals['cRoom'], 'treeHollow', false), objects['gem'], false)) {
    game.log("There's a hollow in that tree over there, looks like a good place to hide things.", '\n');
  };
  return 0;
}

function forest4Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, hasDead: 0};
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (((locals['obj'].vars['isAnimal'] || 0) === 1) && ((locals['obj'].vars['health'] || 0) < 1)) {
      locals['hasDead'] = 1;
    };
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['stick'], false))) && (34 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['stick'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['berries'], false))) && (25 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['berries'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nuts'], false))) && (12 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['nuts'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['fern'], false))) && (20 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['fern'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['mushroom'], false))) && (10 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['mushroom'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['frog'], false))) && (33 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['frog'], locals['cRoom']);
  } else if (18 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'frog', false), );
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['rabbit'], false))) && (40 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['rabbit'], locals['cRoom']);
    if (globals['talkPrompts'] < 2) {
      globals['talkPrompts'] = (globals['talkPrompts'] + 1);
      game.log("There's a rabbit here, bouncing up and down like it has something to say. You could try to TALK TO it", '\n');
    };
  } else if (60 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'rabbit', false), );
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['snake'], false))) && (67 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['snake'], locals['cRoom']);
  } else if (50 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'snake', false), );
  };
  if ((0 > globals['time']) && (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['owl'], false))) && (!(globals['weather'] === 0)) && ((80 > Math.floor(Math.random() * 100)) || (locals['hasDead'] === 1))) {
    game.copyMove(objects['owl'], locals['cRoom']);
    if (20 > Math.floor(Math.random() * 100)) {
      game.log("An owl hoots in the trees.", '\n');
    };
  } else if (25 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'owl', false), );
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['crow'], false))) && (!(globals['killedCrow'] === 1))) {
    if ((locals['hasDead'] === 1) && (75 > Math.floor(Math.random() * 100))) {
      game.copyMove(objects['crow'], locals['cRoom']);
      if (50 > Math.floor(Math.random() * 100)) {
        game.log("A crow squawks at you", '\n');
      };
    } else if (10 > Math.floor(Math.random() * 100)) {
      game.copyMove(objects['crow'], locals['cRoom']);
    };
  } else if ((75 > Math.floor(Math.random() * 100)) || (globals['killedCrow'] === 1)) {
    game.move(locals, game.getInst(locals['cRoom'], 'crow', false), );
  };
  return 0;
}

function forest5Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, bIn: 0};
  if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['tree'], false))) {
    game.copyMove(objects['tree'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['detritus'], false))) && (67 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['detritus'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['root'], false))) && (40 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['root'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['mushroom'], false))) && (10 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['mushroom'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['beetle'], false))) && (40 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['beetle'], locals['cRoom']);
  } else if (67 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'beetle', false), );
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['owl'], false))) && (20 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['owl'], locals['cRoom']);
    if (90 > Math.floor(Math.random() * 100)) {
      game.log("You somehow spot an owl in the dense trees.", '\n');
    };
  } else if (40 === Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'owl', false), );
  };
  if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['bear'], false))) {
    for (let object of game.getObjectsIn(objects['bear'])) {
      locals['b'] = object;
      if ((game.isInLocation(locals['cRoom'], rooms['storage'], locals['b'], false)) && ((locals['b'].vars['health'] || 0) > 0) && (25 > Math.floor(Math.random() * 100)) && (locals['bIn'] === 0)) {
        game.move(locals, locals['b'], locals['cRoom']);
        locals['b'].vars['asked'] = 0;
        locals['bIn'] = 1;
      };
    };
  } else if (1 === 1) {
    game.move(locals, game.getInst(locals['cRoom'], 'bear', false), );
  };
  return 0;
}

function forest6Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, bIn: 0,dc: 0};
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (game.isEqual(locals['obj'], objects['detritus'])) {
      locals['dc'] = (locals['dc'] + 1);
    };
  };
  if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['tree'], false))) {
    game.copyMove(objects['tree'], locals['cRoom']);
  };
  if ((locals['dc'] * 14) < Math.floor(Math.random() * 100)) {
    game.copyMove(objects['detritus'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nuts'], false))) && (34 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['nuts'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['root'], false))) && (40 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['root'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['mushroom'], false))) && (25 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['mushroom'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['rabbit'], false))) && (60 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['rabbit'], locals['cRoom']);
    if (globals['talkPrompts'] < 2) {
      globals['talkPrompts'] = (globals['talkPrompts'] + 1);
      game.log("There's a rabbit here, bouncing up and down like it has something to say. You could try to TALK TO it", '\n');
    };
  } else if (50 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'rabbit', false), );
  };
  if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['bear'], false))) {
    for (let object of game.getObjectsIn(objects['bear'])) {
      locals['b'] = object;
      if ((game.isInLocation(locals['cRoom'], rooms['storage'], locals['b'], false)) && ((locals['b'].vars['health'] || 0) > 0) && (25 > Math.floor(Math.random() * 100)) && (locals['bIn'] === 0)) {
        game.move(locals, locals['b'], locals['cRoom']);
        locals['b'].vars['asked'] = 0;
        locals['bIn'] = 1;
      };
    };
  } else if (1 === 1) {
    game.move(locals, game.getInst(locals['cRoom'], 'bear', false), );
  };
  return 0;
}

function field1Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, bc: 0,hasDead: 0};
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (game.isEqual(locals['obj'], objects['berries'])) {
      locals['bc'] = (locals['bc'] + 1);
    } else if (((locals['obj'].vars['isAnimal'] || 0) === 1) && ((locals['obj'].vars['health'] || 0) < 1)) {
      locals['hasDead'] = 1;
    };
  };
  if ((locals['bc'] < 2) && (25 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['berries'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['herbs'], false))) && (67 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['herbs'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['snake'], false))) && (!(globals['weather'] === 0)) && (67 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['snake'], locals['cRoom']);
  } else if (globals['weather'] === 0) {
    game.move(locals, game.getInst(locals['cRoom'], 'snake', false), );
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['beetle'], false))) && (40 > Math.floor(Math.random() * 100)) && (!(globals['weather'] === 0))) {
    game.copyMove(objects['beetle'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['rabbit'], false))) && (!(globals['weather'] === 0)) && (67 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['rabbit'], locals['cRoom']);
    if (globals['talkPrompts'] < 2) {
      globals['talkPrompts'] = (globals['talkPrompts'] + 1);
      game.log("There's a rabbit here, bouncing up and down like it has something to say. You could try to TALK TO it", '\n');
    };
  } else if (50 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'rabbit', false), );
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['crow'], false))) && (!(globals['weather'] === 0)) && (!(globals['killedCrow'] === 1))) {
    if ((locals['hasDead'] === 1) && (75 > Math.floor(Math.random() * 100))) {
      game.copyMove(objects['crow'], locals['cRoom']);
      if (50 > Math.floor(Math.random() * 100)) {
        game.log("A crow squawks at you", '\n');
      };
    } else if (12 > Math.floor(Math.random() * 100)) {
      game.copyMove(objects['crow'], locals['cRoom']);
    };
  } else if ((90 > Math.floor(Math.random() * 100)) || (globals['killedCrow'] === 1)) {
    game.move(locals, game.getInst(locals['cRoom'], 'crow', false), );
  };
  return 0;
}

function field2Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, bc: 0,hasDead: 0};
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (game.isEqual(locals['obj'], objects['berries'])) {
      locals['bc'] = (locals['bc'] + 1);
    } else if (((locals['obj'].vars['isAnimal'] || 0) === 1) && ((locals['obj'].vars['health'] || 0) < 1)) {
      locals['hasDead'] = 1;
    };
  };
  if ((locals['bc'] < 2) && (40 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['berries'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['herbs'], false))) && (40 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['herbs'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['beetle'], false))) && (40 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['beetle'], locals['cRoom']);
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['rabbit'], false))) && (!(globals['weather'] === 0)) && (75 > Math.floor(Math.random() * 100))) {
    game.copyMove(objects['rabbit'], locals['cRoom']);
    if (globals['talkPrompts'] < 2) {
      globals['talkPrompts'] = (globals['talkPrompts'] + 1);
      game.log("There's a rabbit here, bouncing up and down like it has something to say. You could try to TALK TO it", '\n');
    };
  } else if (50 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'rabbit', false), );
  };
  if ((!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['crow'], false))) && (!(globals['weather'] === 0)) && (!(globals['killedCrow'] === 1))) {
    if ((locals['hasDead'] === 1) && (75 > Math.floor(Math.random() * 100))) {
      game.copyMove(objects['crow'], locals['cRoom']);
      if (50 > Math.floor(Math.random() * 100)) {
        game.log("A crow squawks at you", '\n');
      };
    } else if (12 > Math.floor(Math.random() * 100)) {
      game.copyMove(objects['crow'], locals['cRoom']);
    };
  } else if ((90 > Math.floor(Math.random() * 100)) || (globals['killedCrow'] === 1)) {
    game.move(locals, game.getInst(locals['cRoom'], 'crow', false), );
  };
  return 0;
}

function cabinExteriorEnter(cRoom, cmd) {
  const locals = {cRoom, cmd, hasDead: 0};
  game.move(locals, game.getInst(rooms['cabin'], 'cabinWindow', false), locals['cRoom']);
  if ((0 > globals['time']) && (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['owl'], false))) && (!(globals['weather'] === 0)) && ((75 > Math.floor(Math.random() * 100)) || (locals['hasDead'] === 1))) {
    game.copyMove(objects['owl'], locals['cRoom']);
    if (10 > Math.floor(Math.random() * 100)) {
      game.log("An owl watches you closely.", '\n');
    };
  } else if (50 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'owl', false), );
  };
  if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['crow'], false))) {
    for (let object of game.getObjectsIn(locals['cRoom'])) {
      locals['obj'] = object;
      if (((locals['obj'].vars['isAnimal'] || 0) === 1) && ((locals['obj'].vars['health'] || 0) < 1)) {
        locals['hasDead'] = 1;
      };
    };
    if ((game.getInst(locals['cRoom'], 'cabinWindow', false).vars['isBroken'] || 0) === 1) {
    };
    if (((locals['hasDead'] === 1) || ((game.getInst(locals['cRoom'], 'cabinWindow', false).vars['isBroken'] || 0) === 1)) && (90 > Math.floor(Math.random() * 100))) {
      game.copyMove(objects['crow'], locals['cRoom']);
      if (50 > Math.floor(Math.random() * 100)) {
        game.log("A crow squawks at you", '\n');
      };
    };
  } else if ((25 > Math.floor(Math.random() * 100)) || (globals['killedCrow'] === 1)) {
    game.move(locals, game.getInst(locals['cRoom'], 'crow', false), );
  };
  return 0;
}

function forest5Exit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  for (let object of game.getObjectsIn(objects['bear'])) {
    locals['b'] = object;
    if ((game.isInLocation(locals['cRoom'], locals['cRoom'], locals['b'], false)) && ((locals['b'].vars['health'] || 0) > 0)) {
      game.move(locals, locals['b'], rooms['storage']);
    };
  };
  return 0;
}

function forest6Exit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  for (let object of game.getObjectsIn(objects['bear'])) {
    locals['b'] = object;
    if ((game.isInLocation(locals['cRoom'], locals['cRoom'], locals['b'], false)) && ((locals['b'].vars['health'] || 0) > 0)) {
      game.move(locals, locals['b'], rooms['storage']);
    };
  };
  return 0;
}

function cliff1Enter(cRoom, cmd) {
  const locals = {cRoom, cmd, hasDead: 0};
  if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['crow'], false))) {
    for (let object of game.getObjectsIn(locals['cRoom'])) {
      locals['obj'] = object;
      if (((locals['obj'].vars['isAnimal'] || 0) === 1) && ((locals['obj'].vars['health'] || 0) < 1)) {
        locals['hasDead'] = 1;
      };
    };
    if ((locals['hasDead'] === 1) && (90 > Math.floor(Math.random() * 100))) {
      game.copyMove(objects['crow'], locals['cRoom']);
      if (50 > Math.floor(Math.random() * 100)) {
        game.log("A crow squawks at you", '\n');
      };
    } else if (34 > Math.floor(Math.random() * 100)) {
      game.copyMove(objects['crow'], locals['cRoom']);
    };
  } else if (50 > Math.floor(Math.random() * 100)) {
    game.move(locals, game.getInst(locals['cRoom'], 'crow', false), );
  };
  return 0;
}

function descDetritus(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("Various grasses, leaves, and twigs are strewn about. This detritus could be good for starting a fire, or weaving into some rope-like straps (WORK with the right tool).", '\n');
    game.log("The word DETRITUS is a little long, you could use BRUSH instead.", '\n');
  } else if (1 === 1) {
    game.log("leafy DETRITUS");
  };
  return 0;
}

function descGem(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("This gem sparkles with unnatural brilliance. Carrying it will light up even the darkest caverns.", '\n');
  } else if (1 === 1) {
    game.log("a GEM");
  };
  return 0;
}

function gemEnterPlayer(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  globals['winFindGem'] = 1;
  return 0;
}

function descBoatFrame(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("This is the skeleton of a small wooden boat. It's missing some BOARDs, NAILS, STRAPs, and BOILED-SAP. Can WORK BOAT WITH <item>. In general, can WORK <thing> WITH <thing>.", '\n');
  } else if (1 === 1) {
    game.log("a BOAT frame");
  };
  return 0;
}

function descCaveEntrance1(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("The forest hums and buzzes around you. Soft earth gives way to a jagged maw, beckoning you DOWN.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("Twigs snap underfoot. Amidst boulders and tree trunks a black void beckons you forward. A cave. You could GO DOWN, but there is no telling whether you would come back up.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("Lot's of creatures are crawling around the forest at this time of year. Some might slither. A few might stalk. Off to the side of the path is the entrance to a cave, large enough for you to slip DOWN, or for something to come up.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("The air is a little cooler at this end of the forest, but still hangs with summer smells. There's no break in the trees, save for a clump of rocks, with a black hole at the center. It leads DOWN. How far?", '\n');
    };
    if (!((game.isInLocation(locals['cRoom'], player, objects['gem'], false)) || (game.isInLocation(locals['cRoom'], player, objects['torch'], false)))) {
      game.log("Some sort of light source might help you navigate below.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're at the entrace to a cave.", '\n');
  };
  return 0;
}

function descCaveEntrance2(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("This was clearly a well-trodden path, but has fallen into dis-use. How many feet have walked this ground? How many hooves? Not many flowers this far into the forest, not enough light for them. Or they've all been eaten by a fawn. You hear water dripping faintly, echoing against rock. It could be coming from this large hole in the ground. GO DOWN to find out.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("You notice some broken branches off of the path, signs an animal has come this way. There are animals all over the forest this time of year. It could be a deer, or a bear, or something that lives in that hole. You can't see much, would have to GO DOWN to investigate.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("The path is flanked by high brush on both sides, the forest canopy thick overhead. It's almost like walking down a tunnel. But through the foliage you spot an even darker area. A wide gap in the ground, as if the top has come off a bottle. You could GO DOWN.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("A large hole in the ground is hidden behind a screen of brush. You could GO DOWN. It doesn't look like there's anything living there, no path connecting this hole to the main trail, but who's to say it's not some careful creature.", '\n');
    };
    if (!((game.isInLocation(locals['cRoom'], player, objects['gem'], false)) || (game.isInLocation(locals['cRoom'], player, objects['torch'], false)))) {
      game.log("Some sort of light source might help you navigate below.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're at the entrace to a cave.", '\n');
  };
  return 0;
}

function descCliff1(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((globals['detailedDesc'] === 1) || (10 > Math.floor(Math.random() * 100))) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("You're standing on the edge of a cliff. The fields slope up behind you, the cabin just barely visible back WEST. A fresh, clear stream runs between tall grass.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("Grass dances violently in the wind here. Birds wheel overhead. The stream, flowing so gently through the fields, suddenly crashes off the edge of a cliff. Without climbing gear, EAST surely means death.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("The fields stretching away to your north, east, and west are cut off by fresh air. The ground drops away, forming a rocky cliff. A stream, following the cleft of the fields, falls of the cliff edge.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("Grassy fields abruptly stop at the edge of a cliff, facing east. The drop would surely be fatal. A stream hurtles over, mist rising gently up from it's landing below.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're at the edge of a cliff.", '\n');
  };
  return 0;
}

function descLake1(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((globals['detailedDesc'] === 1) || (10 > Math.floor(Math.random() * 100))) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("You find yourself on the eastern edge of a large lake. It stretches off into the distance, shrouded in fog. The shoreline is mostly granite, with some patches of coarse yellow sand. The water is cool and clear. Frogs croak amidst the bulrushes.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("Trees peel back to reveal a large lake, spreading out luxuriously towards the west.");
      if ((!(globals['moonPhase'] === 0)) && (0 > globals['time'])) {
        game.log(" Moonlight reflects off the surface.");
      };
      game.log(" The crisp snap of fall is beginning to clear away the heat of summer. Some leaves are starting to turn yellow, orange, red.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("The forest is never silent, but here sound carries perfectly across the surface of a wide lake. It carries on to the west, before losing itself in mist. Small fish dart around, never breaking the surface. It's two worlds, above and below.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("A large, wide lake takes over from endless tree bark. The contrast is startling, as if stumbling into an empty ballroom. It's never quiet, but always peaceful here. Sumac crowds around the water's edge.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're on the shore of a lake", '\n');
  };
  return 0;
}

function descForest2(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((globals['detailedDesc'] === 1) || (10 > Math.floor(Math.random() * 100))) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("The trail weaves haphazardly through thinning trees. A dense carpet of bracken gently transitions into dandelions, wild carrot, clover, and virginia rye. A snake slithers past. A group of ants struggle against a twig. The trail you're walking on is littered with maple keys.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("Wafts of fresh air cut through the stench of pine. You're in a forest, but not as deep as elsewhere. It feels safe and playful. Young sugar maples are climbing up towards the sky, stretching their leaves, and swaying in the wind. Virginia rye brushes against your boots.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("This path runs generally NORTH to SOUTH but enjoys it's time in getting there. Wild carrot dots the forest floor like little white fireworks. Broad leaves brush against needles, and the earth is a rich brown under your boots.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("Many creatures have walked this path before you. You pick the most-trodden route, but grasses and bracken are flattened all over. Most of the trees here are younger, with slender trunks. They stand tall, hopeful, and defiant. Older aspen and elm persevere quietly.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're in a forest, the trees are thinner here. There's a trail heading NORTH (and SOUTH).", '\n');
  };
  return 0;
}

function descForest3(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((globals['detailedDesc'] === 1) || (10 > Math.floor(Math.random() * 100))) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("You're in a forest, on a trail following a small stream. The trees aren't too dense but you can't see very far in any direction, due to sharp elevation change in the rocky terrain. Some maples are starting to lose their leaves, coating the path in a collage of decay.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("This forest reminds you of so many autumns before, but it's familiarity does not dull your pleasure nor succeed in extinguishing a flickering sense of wonder. The smell of pine, sap, earth, and fresh water swirl through the air. Water wears away granite ever so slowly.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("You're in a forest. A stream burbles alongside the path, leading generally EAST to WEST, occasionally branching confidently from a rise. Patches of sage, mint, and thyme spill over the landscape. Granite peaks through fallen leaves, brown-pink-white-gold.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("You're in a light forest of poplar, birch, and maple. It seems like an excellent area for cultivating mushrooms. Through the next sudden valley, a prime spot for herbs. A diminutive but determined brook burbles nearby. You avoid stepping in some rabbit droppings.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're in a forest, following a small stream.", '\n');
  };
  return 0;
}

function descForest4(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((globals['detailedDesc'] === 1) || (10 > Math.floor(Math.random() * 100))) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("As summer turns to fall, animals in the forest prepare for winter. Every nut, root, grub, and flower is worth contemplation. The squirrel has made a lovely home in an oak tree. You're surrounded by tall trees, with dandelion, marigold, and bittercress fighting up out of the dirt.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("Trees thin and thicken. You could stand four people around that oak and not touch hands. The forest floor is populated with dandelion, marigold, and bittercress. Oyster mushrooms grow on tree bark, and continue right on growing after the tree falls down dead. Birches shrug of their skin.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("You're in a forest of oak, hemlock, dogwood, and cedar. Daisies wave from atop shelves of limestone. Small creatures fight amongst themselves for dominance of the area. Acorns crunch under your boots as you walk alone the trail. The smell of possibility is in the air.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("Who broke this trail? It's not as well-worn as others, but maintains a consistent heading; there's a definite destination in mind. An oak leaf falls to the ground as you step over a tree root. What brought them here? Were they looking for the same thing you are?", '\n');
    };
  } else if (1 === 1) {
    game.log("You're in a forest.", '\n');
  };
  return 0;
}

function descForest5(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((globals['detailedDesc'] === 1) || (10 > Math.floor(Math.random() * 100))) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("You're deep into a forest of spruce, pine, and cedar. Despite a thick coating of needles on the ground, the trail is clearly discernable: it's worn markedly lower than the rest of the forest floor. When rain falls, it gathers in puddles and washes away debris down the trail.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("You've walked down many forest trails. The smell of pine, now thick in your nose, conjures up memories of songs around the campfire, cold nights, and fireflies in the dark. I wonder where your cedar-strip canoe is now. Is that water in the distance?", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("You're deep into a conifer forest. The ground is matted with needles, and they stick to the soles of your boots with sap. You spot a snake skin coiled up on near the base of a red pine. It's pretty small though. The forest just gets thicker and thicker.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("You hum to yourself while walking down the trail. Tree limbs here are crowded for space, but the trunks themselves are well apart, so the walking is easy as roots seldom crop up. That trunk has clearly been marked by a bear. A spruce needle pricks you as you go by.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're deep in a forest.", '\n');
  };
  return 0;
}

function descForest6(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((globals['detailedDesc'] === 1) || (10 > Math.floor(Math.random() * 100))) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("You're in a dense forest. The undergrowth is thick with fallen branches, leaves, and other detritus. Everything green vies for a spot in the sunlight.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("You're in a wild forest. Many generations of tree have lived and died here. Beech, cedar, oak, and maple feast on the remains of one another. Each gets their turn in the sun. A spray of asters shines smartly amidst brown sticks and leaves.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("You could get lost out here if you're not careful. The trail is faint and barely worn. Lightening-struck trees lean against those still standing, threatening to topple them in turn. You choose your footing carefully. The air is crisp with notes of mint and clover, but always undertones of decay.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("Are you still on a trail? As you head more north west, the path becomes harder to follow. You doubt many other people have come this way. Trees are closing in around you, they loom and lean menacingly. Pine sap forms little stalagmites on top of some roots. That cracked tree-stump over there reminds you of your grandfather.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're deep in a forest.", '\n');
  };
  return 0;
}

function descField1(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((globals['detailedDesc'] === 1) || (10 > Math.floor(Math.random() * 100))) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("A grassy field sprawls before you. You trace the wind across it's face. Goldenrod, virginia rye, wild carrot, milkweed, thistle, and nettle all sway in the breeze. The air is a practiced conductor; everyone playing in perfect harmony. A stream tinkles nearby.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("You're in a meadow, waist-deep in wild grass. The smooth curves or earth are interrupted only by a thin stream running east. The wind whips against your face. A whisp of milkweed flies into your eye, you stop momentarily to blink it out.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("You're in a field. A furrow runs beside the path, carved out by a determined trickle of a stream. The wind races along. Where does it have to go? The dirt is soft beneath your boots. Goldenrod slips through your fingers.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("You're walking along a trail through rolling fields. You've walked far enough to avoid any thistle and nettle, but this meadow is mostly grass. It grows higher near a little stream. The path follows on it's south, occasionally crossing over north.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're in a field.", '\n');
  };
  return 0;
}

function descField2(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((globals['detailedDesc'] === 1) || (10 > Math.floor(Math.random() * 100))) {
    if (12 > Math.floor(Math.random() * 100)) {
      game.log("You're in a large field. It rolls away north to the horizon. A slug is munching on the stem of a wild carrot. You munch on some goldenrod. Is that a voice on the wind? No, it's just the wind.", '\n');
    } else if (globals['dayM3'] === 0) {
      game.log("You're in a large meadow, with seeds rolling down into your boots. The trail is faint here, and not very playful; it proceeds in straight lines. Not that there's much reason for curves. A meadow is diverse in the micro but monotonous in the macro. At least you'll see anything coming from a ways off, if anything comes.", '\n');
    } else if (globals['dayM3'] === 1) {
      game.log("You're in a field, spilling away from you in all directions. The goldenrod is vibrant, the thistles sharp, and the rye tall. You could spend all day here, wrapped in your cloak. The harvest is good for this time of year.", '\n');
    } else if (globals['dayM3'] === 2) {
      game.log("You're in a large field, unbroken by fence or tree. If you weren't already, it would make you want to wander. Every time you see milkweed pods you have an urge to open them.", '\n');
    };
  } else if (1 === 1) {
    game.log("You're in a large field.", '\n');
  };
  return 0;
}

function lake1Always(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['water'], false))) {
    game.copyMove(objects['water'], locals['cRoom']);
  };
  return 0;
}

function snakeInPlayer(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0) > 0) {
    game.log("The snake bites you for 2 damage", '\n');
    player.vars['health'] = ((player.vars['health'] || 0) - 2);
  };
  return 0;
}

function drownFish(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isEqual(objects['water'], game.getParent((locals['cmd']?.[0] ?? getEmptyResource()), false)))) {
    if (!(((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
      game.log("The fish dies", '\n');
    };
    (locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] = 0;
  };
  return 0;
}

function bearInRoom(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0,qn: 0};
  if (((locals['cmd']?.[0] ?? getEmptyResource()).vars['health'] || 0) > 0) {
    if (((locals['cmd']?.[0] ?? getEmptyResource()).vars['enraged'] || 0) === 1) {
      game.log("The bear swings at you");
      (locals['cmd']?.[0] ?? getEmptyResource()).vars['enraged'] = 0;
      locals['dmg'] = Math.floor(Math.floor(Math.random() * 100) / 5);
      if (locals['dmg'] < 5) {
        game.log(", but you manage to dodge it", '\n');
      } else if (1 === 1) {
        game.log(".", '\n');
        game.log("You take ", locals['dmg'].toString(), " damage", '\n');
        player.vars['health'] = ((player.vars['health'] || 0) - locals['dmg']);
      };
    } else if (game.isInLocation(locals['cRoom'], player, objects['gem'], false)) {
      if ((50 > Math.floor(Math.random() * 100)) && (!(((locals['cmd']?.[0] ?? getEmptyResource()).vars['asked'] || 0) === 1))) {
        game.log("A bear asks 'How's your boat coming along?'", '\n');
        (locals['cmd']?.[0] ?? getEmptyResource()).vars['asked'] = 1;
      };
      return 1;
    } else if ((!(game.isInLocation(locals['cRoom'], rooms['qStorage'], objects['ynQ'], false))) && (!(((locals['cmd']?.[0] ?? getEmptyResource()).vars['asked'] || 0) === 1))) {
      game.log("A bear asks you ");
      locals['qn'] = Math.floor(Math.random() * 100);
      if (20 > locals['qn']) {
        game.log("'Do you know the owl?'", '\n');
        game.copyMove(objects['ynQ'], rooms['qStorage']);
        game.getInst(rooms['qStorage'], 'ynQ', false).vars['kOwl'] = 1;
      } else if (40 > locals['qn']) {
        game.log("'Do you know the fish?'", '\n');
        game.copyMove(objects['ynQ'], rooms['qStorage']);
        game.getInst(rooms['qStorage'], 'ynQ', false).vars['kFish'] = 1;
      } else if (60 > locals['qn']) {
        game.log("'Do you know the frog?'", '\n');
        game.copyMove(objects['ynQ'], rooms['qStorage']);
        game.getInst(rooms['qStorage'], 'ynQ', false).vars['kFrog'] = 1;
      } else if (80 > locals['qn']) {
        game.log("'Do you know the rabbit?'", '\n');
        game.copyMove(objects['ynQ'], rooms['qStorage']);
        game.getInst(rooms['qStorage'], 'ynQ', false).vars['kRabbit'] = 1;
      } else if (100 > locals['qn']) {
        game.log("'Do you know the snake?'", '\n');
        game.copyMove(objects['ynQ'], rooms['qStorage']);
        game.getInst(rooms['qStorage'], 'ynQ', false).vars['kSnake'] = 1;
      };
      game.log(" (it expects a YES or NO)", '\n');
      game.getInst(rooms['qStorage'], 'ynQ', false).vars['fromBear'] = 1;
    };
  };
  return 0;
}

function playerAlways(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(
  routines['checkPulse'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']) === 1)) {
    routines['weatherReport'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    game.log("Good game! Thank you for playing", '\n');game.close();
  } else if (1 === 1) {
    routines['inventoryLimit'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    routines['clearQuestions'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    routines['monsterHealth'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    routines['burnFire'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    routines['checkPulse'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    routines['timePasses'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  };
  return 0;
}

function checkPulse(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!((player.vars['health'] || 0) > 0)) {
    game.log("You have died.", '\n');
    routines['recap'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    return 0;
  };
  return 1;
  return 0;
}

function inventoryLimit(cRoom, cmd) {
  const locals = {cRoom, cmd, count: 0};
  for (let object of game.getObjectsIn(player)) {
    locals['obj'] = object;
    locals['count'] = (locals['count'] + 1);
  };
  if (locals['count'] > 7) {
    game.log("You're trying to carry too much. Some items fall on the ground.", '\n');
    if (globals['firstInventoryLimit'] === 1) {
      game.log("Can DROP items you don't want, and TAKE the ones you do.", '\n');
      globals['firstInventoryLimit'] = 0;
    };
    for (let object of game.getObjectsIn(player)) {
      locals['obj'] = object;
      if (locals['count'] > 7) {
        game.move(locals, locals['obj'], locals['cRoom']);
      };
      locals['count'] = (locals['count'] - 1);
    };
  };
  return 0;
}

function clearQuestions(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  for (let object of game.getObjectsIn(rooms['qStorage'])) {
    locals['obj'] = object;
    if (game.isEqual(objects['ynQ'], locals['obj'])) {
      if ((locals['obj'].vars['exp'] || 0) > 0) {
        locals['obj'].vars['exp'] = ((locals['obj'].vars['exp'] || 0) - 1);
      } else if ((locals['obj'].vars['exp'] || 0) === 0) {
        if (((locals['obj'].vars['fromBear'] || 0) === 1) && (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['bear'], false))) {
          game.log("The bear takes your silence as an answer, and sits down to forage", '\n');
        };
        game.move(locals, locals['obj'], );
      };
    };
  };
  return 0;
}

function burnFire(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  for (let object of game.getObjectsIn(objects['fire'])) {
    locals['obj'] = object;
    if ((game.isInLocation(locals['cRoom'], rooms['cabin'], locals['obj'], false)) && ((game.getInst(rooms['cabinExterior'], 'cabinDoor', false).vars['health'] || 0) < 1) && ((game.getInst(rooms['cabinExterior'], 'cabinWindow', false).vars['health'] || 0) < 1)) {
      locals['obj'].vars['fuel'] = ((locals['obj'].vars['fuel'] || 0) - 1);
    } else if ((game.getParent(locals['obj'], true).vars['aboveGround'] || 0) === 1) {
      locals['obj'].vars['fuel'] = ((locals['obj'].vars['fuel'] || 0) - 2);
      if ((game.isInLocation(locals['cRoom'], rooms['cabin'], locals['obj'], false)) && ((game.isInLocation(locals['cRoom'], rooms['cabin'], player, false)) || (game.isInLocation(locals['cRoom'], rooms['cabinExterior'], player, false)))) {
        if (globals['fireInDraftyCabin'] === 1) {
          game.log("The cabin is drafty from a broken door or window, and fire burns more quickly.", '\n');
          globals['fireInDraftyCabin'] = 0;
        };
      };
    } else if (1 === 1) {
      locals['obj'].vars['fuel'] = ((locals['obj'].vars['fuel'] || 0) - 1);
    };
    if ((locals['obj'].vars['fuel'] || 0) < 1) {
      game.move(locals, locals['obj'], );
      if ((game.isInLocation(locals['cRoom'], player, locals['obj'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], locals['obj'], false))) {
        game.log("The fire dies.", '\n');
      };
    } else if (((game.getParent(locals['obj'], false).vars['aboveGround'] || 0) === 1) && (!(game.isInLocation(locals['cRoom'], rooms['cabin'], locals['obj'], false))) && (globals['dry'] > 3)) {
      routines['burnForestDown'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    };
  };
  for (let object of game.getObjectsIn(objects['torch'])) {
    locals['obj'] = object;
    if ((locals['obj'].vars['isLit'] || 0) === 1) {
      locals['obj'].vars['fuel'] = ((locals['obj'].vars['fuel'] || 0) - 1);
      if ((locals['obj'].vars['fuel'] || 0) < 1) {
        if (game.isInLocation(locals['cRoom'], player, locals['obj'], false)) {
          game.log("Your torch dies.", '\n');
        };
        game.move(locals, locals['obj'], );
      } else if (((game.getParent(locals['obj'], true).vars['aboveGround'] || 0) === 1) && (!(game.isInLocation(locals['cRoom'], player, locals['obj'], true))) && (!(game.isInLocation(locals['cRoom'], rooms['cabin'], locals['obj'], true))) && (globals['dry'] > 3)) {
        routines['burnForestDown'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
      };
    };
  };
  return 0;
}

function timePasses(cRoom, cmd) {
  const locals = {cRoom, cmd, newW: 0};
  if (globals['time'] > 0) {
    globals['time'] = (globals['time'] + 1);
    if (globals['time'] === 34) {
      globals['time'] = -1;
      if (globals['weather'] === 0) {
        globals['dry'] = 0;
      };
      if ((locals['cRoom'].vars['aboveGround'] || 0) === 1) {
        game.log("Night falls.", '\n');
      } else if (1 === 1) {
        game.log("Outside, night falls.", '\n');
      };
      routines['checkSleep'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
      globals['moonPhase'] = (globals['moonPhase'] + 1);
      if (globals['moonPhase'] === 8) {
        globals['moonPhase'] = 0;
      } else if (globals['moonPhase'] === 4) {
        game.log("It's a full moon. Be careful, all your weapon and tool actions are more erratic.", '\n');
      };
    };
  } else if (globals['time'] < 0) {
    globals['time'] = (globals['time'] + -1);
    if (globals['time'] === -18) {
      globals['time'] = 1;
      if ((locals['cRoom'].vars['aboveGround'] || 0) === 1) {
        game.log("Morning breaks.", '\n');
      } else if (1 === 1) {
        game.log("Outside, morning comes.", '\n');
      };
      routines['checkFood'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    } else if (globals['time'] === -8) {
      globals['napsToday'] = 0;
      globals['day'] = (globals['day'] + 1);
      globals['dayM3'] = (globals['dayM3'] + 1);
      if (globals['dayM3'] === 3) {
        globals['dayM3'] = 0;
      };
      locals['newW'] = Math.floor(Math.random() * 100);
      if (30 > locals['newW']) {
        globals['weather'] = 3;
        globals['dry'] = (globals['dry'] + 1);
      } else if (60 > locals['newW']) {
        globals['weather'] = 2;
        globals['dry'] = (globals['dry'] + 1);
      } else if (90 > locals['newW']) {
        globals['weather'] = 1;
        globals['dry'] = 0;
      } else if (100 > locals['newW']) {
        globals['weather'] = 0;
        globals['dry'] = (globals['dry'] + 1);
      };
    };
  };
  return 0;
}

function weatherReport(cRoom, cmd) {
  const locals = {cRoom, cmd, isNight: 0,untilFull: 0};
  if (globals['time'] > 23) {
    locals['isNight'] = 0;
    game.log("It's evening, ");
  } else if (globals['time'] > 15) {
    locals['isNight'] = 0;
    game.log("It's the afternoon, ");
  } else if (globals['time'] > 7) {
    locals['isNight'] = 0;
    game.log("It's morning, ");
  } else if (globals['time'] > 0) {
    locals['isNight'] = 0;
    game.log("It's early morning, ");
  } else if (globals['time'] < -8) {
    locals['isNight'] = 1;
    game.log("It's late at night, ");
  } else if (globals['time'] < 0) {
    locals['isNight'] = 1;
    game.log("It's night, ");
  };
  if (globals['moonPhase'] === 0) {
    game.log("new moon, ");
  } else if ((globals['moonPhase'] > 0) && (globals['moonPhase'] < 4)) {
    locals['untilFull'] = (3 - globals['moonPhase']);
    game.log("the moon is waxing (", locals['untilFull'].toString(), " day");
    if (!(locals['untilFull'] === 1)) {
      game.log("s");
    };
    game.log(" until full), ");
  } else if (globals['moonPhase'] === 4) {
    game.log("full moon, ");
  } else if ((globals['moonPhase'] > 4) && (globals['moonPhase'] < 8)) {
    game.log("the moon is waning, ");
  };
  if (globals['weather'] === 3) {
    if (locals['isNight'] === 0) {
      game.log("and there's a clear blue sky overhead. ");
    } else if (locals['isNight'] === 1) {
      game.log("and there are thousands of stars above you. ");
    };
  } else if (globals['weather'] === 2) {
    if (locals['isNight'] === 0) {
      game.log("but a dim, overcast sky presses down on you. ");
    } else if (locals['isNight'] === 1) {
      game.log("but no light penetrates through the cloudy sky. ");
    };
  } else if (globals['weather'] === 1) {
    if (locals['isNight'] === 0) {
      game.log("a persistent drizzle dampens everything around. ");
    } else if (locals['isNight'] === 1) {
      game.log("heavy raindrops fall haphazardly. ");
    };
  } else if (globals['weather'] === 0) {
    if (locals['isNight'] === 0) {
      game.log("storm clouds are building. ");
    } else if (locals['isNight'] === 1) {
      game.log("lightning and thunder beat down savagely around you. ");
    };
  };
  if (globals['dry'] < 1) {
  } else if (globals['dry'] < 2) {
    game.log("There are still puddles on the ground from recent rain.", '\n');
  } else if (globals['dry'] < 4) {
    game.log("The ground is still damp from rain.", '\n');
  } else if (1 === 1) {
    game.log("It's been ", globals['dry'].toString(), " days since the last rain.", '\n');
  };
  return 0;
}

function checkFood(cRoom, cmd) {
  const locals = {cRoom, cmd, e: 0};
  if (!((player.vars['health'] || 0) > 0)) {
    return 1;
  };
  locals['e'] = (globals['totalFood'] - (globals['day'] * 2));
  if (locals['e'] > 6) {
    player.vars['health'] = ((player.vars['health'] || 0) - (locals['e'] + globals['day']));
    game.log("You lose some health from over-eating.", '\n');
  } else if ((player.vars['health'] || 0) < 20) {
    game.log("EAT some food to re-gain health", '\n');
  };
  return 0;
}

function checkSleep(cRoom, cmd) {
  const locals = {cRoom, cmd, z: 0};
  if (globals['napsToday'] > 5) {
    game.log("You lose 10 health from napping too much today, but also gained 1 sleep", '\n');
    globals['totalSleeps'] = (globals['totalSleeps'] + 1);
  } else if (globals['napsToday'] > 2) {
    game.log("You gain 1 sleep from the naps you took today", '\n');
    globals['totalSleeps'] = (globals['totalSleeps'] + 1);
  };
  locals['z'] = (globals['totalSleeps'] - globals['day']);
  if (locals['z'] < -2) {
    player.vars['health'] = ((player.vars['health'] || 0) - 40);
    game.log("You lose 40 health from lack of sleep.", '\n');
  } else if (locals['z'] < -1) {
    player.vars['health'] = ((player.vars['health'] || 0) - 20);
    game.log("You lose 20 health from lack of sleep.", '\n');
  } else if (locals['z'] < 0) {
    if ((locals['cRoom'].vars['aboveGround'] || 0) === 1) {
      game.log("You will lose health if you don't get enough SLEEP.", '\n');
    };
  } else if (locals['z'] > 3) {
    player.vars['health'] = ((player.vars['health'] || 0) - 20);
    game.log("You lose 10 health from over-sleeping.", '\n');
  };
  return 0;
}

function playerActEnter(cRoom, cmd) {
  const locals = {cRoom, cmd, hasLight: 0};
  routines['vDescRoom'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  locals['hasLight'] = 
  routines['roomHasLight'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  if (globals['childMonsterWillChase'] === 1) {
    for (let object of game.getObjectsIn(objects['childMonster'])) {
      locals['obj'] = object;
      game.move(locals, locals['obj'], locals['cRoom']);
    };
  } else if (((game.isEqual(locals['cRoom'], rooms['den1'])) || (game.isEqual(locals['cRoom'], rooms['den2'])) || (game.isEqual(locals['cRoom'], rooms['den3'])) || (game.isEqual(locals['cRoom'], rooms['den4']))) && (45 > Math.floor(Math.random() * 100))) {
    game.move(locals, game.getInst(rooms['storage'], 'childMonster', false), locals['cRoom']);
  };
  if (globals['parentMonsterWillChase'] === -1) {
    globals['parentMonsterWillChase'] = 0;
    for (let object of game.getObjectsIn(objects['parentMonster'])) {
      locals['obj'] = object;
      locals['obj'].vars['encounter'] = ((locals['obj'].vars['encounter'] || 0) + 1);
      locals['obj'].vars['maxHealth'] = ((locals['obj'].vars['maxHealth'] || 0) + globals['parentMonsterHealthStep']);
      if ((locals['obj'].vars['maxHealth'] || 0) > 1500) {
        locals['obj'].vars['maxHealth'] = 1500;
      };
    };
  } else if (globals['parentMonsterWillChase'] === 1) {
    for (let object of game.getObjectsIn(objects['parentMonster'])) {
      locals['obj'] = object;
      game.move(locals, locals['obj'], locals['cRoom']);
    };
  } else if ((locals['hasLight'] === 1) && (globals['parentMonsterFirstEncounter'] === 1) && ((locals['cRoom'].vars['aboveGround'] || 0) === 0) && (25 > Math.floor(Math.random() * 100))) {
    game.move(locals, game.getInst(rooms['storage'], 'parentMonster', false), locals['cRoom']);
  } else if ((locals['hasLight'] === 1) && (globals['parentMonsterFirstEncounter'] === 0) && ((locals['cRoom'].vars['aboveGround'] || 0) === 0) && (15 > Math.floor(Math.random() * 100))) {
    game.move(locals, game.getInst(rooms['storage'], 'parentMonster', false), locals['cRoom']);
  } else if ((locals['hasLight'] === 0) && (globals['parentMonsterFirstEncounter'] === 0)) {
    if ((game.isInLocation(locals['cRoom'], rooms['den1'], player, false)) || (game.isInLocation(locals['cRoom'], rooms['den2'], player, false)) || (game.isInLocation(locals['cRoom'], rooms['den3'], player, false)) || (game.isInLocation(locals['cRoom'], rooms['den4'], player, false))) {
      if (34 > Math.floor(Math.random() * 100)) {
        game.move(locals, game.getInst(rooms['storage'], 'parentMonster', false), locals['cRoom']);
      };
    } else if (20 > Math.floor(Math.random() * 100)) {
      game.move(locals, game.getInst(rooms['storage'], 'parentMonster', false), locals['cRoom']);
    };
  };
  if (((locals['cRoom'].vars['aboveGround'] || 0) === 0) && (locals['hasLight'] === 1)) {
    if (20 > Math.floor(Math.random() * 100)) {
      game.copyMove(objects['caveSpider'], locals['cRoom']);
      if (50 > Math.floor(Math.random() * 100)) {
        game.copyMove(objects['caveSpider'], locals['cRoom']);
      };
      if (10 > Math.floor(Math.random() * 100)) {
        game.copyMove(objects['caveSpider'], locals['cRoom']);
      };
    };
  };
  if ((locals['cRoom'].vars['aboveGround'] || 0) === 0) {
    if ((0 > globals['time']) && (25 > Math.floor(Math.random() * 100))) {
      game.copyMove(objects['bat'], locals['cRoom']);
    } else if (5 > Math.floor(Math.random() * 100)) {
      game.copyMove(objects['bat'], locals['cRoom']);
    };
  };
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (((locals['obj'].vars['isAnimal'] || 0) === 1) && (!((locals['obj'].vars['health'] || 0) > 0)) && (34 > Math.floor(Math.random() * 100))) {
      game.move(locals, locals['obj'], );
    };
  };
  return 0;
}

function recap(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['winEnterCabin'] === 1) {
    game.log("You entered the cabin.", '\n');
  } else if (1 === 1) {
    game.log("You did not enter the cabin.", '\n');
  };
  if (globals['winLightFire'] === 1) {
    game.log("You built a fire.", '\n');
  } else if (1 === 1) {
    game.log("You did not build a fire.", '\n');
  };
  if (globals['winCookMeal'] === 1) {
    game.log("You cooked and ate a meal.", '\n');
  } else if (1 === 1) {
    game.log("You did not eat a cooked meal.", '\n');
  };
  if (globals['winFindGem'] === 1) {
    game.log("You found the gem.", '\n');
  } else if (1 === 1) {
    game.log("You did not find a gem.", '\n');
  };
  if (globals['winEnterCave'] === 1) {
    game.log("You entered the caves.", '\n');
  } else if (1 === 1) {
    game.log("You did not enter the caves.", '\n');
  };
  if (globals['winKillMonster'] > 1) {
    game.log("You killed both monsters.", '\n');
  } else if (globals['winKillMonster'] === 1) {
    game.log("You killed a monster.", '\n');
  } else if (1 === 1) {
    game.log("You did not kill a monster.", '\n');
  };
  if (game.isInLocation(locals['cRoom'], rooms['cabin'], objects['note'], false)) {
    game.log("You left a message.", '\n');
  } else if (1 === 1) {
    game.log("You did not leave a message.", '\n');
  };
  if (globals['winBuildBoat'] === 1) {
    game.log("You built a boat.", '\n');
  } else if (1 === 1) {
    game.log("You did not build a boat.", '\n');
  };
  if (globals['winCheat'] === 1) {
    game.log("You cheated.", '\n');
  };
  return 0;
}

function roomHasLight(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (((locals['cRoom'].vars['aboveGround'] || 0) === 1) && (!((0 > globals['time']) && (globals['moonPhase'] === 0)))) {
    return 1;
  } else if ((game.isInLocation(locals['cRoom'], locals['cRoom'], objects['fire'], false)) || ((game.getInst(locals['cRoom'], 'torch', false).vars['isLit'] || 0) === 1) || ((game.getInst(player, 'torch', false).vars['isLit'] || 0) === 1)) {
    return 1;
  } else if ((game.isInLocation(locals['cRoom'], locals['cRoom'], objects['gem'], false)) || (game.isInLocation(locals['cRoom'], player, objects['gem'], false))) {
    return 1;
  };
  return 0;
  return 0;
}

function monsterHealth(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  for (let object of game.getObjectsIn(objects['parentMonster'])) {
    locals['inst'] = object;
    if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], locals['inst'], false))) {
      locals['inst'].vars['health'] = ((locals['inst'].vars['health'] || 0) + 1);
      if ((locals['inst'].vars['health'] || 0) > (locals['inst'].vars['maxHealth'] || 0)) {
        locals['inst'].vars['health'] = (locals['inst'].vars['maxHealth'] || 0);
      };
    };
  };
  for (let object of game.getObjectsIn(objects['childMonster'])) {
    locals['inst'] = object;
    if (!(game.isInLocation(locals['cRoom'], locals['cRoom'], locals['inst'], false))) {
      locals['inst'].vars['health'] = ((locals['inst'].vars['health'] || 0) + 1);
      if ((locals['inst'].vars['health'] || 0) > (locals['inst'].vars['maxHealth'] || 0)) {
        locals['inst'].vars['health'] = (locals['inst'].vars['maxHealth'] || 0);
      };
    };
  };
  return 0;
}

function burnForestDown(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("It hasn't rained in awhile. Fire catches, and the forest burns down.", '\n');
  globals['forestBurnedDown'] = 1;
  for (let object of game.getObjectsIn(rooms['caveEntrance1'])) {
    locals['obj'] = object;
    game.move(locals, locals['obj'], );
  };
  for (let object of game.getObjectsIn(rooms['caveEntrance2'])) {
    locals['obj'] = object;
    game.move(locals, locals['obj'], );
  };
  if ((locals['cRoom'].vars['aboveGround'] || 0) === 1) {
    player.vars['health'] = 0;
    return 1;
  };
  return 0;
}

function vDescRoom(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.describe(locals['cRoom']);
  return 0;
}

function vRoomDetail(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['firstWhereAmI'] === 1) {
    globals['firstWhereAmI'] = 0;
    game.log("WHERE AM I describes the current room, and may go into more detail than a regular description of the room.", '\n');
  };
  globals['detailedDesc'] = 1;
  game.describe(locals['cRoom']);
  globals['detailedDesc'] = 0;
  return 0;
}

function vDescObjectsInRoom(cRoom, cmd) {
  const locals = {cRoom, cmd, count: 0};
  if (globals['firstWhatIsHere'] === 1) {
    globals['firstWhatIsHere'] = 0;
    game.log("WHAT IS HERE lists interactable objects in the immediate vicinity. Interactions may not be obvious. Objects nested inside other objects are not listed, but might show up if you EXAMINE their container.", '\n');
  };
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    locals['count'] = (locals['count'] + 1);
    game.describe(locals['obj']);
    game.log('\n');
  };
  if (locals['count'] === 0) {
    game.log("This space appears to be empty.", '\n');
  };
  return 0;
}

function addToFire(cRoom, cmd) {
  const locals = {cRoom, cmd, hasEdbl: 0,hasSap: 0,hasShm: 0,onlyEdbl: 0};
  if ((game.isEqual(objects['kettle'], (locals['cmd']?.[1] ?? getEmptyResource()))) || (game.isEqual(objects['bucket'], (locals['cmd']?.[1] ?? getEmptyResource()))) || (game.isEqual(objects['cup'], (locals['cmd']?.[1] ?? getEmptyResource())))) {
    locals['onlyEdbl'] = 1;
    for (let object of game.getObjectsIn((locals['cmd']?.[1] ?? getEmptyResource()))) {
      locals['obj'] = object;
      if ((locals['obj'].vars['isEdible'] || 0) === 1) {
        locals['hasEdbl'] = 1;
      } else if ((game.isEqual(objects['sap'], locals['obj'])) || (game.isEqual(objects['boiledSap'], locals['obj']))) {
        locals['hasSap'] = 1;
        locals['onlyEdbl'] = 0;
      } else if (!(game.isEqual(objects['water'], locals['obj']))) {
        locals['onlyEdbl'] = 0;
      };
    };
    if (locals['hasSap'] === 1) {
      game.log("Everything in there becomes BOILED-SAP", '\n');
      for (let object of game.getObjectsIn((locals['cmd']?.[1] ?? getEmptyResource()))) {
        locals['obj'] = object;
        game.move(locals, locals['obj'], );
      };
      game.copyMove(objects['boiledSap'], (locals['cmd']?.[1] ?? getEmptyResource()));
    } else if ((locals['onlyEdbl'] === 1) && (locals['hasEdbl'] === 1) && (game.isInLocation(locals['cRoom'], (locals['cmd']?.[1] ?? getEmptyResource()), objects['water'], false))) {
      if (game.isInLocation(locals['cRoom'], (locals['cmd']?.[1] ?? getEmptyResource()), objects['mushroom'], false)) {
        locals['hasShm'] = 1;
      };
      if (game.isEqual(objects['cup'], (locals['cmd']?.[1] ?? getEmptyResource()))) {
        game.log("You boil up some tea. Can DRINK it.", '\n');
        for (let object of game.getObjectsIn((locals['cmd']?.[1] ?? getEmptyResource()))) {
          locals['obj'] = object;
          game.move(locals, locals['obj'], );
        };
        game.copyMove(objects['tea'], (locals['cmd']?.[1] ?? getEmptyResource()));
        if (locals['hasShm'] === 1) {
          game.getInst((locals['cmd']?.[1] ?? getEmptyResource()), 'tea', false).vars['hasMushroom'] = 1;
        };
      } else if (1 === 1) {
        game.log("You boil up some soup. Can EAT it.", '\n');
        for (let object of game.getObjectsIn((locals['cmd']?.[1] ?? getEmptyResource()))) {
          locals['obj'] = object;
          game.move(locals, locals['obj'], );
        };
        game.copyMove(objects['soup'], (locals['cmd']?.[1] ?? getEmptyResource()));
        if (locals['hasShm'] === 1) {
          game.getInst((locals['cmd']?.[1] ?? getEmptyResource()), 'soup', false).vars['hasMushroom'] = 1;
        };
      };
    } else if (1 === 1) {
      game.log("You need water and something edible to make soup: ADD BERRIES TO KETTLE.", '\n');
    };
  } else if (game.isEqual(objects['obsidianShard'], (locals['cmd']?.[1] ?? getEmptyResource()))) {
    game.log("It sits there in the fire, looking black.", '\n');
  } else if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
    game.log("The edge goes dull", '\n');
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['damage'] = 0;
  } else if (game.isEqual(objects['log'], (locals['cmd']?.[1] ?? getEmptyResource()))) {
    globals['winLightFire'] = 1;
    game.log("The fire will continue to burn for another hour or two.", '\n');
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['fuel'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['fuel'] || 0) + 12);
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
  } else if (game.isEqual(objects['roughBoard'], (locals['cmd']?.[1] ?? getEmptyResource()))) {
    globals['winLightFire'] = 1;
    game.log("The fire will continue to burn for a little longer.", '\n');
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['fuel'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['fuel'] || 0) + 5);
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
  } else if (game.isEqual(objects['water'], (locals['cmd']?.[1] ?? getEmptyResource()))) {
    game.log("The fire dies with a sputter.", '\n');
    game.move(locals, (locals['cmd']?.[2] ?? getEmptyResource()), );
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    game.copyMove(objects['charcoal'], locals['cRoom']);
  } else if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['isAnimal'] || 0) === 1) {
    game.log("It stinks, but goes up in smoke.", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
  } else if (1 === 1) {
    game.log("It goes up in smoke.", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
  };
  return 0;
}

function addToStone(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual(objects['water'], (locals['cmd']?.[1] ?? getEmptyResource()))) {
    game.log("The stone is damp and ready to sharpen weapons. It will dry out eventually though.", '\n');
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['wetness'] = 20;
  } else if (game.isEqual(objects['water'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    game.log("The stone is damp and ready to sharpen weapons. It will dry out eventually though.", '\n');
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['wetness'] = 20;
  } else if (1 === 1) {
    game.log("Are you sure there's WATER around?", '\n');
  };
  return 0;
}

function add(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual(objects['fire'], (locals['cmd']?.[1] ?? getEmptyResource()))) {
    game.log("You burn yourself, and take 2 damage.", '\n');
    player.vars['health'] = ((player.vars['health'] || 0) - 2);
  } else if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['noTake'] || 0) === 1) {
    game.log("Nice try", '\n');
  } else if ((game.isEqual(objects['riverStone'], (locals['cmd']?.[1] ?? getEmptyResource()))) || (game.isEqual(objects['riverStone'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
    routines['addToStone'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  } else if ((game.isEqual(objects['detritus'], (locals['cmd']?.[2] ?? getEmptyResource()))) && (!(game.isEqual(objects['water'], (locals['cmd']?.[1] ?? getEmptyResource())))) && ((game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[2] ?? getEmptyResource()), false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), false)))) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[2] ?? getEmptyResource()));
    game.log("Added", '\n');
  } else if (game.isEqual(objects['water'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    if (game.isEqual(objects['water'], (locals['cmd']?.[1] ?? getEmptyResource()))) {
      game.log("The water gets wetter.", '\n');
    } else if (1 === 1) {
      game.log("splash", '\n');
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[2] ?? getEmptyResource()));
    };
  } else if (game.isEqual(objects['fire'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    routines['addToFire'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  } else if ((game.isEqual(objects['kettle'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['bucket'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['cup'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
    if (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[3] ?? getEmptyResource()))) {
      game.log("Where is that?", '\n');
    } else if ((game.isEqual(objects['water'], (locals['cmd']?.[1] ?? getEmptyResource()))) || (game.isEqual(objects['sap'], (locals['cmd']?.[1] ?? getEmptyResource()))) || (((locals['cmd']?.[1] ?? getEmptyResource()).vars['isEdible'] || 0) === 1)) {
      if ((game.isEqual(objects['water'], (locals['cmd']?.[1] ?? getEmptyResource()))) && (game.isInLocation(locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), objects['water'], false))) {
      } else if (1 === 1) {
        game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[2] ?? getEmptyResource()));
      };
      game.log("Added.", '\n');
    } else if (1 === 1) {
      game.log("You can't put that in there", '\n');
    };
  } else if (game.isEqual(objects['stick'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    if ((game.isEqual(objects['sap'], (locals['cmd']?.[1] ?? getEmptyResource()))) || (game.isEqual(objects['boiledSap'], (locals['cmd']?.[1] ?? getEmptyResource())))) {
      if ((game.isInLocation(locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), objects['sap'], false)) || (game.isInLocation(locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), objects['boiledSap'], false))) {
        game.log("This stick already has some sticky stuff on it.", '\n');
        return 0;
      };
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[2] ?? getEmptyResource()));
    } else if ((game.isEqual(objects['detritus'], (locals['cmd']?.[1] ?? getEmptyResource()))) || (game.isEqual(objects['bulrush'], (locals['cmd']?.[1] ?? getEmptyResource())))) {
      if ((game.isInLocation(locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), objects['detritus'], false)) || (game.isInLocation(locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), objects['bulrush'], false))) {
        game.log("This stick already has some tinder.", '\n');
        return 0;
      };
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[2] ?? getEmptyResource()));
    } else if (1 === 1) {
      game.log("To make a torch, add SAP or BOILED-SAP, then DETRITUS or BULRUSH.", '\n');
      return 0;
    };
    if (!((game.isInLocation(locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), objects['sap'], false)) || (game.isInLocation(locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), objects['boiledSap'], false)))) {
      game.log("Now add some SAP or BOILED-SAP", '\n');
    } else if (!((game.isInLocation(locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), objects['detritus'], false)) || (game.isInLocation(locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), objects['bulrush'], false)))) {
      game.log("Now add some DETRITUS or BULRUSH", '\n');
    } else if (1 === 1) {
      game.log("You've made yourself a TORCH. SPARK FLINT AT TORCH to light it.", '\n');
      game.copyMove(objects['torch'], player);
      game.move(locals, (locals['cmd']?.[2] ?? getEmptyResource()), );
    };
  } else if (game.isEqual(objects['pickAxe'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    if (game.isEqual(objects['strap'], (locals['cmd']?.[1] ?? getEmptyResource()))) {
      if (game.isInLocation(locals['cRoom'], (locals['cmd']?.[2] ?? getEmptyResource()), objects['strap'], false)) {
        game.log("Already has a strap on it", '\n');
      } else if (1 === 1) {
        game.log("You tie the strap to the pick-axe.", '\n');
        game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[2] ?? getEmptyResource()));
      };
    } else if (1 === 1) {
      game.log("You can tie a strap to the pick-axe using ADD, but that's about it", '\n');
    };
  } else if (1 === 1) {
    game.log("Can't put that in there", '\n');
  };
  return 0;
}

function bug(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['maxMaxHealth'] === 100) {
    globals['maxMaxHealth'] = 150;
    player.vars['maxHealth'] = ((player.vars['maxHealth'] || 0) + 50);
    game.log("Your suddenly see the potential for growth, and a powerful thirst consumes you.", '\n');
    player.vars['tFrog'] = 2;
  };
  return 0;
}

function cheat(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  globals['winCheat'] = 1;
  if (!(game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[2] ?? getEmptyResource())))) {
    game.copyMove((locals['cmd']?.[1] ?? getEmptyResource()), player);
    return 1;
  };
  player.vars['tCrow'] = 2;
  player.vars['tOwl'] = 2;
  player.vars['tFish'] = 2;
  player.vars['tFrog'] = 2;
  player.vars['tRabbit'] = 2;
  player.vars['tSnake'] = 2;
  for (let object of game.getObjectsIn(objects['sword'])) {
    locals['inst'] = object;
    locals['inst'].vars['maxDamage'] = 45;
  };
  for (let object of game.getObjectsIn(objects['axe'])) {
    locals['inst'] = object;
    locals['inst'].vars['maxDamage'] = 16;
  };
  for (let object of game.getObjectsIn(objects['knife'])) {
    locals['inst'] = object;
    locals['inst'].vars['maxDamage'] = 6;
  };
  for (let object of game.getObjectsIn(objects['obsidianShard'])) {
    locals['inst'] = object;
    if ((locals['inst'].vars['maxDamage'] || 0) === 50) {
      locals['inst'].vars['maxDamage'] = 70;
      if ((locals['inst'].vars['damage'] || 0) === 50) {
        locals['inst'].vars['damage'] = 70;
      };
    };
  };
  globals['combatHighChance'] = 75;
  globals['maxMaxHealth'] = 150;
  globals['parentMonsterAtkDmg'] = 28;
  globals['childMonsterAtkDmg'] = 2;
  globals['parentMonsterHealthStep'] = 5;
  for (let object of game.getObjectsIn(objects['parentMonster'])) {
    locals['m'] = object;
    locals['m'].vars['maxHealth'] = ((locals['m'].vars['maxHealth'] || 0) - 10);
  };
  for (let object of game.getObjectsIn(objects['childMonster'])) {
    locals['m'] = object;
    locals['m'].vars['maxHealth'] = 90;
  };
  if (!(game.isInLocation(locals['cRoom'], rooms['forest3'], objects['treeHollow'], false))) {
    game.move(locals, game.getInst(rooms['storage'], 'treeHollow', false), rooms['forest3']);
  };
  if (0 < (game.getInst(rooms['cabinExterior'], 'cabinDoor', false).vars['health'] || 0)) {
    game.getInst(rooms['cabinExterior'], 'cabinDoor', false).vars['health'] = 0;
  };
  if (0 < (game.getInst(rooms['cavern1'], 'stoneDoor', false).vars['health'] || 0)) {
    game.getInst(rooms['cavern1'], 'stoneDoor', false).vars['health'] = 0;
  };
  for (let object of game.getObjectsIn(objects['childMonster'])) {
    locals['m'] = object;
    if (0 < (locals['m'].vars['health'] || 0)) {
      locals['m'].vars['health'] = 0;
      globals['winKillMonster'] = (globals['winKillMonster'] + 1);
      game.move(locals, locals['m'], rooms['den1']);
    };
  };
  player.vars['hasMagicRing'] = 1;
  globals['foundMagicRing'] = 1;
  for (let object of game.getObjectsIn(objects['sword'])) {
    locals['s'] = object;
    game.move(locals, locals['s'], player);
  };
  for (let object of game.getObjectsIn(objects['sword'])) {
    locals['inst'] = object;
    locals['inst'].vars['damage'] = 45;
  };
  for (let object of game.getObjectsIn(objects['axe'])) {
    locals['inst'] = object;
    locals['inst'].vars['damage'] = 16;
  };
  for (let object of game.getObjectsIn(objects['knife'])) {
    locals['inst'] = object;
    locals['inst'].vars['damage'] = 6;
  };
  player.vars['maxHealth'] = 150;
  player.vars['health'] = 150;
  game.move(locals, player, rooms['den1']);
  globals['parentMonsterFirstEncounter'] = 0;
  for (let object of game.getObjectsIn(objects['parentMonster'])) {
    locals['m'] = object;
    game.move(locals, locals['m'], rooms['den1']);
  };
  return 0;
}

function debug(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("Player vars", '\n');
  for (let varItem of game.getVariablesOf(player)) {
    locals['k'] = varItem.name;
    locals['v'] = varItem.val;
    game.log(locals['k'], ": ", locals['v'].toString(), '\n');
  };
  game.log("Random vars", '\n');
  game.log("KILLED-CROW: ", globals['killedCrow'].toString(), '\n');
  game.log("COMBAT-HIGH-CHANCE: ", globals['combatHighChance'].toString(), '\n');
  game.log("COMBAT-DAMAGE: ", globals['combatDamage'].toString(), '\n');
  game.log("COMBAT-MAX-DAMAGE: ", globals['combatMaxDamage'].toString(), '\n');
  game.log("FIRE-IN-DRAFTY-CABIN: ", globals['fireInDraftyCabin'].toString(), '\n');
  game.log("FOREST-BURNED-DOWN: ", globals['forestBurnedDown'].toString(), '\n');
  game.log("TOTAL-SLEEPS: ", globals['totalSleeps'].toString(), '\n');
  game.log("NAPS-TODAY: ", globals['napsToday'].toString(), '\n');
  game.log("TOTAL-FOOD: ", globals['totalFood'].toString(), '\n');
  game.log("FOUND-MAGIC-RING: ", globals['foundMagicRing'].toString(), '\n');
  game.log("Monster vars", '\n');
  game.log("PARENT-MONSTER-HEALTH-STEP: ", globals['parentMonsterHealthStep'].toString(), '\n');
  game.log("PARENT-MONSTER-ATK-DMG: ", globals['parentMonsterAtkDmg'].toString(), '\n');
  game.log("CHILD-MONSTER-ATK-DMG: ", globals['childMonsterAtkDmg'].toString(), '\n');
  game.log("PARENT-MONSTER-FIRST-ENCOUNTER: ", globals['parentMonsterFirstEncounter'].toString(), '\n');
  game.log("PARENT-MONSTER-WILL-CHASE: ", globals['parentMonsterWillChase'].toString(), '\n');
  game.log("CHILD-MONSTER-WILL-CHASE: ", globals['childMonsterWillChase'].toString(), '\n');
  game.log("Win vars", '\n');
  game.log("WIN-ENTER-CABIN: ", globals['winEnterCabin'].toString(), '\n');
  game.log("WIN-LIGHT-FIRE: ", globals['winLightFire'].toString(), '\n');
  game.log("WIN-COOK-MEAL: ", globals['winCookMeal'].toString(), '\n');
  game.log("WIN-FIND-GEM: ", globals['winFindGem'].toString(), '\n');
  game.log("WIN-ENTER-CAVE: ", globals['winEnterCave'].toString(), '\n');
  game.log("WIN-KILL-MONSTER: ", globals['winKillMonster'].toString(), '\n');
  game.log("WIN-WRITE-NOTE: (later)", '\n');
  game.log("WIN-BUILD-BOAT: ", globals['winBuildBoat'].toString(), '\n');
  game.log("Conditions", '\n');
  if ((locals['cRoom'].vars['aboveGround'] || 0) === 1) {
    game.log("ABOVE-GROUND: 1", '\n');
  } else if (1 === 1) {
    game.log("ABOVE-GROUND: !1", '\n');
  };
  if (
  routines['roomHasLight'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']) === 1) {
    game.log("ROOM-HAS-LIGHT: 1", '\n');
  } else if (1 === 1) {
    game.log("ROOM-HAS-LIGHT: !1", '\n');
  };
  return 0;
}

function drop(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[1] ?? getEmptyResource()), false)) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), locals['cRoom']);
    game.log("Dropped ", '\n');
    if (game.isEqual(objects['water'], (locals['cmd']?.[1] ?? getEmptyResource()))) {
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    };
  };
  return 0;
}

function eat(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['isEdible'] || 0) === 1)) {
    game.log("You can't eat that.", '\n');
    if (50 > Math.floor(Math.random() * 100)) {
      game.log("But you try anyway. Take 2 damage", '\n');
      player.vars['health'] = ((player.vars['health'] || 0) - 2);
    };
  } else if ((game.isEqual(objects['soup'], (locals['cmd']?.[1] ?? getEmptyResource()))) || (game.isEqual(objects['tea'], (locals['cmd']?.[1] ?? getEmptyResource())))) {
    globals['winCookMeal'] = 1;
    game.log("Yum. You gain 40 health");
    if ((game.isEqual(objects['tea'], (locals['cmd']?.[1] ?? getEmptyResource()))) && ((player.vars['maxHealth'] || 0) < globals['maxMaxHealth'])) {
      game.log(", and feel yourself growing stronger", '\n');
      player.vars['maxHealth'] = ((player.vars['maxHealth'] || 0) + 25);
    } else if (1 === 1) {
      game.log('\n');
    };
    player.vars['health'] = ((player.vars['health'] || 0) + 40);
    if ((((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasMushroom'] || 0) === 1) && (!(game.isInLocation(locals['cRoom'], rooms['forest3'], objects['treeHollow'], false))) && ((game.isInLocation(locals['cRoom'], game.getInst(rooms['forest3'], 'treeHollow', false), objects['gem'], false)) || (game.isInLocation(locals['cRoom'], game.getInst(rooms['storage'], 'treeHollow', false), objects['gem'], false)))) {
      game.log("You have a vision of a sparkling gem, hidden in the hollow of a tree, east of the forest lake.", '\n');
      game.move(locals, game.getInst(rooms['storage'], 'treeHollow', false), rooms['forest3']);
    };
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
  } else if ((game.isEqual(objects['bones'], (locals['cmd']?.[1] ?? getEmptyResource()))) || (game.isEqual(objects['mushroom'], (locals['cmd']?.[1] ?? getEmptyResource()))) || (game.isEqual(objects['root'], (locals['cmd']?.[1] ?? getEmptyResource())))) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    if (50 > Math.floor(Math.random() * 100)) {
      game.log("It's a little chewy, but ok. You gain 5 health.", '\n');
      player.vars['health'] = ((player.vars['health'] || 0) + 5);
    } else if (1 === 1) {
      game.log("It doesn't agree with you. Take 5 damage", '\n');
      player.vars['health'] = ((player.vars['health'] || 0) - 5);
    };
  } else if (1 === 1) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    game.log("You gain 10 health.");
    if (globals['winCookMeal'] === 0) {
      game.log(" Cook some soup for a heartier meal.");
    };
    game.log('\n');
    player.vars['health'] = ((player.vars['health'] || 0) + 10);
  };
  if ((player.vars['health'] || 0) > (player.vars['maxHealth'] || 0)) {
    player.vars['health'] = (player.vars['maxHealth'] || 0);
  };
  return 0;
}

function book_empty(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], (locals['cmd']?.[1] ?? getEmptyResource()), objects['bookPage'], false)) {
    game.move(locals, game.getInst((locals['cmd']?.[1] ?? getEmptyResource()), 'bookPage', false), player);
    game.log("You've ripped out some blank pages from the book.", '\n');
  } else if (1 === 1) {
    game.log("The book is already empty.", '\n');
  };
  return 0;
}

function empty(cRoom, cmd) {
  const locals = {cRoom, cmd, count: 0};
  for (let object of game.getObjectsIn((locals['cmd']?.[1] ?? getEmptyResource()))) {
    locals['obj'] = object;
    if (!((locals['obj'].vars['noTake'] || 0) === 1)) {
      game.move(locals, locals['obj'], locals['cRoom']);
      locals['count'] = (locals['count'] + 1);
    };
  };
  if (locals['count'] > 0) {
    game.log("Emptied", '\n');
  };
  return 0;
}

function enter(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], rooms['cabinExterior'], player, false)) {
    if ((game.getInst(rooms['cabinExterior'], 'cabinDoor', false).vars['isLocked'] || 0) === 0) {
      game.move(locals, player, rooms['cabin']);
    } else if ((game.getInst(rooms['cabinExterior'], 'cabinWindow', false).vars['isBroken'] || 0) === 1) {
      game.move(locals, player, rooms['cabin']);
    } else if (1 === 1) {
      game.log("The door is locked. Where's the key?", '\n');
    };
  } else if (1 === 1) {
    game.log("You can't", '\n');
  };
  return 0;
}

function eunice(cRoom, cmd) {
  const locals = {cRoom, cmd, diff: 0};
  for (let object of game.getObjectsIn(objects['sword'])) {
    locals['inst'] = object;
    if ((locals['inst'].vars['maxDamage'] || 0) === 30) {
      locals['inst'].vars['maxDamage'] = 45;
      locals['diff'] = 1;
    };
  };
  for (let object of game.getObjectsIn(objects['axe'])) {
    locals['inst'] = object;
    if ((locals['inst'].vars['maxDamage'] || 0) === 10) {
      locals['inst'].vars['maxDamage'] = 16;
      locals['diff'] = 1;
    };
  };
  for (let object of game.getObjectsIn(objects['knife'])) {
    locals['inst'] = object;
    if ((locals['inst'].vars['maxDamage'] || 0) === 4) {
      locals['inst'].vars['maxDamage'] = 6;
      locals['diff'] = 1;
    };
  };
  for (let object of game.getObjectsIn(objects['obsidianShard'])) {
    locals['inst'] = object;
    if ((locals['inst'].vars['maxDamage'] || 0) === 50) {
      locals['inst'].vars['maxDamage'] = 70;
      if ((locals['inst'].vars['damage'] || 0) === 50) {
        locals['inst'].vars['damage'] = 70;
      };
      locals['diff'] = 1;
    };
  };
  if (locals['diff'] === 1) {
    game.log("You suddenly understand how the owl keeps it's talons so sharp. Your weapons could be deadlier.", '\n');
    player.vars['tOwl'] = 2;
  };
  return 0;
}

function ever(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['parentMonsterAtkDmg'] === 40) {
    globals['parentMonsterAtkDmg'] = 28;
    globals['childMonsterAtkDmg'] = 2;
    game.log("You realize that sometimes you can't change the die, only the wager. Enemies can't hit you nearly as hard when you've got less exposure.", '\n');
    player.vars['tRabbit'] = 2;
  };
  return 0;
}

function examine(cRoom, cmd) {
  const locals = {cRoom, cmd, count: 0,r: 0};
  if (globals['firstExamine'] === 1) {
    globals['firstExamine'] = 0;
    game.log("The EXAMINE command will list items nested inside an object, and might also tell you more about the object itself. If there's any interesting items in this object, you can try to EMPTY it, then TAKE items off the ground.", '\n');
  };
  globals['detailedDesc'] = 1;
  game.describe((locals['cmd']?.[1] ?? getEmptyResource()));
  globals['detailedDesc'] = 0;
  game.log('\n');
  for (let object of game.getObjectsIn((locals['cmd']?.[1] ?? getEmptyResource()))) {
    locals['obj'] = object;
    locals['count'] = (locals['count'] + 1);
  };
  if (locals['count'] === 0) {
    return 1;
  };
  game.log("items inside:", '\n');
  for (let object of game.getObjectsIn((locals['cmd']?.[1] ?? getEmptyResource()))) {
    locals['obj'] = object;
    game.describe(locals['obj']);
    game.log('\n');
  };
  return 0;
}

function exit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], rooms['cabin'], player, false)) {
    game.getInst(rooms['cabinExterior'], 'cabinDoor', false).vars['isLocked'] = 0;
    game.move(locals, player, rooms['cabinExterior']);
  } else if (1 === 1) {
    game.log("You're not in the cabin", '\n');
  };
  return 0;
}

function great(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual(locals['cRoom'], rooms['cavern1'])) {
    if ((game.getInst(rooms['cavern1'], 'stoneDoor', false).vars['isLocked'] || 0) === 1) {
      game.getInst(rooms['cavern1'], 'stoneDoor', false).vars['isLocked'] = 0;
      game.log("The stone door clicks, and slides open", '\n');
      game.move(locals, player, rooms['crypt']);
    } else if (1 === 1) {
      game.log("The stone door rumbles, but it can't open any further", '\n');
    };
    player.vars['tBat'] = 2;
  } else if ((game.getInst(rooms['cavern1'], 'stoneDoor', false).vars['isLocked'] || 0) === 1) {
    game.log("You hear a deep but faint rumble of stone on stone; something is trying to move. Maybe you're not in the right place.", '\n');
  } else if (1 === 1) {
    game.log("Far away, you hear the stone door rumbling.", '\n');
  };
  return 0;
}

function growing(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isInLocation(locals['cRoom'], rooms['forest3'], objects['treeHollow'], false))) {
    game.move(locals, game.getInst(rooms['storage'], 'treeHollow', false), rooms['forest3']);
    if (game.isInLocation(locals['cRoom'], rooms['forest3'], player, false)) {
      game.log("You notice a hollow in that tree over there, looks like a good place to hide things.", '\n');
    } else if (1 === 1) {
      game.log("A voice on the breeze whispers 'You'll never find my tree hollow'", '\n');
    };
  };
  if (!((player.vars['tCrow'] || 0) === 2)) {
    player.vars['tCrow'] = 2;
    game.log("You realize that all things are one, and finding a way through is not so difficult.", '\n');
  };
  return 0;
}

function cabinDoor_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (!(game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['cabinDoor']))) {
    return 0;
  };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
      globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
      globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    } else if (1 === 1) {
      return 0;
    };
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("The door takes no damage", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
    game.log("You hit the door for ", locals['dmg'].toString(), " damage.", '\n');
    if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) < 1) {
      for (let i of Array.from(Array(Math.max(0, ((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0))).keys())) {
        locals['v'] = i;
        game.copyMove(objects['roughBoard'], locals['cRoom']);
      };
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = 0;
      game.log("It's broken into some rough BOARDs", '\n');
    };
  } else if (1 === 1) {
    game.log("It's already broken.", '\n');
  };
  return 0;
}

function book_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['book']))) {
    return 0;
  };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0) {
    game.log("Why are you hitting this book?", '\n');
    if (game.isInLocation(locals['cRoom'], (locals['cmd']?.[1] ?? getEmptyResource()), objects['bookPage'], false)) {
      game.move(locals, game.getInst((locals['cmd']?.[1] ?? getEmptyResource()), 'bookPage', false), locals['cRoom']);
      game.log("You've cut out some pages onto the floor.", '\n');
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) + -1);
  } else if (1 === 1) {
    game.log("The book is dead.", '\n');
    if (game.isInLocation(locals['cRoom'], (locals['cmd']?.[1] ?? getEmptyResource()), objects['bookPage'], false)) {
      game.move(locals, game.getInst((locals['cmd']?.[1] ?? getEmptyResource()), 'bookPage', false), );
    };
  };
  return 0;
}

function table_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (!(game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['table']))) {
    return 0;
  };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
      globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
      globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    } else if (1 === 1) {
      return 0;
    };
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("The table takes no damage", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
    game.log("You hit the table for ", locals['dmg'].toString(), " damage.", '\n');
    if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) < 1) {
      for (let i of Array.from(Array(Math.max(0, ((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0))).keys())) {
        locals['v'] = i;
        game.copyMove(objects['roughBoard'], locals['cRoom']);
      };
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = 0;
      game.log("It's broken into some rough BOARDs", '\n');
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    };
  };
  return 0;
}

function chair_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
      globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
      globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    } else if (1 === 1) {
      return 0;
    };
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("The chair takes no damage", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
    game.log("You hit the chair for ", locals['dmg'].toString(), " damage.", '\n');
    if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) < 1) {
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = 0;
      game.copyMove(objects['roughBoard'], locals['cRoom']);
      game.log("It's broken into a rough BOARD", '\n');
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    };
  };
  return 0;
}

function bedFrame_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (!(game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['bedFrame']))) {
    return 0;
  };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
      globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
      globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    } else if (1 === 1) {
      return 0;
    };
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("The bed takes no damage", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
    game.log("You hit the bed for ", locals['dmg'].toString(), " damage.", '\n');
    if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) < 1) {
      for (let i of Array.from(Array(Math.max(0, ((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0))).keys())) {
        locals['v'] = i;
        game.copyMove(objects['roughBoard'], locals['cRoom']);
      };
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = 0;
      game.log("It's broken into some rough BOARDs", '\n');
    };
  };
  return 0;
}

function cabinWindow_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['isBroken'] || 0) === 1)) {
    game.log("You smash the window, taking 1 damage in the process.", '\n');
    player.vars['health'] = ((player.vars['health'] || 0) - 1);
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['isBroken'] = 1;
    game.getInst(rooms['cabinExterior'], 'cabinDoor', false).vars['isLocked'] = 0;
    game.move(locals, player, rooms['cabin']);
  };
  return 0;
}

function caveSpider_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if ((game.isEqual(objects['caveSpider'], (locals['cmd']?.[1] ?? getEmptyResource()))) && (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0)) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] > 0) {
      game.log("You hit the spider for ", locals['dmg'].toString(), " damage.", '\n');
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
    };
    if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
      game.log("It's dead.", '\n');
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    };
  };
  return 0;
}

function childMonster_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (!(game.isEqual(objects['childMonster'], (locals['cmd']?.[1] ?? getEmptyResource())))) {
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
  } else if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] > 0) {
      game.log("You hit the monster for ", locals['dmg'].toString(), " damage.", '\n');
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
    };
    if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
      game.log("It's dead.", '\n');
      globals['winKillMonster'] = (globals['winKillMonster'] + 1);
      globals['childMonsterWillChase'] = 0;
    };
  };
  return 0;
}

function parentMonster_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (!(game.isEqual(objects['parentMonster'], (locals['cmd']?.[1] ?? getEmptyResource())))) {
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
  } else if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] > 0) {
      game.log("You hit the monster for ", locals['dmg'].toString(), " damage.", '\n');
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
    };
    if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
      game.log("It's dead.", '\n');
      globals['winKillMonster'] = (globals['winKillMonster'] + 1);
      globals['parentMonsterWillChase'] = 0;
    };
  };
  return 0;
}

function stoneDoor_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (game.isEqual(objects['stoneDoor'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    return 0;
  } else if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) === 0) {
    return 0;
  } else if (1 > ((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0)) {
    game.log("It's already cracked.", '\n');
    return 0;
  };
  globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
  globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
  locals['dmg'] = 
  routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  if (locals['dmg'] === 0) {
    game.log("The door takes no damage", '\n');
    return 0;
  };
  (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
  if (!(0 < ((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0))) {
    game.log("You manage to crack the rock, breaking the door.", '\n');
    game.move(locals, player, rooms['crypt']);
  };
  return 0;
}

function rock_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (5 > Math.floor(Math.random() * 100)) {
    game.log("You found a lump of GOLD.", '\n');
    game.copyMove(objects['goldLump'], locals['cRoom']);
  };
  return 0;
}

function cursedSkull_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("The skull splits into three pieces, and spirit rises up.", '\n');
  player.vars['health'] = 0;
  return 0;
}

function hit_axe(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isEqual(objects['axe'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
  };
  if ((player.vars['tCrow'] || 0) === 2) {
  } else if ((0 < ((locals['cmd']?.[1] ?? getEmptyResource()).vars['maxDamage'] || 0)) || (0 < ((locals['cmd']?.[1] ?? getEmptyResource()).vars['isHard'] || 0))) {
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0) - 2);
    game.log("Your axe takes 2 points of damage", '\n');
  } else if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['isSoft'] || 0) === 1) {
  } else if (1 === 1) {
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0) - 1);
    game.log("Your axe takes 1 point of damage", '\n');
  };
  if (0 > ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0)) {
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = 0;
  };
  return 0;
}

function hit_knife(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isEqual(objects['knife'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
  };
  if ((player.vars['tCrow'] || 0) === 2) {
  } else if ((((locals['cmd']?.[1] ?? getEmptyResource()).vars['isSoft'] || 0) === 1) && (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['isHard'] || 0) === 1))) {
  } else if (1 === 1) {
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0) - 1);
    game.log("Your knife takes 1 point of damage", '\n');
    if (0 > ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0)) {
      (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = 0;
    };
  };
  return 0;
}

function hit_obsidianShard(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isEqual(objects['obsidianShard'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
  };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['isHard'] || 0) === 1) {
    game.log("The obsidian shatters", '\n');
    game.move(locals, (locals['cmd']?.[2] ?? getEmptyResource()), );
  };
  (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0) - 28);
  game.log("The obsidian's edge chips. It takes 28 points of damage", '\n');
  if (0 > ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0)) {
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = 0;
  };
  return 0;
}

function hit_pickAxe(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  return 0;
}

function hit_sword(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isEqual(objects['axe'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
  };
  if ((player.vars['tCrow'] || 0) === 2) {
  } else if ((0 < ((locals['cmd']?.[1] ?? getEmptyResource()).vars['maxDamage'] || 0)) || (0 < ((locals['cmd']?.[1] ?? getEmptyResource()).vars['isHard'] || 0))) {
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0) - 5);
    game.log("Your sword takes 5 points of damage", '\n');
  } else if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['isSoft'] || 0) === 1) {
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0) - 1);
  } else if (1 === 1) {
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0) - 2);
    game.log("Your sword takes 2 points of damage", '\n');
  };
  if (0 > ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0)) {
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] = 0;
  };
  return 0;
}

function hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  for (let object of game.getObjectsIn(rooms['qStorage'])) {
    locals['obj'] = object;
    game.move(locals, locals['obj'], );
  };
  return 0;
}

function tree_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("The tree takes no damage", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
    game.log("The tree takes ", locals['dmg'].toString(), " damage", '\n');
    if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) < 1) {
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
      game.copyMove(objects['log'], locals['cRoom']);
      game.copyMove(objects['log'], locals['cRoom']);
      game.copyMove(objects['log'], locals['cRoom']);
      game.copyMove(objects['log'], locals['cRoom']);
      game.copyMove(objects['stick'], locals['cRoom']);
      game.copyMove(objects['stick'], locals['cRoom']);
      game.log("You've chopped the tree into four LOGs and two STICKs", '\n');
    };
  };
  if ((((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0) && (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) && (!(game.isInLocation(locals['cRoom'], locals['cRoom'], objects['sap'], false)))) {
    game.copyMove(objects['sap'], locals['cRoom']);
  };
  return 0;
}

function log_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("The log takes no damage", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
    game.log("The log takes ", locals['dmg'].toString(), " damage", '\n');
    if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) < 1) {
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
      game.copyMove(objects['roughBoard'], locals['cRoom']);
      game.copyMove(objects['roughBoard'], locals['cRoom']);
      game.copyMove(objects['roughBoard'], locals['cRoom']);
      game.copyMove(objects['roughBoard'], locals['cRoom']);
      game.log("You've chopped the log into four rough BOARDs", '\n');
    };
  };
  return 0;
}

function mushroom_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("'can you feel your heart burning? can you feel the struggle within? the fear within me is beyond anything your soul can make. you cannot kill me in a way that matters'", '\n');
  return 0;
}

function frog_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['frog'])) && (game.isEqual((locals['cmd']?.[2] ?? getEmptyResource()), objects['frog']))) {
    game.log("They bounce off eachother", '\n');
  } else if (game.isEqual((locals['cmd']?.[2] ?? getEmptyResource()), objects['frog'])) {
    if (!(((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0) === 0)) {
      game.log("The frog dies", '\n');
      (locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] = 0;
    };
  } else if ((game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['frog'])) && (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) === 0)) {
    game.log("The frog is already dead", '\n');
  } else if ((game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['frog'])) && (game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[1] ?? getEmptyResource()), false))) {
    game.log("The frog dies", '\n');
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
  } else if (1 === 1) {
    if (80 > Math.floor(Math.random() * 100)) {
      game.log("You hit the frog, it dies", '\n');
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
    } else if (1 === 1) {
      game.log("The frog is too quick for you, better luck next time", '\n');
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    };
  };
  return 0;
}

function bear_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (game.isEqual(objects['bear'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
  };
  (locals['cmd']?.[1] ?? getEmptyResource()).vars['enraged'] = 1;
  if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
  } else if (1 === 1) {
    game.log("You can't hit it with that", '\n');
    return 0;
  };
  locals['dmg'] = 
  routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  if (locals['dmg'] === 0) {
    game.log("The bear takes no damage", '\n');
    return 1;
  };
  (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) - locals['dmg']);
  game.log("You hit the bear for ", locals['dmg'].toString(), " damage.", '\n');
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    game.log("The bear is dead", '\n');
  } else if (1 === 1) {
    game.log("It lumbers away", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), rooms['storage']);
  };
  return 0;
}

function snake_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual(objects['snake'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0) > 0) {
      (locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] = 0;
      game.log("The snake dies", '\n');
    };
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
  };
  if (game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[1] ?? getEmptyResource()), false)) {
    game.log("You kill it.", '\n');
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
  } else if (1 === 1) {
    if (67 > Math.floor(Math.random() * 100)) {
      game.log("You hit it, and it dies", '\n');
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
    } else if (1 === 1) {
      game.log("The snake is too quick for you, better luck next time", '\n');
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    };
  };
  return 0;
}

function rabbit_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual(objects['rabbit'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0) > 0) {
      (locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] = 0;
      game.log("The rabbit dies", '\n');
    };
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
  };
  if (game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[1] ?? getEmptyResource()), false)) {
    game.log("You kill it.", '\n');
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
  } else if (1 === 1) {
    if (40 > Math.floor(Math.random() * 100)) {
      game.log("You hit it, and it dies", '\n');
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
    } else if (1 === 1) {
      game.log("The rabbit is too quick for you, better luck next time", '\n');
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    };
  };
  return 0;
}

function crow_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual(objects['crow'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0) > 0) {
      (locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] = 0;
      game.log("The crow dies", '\n');
    };
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
  };
  if (game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[1] ?? getEmptyResource()), false)) {
    game.log("You kill it.", '\n');
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
  } else if (game.isEqual(objects['knife'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    game.log("You throw your knife at the crow.", '\n');
    if ((5 > Math.floor(Math.random() * 100)) && (globals['killedCrow'] === 0)) {
      game.log("You hit it, and it dies.", '\n');
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
      game.move(locals, game.getInst(player, 'knife', false), locals['cRoom']);
      globals['killedCrow'] = 1;
    } else if (1 === 1) {
      game.log("You miss, and lose your knife. The crow laughs at you.", '\n');
      game.move(locals, game.getInst(locals['cRoom'], 'knife', false), );
      game.move(locals, game.getInst(player, 'knife', false), );
    };
  } else if (1 === 1) {
    game.log("Try hitting it with your knife", '\n');
  };
  return 0;
}

function owl_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual(objects['owl'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0) > 0) {
      (locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] = 0;
      game.log("The owl dies", '\n');
    };
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
  };
  if (game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[1] ?? getEmptyResource()), false)) {
    game.log("You kill it.", '\n');
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
  } else if (1 === 1) {
    game.log("You miss, and the owl flies away.", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
  };
  return 0;
}

function bat_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual(objects['bat'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0) > 0) {
      (locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] = 0;
      game.log("The bat dies", '\n');
    };
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
  };
  if (game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[1] ?? getEmptyResource()), false)) {
    game.log("You kill it.", '\n');
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
  } else if (1 === 1) {
    if (50 > Math.floor(Math.random() * 100)) {
      game.log("You hit it, and it dies", '\n');
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = 0;
    } else if (1 === 1) {
      game.log("The bat just keeps flapping around", '\n');
    };
  };
  return 0;
}

function inventory(cRoom, cmd) {
  const locals = {cRoom, cmd, count: 0};
  game.log("You have ", (player.vars['health'] || 0).toString(), " health (of a maximum ", (player.vars['maxHealth'] || 0).toString(), ")");
  for (let object of game.getObjectsIn(player)) {
    locals['obj'] = object;
    locals['count'] = (locals['count'] + 1);
  };
  if (locals['count'] === 0) {
    game.log(", and you're not carrying anything.", '\n');
  } else if (1 === 1) {
    game.log(", and you're carrying:", '\n');
    for (let object of game.getObjectsIn(player)) {
      locals['obj'] = object;
      locals['count'] = (locals['count'] + 1);
      game.describe(locals['obj']);
      game.log('\n');
    };
  };
  return 0;
}

function jump(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("No thank you", '\n');
  return 0;
}

function listening(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['parentMonsterHealthStep'] === 8) {
    globals['parentMonsterHealthStep'] = 5;
    for (let object of game.getObjectsIn(objects['parentMonster'])) {
      locals['m'] = object;
      locals['m'].vars['maxHealth'] = ((locals['m'].vars['maxHealth'] || 0) - 10);
    };
    for (let object of game.getObjectsIn(objects['childMonster'])) {
      locals['m'] = object;
      locals['m'].vars['maxHealth'] = 90;
    };
    game.log("You can suddenly picture the path ahead, enemies weaken at your step.", '\n');
    player.vars['tSnake'] = 2;
  };
  return 0;
}

function look(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['firstLookAround'] === 1) {
    game.log("LOOK AROUND runs two commands:", '\n');
  };
  routines['vRoomDetail'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  if (!(globals['firstLookAround'] === 1)) {
    game.log("items:", '\n');
  };
  routines['vDescObjectsInRoom'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  if (globals['firstLookAround'] === 1) {
    globals['firstLookAround'] = 0;
  };
  return 0;
}

function no(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((game.getInst(rooms['qStorage'], 'ynQ', false).vars['fromCrow'] || 0) === 1) {
    game.log("'Shame'", '\n');
    game.move(locals, game.getInst(rooms['qStorage'], 'ynQ', false), );
  } else if ((game.getInst(rooms['qStorage'], 'ynQ', false).vars['fromBear'] || 0) === 1) {
    game.log("The bear shrugs, then sits down to forage", '\n');
    game.getInst(locals['cRoom'], 'bear', false).vars['asked'] = 1;
    game.move(locals, game.getInst(rooms['qStorage'], 'ynQ', false), );
  };
  return 0;
}

function open(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((game.isEqual(locals['cRoom'], rooms['cabin'])) && (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[2] ?? getEmptyResource())))) {
    game.log("Assuming you typed OPEN DOOR, try 'GO ...' instead", '\n');
    if (10 > Math.floor(Math.random() * 100)) {
      game.log("You see, an instance can only exist in one room at a time. That leaves us with two options; A: one CABIN-DOOR in CABIN and a second CABIN-DOOR in CABIN-EXTERIOR; and B: one CABIN-DOOR that follows the player back-and-forth between CABIN and CABIN-EXTERIOR (this is what CABIN-WINDOW does). Option A is annoying if the player HITs a door. Option B is annoying because the instance could get lost. Having already spent too much time on this game, I chose option C.", '\n');
    };
  };
  return 0;
}

function cabinDoor_open(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual(locals['cRoom'], rooms['cabin'])) {
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['isLocked'] = 0;
    game.move(locals, player, rooms['cabinExterior']);
  } else if ((((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) < 1) || (((locals['cmd']?.[1] ?? getEmptyResource()).vars['isLocked'] || 0) === 0)) {
    game.move(locals, player, rooms['cabin']);
  } else if (((game.isInLocation(locals['cRoom'], player, objects['cabinDoorKey'], false)) || (game.isInLocation(locals['cRoom'], player, objects['masterKey'], false))) && (((locals['cmd']?.[1] ?? getEmptyResource()).vars['isLocked'] || 0) === 1)) {
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['isLocked'] = 0;
    game.log("The key works.", '\n');
    game.move(locals, player, rooms['cabin']);
  } else if (1 === 1) {
    game.log("The door is locked. You should LOOK AROUND for a key.", '\n');
  };
  return 0;
}

function stoneDoor_open(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual(locals['cRoom'], rooms['crypt'])) {
    game.log("You press a small panel in the wall. The door slides open.", '\n');
    game.getInst(rooms['crypt'], 'stoneDoor', false).vars['isLocked'] = 0;
    game.move(locals, player, rooms['cavern1']);
  } else if (game.isInLocation(locals['cRoom'], player, objects['masterKey'], false)) {
    game.log("The key fits into a crevice of the door. It slides open.", '\n');
    game.getInst(rooms['crypt'], 'stoneDoor', false).vars['isLocked'] = 0;
    game.move(locals, player, rooms['crypt']);
  };
  return 0;
}

function pee(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['fire'])) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    game.log("The fire goes out.", '\n');
    game.copyMove(objects['charcoal'], locals['cRoom']);
  } else if (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['torch'])) {
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['lit'] = 0;
    game.log("The torch goes out.", '\n');
  } else if (1 === 1) {
    game.log("Cool.", '\n');
  };
  return 0;
}

function school(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['combatHighChance'] === 50) {
    globals['combatHighChance'] = 75;
    game.log("Your weapons suddenly feel lighter, nimbler. Water can crash, or it can flow.", '\n');
    player.vars['tFish'] = 2;
  };
  return 0;
}

function sleep(cRoom, cmd) {
  const locals = {cRoom, cmd, st: 0};
  if (!(game.isInLocation(locals['cRoom'], player, objects['cloak'], false))) {
    game.log("It's not safe to sleep without your cloak.", '\n');
    return 0;
  } else if (-9 > globals['time']) {
    globals['napsToday'] = (globals['napsToday'] + 1);
    game.log("You settle down for what sleep you can get.", '\n');
    locals['st'] = (globals['time'] - -17);
  } else if (0 > globals['time']) {
    globals['totalSleeps'] = (globals['totalSleeps'] + 1);
    game.log("You settle down for the night.", '\n');
    locals['st'] = (globals['time'] - -17);
  } else if (1 === 1) {
    globals['napsToday'] = (globals['napsToday'] + 1);
    game.log("You settle in for a nice nap.", '\n');
    locals['st'] = 4;
  };
  for (let i of Array.from(Array(Math.max(0, locals['st'])).keys())) {
    locals['val'] = i;
    routines['burnFire'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    routines['checkPulse'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    routines['timePasses'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  };
  return 0;
}

function flint_spark(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isEqual(objects['flint'], (locals['cmd']?.[1] ?? getEmptyResource())))) {
    return 0;
  };
  if ((game.isEqual(objects['stick'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['detritus'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['sap'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['bulrush'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['book'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['bookPage'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['note'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['strap'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
    game.log("It catches. Keep the fire alive by ADDing some logs TO it.", '\n');
    game.move(locals, (locals['cmd']?.[2] ?? getEmptyResource()), );
    game.copyMove(objects['fire'], locals['cRoom']);
    for (let object of game.getObjectsIn(locals['cRoom'])) {
      locals['obj'] = object;
      if (game.isEqual(objects['charcoal'], locals['obj'])) {
        game.move(locals, locals['obj'], );
      };
    };
    if ((locals['cRoom'].vars['aboveGround'] || 0) === 0) {
      game.log("You take 1 damage from smoke", '\n');
      player.vars['health'] = ((player.vars['health'] || 0) - 1);
    } else if ((game.isEqual(locals['cRoom'], rooms['cabin'])) && ((1 > (game.getInst(rooms['cabinExterior'], 'cabinDoor', false).vars['health'] || 0)) || ((1 > (game.getInst(rooms['cabin'], 'cabinWindow', false).vars['health'] || 0)) && (1 > (game.getInst(rooms['cabinExterior'], 'cabinWindow', false).vars['health'] || 0))))) {
      globals['fireInDraftyCabin'] = 1;
    };
  } else if (game.isEqual(objects['torch'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    game.log("It catches, but torches burn up pretty quickly. You won't have light for long.", '\n');
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['isLit'] = 1;
  } else if (game.isEqual((locals['cmd']?.[2] ?? getEmptyResource()), (locals['cmd']?.[3] ?? getEmptyResource()))) {
    game.log("Nope.", '\n');
  };
  return 0;
}

function swim(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("The water looks a little chilly, it would be better if we had a boat, or maybe built one?", '\n');
  return 0;
}

function take_rock(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['rockPile'], false)) {
    game.copyMove(objects['rock'], player);
  };
  return 0;
}

function take_rockPile(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), locals['cRoom']);
  game.copyMove(objects['rock'], player);
  return 0;
}

function take_magicRing(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("The ring fits perfectly onto you're finger, then painlessly melts into your skin.", '\n');
  player.vars['hasMagicRing'] = 1;
  game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
  return 0;
}

function take_caveSpider(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("You crush all of the cave spiders.", '\n');
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    if (game.isEqual(objects['caveSpider'], locals['obj'])) {
      game.move(locals, locals['obj'], );
    };
  };
  return 0;
}

function sword_take(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((player.vars['hasMagicRing'] || 0) === 1) {
    game.log("You pick up the sword. It hums with power.", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
  } else if (game.isInLocation(locals['cRoom'], player, objects['cursedSkull'], false)) {
    game.log("You pick up the sword. It seems to recognize it's owner.", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
  } else if (1 === 1) {
    game.log("As you pick the sword up, it turns to dust in your hands.", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
  };
  return 0;
}

function take_detritus(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  for (let object of game.getObjectsIn((locals['cmd']?.[1] ?? getEmptyResource()))) {
    locals['obj'] = object;
    game.move(locals, locals['obj'], locals['cRoom']);
  };
  game.log("Picked up", '\n');
  game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
  return 0;
}

function take_frog(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    game.log("Picked up", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
  } else if (75 > Math.floor(Math.random() * 100)) {
    game.log("You grab it.", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
  } else if (1 === 1) {
    game.log("It slips away.", '\n');
  };
  return 0;
}

function take_fish(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((game.isEqual(objects['water'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false))) && (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    if (((game.isInLocation(locals['cRoom'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false), objects['berries'], false)) || (game.isInLocation(locals['cRoom'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false), objects['nuts'], false))) && (40 > Math.floor(Math.random() * 100))) {
      game.log("You caught it", '\n');
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
    } else if (!((game.isInLocation(locals['cRoom'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false), objects['berries'], false)) || (game.isInLocation(locals['cRoom'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false), objects['nuts'], false)))) {
      game.log("It helps if there's some good bait in the water.", '\n');
    } else if (1 === 1) {
      game.log("Try again, it takes patience to catch a fish.", '\n');
    };
  } else if (1 === 1) {
    game.log("Picked up", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
  };
  return 0;
}

function take_rabbit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0) {
    if ((game.isInLocation(locals['cRoom'], player, objects['berries'], false)) || (game.isInLocation(locals['cRoom'], player, objects['herbs'], false)) || (game.isInLocation(locals['cRoom'], player, objects['fern'], false))) {
      game.log("Picked up", '\n');
    } else if (1 === 1) {
      game.log("It darts away, you're not carrying any food it likes.", '\n');
    };
  } else if (1 === 1) {
    game.log("Picked up", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
  };
  return 0;
}

function take_snake(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0) {
    game.log("The snake doesn't like this", '\n');
  } else if (1 === 1) {
    game.log("Picked up", '\n');
  };
  game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
  return 0;
}

function take_bat(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("The bat is too nimble.", '\n');
  return 0;
}

function take_bear(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("This bear doesn't want a hug; it snaps your neck.", '\n');
  player.vars['health'] = 0;
  return 0;
}

function water_take(cRoom, cmd) {
  const locals = {cRoom, cmd, cc: 0};
  if ((game.isInLocation(locals['cRoom'], player, objects['bucket'], false)) || (game.isInLocation(locals['cRoom'], player, objects['kettle'], false)) || (game.isInLocation(locals['cRoom'], player, objects['cup'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['bucket'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['kettle'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['cup'], false))) {
    locals['cc'] = 1;
  };
  if ((game.isInLocation(locals['cRoom'], player, objects['bucket'], false)) && (!(game.isInLocation(locals['cRoom'], game.getInst(player, 'bucket', false), objects['water'], false)))) {
    game.log("You fill the bucket with water", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), game.getInst(player, 'bucket', false));
  } else if ((game.isInLocation(locals['cRoom'], player, objects['kettle'], false)) && (!(game.isInLocation(locals['cRoom'], game.getInst(player, 'kettle', false), objects['water'], false)))) {
    game.log("You fill the kettle with water", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), game.getInst(player, 'kettle', false));
  } else if ((game.isInLocation(locals['cRoom'], player, objects['cup'], false)) && (!(game.isInLocation(locals['cRoom'], game.getInst(player, 'cup', false), objects['water'], false)))) {
    game.log("You fill the cup with water", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), game.getInst(player, 'cup', false));
  } else if ((game.isInLocation(locals['cRoom'], locals['cRoom'], objects['bucket'], false)) && (!(game.isInLocation(locals['cRoom'], game.getInst(locals['cRoom'], 'bucket', false), objects['water'], false)))) {
    game.log("You fill the bucket with water", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), game.getInst(locals['cRoom'], 'bucket', false));
  } else if ((game.isInLocation(locals['cRoom'], locals['cRoom'], objects['kettle'], false)) && (!(game.isInLocation(locals['cRoom'], game.getInst(locals['cRoom'], 'kettle', false), objects['water'], false)))) {
    game.log("You fill the kettle with water", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), game.getInst(locals['cRoom'], 'kettle', false));
  } else if ((game.isInLocation(locals['cRoom'], locals['cRoom'], objects['cup'], false)) && (!(game.isInLocation(locals['cRoom'], game.getInst(locals['cRoom'], 'cup', false), objects['water'], false)))) {
    game.log("You fill the cup with water", '\n');
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), game.getInst(locals['cRoom'], 'cup', false));
  } else if (locals['cc'] === 1) {
    game.log("All the containers are already full.", '\n');
  } else if (1 === 1) {
    game.log("You need a BUCKET, KETTLE, or CUP to carry the water.", '\n');
  };
  return 0;
}

function take(cRoom, cmd) {
  const locals = {cRoom, cmd, pu: 0};
  for (let object of game.getObjectsIn(rooms['qStorage'])) {
    locals['obj'] = object;
    game.move(locals, locals['obj'], );
  };
  if (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[2] ?? getEmptyResource()))) {
    game.log("There's nothing like that here.", '\n');
  } else if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['noTake'] || 0) === 1) {
    game.log("This can't be picked up", '\n');
  } else if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['ownTake'] || 0) === 1) {
  } else if (game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[1] ?? getEmptyResource()), false)) {
    for (let object of game.getObjectsIn(locals['cRoom'])) {
      locals['obj'] = object;
      if ((game.isEqual(locals['obj'], (locals['cmd']?.[1] ?? getEmptyResource()))) && (!(locals['pu'] === 1))) {
        game.log("Picked up", '\n');
        game.move(locals, locals['obj'], player);
        locals['pu'] = 1;
      };
    };
    if (!(locals['pu'] === 1)) {
      game.log("There's none in here", '\n');
    };
  } else if (game.isInLocation(locals['cRoom'], locals['cRoom'], (locals['cmd']?.[1] ?? getEmptyResource()), false)) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
    game.log("Picked up", '\n');
    if (globals['firstTake'] === 1) {
      globals['firstTake'] = 0;
      game.log("List items you're carrying using INVENTORY.", '\n');
    };
  } else if (1 === 1) {
    game.log("Huh?", '\n');
  };
  return 0;
}

function talk_beetle(cRoom, cmd) {
  const locals = {cRoom, cmd, n: 0};
  locals['n'] = Math.floor(Math.random() * 100);
  if (20 > locals['n']) {
    game.log("It flaps it's wings", '\n');
  } else if (40 > locals['n']) {
    game.log("It rolls away", '\n');
  } else if (60 > locals['n']) {
    game.log("It points urgently upwards. Or maybe it's just stretching.", '\n');
  } else if (80 > locals['n']) {
    game.log("It opens and closes it's carapace.", '\n');
  } else if (100 > locals['n']) {
    game.log("It doesn't listen.", '\n');
  };
  return 0;
}

function talk_owl(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    return 0;
  };
  if (((player.vars['tOwl'] || 0) === 0) || (globals['dayM3'] === 0)) {
    game.log("'Would you like to hear how EUNICE BROKE MY HEART? She left one night without saying goodbye.'", '\n');
  } else if (globals['dayM3'] === 1) {
    game.log("'Would you like to hear how EUNICE BROKE MY HEART? He left one night without saying goodbye.'", '\n');
  } else if (globals['dayM3'] === 2) {
    game.log("'Would you like to hear how EUNICE BROKE MY HEART? They left one night without saying goodbye.'", '\n');
  };
  game.log("The owl trails off, lost in thought.", '\n');
  if ((player.vars['tOwl'] || 0) < 1) {
    player.vars['tOwl'] = 1;
  };
  return 0;
}

function talk_crow(cRoom, cmd) {
  const locals = {cRoom, cmd, n: 0};
  locals['n'] = Math.floor(Math.random() * 100);
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    return 0;
  };
  if ((game.isInLocation(locals['cRoom'], rooms['qStorage'], objects['ynQ'], false)) && ((game.getInst(rooms['qStorage'], 'ynQ', false).vars['fromCrow'] || 0) === 1)) {
    game.getInst(rooms['qStorage'], 'ynQ', false).vars['exp'] = 1;
    if (34 > locals['n']) {
      game.log("'Are you deaf? Or just ugly?'", '\n');
    } else if (67 > locals['n']) {
      game.log("'I said WOULD YOU LIKE TO FLY'", '\n');
    } else if (100 > locals['n']) {
      game.log("The crow says a rude word", '\n');
    };
  } else if (34 > locals['n']) {
    game.log("'GROWING UP BACK HOME, my sister always used to tease me, so I'd try to steal her toys.'", '\n');
  } else if (67 > locals['n']) {
    game.log("'GROWING UP BACK HOME, I'd try to steal my sisters toys. She'd always tease me for it.'", '\n');
  } else if (100 > locals['n']) {
    game.log("'Don't you wish you could fly?'", '\n');
    game.copyMove(objects['ynQ'], rooms['qStorage']);
    game.getInst(rooms['qStorage'], 'ynQ', false).vars['fromCrow'] = 1;
  };
  if ((player.vars['tCrow'] || 0) < 1) {
    player.vars['tCrow'] = 1;
  };
  return 0;
}

function talk_fish(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    return 0;
  };
  if (((game.isEqual(objects['water'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false))) && ((game.isInLocation(locals['cRoom'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false), objects['berries'], false)) || (game.isInLocation(locals['cRoom'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false), objects['nuts'], false)))) || (game.isEqual(locals['cRoom'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false))) || (game.isEqual(player, game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false)))) {
    if (40 > Math.floor(Math.random() * 100)) {
      game.log("'You know, I don't think SCHOOL IS FOR ME. Oh sure I got a degree, but I haven't kept in touch with anyone from those days, and I don't miss it. I just felt out of place the whole time, my scales were crawling. It was nice to have some structure, but I prefer learning at my own pace.'", '\n');
      if ((player.vars['tFish'] || 0) < 1) {
        player.vars['tFish'] = 1;
      };
    } else if (1 === 1) {
      game.log("You know what they say about fishing", '\n');
    };
  } else if ((game.isEqual(objects['water'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false))) && (!((game.isInLocation(locals['cRoom'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false), objects['berries'], false)) || (game.isInLocation(locals['cRoom'], game.getParent((locals['cmd']?.[1] ?? getEmptyResource()), false), objects['nuts'], false))))) {
    game.log("Try adding some BERRIES or NUTS to the water", '\n');
  };
  return 0;
}

function talk_frog(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    return 0;
  };
  if (((player.vars['tFrog'] || 0) === 0) || (globals['dayM3'] === 0)) {
    game.log("'You ain't never seen A BUG THAT BIG. Beefy sucker, huge wings, beady eyes, flying real low to the ground. I nabbed it from right here, sitting just like this, and it knew it was right away licked. Had an earthy aftertaste, notes of herb, kept me fed for the whole day.'", '\n');
  } else if (globals['dayM3'] === 1) {
    game.log("'I ain't ever caught A BUG THAT BIG before. Beefy sucker, huge wings, diamond eyes, flapping away, darting through the trees. I surprised it from over there, had to open pretty wide. Bit of a wormy aftertaste, and crunchier than I'm used to, but it kept me fed for a whole week.'", '\n');
  } else if (globals['dayM3'] === 2) {
    game.log("'Nobody's ever seen A BUG THAT BIG again. Beefy sucker, huge wings, a hundred eyes, faster than stink, and flying higher than any crow I've ever seen. I had to leap up in order to grab it, and it fought me all the way down, burnt my tongue something fierce. Bit of a metallic aftertaste, and the screams were so loud, but it kept me fed for a whole month.'", '\n');
  };
  if ((player.vars['tFrog'] || 0) < 1) {
    player.vars['tFrog'] = 1;
  };
  return 0;
}

function talk_rabbit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    return 0;
  };
  if (((player.vars['tRabbit'] || 0) === 0) || (globals['dayM3'] === 0)) {
    game.log("'I don't know if you EVER PLAY THE LOTTERY. Last week, I won half a dozen carrots. It's all thanks to my lucky pebble: I always keep it close, and it helps me roll. Yes, the dice was in my favour that day.'", '\n');
  } else if (globals['dayM3'] === 1) {
    game.log("'Be careful if you EVER PLAY THE LOTTERY. Two years ago, I won half a dozen carrots. It was all thanks to my lucky pebble; still keep it close, but it hasn't been helping me roll lately. The die are no longer in my favour.'", '\n');
  } else if (globals['dayM3'] === 2) {
    game.log("'Got my lucky pebble if i EVER PLAY THE LOTTERY. Next week, I'm going to win half a dozen carrots. I'll keep the pebble right here under my paw, and it'll help me roll. I've already found it, so I know it's lucky. The die will surely be in my favour.'", '\n');
  };
  if ((player.vars['tRabbit'] || 0) < 1) {
    player.vars['tRabbit'] = 1;
  };
  return 0;
}

function talk_snake(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    return 0;
  };
  if (((player.vars['tSnake'] || 0) === 0) || (globals['dayM3'] === 0)) {
    game.log("'Every night I fall asleep LISTENING TO THE RIVER. I dream of flying, curving through the sky, over the trees and through the clouds. My body grows larger and larger until I'm size of the river. When I land, my body dissolves into water.'", '\n');
  } else if (globals['dayM3'] === 1) {
    game.log("'Most nights I fall asleep LISTENING TO THE RIVER. I dream of flying, sliding around the sky, over spiky trees and through soft clouds. My body balloons to the size of the river. When I'm done flying, I crash back to the earth and my body becomes water.'", '\n');
  } else if (globals['dayM3'] === 2) {
    game.log("'Some nights I fall asleep LISTENING TO THE RIVER. I dream of flying, hurtling through the air, the clouds cold around me, trees far below. My body bloats until I'm the size of the river. I crash back to earth, exploding into water.'", '\n');
  };
  if ((player.vars['tSnake'] || 0) < 1) {
    player.vars['tSnake'] = 1;
  };
  return 0;
}

function talk_bat(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    return 0;
  };
  if (40 > Math.floor(Math.random() * 100)) {
    game.log("'I once saw a GREAT BALL OF FIRE in the sky. Took me 15 minutes, but it was there.'", '\n');
  } else if (1 === 1) {
    game.log("It doesn't hear you.", '\n');
  };
  if ((player.vars['tBat'] || 0) < 1) {
    player.vars['tBat'] = 1;
  };
  return 0;
}

function talk_bear(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0)) {
    return 0;
  };
  game.log("The bear will talk when it's ready", '\n');
  return 0;
}

function time(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("It's day ", globals['day'].toString(), ", time is ", globals['time'].toString(), '\n');
  return 0;
}

function weather(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  routines['weatherReport'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  return 0;
}

function what(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  routines['vDescObjectsInRoom'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  return 0;
}

function where(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  routines['vRoomDetail'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  return 0;
}

function cabinDoor_work(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (!(game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['cabinDoor']))) {
    return 0;
  } else if (!(((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0)) {
    game.log("You can only work on this door with a weapon.", '\n');
    return 0;
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) < 3)) {
    game.log("Already as repaired as it's going to be.", '\n');
    return 0;
  };
  if (((game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['roughBoard'], false))) && ((game.isInLocation(locals['cRoom'], player, objects['nails'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nails'], false)))) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("Nothing happens to the door", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) + locals['dmg']);
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) + 1);
    if (game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) {
      game.move(locals, game.getInst(player, 'roughBoard', false), );
    } else if (1 === 1) {
      game.move(locals, game.getInst(locals['cRoom'], 'roughBoard', false), );
    };
    game.log("You repair the door with a rough board for ", locals['dmg'].toString(), " health.", '\n');
  } else if (1 === 1) {
    game.log("You need a rough board and nails in order to repair this door.", '\n');
  };
  return 0;
}

function cabinWindow_work(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (!(game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['cabinWindow']))) {
    return 0;
  } else if (!(((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0)) {
    game.log("You can only work on this window with a weapon.", '\n');
    return 0;
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['isBroken'] || 0) === 1)) {
    game.log("Already as repaired as it's going to be.", '\n');
    return 0;
  };
  if (((game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['roughBoard'], false))) && ((game.isInLocation(locals['cRoom'], player, objects['nails'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nails'], false)))) {
    if (game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) {
      game.move(locals, game.getInst(player, 'roughBoard', false), );
    } else if (1 === 1) {
      game.move(locals, game.getInst(locals['cRoom'], 'roughBoard', false), );
    };
    game.log("You repair the window with a rough board", '\n');
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['isBroken'] = 0;
  } else if (1 === 1) {
    game.log("You need a rough board and nails in order to repair this window.", '\n');
  };
  return 0;
}

function table_work(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if ((((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) && ((game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['roughBoard'], false))) && ((game.isInLocation(locals['cRoom'], player, objects['nails'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nails'], false))) && (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) < 2)) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("Nothing happens to the table", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) + locals['dmg']);
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) + 1);
    if (game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) {
      game.move(locals, game.getInst(player, 'roughBoard', false), );
    } else if (1 === 1) {
      game.move(locals, game.getInst(locals['cRoom'], 'roughBoard', false), );
    };
    game.log("You repair the table with a rough board for ", locals['dmg'].toString(), " health.", '\n');
  };
  return 0;
}

function chair_work(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if ((((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) && ((game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['roughBoard'], false))) && ((game.isInLocation(locals['cRoom'], player, objects['nails'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nails'], false))) && (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) < 1)) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("Nothing happens to the chair", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) + locals['dmg']);
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) + 1);
    if (game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) {
      game.move(locals, game.getInst(player, 'roughBoard', false), );
    } else if (1 === 1) {
      game.move(locals, game.getInst(locals['cRoom'], 'roughBoard', false), );
    };
    game.log("You repair the chair with a rough board for ", locals['dmg'].toString(), " health.", '\n');
  };
  return 0;
}

function bedFrame_work(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if ((((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) && ((game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['roughBoard'], false))) && ((game.isInLocation(locals['cRoom'], player, objects['nails'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nails'], false))) && (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) < 2)) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("Nothing happens to the bed", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) + locals['dmg']);
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) + 1);
    if (game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) {
      game.move(locals, game.getInst(player, 'roughBoard', false), );
    } else if (1 === 1) {
      game.move(locals, game.getInst(locals['cRoom'], 'roughBoard', false), );
    };
    game.log("You repair the bed with a rough board for ", locals['dmg'].toString(), " health.", '\n');
  };
  return 0;
}

function bones_work(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((game.isEqual(objects['bones'], (locals['cmd']?.[1] ?? getEmptyResource()))) && (game.isEqual(objects['obsidianShard'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0) > Math.floor(((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) / 2)) {
      game.log("With obsidian's precision, you're able to carve out a master key.", '\n');
      game.copyMove(objects['masterKey'], player);
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    } else if (1 === 1) {
      game.log("The obsidian is too dull", '\n');
    };
  };
  return 0;
}

function riverStone_work(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(game.isEqual((locals['cmd']?.[2] ?? getEmptyResource()), objects['riverStone']))) {
    return 0;
  };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
    if (game.isEqual(objects['obsidianShard'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
      game.log("The obsidian shatters.", '\n');
      game.move(locals, (locals['cmd']?.[2] ?? getEmptyResource()), );
    };
    if (game.isEqual(objects['pickAxe'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
      return 0;
    };
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['wet'] || 0) < 1) {
      game.log("The stone isn't wet, so it just dulls the edge.", '\n');
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['damage'] = Math.floor(((locals['cmd']?.[1] ?? getEmptyResource()).vars['damage'] || 0) / 2);
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['damage'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['damage'] || 0) + 5);
    if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['damage'] || 0) > ((locals['cmd']?.[1] ?? getEmptyResource()).vars['maxDamage'] || 0)) {
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['damage'] = ((locals['cmd']?.[1] ?? getEmptyResource()).vars['maxDamage'] || 0);
    };
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['wetness'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['wetness'] || 0) - 1);
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['wetness'] || 0) < 0) {
      (locals['cmd']?.[2] ?? getEmptyResource()).vars['wetness'] = 0;
    };
  };
  return 0;
}

function detritus_work(cRoom, cmd) {
  const locals = {cRoom, cmd, f2: 0};
  if ((game.isEqual(objects['detritus'], (locals['cmd']?.[1] ?? getEmptyResource()))) && (game.isEqual(objects['detritus'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), rooms['storage']);
    if (game.isInLocation(locals['cRoom'], player, objects['detritus'], true)) {
      for (let object of game.getObjectsIn(game.getInst(player, 'detritus', true))) {
        locals['obj'] = object;
        game.move(locals, locals['obj'], locals['cRoom']);
      };
      game.move(locals, game.getInst(player, 'detritus', true), );
      locals['f2'] = 1;
    } else if (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['detritus'], true)) {
      for (let object of game.getObjectsIn(game.getInst(locals['cRoom'], 'detritus', true))) {
        locals['obj'] = object;
        game.move(locals, locals['obj'], locals['cRoom']);
      };
      game.move(locals, game.getInst(locals['cRoom'], 'detritus', true), );
      locals['f2'] = 1;
    };
    if (locals['f2'] === 1) {
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
      game.copyMove(objects['strap'], locals['cRoom']);
      game.log("You weave the grasses together to form a STRAP", '\n');
    } else if (1 === 1) {
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), locals['cRoom']);
    };
  } else if ((game.isEqual(objects['detritus'], (locals['cmd']?.[1] ?? getEmptyResource()))) && (game.isEqual(objects['bones'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    game.copyMove(objects['strap'], locals['cRoom']);
    game.log("You stretch the grasses over an ad-hoc bone loom, then pick them together into a STRAP", '\n');
  };
  return 0;
}

function pickAxe_work(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((game.isEqual(objects['pickAxe'], (locals['cmd']?.[1] ?? getEmptyResource()))) && (game.isEqual(objects['strap'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
    if (game.isInLocation(locals['cRoom'], (locals['cmd']?.[1] ?? getEmptyResource()), objects['strap'], false)) {
      game.log("This pick-axe already has a strap tied to it", '\n');
    } else if (1 === 1) {
      game.move(locals, (locals['cmd']?.[2] ?? getEmptyResource()), (locals['cmd']?.[1] ?? getEmptyResource()));
      game.log("You tie a strap to the pick-axe. It reminds you of mountain-climbing expeditions, of days spent rappelling down cliffs.", '\n');
    };
  };
  return 0;
}

function boatFrame_work(cRoom, cmd) {
  const locals = {cRoom, cmd, bc: 0,lc: 0,nc: 0,sc: 0};
  if (game.isEqual(objects['boatFrame'], (locals['cmd']?.[2] ?? getEmptyResource()))) {
    game.log("Try it the other way around", '\n');
    return 0;
  };
  if ((game.isEqual(objects['strap'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['roughBoard'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['nails'], (locals['cmd']?.[2] ?? getEmptyResource()))) || (game.isEqual(objects['boiledSap'], (locals['cmd']?.[2] ?? getEmptyResource())))) {
    game.move(locals, (locals['cmd']?.[2] ?? getEmptyResource()), (locals['cmd']?.[1] ?? getEmptyResource()));
    for (let object of game.getObjectsIn((locals['cmd']?.[1] ?? getEmptyResource()))) {
      locals['obj'] = object;
      if (game.isEqual(objects['strap'], locals['obj'])) {
        locals['sc'] = (locals['sc'] + 1);
      } else if (game.isEqual(objects['roughBoard'], locals['obj'])) {
        locals['bc'] = (locals['bc'] + 1);
      } else if (game.isEqual(objects['nails'], locals['obj'])) {
        locals['nc'] = (locals['nc'] + 1);
      } else if (game.isEqual(objects['boiledSap'], locals['obj'])) {
        locals['lc'] = (locals['lc'] + 1);
      };
    };
    if ((1 < locals['sc']) && (9 < locals['bc']) && (0 < locals['nc']) && (0 < locals['sc'])) {
      globals['winBuildBoat'] = 1;
      game.log("You've build a boat.", '\n');
      routines['recap'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
      game.log("It's day ", globals['day'].toString(), '\n');
      routines['weatherReport'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
      game.log("You push the boat off, heading west across lake, to adventures unknown.", '\n');
      game.log("Good game! Thank you for playing", '\n');game.close();
    } else if (1 === 1) {
      game.log("The boat needs:", '\n');
      game.log("2 straps, has ", locals['sc'].toString(), '\n');
      game.log("10 boards, has ", locals['bc'].toString(), '\n');
      game.log("1 nails, has ", locals['nc'].toString(), '\n');
      game.log("1 boiled-sap, has ", locals['lc'].toString(), '\n');
    };
  } else if (game.isEqual((locals['cmd']?.[2] ?? getEmptyResource()), (locals['cmd']?.[3] ?? getEmptyResource()))) {
    game.log("I'm not sure what you mean. Do you have that?", '\n');
  } else if (1 === 1) {
    game.log("Building a boat requires STRAPs, BOARDs, NAILS, and BOILED-SAP", '\n');
  };
  return 0;
}

function write(cRoom, cmd) {
  const locals = {cRoom, cmd, hasC: 0,hasP: 0};
  if (((game.isInLocation(locals['cRoom'], locals['cRoom'], objects['charcoal'], false)) || (game.isInLocation(locals['cRoom'], player, objects['charcoal'], false))) && ((game.isInLocation(locals['cRoom'], locals['cRoom'], objects['bookPage'], false)) || (game.isInLocation(locals['cRoom'], player, objects['bookPage'], false)))) {
    game.copyMove(objects['note'], locals['cRoom']);
  } else if (1 === 1) {
    game.log("You need both paper and charcoal.", '\n');
  };
  return 0;
}

function yes(cRoom, cmd) {
  const locals = {cRoom, cmd, tf: 0};
  if ((game.getInst(rooms['qStorage'], 'ynQ', false).vars['fromCrow'] || 0) === 1) {
    if ((game.isInLocation(locals['cRoom'], locals['cRoom'], objects['crow'], false)) && ((game.getInst(locals['cRoom'], 'crow', false).vars['health'] || 0) > 0)) {
      game.log("'Not me'", '\n');
      game.log("The crow flies off", '\n');
      game.move(locals, game.getInst(locals['cRoom'], 'crow', false), );
    };
    game.move(locals, game.getInst(rooms['qStorage'], 'ynQ', false), );
    return 1;
  };
  if ((game.getInst(rooms['qStorage'], 'ynQ', false).vars['kOwl'] || 0) === 1) {
    if ((player.vars['tOwl'] || 0) === 0) {
      locals['tf'] = -1;
    } else if (1 === 1) {
      locals['tf'] = 1;
    };
  } else if ((game.getInst(rooms['qStorage'], 'ynQ', false).vars['kFish'] || 0) === 1) {
    if ((player.vars['tFish'] || 0) === 0) {
      locals['tf'] = -1;
    } else if (1 === 1) {
      locals['tf'] = 1;
    };
  } else if ((game.getInst(rooms['qStorage'], 'ynQ', false).vars['kFrog'] || 0) === 1) {
    if ((player.vars['tFrog'] || 0) === 0) {
      locals['tf'] = 0;
    } else if (1 === 1) {
      locals['tf'] = 1;
    };
  } else if ((game.getInst(rooms['qStorage'], 'ynQ', false).vars['kRabbit'] || 0) === 1) {
    if ((player.vars['tRabbit'] || 0) === 0) {
      locals['tf'] = -1;
    } else if (1 === 1) {
      locals['tf'] = 1;
    };
  } else if ((game.getInst(rooms['qStorage'], 'ynQ', false).vars['kSnake'] || 0) === 1) {
    if ((player.vars['tSnake'] || 0) === 0) {
      locals['tf'] = -1;
    } else if (1 === 1) {
      locals['tf'] = 1;
    };
  };
  if ((locals['tf'] === 1) && (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['bear'], false))) {
    game.log("'Good, they are my friend.'", '\n');
    if (!(game.isInLocation(locals['cRoom'], rooms['forest3'], objects['treeHollow'], false))) {
      game.log("'The crow is always stealing things. It hides them in a tree hollow, east of the lake.'", '\n');
      game.move(locals, game.getInst(rooms['storage'], 'treeHollow', false), rooms['forest3']);
    };
    game.log("Conversation over, the bear sits down to forage", '\n');
    game.getInst(locals['cRoom'], 'bear', false).vars['asked'] = 1;
    game.move(locals, game.getInst(rooms['qStorage'], 'ynQ', false), );
  } else if ((locals['tf'] === -1) && (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['bear'], false))) {
    game.log("'Liar'", '\n');
    game.move(locals, game.getInst(rooms['qStorage'], 'ynQ', false), );
    game.getInst(locals['cRoom'], 'bear', false).vars['enraged'] = 1;
  };
  return 0;
}

