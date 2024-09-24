import { game, getEmptyResource } from './engine.js';
import { player, globals } from './globals.js';
import { objects } from './objects.js';
import { rooms } from './rooms.js';

export const routines = {
  descCabinExterior: descCabinExterior,
  descBook: descBook,
  descCabinWindow: descCabinWindow,
  descBtnRm1: descBtnRm1,
  btnRm1North: btnRm1North,
  btnRm1East: btnRm1East,
  btnRm1South: btnRm1South,
  btnRm1West: btnRm1West,
  descBtnRm2: descBtnRm2,
  btnRm2East: btnRm2East,
  btnRm2South: btnRm2South,
  btnRm2West: btnRm2West,
  descMagicRing: descMagicRing,
  exitHoleRm: exitHoleRm,
  descObsidian: descObsidian,
  cavern1North: cavern1North,
  rollDmg: rollDmg,
  descForest1: descForest1,
  descRiverStone: descRiverStone,
  caveEntrance1North: caveEntrance1North,
  caveEntrance1East: caveEntrance1East,
  caveEntrance1South: caveEntrance1South,
  caveEntrance1West: caveEntrance1West,
  caveEntrance2East: caveEntrance2East,
  caveEntrance2South: caveEntrance2South,
  caveEntrance2West: caveEntrance2West,
  prsiRiverStone: prsiRiverStone,
  playerAlways: playerAlways,
  checkPulse: checkPulse,
  inventoryLimit: inventoryLimit,
  burnFire: burnFire,
  timePasses: timePasses,
  weatherReport: weatherReport,
  checkHealth: checkHealth,
  checkFood: checkFood,
  checkSleep: checkSleep,
  playerActEnter: playerActEnter,
  prsiWater: prsiWater,
  vDescRoom: vDescRoom,
  vRoomDetail: vRoomDetail,
  vDescObjectsInRoom: vDescObjectsInRoom,
};

