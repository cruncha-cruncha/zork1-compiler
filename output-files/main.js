import { rooms } from "./rooms.js";
import { routines } from "./routines.js";
import { player } from "./globals.js";
import {
  objects,
  lookupBySynonym as lookupObjectBySynonym,
} from "./objects.js";
import { parseInput } from "./parser.js";

let command = {};
let logBuffer = [];

export const log = (...args) => {
  let hasNewline = args.some(
    (arg) => typeof arg === "string" && arg.includes("\n")
  );

  if (hasNewline) {
    console.log(...logBuffer, ...args);
    logBuffer = [];
  } else {
    logBuffer.push(...args);
  }
};

const getRoutineArgs = (command) => {
  const { prsa, prso, prsi } = command;
  const currentRoom = getRoom(player.currentRoom);
  return [
    player,
    currentRoom,
    prsa,
    prso ? prso.val : null,
    prsi ? prsi.val : null,
  ];
};

export const handleRawInput = (input) => {
  let result = parseInput(input);

  if (result) {
    command = result;

    if ("move" in command) {
      handleMove(command.move);
    } else if ("routine" in command) {
      const routine = routines[command.routine].func;
      routine(...getRoutineArgs(command));
    }
  } else {
    log("I don't understand that command.\n");
  }
};

export const describe = (location) => {
  if ("text" in location.desc) {
    log(location.desc.text);
  } else if ("routine" in location.desc) {
    const routine = routines[location.desc.routine].func;
    routine(...getRoutineArgs(command));
  }
};

export const getRoom = (roomName) => {
  return rooms[roomName];
};

export const getObject = (objectName) => {
  return objects[objectName];
};

export const getLocation = (name) => {
  if (name in rooms) {
    return rooms[name];
  }

  if (name in objects) {
    return objects[name];
  }

  if (name === "cmdPrso" && command.prso) {
    return command.prso.val;
  }

  console.error("getLocation: name not found:", name);

  return {
    desc: {},
    vars: {},
    objects: [],
  };
};

export const getVariablesOf = (obj) => {
  if (!obj || !("vars" in obj)) {
    console.error("getVariablesOf: obj is not valid", obj);
    return [];
  }

  return Object.keys(obj.vars).map((name) => ({ name, val: obj.vars[name] }));
};

export const getObjectsIn = (something) => {
  return something.objects.map((objName) => getObject(objName));
};

export const findObjectMatchingParsedWord = (word, params) => {
  const object = lookupObjectBySynonym(word);
  if (!object) {
    return { objectNum: 0, objectVal: null };
  }

  // TODO

  // should not be able to interact with objects that are not in the same room or on the player
  // should not be able to interact with nested objects
  // params[0].restrictions is just a list of variable names that must be present on the object, doesn't matter what their value is

  return { objectNum: 1, objectVal: object };
};

export const handleMove = (direction) => {
  let roomName = player.currentRoom;
  let actualRoom = rooms[roomName];

  let result = actualRoom.move[direction];

  if (!result) {
    log("You can't go that way.\n");
    return;
  }

  // TODO: we need to maintain an object { [roomName]: [list of objects in room] }
  // so we can check if the object is in the room before we can interact with it
  // don't worry about nested objects, we can recurse to find them
  // need to keep the lookup object up-to-date

  if ("text" in result) {
    log(result.text, "\n");
  } else if ("room" in result) {
    player.currentRoom = result.room;
    describe(getRoom(player.currentRoom));
    log("\n");
  } else if ("routine" in result) {
    const routine = routines[result.routine].func;
    routine(...getRoutineArgs(command));
  }
};
