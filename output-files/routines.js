import { game } from './game.js';
import { player, globals } from './globals.js';
import { objects } from './objects.js';
import { rooms } from './rooms.js';

export const routines = {
  descCabinExterior: {
    isRoutine: 'descCabinExterior',
    func: descCabinExterior,
  },
  prsoCabinDoor: {
    isRoutine: 'prsoCabinDoor',
    func: prsoCabinDoor,
  },
  descBook: {
    isRoutine: 'descBook',
    func: descBook,
  },
  prsoBook: {
    isRoutine: 'prsoBook',
    func: prsoBook,
  },
  prsoTable: {
    isRoutine: 'prsoTable',
    func: prsoTable,
  },
  descForest1: {
    isRoutine: 'descForest1',
    func: descForest1,
  },
  prsoDetritus: {
    isRoutine: 'prsoDetritus',
    func: prsoDetritus,
  },
  timePasses: {
    isRoutine: 'timePasses',
    func: timePasses,
  },
  weatherReport: {
    isRoutine: 'weatherReport',
    func: weatherReport,
  },
  vDescRoom: {
    isRoutine: 'vDescRoom',
    func: vDescRoom,
  },
  vRoomDetail: {
    isRoutine: 'vRoomDetail',
    func: vRoomDetail,
  },
  vDescObjectsInRoom: {
    isRoutine: 'vDescObjectsInRoom',
    func: vDescObjectsInRoom,
  },
  vInventory: {
    isRoutine: 'vInventory',
    func: vInventory,
  },
  vSparkAt: {
    isRoutine: 'vSparkAt',
    func: vSparkAt,
  },
  vHitWith: {
    isRoutine: 'vHitWith',
    func: vHitWith,
  },
  vOpen: {
    isRoutine: 'vOpen',
    func: vOpen,
  },
  vClose: {
    isRoutine: 'vClose',
    func: vClose,
  },
  vExamine: {
    isRoutine: 'vExamine',
    func: vExamine,
  },
  vEat: {
    isRoutine: 'vEat',
    func: vEat,
  },
  vPutIn: {
    isRoutine: 'vPutIn',
    func: vPutIn,
  },
  vTalkTo: {
    isRoutine: 'vTalkTo',
    func: vTalkTo,
  },
  vTake: {
    isRoutine: 'vTake',
    func: vTake,
  },
  vTakeOut: {
    isRoutine: 'vTakeOut',
    func: vTakeOut,
  },
  vPeeOn: {
    isRoutine: 'vPeeOn',
    func: vPeeOn,
  },
  vCook: {
    isRoutine: 'vCook',
    func: vCook,
  },
  vWriteNote: {
    isRoutine: 'vWriteNote',
    func: vWriteNote,
  },
  vSleep: {
    isRoutine: 'vSleep',
    func: vSleep,
  },
  vDrop: {
    isRoutine: 'vDrop',
    func: vDrop,
  },
  vWorkWith: {
    isRoutine: 'vWorkWith',
    func: vWorkWith,
  },
  vSwim: {
    isRoutine: 'vSwim',
    func: vSwim,
  },
  vJump: {
    isRoutine: 'vJump',
    func: vJump,
  },
  vWhereToGo: {
    isRoutine: 'vWhereToGo',
    func: vWhereToGo,
  },
};

function descCabinExterior(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  if (((locals['prso'].vars['firstTime'] || 0) === 1)) {
    locals['prso'].vars['firstTime'] = 0;
    game.log("You're at the transition between a forest and a field. There are trails in all directions, but at the center a cabin! It's got a scenic window facing the fields. You could try to OPEN DOOR", '\n');
    return 1;
  }
  game.log("You're outside a cabin on the edge of a forest, overlooking some fields.", '\n');
  if (((rooms['cabin'].vars['isLocked'] || 0) === 1)) {
    game.log("The cabin door is locked. Where's the key?", '\n');
  }
  return 0;
}