export const handlers = {
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
  examine: {
    func: examine,
    objHandlers: {
    },
  },
  hit: {
    objHandlers: {
      book: {
        before: book_hit,
      },
      cabinDoor: {
        before: cabinDoor_hit,
      },
      cabinWindow: {
        before: cabinWindow_hit,
      },
      chair: {
        before: chair_hit,
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
  look: {
    func: look,
    objHandlers: {
    },
  },
  open: {
    objHandlers: {
      cabinDoor: {
        before: cabinDoor_open,
      },
    },
  },
  pee: {
    func: pee,
    objHandlers: {
    },
  },
  sleep: {
    func: sleep,
    objHandlers: {
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
      cabinDoor: {
        before: cabinDoor_work,
      },
      chair: {
        before: chair_work,
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
};

function descCabinExterior(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((rooms['cabinExterior'].vars['firstTime'] || 0) === 1) {
    rooms['cabinExterior'].vars['firstTime'] = 0;
    game.log("You're at the transition between a forest and a field. There are trails in all directions, with a cabin at the center. It's got a scenic window facing the fields. You could try to OPEN DOOR", '\n');
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
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) < 1) {
    game.log("a torn-up book");
    return 1;
  } else if (globals['detailedDesc'] === 1) {
    game.log("This leather-bound journal's yellowing pages are covered in scratchy handwriting, probably done with charcoal. You can make out a few passages:", '\n');
    game.log("...boat is coming along, should be ready in...", '\n');
    game.log("...beast is hungry, what does it eat down there? No use mining any more...", '\n');
    game.log("...lost the gem, a crow must have taken it...", '\n');
    return 1;
  };
  game.log("a BOOK");
  return 0;
}

function descCabinWindow(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['isBroken'] || 0) === 1) {
    game.log("a broken WINDOW", '\n');
  } else if (1 === 1) {
    game.log("a WINDOW", '\n');
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
  } else if ((rooms['btnRm2'].vars['step'] || 0) === 3) {
    rooms['btnRm2'].vars['step'] = 2;
    game.log("You appear to be in the same room, but something has changed.", '\n');
  } else if (1 === 1) {
    rooms['btnRm2'].vars['step'] = (rooms['btnRm2'].vars['start'] || 0);
    game.log("You appear to be in the same room that you started from.", '\n');
  };
  return 0;
}

function btnRm2South(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (((rooms['btnRm2'].vars['step'] || 0) === 0) || ((rooms['btnRm2'].vars['step'] || 0) === 3)) {
    game.move(locals, player, rooms['mazeRest2']);
  } else if (1 === 1) {
    rooms['btnRm2'].vars['step'] = (rooms['btnRm2'].vars['start'] || 0);
    game.log("You appear to be in the same room that you started from.", '\n');
  };
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
      game.copyMove(locals, objects['magicRing'], locals['cRoom']);
      game.log("And something shiny appeared on the ground.", '\n');
    } else if (Math.floor(Math.random() * 100) < 67) {
      game.copyMove(locals, objects['obsidianShard'], locals['cRoom']);
      game.log("And something shiny appeared on the ground.", '\n');
    };
  } else if (1 === 1) {
    rooms['btnRm2'].vars['step'] = (rooms['btnRm2'].vars['start'] || 0);
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

function exitHoleRm(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((!(game.isInLocation(locals['cRoom'], player, objects['torch'], false))) && (!(game.isInLocation(locals['cRoom'], player, objects['gem'], false)))) {
    game.log("Without light to see where you're going, you fall into the hole and die.", '\n');
    player.vars['health'] = -1;
  };
  return 0;
}

function descObsidian(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("This obsidian shard is extremely sharp but very brittle; you might only get one good hit with it. And be careful: you can hurt yourself by using a dull implement.");
  } else if (1 === 1) {
    game.log("an OBSIDIAN shard");
  };
  return 0;
}

function cavern1North(cRoom, cmd) {
  const locals = {cRoom, cmd, };
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
    if ((player.vars['health'] || 0) < 1) {
      game.log("You've fatally injured yourself while using the implement.", '\n');
    } else if (1 === 1) {
      game.log("Your implement was dull, causing you to injure yourself while using it.", '\n');
      game.log("You have ", (player.vars['health'] || 0).toString(), " health left.", '\n');
    };
    locals['dmg'] = 0;
  };
  return locals['dmg'];
  return 0;
}

function descForest1(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((rooms['forest1'].vars['firstTime'] || 0) === 1) {
    rooms['forest1'].vars['firstTime'] = 0;
    game.log("It's the end of summer, and you're in a dense forest. Birds and other small creatures can be heard rustling, buzzing, and chirping through the undergrowth. The smell of pine hangs thick in the air. Light filters down through needles and leaves.", '\n');
    routines['weatherReport'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    game.log("There's a trail up ahead, it looks like you could GO NORTH", '\n');
  } else if (globals['detailedDesc'] === 1) {
    game.log("You're in dense forest. The undergrowth is thick with dead branches, leaves, and other detritus. Vines, bushes, and brush all vie for a spot in the sunlight. Even during a thunderstorm, the wind barely penetrates this far down, leaving the air hot and thick with the smell of pine.", '\n');
    game.log("Despite the foliage, there's a well-defined trail heading NORTH and WEST", '\n');
  } else if (1 === 1) {
    game.log("You're in a dense forest. There's a trail heading NORTH and WEST.", '\n');
  };
  return 0;
}

function descRiverStone(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['detailedDesc'] === 1) {
    game.log("A flat and smooth river stone. Perfect for sharpening weapons or tools when wet.", '\n');
  } else if (1 === 1) {
    game.log("a smooth STONE");
  };
  return 0;
}

function caveEntrance1North(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['forestBurnedDown'] === 1) {
    game.log("Charred debris blocks your way.", '\n');
  } else if (1 === 1) {
    game.move(locals, player, rooms['forest5']);
  };
  return 0;
}

function caveEntrance1East(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['forestBurnedDown'] === 1) {
    game.log("Charred debris blocks your way.", '\n');
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
    game.log("Charred debris blocks your way.", '\n');
  } else if (1 === 1) {
    game.move(locals, player, rooms['forest4']);
  };
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

