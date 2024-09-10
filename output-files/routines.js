import { log, describe, getRoom, getObject, getLocation, getVariablesOf, getObjectsIn } from './main.js';

export const routines = {
  journalDescFcn: {
    isRoutine: 'journalDescFcn',
    func: journalDescFcn,
  },
  vDescRoom: {
    isRoutine: 'vDescRoom',
    func: vDescRoom,
  },
  vListMyVars: {
    isRoutine: 'vListMyVars',
    func: vListMyVars,
  },
  vListMyObjects: {
    isRoutine: 'vListMyObjects',
    func: vListMyObjects,
  },
  vListObjects: {
    isRoutine: 'vListObjects',
    func: vListObjects,
  },
  vListObjectVars: {
    isRoutine: 'vListObjectVars',
    func: vListObjectVars,
  },
};

function journalDescFcn(player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = { player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  log("A journal");
}

function vDescRoom(player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = { player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  describe(locals['currentRoom']);
  log('\n');
}

function vListMyVars(player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = { player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  log("Player variables:", '\n');
  for (let varItem of getVariablesOf(locals['player'])) {
    locals['name'] = varItem.name;
    locals['val'] = varItem.val;
    log(locals['name'], ": ", locals['val'], '\n');
  }
}

function vListMyObjects(player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = { player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  log("Objects in player:", '\n');
  for (let object of getObjectsIn(locals['player'])) {
    locals['obj'] = object;
    describe(locals['obj']);
    log('\n');
  }
}

function vListObjects(player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = { player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi, };
  for (let object of getObjectsIn(locals['currentRoom'])) {
    locals['obj'] = object;
    describe(locals['obj']);
    log('\n');
  }
}

function vListObjectVars(player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {
  const locals = { player, currentRoom, cmdPrsa, cmdPrso, cmdPrsi, count: 0};
  for (let varItem of getVariablesOf(locals['cmdPrso'])) {
    locals['name'] = varItem.name;
    locals['val'] = varItem.val;
    log(locals['name'], ": ", locals['val'], '\n');
    locals['count'] = (locals['count'] + 1);
  }
  if ((locals['count'] < 1)) {
    log("No vars", '\n');
  }
}