function prsoCabinDoor(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  if ((locals['cmd'] === "OPEN")) {
    if ((game.isInLocation(locals['cRoom'], player, objects['cabinDoorKey'], false) && ((rooms['cabin'].vars['isLocked'] || 0) === 1))) {
      locals['prso'].vars['isLocked'] = 0;
      game.log("The key works!", '\n');
      game.move(locals, player, rooms['cabin'])
    } else if (((rooms['cabin'].vars['isLocked'] || 0) === 0)) {
      game.move(locals, player, rooms['cabin'])
    } else if ((1 === 1)) {
      game.log("The door is locked. You should LOOK AROUND for a key.", '\n');
    }
  }
  return 0;
}

function descBook(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  if (((locals['prso'].vars['health'] || 0) === 0)) {
    game.log("a torn-up book");
    return 1;
  } else if ((globals['detailedDesc'] === 1)) {
    game.log("This leather-bound journal's yellowing pages are covered in scratchy handwriting, probably done with charcoal. You can make out a few passages:", '\n');
    game.log("...boat is coming along, should be ready in...", '\n');
    game.log("...beast is hungry, what does it eat down there? No use mining any more...", '\n');
    game.log("...lost the gem, a crow must have taken it...", '\n');
    return 1;
  }
  game.log("a BOOK");
  return 0;
}

function prsoBook(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  if ((((locals['prso'].vars['health'] || 0) > 0) && (locals['cmd'] === "EMPTY") && game.isInLocation(locals['cRoom'], locals['prso'], objects['bookPages'], false))) {
    game.move(locals, game.getInst(locals['prso'], 'bookPages'), player)
    game.log("You've ripped out some blank pages from the book", '\n');
  } else if ((locals['cmd'] === "HIT")) {
    if (((locals['prso'].vars['health'] || 0) > 0)) {
      game.log("Why are you hitting this book?", '\n');
      if (game.isInLocation(locals['cRoom'], locals['prso'], objects['bookPages'], false)) {
        game.move(locals, game.getInst(locals['prso'], 'bookPages'), locals['cRoom'])
        game.log("You've cut out some pages onto the floor.", '\n');
      }
      locals['prso'].vars['health'] = ((locals['prso'].vars['health'] || 0) + -1);
    }
  }
  return 0;
}

function prsoTable(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function descForest1(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  if (((rooms['forest1'].vars['firstTime'] || 0) === 1)) {
    rooms['forest1'].vars['firstTime'] = 0;
    game.log("It's the end of summer, and you're in a dense forest. Birds and other small creatures can be heard rustling, buzzing, and chirping through the undergrowth. The smell of pine hangs thick in the air. Light filters down through needles and leaves.", '\n');routines['weatherReport'].func(locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi'])
    game.log("There's a trail up ahead, it looks like you could GO NORTH", '\n');
  } else if ((globals['detailedDesc'] === 1)) {
    game.log("You're in dense forest. The undergrowth is thick with dead branches, leaves, and other detritus. Vines, bushes, and brush all vie for a spot in the sunlight. Even during a thunderstorm, the wind barely penetrates this far down, leaving the air hot and thick with the smell of pine.", '\n');
    game.log("Despite the foliage, there's a well-defined trail heading NORTH and WEST", '\n');
  } else if ((1 === 1)) {
    game.log("You're in a dense forest. There's a trail heading NORTH and WEST.", '\n');
  }
  return 0;
}

function prsoDetritus(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, count: 0};
  if ((locals['cmd'] === "EMPTY")) {
    for (let object of game.getObjectsIn(locals['prso'])) {
      locals['obj'] = object;
      game.move(locals, locals['obj'], locals['cRoom'])
      locals['count'] = (locals['count'] + 1);
    }
    if ((locals['count'] > 0)) {
      game.log("Emptied contents", '\n');
    } else if ((1 === 1)) {
      game.log("Nothing to unpack", '\n');
    }
  }
  return 0;
}

function timePasses(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, newW: 0};
  if ((globals['time'] > 0)) {
    globals['time'] = (globals['time'] + 1);
    if ((globals['time'] === 34)) {
      globals['time'] = -1;
      if ((globals['weather'] === 0)) {
        globals['dry'] = 0;
      }
    }
  } else if ((globals['time'] < 0)) {
    globals['time'] = (globals['time'] + -1);
    if ((globals['time'] === -18)) {
      globals['time'] = 1;
    } else if ((globals['time'] === -8)) {
      globals['day'] = (globals['day'] + 1);
      globals['moonPhase'] = (globals['moonPhase'] + 1);
      if ((globals['moonPhase'] === 24)) {
        globals['moonPhase'] = 0;
      }
      locals['newW'] = Math.floor(Math.random() * 100);
      if ((30 > locals['newW'])) {
        globals['weather'] = 3;
        globals['dry'] = (globals['dry'] + 1);
      } else if ((60 > locals['newW'])) {
        globals['weather'] = 2;
        globals['dry'] = (globals['dry'] + 1);
      } else if ((90 > locals['newW'])) {
        globals['weather'] = 1;
        globals['dry'] = 0;
      } else if ((100 > locals['newW'])) {
        globals['weather'] = 0;
        globals['dry'] = (globals['dry'] + 1);
      }
    }
  }
  return 0;
}