function prsiRiverStone(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((locals['cmd'] === "WORK") && ((((locals['cmd']?.[1] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) || (((locals['cmd']?.[1] ?? getEmptyResource()).vars['maxHealth'] || 0) > 0))) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['wet'] || 0) < 1) {
      game.log("The stone isn't wet, so it just dulls the edge.", '\n');
      return 1;
    };
  } else if (((locals['cmd'] === "ADD") || (locals['cmd'] === "POUR")) && (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['water']))) {
    (locals['cmd']?.[2] ?? getEmptyResource()).vars['wet'] = 10;
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    game.log("The river stone is wet. You could WORK KNIFE WITH STONE to sharpen it.", '\n');
  };
  return 0;
}

function playerAlways(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  routines['burnFire'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  routines['inventoryLimit'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  routines['timePasses'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  routines['checkPulse'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  return 0;
}

function checkPulse(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((player.vars['health'] || 0) < 1) {
    game.log("You are dead", '\n');
  };
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

function burnFire(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  for (let object of game.getObjectsIn(objects['fire'])) {
    locals['obj'] = object;
    if ((game.isInLocation(locals['cRoom'], rooms['cabin'], locals['obj'], false)) && ((game.getInst(rooms['cabin'], 'cabinDoor', false).vars['health'] || 0) < 1) && ((game.getInst(rooms['cabin'], 'cabinWindow', false).vars['health'] || 0) < 1)) {
      locals['obj'].vars['fuel'] = ((locals['obj'].vars['fuel'] || 0) - 1);
    } else if ((game.getParent(locals['obj'], true).vars['aboveGround'] || 0) === 1) {
      locals['obj'].vars['fuel'] = ((locals['obj'].vars['fuel'] || 0) - 2);
      if ((game.isInLocation(locals['cRoom'], rooms['cabin'], locals['obj'], false)) && ((game.isInLocation(locals['cRoom'], rooms['cabin'], player, false)) || (game.isInLocation(locals['cRoom'], rooms['cabinExterior'], player, false)))) {
        game.log("The cabin is drafty from a broken door or window, and fire burns more quickly.", '\n');
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
      game.log("It hasn't rained in awhile. Fire catches, and the forest burns down.", '\n');
      globals['forestBurnedDown'] = 1;
      if ((locals['cRoom'].vars['aboveGround'] || 0) === 1) {
        game.log("You die.", '\n');
        player.vars['health'] = -1;
        return 1;
      };
    };
  };
  for (let object of game.getObjectsIn(objects['torch'])) {
    locals['obj'] = object;
    if ((locals['obj'].vars['lit'] || 0) === 1) {
      locals['obj'].vars['fuel'] = ((locals['obj'].vars['fuel'] || 0) - 1);
      if ((locals['obj'].vars['fuel'] || 0) < 1) {
        game.move(locals, locals['obj'], );
        if ((game.isInLocation(locals['cRoom'], player, locals['obj'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], locals['obj'], false))) {
          game.log("The fire dies.", '\n');
        };
      } else if (((game.getParent(locals['obj'], false).vars['aboveGround'] || 0) === 1) && (!(game.isInLocation(locals['cRoom'], rooms['cabin'], locals['obj'], false))) && (globals['dry'] > 3)) {
        game.log("It hasn't rained in awhile. Fire catches, and the forest burns down.", '\n');
        globals['forestBurnedDown'] = 1;
        if ((locals['cRoom'].vars['aboveGround'] || 0) === 1) {
          game.log("You die.", '\n');
          player.vars['health'] = -1;
          return 1;
        };
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
      game.log("Night falls.", '\n');
      routines['checkSleep'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
      routines['checkHealth'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    };
  } else if (globals['time'] < 0) {
    globals['time'] = (globals['time'] + -1);
    if (globals['time'] === -18) {
      globals['time'] = 1;
      game.log("Morning comes.", '\n');
      routines['checkFood'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
      routines['checkHealth'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    } else if (globals['time'] === -8) {
      globals['napsToday'] = 0;
      globals['day'] = (globals['day'] + 1);
      globals['moonPhase'] = (globals['moonPhase'] + 1);
      if (globals['moonPhase'] === 8) {
        globals['moonPhase'] = 0;
      } else if (globals['moonPhase'] === 4) {
        game.log("It's a full moon. Be careful, all your weapon and tool actions are more erratic.", '\n');
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
  if (globals['time'] > 48) {
    locals['isNight'] = 0;
    game.log("It's evening, ");
  } else if (globals['time'] > 32) {
    locals['isNight'] = 0;
    game.log("It's the afternoon, ");
  } else if (globals['time'] > 16) {
    locals['isNight'] = 0;
    game.log("It's morning, ");
  } else if (globals['time'] > 0) {
    locals['isNight'] = 0;
    game.log("It's early morning, ");
  } else if (globals['time'] < -16) {
    locals['isNight'] = 1;
    game.log("It's night, ");
  } else if (globals['time'] < 0) {
    locals['isNight'] = 1;
    game.log("It's late at night, ");
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

function checkHealth(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((player.vars['health'] || 0) < 1) {
    game.log("You are dead.", '\n');
  } else if (1 === 1) {
    game.log("You have ", (player.vars['health'] || 0).toString(), " health.", '\n');
  };
  return 0;
}

function checkFood(cRoom, cmd) {
  const locals = {cRoom, cmd, e: 0};
  if ((player.vars['health'] || 0) < 1) {
    return 1;
  };
  locals['e'] = (globals['totalFood'] - (globals['day'] * 2));
  if (locals['e'] < -5) {
    player.vars['health'] = ((player.vars['health'] || 0) - 20);
    game.log("You lost 20 health from hunger.", '\n');
  } else if (locals['e'] < -3) {
    player.vars['health'] = ((player.vars['health'] || 0) - 10);
    game.log("You lost 10 health from hunger.", '\n');
  } else if (locals['e'] < 0) {
    player.vars['health'] = ((player.vars['health'] || 0) - 2);
    game.log("You lost 2 health from hunger.", '\n');
  } else if (locals['e'] > 6) {
    player.vars['health'] = ((player.vars['health'] || 0) - (locals['e'] + globals['day']));
    game.log("You lost some health from over-eating.", '\n');
  };
  return 0;
}

function checkSleep(cRoom, cmd) {
  const locals = {cRoom, cmd, z: 0};
  if (globals['napsToday'] > 5) {
    game.log("You lost 10 health from napping too much today, but also gained 1 sleep", '\n');
    globals['totalSleeps'] = (globals['totalSleeps'] + 1);
  } else if (globals['napsToday'] > 2) {
    game.log("You gained 1 sleep from the naps you took today", '\n');
    globals['totalSleeps'] = (globals['totalSleeps'] + 1);
  };
  locals['z'] = (globals['totalSleeps'] - globals['day']);
  if (locals['z'] < -2) {
    player.vars['health'] = ((player.vars['health'] || 0) - 40);
    game.log("You lost 40 health from lack of sleep.", '\n');
  } else if (locals['z'] < -1) {
    player.vars['health'] = ((player.vars['health'] || 0) - 20);
    game.log("You lost 20 health from lack of sleep.", '\n');
  } else if (locals['z'] < 0) {
    game.log("You will lose health if you don't get enough sleep.", '\n');
  } else if (locals['z'] > 3) {
    player.vars['health'] = ((player.vars['health'] || 0) - 20);
    game.log("You lost 20 health from over-sleeping.", '\n');
  };
  return 0;
}

function playerActEnter(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  routines['vDescRoom'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
  if ((locals['cRoom'].vars['aboveGround'] || 0) === 0) {
  };
  return 0;
}

function prsiWater(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((locals['cmd'] === "ADD") || (locals['cmd'] === "PUT")) {
    if (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['water'])) {
      game.log("The water gets more watery.", '\n');
      game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
      return 1;
    } else if (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['torch'])) {
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['lit'] = 0;
      game.log("The torch hisses and dies as it lands in the water.", '\n');
    } else if (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['riverStone'])) {
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['wet'] = 10;
      game.log("The river stone is wet. You could WORK KNIFE WITH STONE to sharpen it.", '\n');
    } else if (1 === 1) {
      game.log("You put it in the water.", '\n');
    };
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), (locals['cmd']?.[2] ?? getEmptyResource()));
  } else if (locals['cmd'] === "FILL") {
    if ((game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['kettle'])) || (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['bucket'])) || (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['cup'])) || (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['treeHollow'])) || (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['boat']))) {
      game.copyMove(locals, objects['water'], (locals['cmd']?.[1] ?? getEmptyResource()));
      game.log("You fill it with water.", '\n');
    };
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
    game.log("WHAT IS HERE lists interactable objects in the immediate vicinity. Interactions may not be obvious.", '\n');
    game.log("Objects nested inside other objects are not listed, but might show up if you EXAMINE their container.", '\n');
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

function drop(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], player, (locals['cmd']?.[1] ?? getEmptyResource()), false)) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), locals['cRoom']);
  };
  return 0;
}

function eat(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['edible'] || 0) === 1)) {
    game.log("You can't eat that.", '\n');
  };
  return 0;
}

