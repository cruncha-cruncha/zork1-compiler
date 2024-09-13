import { game } from './game.js';
import { player, globals } from './globals.js';
import { objects } from './objects.js';
import { rooms } from './rooms.js';

export const routines = {
  canEnterCabin: {
    isRoutine: 'canEnterCabin',
    func: canEnterCabin,
  },
  timePasses: {
    isRoutine: 'timePasses',
    func: timePasses,
  },
  vDetritus1Prso: {
    isRoutine: 'vDetritus1Prso',
    func: vDetritus1Prso,
  },
  fDescForest1: {
    isRoutine: 'fDescForest1',
    func: fDescForest1,
  },
  vWeatherReport: {
    isRoutine: 'vWeatherReport',
    func: vWeatherReport,
  },
  fDescCabinExterior: {
    isRoutine: 'fDescCabinExterior',
    func: fDescCabinExterior,
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
  fCabinDoorPrso: {
    isRoutine: 'fCabinDoorPrso',
    func: fCabinDoorPrso,
  },
  vThrowAt: {
    isRoutine: 'vThrowAt',
    func: vThrowAt,
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
  vSing: {
    isRoutine: 'vSing',
    func: vSing,
  },
  vWhisper: {
    isRoutine: 'vWhisper',
    func: vWhisper,
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
  vBait: {
    isRoutine: 'vBait',
    func: vBait,
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

function canEnterCabin(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  if ((((objects['cabinDoor'].vars['isLocked'] || 0) === 1) && ((objects['cabinWindow'].vars['isSmashed'] || 0) === 0))) {
    game.log("There's no way into the cabin, you need to break something or find a key.", '\n');
  } else if ((1 === 1)) {
    game.move(locals, player, rooms['cabin'])
  }
  return 0;
}

function timePasses(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, newW: 0};
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
      globals['moon'] = (globals['moon'] + 1);
      if ((globals['moon'] === 24)) {
        globals['moon'] = 0;
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

function vDetritus1Prso(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  if ((locals['cmdPrsa'] === "UNPACK")) {
    for (let object of game.getObjectsIn(objects['detritus1'])) {
      locals['obj'] = object;
      if (!((locals['obj'].vars['noTake'] || 0) === 1)) {
        game.move(locals, locals['obj'], locals['currentRoom'])
      }
    }
    game.log("Unpackable items are now on the ground.", '\n');
  }
  return 0;
}

function fDescForest1(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  if (((rooms['forest1'].vars['firstTime'] || 0) === 1)) {
    rooms['forest1'].vars['firstTime'] = 0;
    game.log("It's the end of summer. You're in a dense forest. Birds and other small creatures can be heard rustling through the undergrowth and the smell of pine hangs thick in the air. Light filters down through needles and leaves.", '\n');routines['vWeatherReport'].func(locals['currentRoom'], locals['prsa'], locals['prso'], locals['prsi'])
    game.log("There's a trail up ahead, it looks like you could GO NORTH", '\n');
  } else if ((globals['detailedDesc'] === 1)) {
    game.log("You're in dense forest. The undergrowth is thick with dead branches, leaves, and other detritus. Vines, bushes, and brush all vie for a spot in the sunlight. Even during a thunderstorm, the wind barely penetrates this far down, leaving the air hot and thick with the smell of pine.", '\n');
    game.log("Despite the foliage, there's a well-defined trail heading NORTH and WEST", '\n');
  } else if ((1 === 1)) {
    game.log("You're in a dense forest. There's a trail heading NORTH and WEST.", '\n');
  }
  return 0;
}

function vWeatherReport(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, isNight: 0};
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
  if ((globals['moon'] === 0)) {
    game.log("new moon, ");
  } else if (((globals['moon'] > 0) && (globals['moon'] < 12))) {
    game.log("the moon is waxing, ");
  } else if ((globals['moon'] === 12)) {
    game.log("full moon, ");
  } else if (((globals['moon'] > 12) && (globals['moon'] < 24))) {
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

function fDescCabinExterior(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  if (((rooms['cabinExterior'].vars['firstTime'] || 0) === 1)) {
    rooms['cabinExterior'].vars['firstTime'] = 0;
    game.log("You're at the transition between a forest and a field. There are trails in all directions, but at the center a cabin! It's got a scenic window facing the fields. You could try to OPEN DOOR", '\n');
    return 1;
  }
  game.log("You're outside a cabin on the edge of a forest, overlooking some fields.", '\n');
  if (((objects['cabinDoor'].vars['isLocked'] || 0) === 1)) {
    game.log("The cabin door is locked. Where's the key?", '\n');
  }
  return 0;
}

function vDescRoom(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.describe(locals['currentRoom']);
  return 0;
}

function vRoomDetail(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  globals['detailedDesc'] = 1;
  game.describe(locals['currentRoom']);
  globals['detailedDesc'] = 0;
  return 0;
}

function vDescObjectsInRoom(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, count: 0};
  if ((globals['firstLookAround'] === 1)) {
    globals['firstLookAround'] = 0;
    game.log("This command lists interactable objects in the immediate vicinity. Interactions may not be obvious.", '\n');
    game.log("Objects nested inside other objects are not listed, but might show up if you EXAMINE their container.", '\n');
  }
  for (let object of game.getObjectsIn(locals['currentRoom'])) {
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

function vInventory(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, count: 0};
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

function fCabinDoorPrso(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  if ((locals['cmdPrsa'] === "OPEN")) {
    if ((game.isInLocation(locals['currentRoom'], player, objects['cabinDoorKey'], false) && ((objects['cabinDoor'].vars['isLocked'] || 0) === 1))) {
      objects['cabinDoor'].vars['isLocked'] = 0;
      game.log("The key works!", '\n');
      game.move(locals, player, rooms['cabin'])
    } else if (((objects['cabinDoor'].vars['isLocked'] || 0) === 0)) {
      game.move(locals, player, rooms['cabin'])
    } else if ((1 === 1)) {
      game.log("The door is locked. You should LOOK AROUND for a key.", '\n');
    }
  }
  return 0;
}

function vThrowAt(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vSparkAt(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("Spark it at what?", '\n');
  return 0;
}

function vHitWith(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vOpen(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  return 0;
}

function vClose(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vExamine(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, count: 0,r: 0};
  globals['detailedDesc'] = 1;
  game.describe(locals['cmdPrso']);
  globals['detailedDesc'] = 0;
  game.log('\n');
  if ((globals['firstExamine'] === 1)) {
    globals['firstExamine'] = 0;
    game.log("If there's any interesting item in this object, you can UNPACK it to remove the nested items. After that, can TAKE the item off the ground.", '\n');
  }
  game.log("...contains:", '\n');
  for (let object of game.getObjectsIn(locals['cmdPrso'])) {
    locals['obj'] = object;
    game.describe(locals['obj']);
    game.log('\n');
    locals['count'] = (locals['count'] + 1);
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

function vEat(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vPutIn(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vTalkTo(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vSing(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vWhisper(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vTake(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  if (((locals['cmdPrso'].vars['noTake'] || 0) === 1)) {
    game.log("This can't be picked up.", '\n');
  } else if ((1 === 1)) {
    game.move(locals, locals['cmdPrso'], player)
    game.log("Picked up!", '\n');
  }
  return 0;
}

function vTakeOut(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  return 0;
}

function vPeeOn(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vCook(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vWriteNote(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vSleep(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vDrop(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vWorkWith(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("// TODO", '\n');
  return 0;
}

function vBait(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  if (!((locals['cmdPrso'].vars['canBait'] || 0) === 1)) {
    game.log("Can't bait that", '\n');
    return 0;
  }
  locals['cmdPrso'].vars['baited'] = 1;
  game.log("Baited, SET OBJECT DOWN to see if it will catch anything.", '\n');
  return 1;
  return 0;
}

function vSwim(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("The water looks a little chilly, it would be better if we had a boat, or maybe built one?", '\n');
  return 0;
}

function vJump(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("No thank you", '\n');
  return 0;
}

function vWhereToGo(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  game.log("There's only one way to find out. Directions to try are: UP DOWN IN OUT NORTH SOUTH EAST WEST.", '\n');
  return 0;
}