function weatherReport(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, isNight: 0,untilFull: 0};
  if ((globals['time'] > 48)) {
    locals['isNight'] = 0;
    game.log("It's evening, ");
  } else if ((globals['time'] > 32)) {
    locals['isNight'] = 0;
    game.log("It's the afternoon, ");
  } else if ((globals['time'] > 16)) {
    locals['isNight'] = 0;
    game.log("It's morning, ");
  } else if ((globals['time'] > 0)) {
    locals['isNight'] = 0;
    game.log("It's early morning, ");
  } else if ((globals['time'] < -16)) {
    locals['isNight'] = 1;
    game.log("It's night, ");
  } else if ((globals['time'] < 0)) {
    locals['isNight'] = 1;
    game.log("It's late at night, ");
  }
  if ((globals['moonPhase'] === 0)) {
    game.log("new moon, ");
  } else if (((globals['moonPhase'] > 0) && (globals['moonPhase'] < 12))) {
    locals['untilFull'] = (globals['moonPhase'] - 11);
    game.log("the moon is waxing (", locals['untilFull'].toString(), " day");
    if (!(locals['untilFull'] === 1)) {
      game.log("s");
    }
    game.log(" until full), ");
  } else if ((globals['moonPhase'] === 12)) {
    game.log("full moon, ");
  } else if (((globals['moonPhase'] > 12) && (globals['moonPhase'] < 24))) {
    game.log("the moon is waning, ");
  }
  if ((globals['weather'] === 3)) {
    if ((locals['isNight'] === 0)) {
      game.log("and there's a clear blue sky overhead. ");
    } else if ((locals['isNight'] === 1)) {
      game.log("and there are thousands of stars above you. ");
    }
  } else if ((globals['weather'] === 2)) {
    if ((locals['isNight'] === 0)) {
      game.log("but a dim, overcast sky presses down on you. ");
    } else if ((locals['isNight'] === 1)) {
      game.log("but no light penetrates through the cloudy sky. ");
    }
  } else if ((globals['weather'] === 1)) {
    if ((locals['isNight'] === 0)) {
      game.log("a persistent drizzle dampens everything around. ");
    } else if ((locals['isNight'] === 1)) {
      game.log("heavy raindrops fall haphazardly. ");
    }
  } else if ((globals['weather'] === 0)) {
    if ((locals['isNight'] === 0)) {
      game.log("storm clouds are building. ");
    } else if ((locals['isNight'] === 1)) {
      game.log("lightning and thunder beat down savagely around you. ");
    }
  }
  if ((globals['dry'] < 1)) {
  } else if ((globals['dry'] < 2)) {
    game.log("There are still puddles on the ground from recent rain.", '\n');
  } else if ((globals['dry'] < 4)) {
    game.log("The ground is still damp from rain.", '\n');
  } else if ((1 === 1)) {
    game.log("It's been ", globals['dry'].toString(), " days since the last rain.", '\n');
  }
  return 0;
}