function book_empty(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isInLocation(locals['cRoom'], (locals['cmd']?.[1] ?? getEmptyResource()), objects['bookPage'], false)) {
    game.move(locals, game.getInst((locals['cmd']?.[1] ?? getEmptyResource()), 'bookPage', false), player);
    game.log("You've ripped out some blank pages from the book", '\n');
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
    game.log("nothing inside", '\n');
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

function cabinDoor_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if (!(game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['cabinDoor']))) {
    return 0;
  };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0) {
    if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) {
      globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['damage'] || 0);
      globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0);
    } else if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0) > 0) {
      globals['combatDamage'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0) * 2);
      globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0);
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
      for (let i of Array.from(Array(((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0)).keys())) {
        locals['v'] = i;
        game.copyMove(locals, objects['roughBoard'], locals['cRoom']);
      };
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = 0;
      game.log("It's broken into some ROUGH-BOARDs", '\n');
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
    } else if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0) > 0) {
      globals['combatDamage'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0) * 2);
      globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0);
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
      for (let i of Array.from(Array(((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0)).keys())) {
        locals['v'] = i;
        game.copyMove(locals, objects['roughBoard'], locals['cRoom']);
      };
      (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = 0;
      game.log("It's broken into some ROUGH-BOARDs", '\n');
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
    } else if (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0) > 0) {
      globals['combatDamage'] = (((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0) * 2);
      globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0);
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
      game.copyMove(locals, objects['roughBoard'], locals['cRoom']);
      game.log("It's broken into a ROUGH-BOARD", '\n');
    };
  };
  return 0;
}

function cabinWindow_hit(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['isBroken'] || 0) === 1)) {
    game.log("You smash the window, taking 1 damage in the process.", '\n');
    player.vars['health'] = ((player.vars['health'] || 0) - 1);
    game.copyMove(locals, objects['glassShard'], rooms['cabin']);
    game.move(locals, player, rooms['cabin']);
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
      game.copyMove(locals, objects['log'], locals['cRoom']);
      game.copyMove(locals, objects['log'], locals['cRoom']);
      game.copyMove(locals, objects['log'], locals['cRoom']);
      game.copyMove(locals, objects['log'], locals['cRoom']);
      game.copyMove(locals, objects['stick'], locals['cRoom']);
      game.copyMove(locals, objects['stick'], locals['cRoom']);
      game.log("You've chopped the tree into four LOGs and two STICKs", '\n');
    };
  };
  if ((((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) > 0) && ((((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxDamage'] || 0) > 0) || (((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0) > 0))) {
    game.copyMove(locals, objects['sap'], player);
    game.log("You pick up some tree SAP", '\n');
  };
  return 0;
}

function inventory(cRoom, cmd) {
  const locals = {cRoom, cmd, count: 0};
  for (let object of game.getObjectsIn(player)) {
    locals['obj'] = object;
    locals['count'] = (locals['count'] + 1);
    game.describe(locals['obj']);
    game.log('\n');
  };
  return 0;
}

function jump(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("No thank you", '\n');
  return 0;
}

function look(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (globals['firstLookAround'] === 1) {
    game.log("LOOK AROUND runs two commands for us:", '\n');
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

function cabinDoor_open(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if ((((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) < 1) || (((locals['cmd']?.[1] ?? getEmptyResource()).vars['isLocked'] || 0) === 0)) {
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

function pee(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['fire'])) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), );
    game.log("The fire goes out.", '\n');
  } else if (game.isEqual((locals['cmd']?.[1] ?? getEmptyResource()), objects['torch'])) {
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['lit'] = 0;
    game.log("The torch goes out.", '\n');
  } else if (1 === 1) {
    game.log("Cool.", '\n');
  };
  return 0;
}

function sleep(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("// TODO", '\n');
  return 0;
}

function swim(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  game.log("The water looks a little chilly, it would be better if we had a boat, or maybe built one?", '\n');
  return 0;
}

function take(cRoom, cmd) {
  const locals = {cRoom, cmd, };
  if (((locals['cmd']?.[1] ?? getEmptyResource()).vars['noTake'] || 0) === 1) {
    game.log("This can't be picked up", '\n');
  } else if (1 === 1) {
    game.move(locals, (locals['cmd']?.[1] ?? getEmptyResource()), player);
    game.log("Picked up", '\n');
  };
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
  } else if (!(((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0) > 0)) {
    game.log("You can only work on this door with a tool.", '\n');
    return 0;
  } else if (!(((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) < 3)) {
    game.log("Already as repaired as it's going to be.", '\n');
    return 0;
  };
  if (((game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['roughBoard'], false))) && ((game.isInLocation(locals['cRoom'], player, objects['nails'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nails'], false)))) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("Nothing happens to the door", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) + locals['dmg']);
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) + 1);
    game.move(locals, game.getInst(player, 'roughBoard', false), );
    game.log("You repair the door with a rough-board for ", locals['dmg'].toString(), " health.", '\n');
  } else if (1 === 1) {
    game.log("You need a rough-board and nails in order to repair this door.", '\n');
  };
  return 0;
}

function table_work(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if ((((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0) > 0) && ((game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['roughBoard'], false))) && ((game.isInLocation(locals['cRoom'], player, objects['nails'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nails'], false))) && (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) < 2)) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("Nothing happens to the table", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) + locals['dmg']);
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) + 1);
    game.move(locals, game.getInst(player, 'roughBoard', false), );
    game.log("You repair the table with a rough-board for ", locals['dmg'].toString(), " health.", '\n');
  };
  return 0;
}

function chair_work(cRoom, cmd) {
  const locals = {cRoom, cmd, dmg: 0};
  if ((((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0) > 0) && ((game.isInLocation(locals['cRoom'], player, objects['roughBoard'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['roughBoard'], false))) && ((game.isInLocation(locals['cRoom'], player, objects['nails'], false)) || (game.isInLocation(locals['cRoom'], locals['cRoom'], objects['nails'], false))) && (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) < 1)) {
    globals['combatDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['health'] || 0);
    globals['combatMaxDamage'] = ((locals['cmd']?.[2] ?? getEmptyResource()).vars['maxHealth'] || 0);
    locals['dmg'] = 
    routines['rollDmg'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi']);
    if (locals['dmg'] === 0) {
      game.log("Nothing happens to the chair", '\n');
      return 1;
    };
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['health'] || 0) + locals['dmg']);
    (locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] = (((locals['cmd']?.[1] ?? getEmptyResource()).vars['hasBoards'] || 0) + 1);
    game.move(locals, game.getInst(player, 'roughBoard', false), );
    game.log("You repair the chair with a rough-board for ", locals['dmg'].toString(), " health.", '\n');
  };
  return 0;
}

function write(cRoom, cmd) {
  const locals = {cRoom, cmd, hasC: 0,hasP: 0};
  if (((game.isInLocation(locals['cRoom'], locals['cRoom'], objects['charcoal'], false)) || (game.isInLocation(locals['cRoom'], player, objects['charcoal'], false))) && ((game.isInLocation(locals['cRoom'], locals['cRoom'], objects['bookPage'], false)) || (game.isInLocation(locals['cRoom'], player, objects['bookPage'], false)))) {
    game.copyMove(locals, objects['note'], locals['cRoom']);
  } else if (1 === 1) {
    game.log("You need paper and charcoal.", '\n');
  };
  return 0;
}