function vDescRoom(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  game.describe(locals['cRoom']);
  return 0;
}

function vRoomDetail(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  globals['detailedDesc'] = 1;
  game.describe(locals['cRoom']);
  globals['detailedDesc'] = 0;
  return 0;
}

function vDescObjectsInRoom(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, count: 0};
  if ((globals['firstLookAround'] === 1)) {
    globals['firstLookAround'] = 0;
    game.log("This command lists interactable objects in the immediate vicinity. Interactions may not be obvious.", '\n');
    game.log("Objects nested inside other objects are not listed, but might show up if you EXAMINE their container.", '\n');
  }
  for (let object of game.getObjectsIn(locals['cRoom'])) {
    locals['obj'] = object;
    locals['count'] = (locals['count'] + 1);
    game.describe(locals['obj']);
    game.log('\n');
  }
  if ((locals['count'] === 0)) {
    game.log("This space appears to be empty.", '\n');
  }
  return 0;
}

function vInventory(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, count: 0};
  for (let object of game.getObjectsIn(player)) {
    locals['obj'] = object;
    locals['count'] = (locals['count'] + 1);
    game.describe(locals['obj']);
    game.log('\n');
  }
  if ((locals['count'] === 0)) {
    game.log("There's nothing in your inventory.", '\n');
  }
  return 0;
}

function vSparkAt(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vHitWith(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vOpen(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vClose(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vExamine(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, count: 0,r: 0};
  globals['detailedDesc'] = 1;
  game.describe(locals['prso']);
  globals['detailedDesc'] = 0;
  game.log('\n');
  if ((globals['firstExamine'] === 1)) {
    globals['firstExamine'] = 0;
    game.log("The EXAMINE command will list items nested inside the object, and might also tell you more about the object itself. If there's any interesting items in this object, you can EMPTY it to remove the nested items. After that, can TAKE an item off the ground.", '\n');
  }
  game.log("and inside:", '\n');
  for (let object of game.getObjectsIn(locals['prso'])) {
    locals['obj'] = object;
    locals['count'] = (locals['count'] + 1);
    game.describe(locals['obj']);
    game.log('\n');
  }
  if ((locals['count'] === 0)) {
    locals['r'] = Math.floor(Math.random() * 100);
    if ((10 > locals['r'])) {
      game.log("a whole lotta nuthin", '\n');
    } else if ((30 > locals['r'])) {
      game.log("zilch", '\n');
    } else if ((50 > locals['r'])) {
      game.log("nada", '\n');
    } else if ((100 > locals['r'])) {
      game.log("nothing", '\n');
    }
  }
  return 0;
}

function vEat(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vPutIn(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vTalkTo(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vTake(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  if (((locals['prso'].vars['noTake'] || 0) === 1)) {
    game.log("This can't be picked up.", '\n');
  } else if ((1 === 1)) {
    game.move(locals, locals['prso'], player)
    game.log("Picked up!", '\n');
  }
  return 0;
}

function vTakeOut(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vPeeOn(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vCook(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vWriteNote(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vSleep(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vDrop(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vWorkWith(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  return 0;
}

function vSwim(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  game.log("The water looks a little chilly, it would be better if we had a boat, or maybe built one?", '\n');
  return 0;
}

function vJump(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  game.log("No thank you", '\n');
  return 0;
}

function vWhereToGo(cRoom, cmd, prso, prsi) {
  const locals = {cRoom, cmd, prso, prsi, };
  game.log("There's only one way to find out. Directions to try are: UP DOWN IN OUT NORTH SOUTH EAST WEST.", '\n');
  return 0;
}

