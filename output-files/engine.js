import { rooms } from "./rooms.js";
import { player } from "./globals.js";
import {
  objects,
  translateSynonym as lookupObjectNamesBySynonym,
} from "./objects.js";
import { parseInput } from "./parser.js";
import { routines, handlers } from "./routines.js";
import { newHooks } from "./hooks.js";

// instead of returning null, return an empty resource object
// this allows chained operations to fail silently
// for example <MOVE PLAYER <LOC <CMD 1>>> might not work:
// - <CMD 1> might be undefined (syntax has no OBJECT)
// - <LOC ...> might not be a room
export const getEmptyResource = () => ({
  isEmpty: true,
  objects: [],
  desc: {},
  loc: {},
  vars: {},
  hooks: {},
});

// use a random 12 character string for object instance ids
function gen_inst_id() {
  const characters = "abcdefghijklmnopqrstuvwxyz";
  let result = "";
  for (let i = 0; i < 12; i++) {
    result += characters.charAt(Math.floor(Math.random() * 26));
  }
  return "inst_" + result;
}

// a giant closure that keeps track of everything
const newGame = () => {
  // aka text output, errors use console.error
  // defaults to console.log, but can override by calling setLogger
  let logger = console.log;

  // hooks is a priority queue, and the only way that a handler/routine can run
  const hooks = newHooks();

  // current room the player is in
  let currentRoom = player.startRoom
    ? rooms[player.startRoom]
    : Object.values(rooms)[0];

  // similar to but not quite the same as parser output
  let command = { prsa: "", cmds: [] };

  // aka game over, set by <END>
  // won't accept any more input if true (game over)
  let is_over = false;

  // is safe to assume that all args are strings
  function log(...args) {
    logger(...args);
  }

  // aka <DESC ... >
  // handle (DESC ... ) of an object, object instance, room, or the player
  // returns nothing
  function describe(fakePrso) {
    if (!fakePrso || typeof fakePrso !== "object") {
      return;
    }

    let desc = null;
    if ("isObject" in fakePrso && "isInst" in fakePrso) {
      // if object instance, get description from the object
      desc = objects[fakePrso.isObject].desc;
    } else {
      desc = fakePrso.desc;
    }

    if (!desc) {
      return;
    } else if ("text" in desc) {
      log(desc.text);
    } else if ("routine" in desc) {
      // is executed immediately
      hooks.callDescription(routines[desc.routine], currentRoom, [
        fakePrso,
        ...command.cmds.slice(1),
      ]);
    }
  }

  // handle "GO <direction>" from the user
  // returns a boolean
  function handleGo(direction) {
    let result = currentRoom.move[direction];

    if (!result) {
      return false;
    }

    if ("text" in result) {
      log(result.text, "\n");
    } else if ("room" in result) {
      if (currentRoom.hooks.exit) {
        hooks.insert(
          "ROOM-ACT-EXIT",
          routines[currentRoom.hooks.exit],
          currentRoom,
          command.cmds
        );
      }
      if (player.hooks.exit) {
        hooks.insert(
          "PLAYER-ACT-EXIT",
          routines[player.hooks.exit],
          currentRoom,
          command.cmds
        );
      }

      currentRoom = rooms[result.room];

      if (player.hooks.enter) {
        hooks.insert(
          "PLAYER-ACT-ENTER",
          routines[player.hooks.enter],
          currentRoom,
          command.cmds
        );
      }
      if (currentRoom.hooks.enter) {
        hooks.insert(
          "ROOM-ACT-ENTER",
          routines[currentRoom.hooks.enter],
          currentRoom,
          command.cmds
        );
      }
    } else if ("routine" in result) {
      hooks.insert(
        "GO-ACTION",
        routines[result.routine],
        currentRoom,
        command.cmds
      );
    }

    hooks.callAll();
    return true;
  }

  // nested objects are stored like: object: { [objName]: [...instId] }
  // for example object: { "chair": ["inst_1", "inst_3"], "table": ["inst_2"] }
  // params: nestedObj is object, name is objName, inst is instId
  // if called with name = "chair" and inst = "inst_1", would return { "chair": ["inst_3"], "table": ["inst_2"] }
  // if called with name = "table" and inst = "inst_2", would return { "chair": ["inst_1", "inst_3"] }
  // cannot and should not be called from outside this closure
  function filterRemoveObjectInstance(nestedObjs, name, inst) {
    const out = Object.keys(nestedObjs).reduce((acc, objName) => {
      let list = nestedObjs[objName];
      if (objName === name) {
        list = nestedObjs[objName].filter((id) => id !== inst);
      }

      if (list.length > 0) {
        return { ...acc, [objName]: list };
      } else {
        return acc;
      }
    }, {});

    return out;
  }

  // aka <MOVE ... >
  // move thing to destination, or delete thing if destination is undefined
  // thing must be an object instance or the player
  // destination must be an object instance, room, or the player
  // need to pass locals here in case we move player to a new room, in which case the current room (C-ROOM) changes
  // returns a boolean, indicating success
  function move(locals, thing, destination) {
    if (!thing || typeof thing !== "object") {
      return false;
    }

    if (!destination || typeof destination !== "object") {
      if (!("isObject" in thing) || !("isInst" in thing)) {
        return false;
      }

      // delete thing and all nested object instances from objects
      const delCopies = [];
      const addCopies = (objInst) => {
        delCopies.push({ name: objInst.isObject, inst: objInst.isInst });

        for (const objName of Object.keys(objInst.objects)) {
          for (const id of objInst.objects[objName]) {
            addCopies(objects[objName].copies[id]);
          }
        }
      };
      addCopies(thing);

      for (const copy of delCopies) {
        delete objects[copy.name].copies[copy.inst];
      }

      // delete thing from it's location
      const { scope = "", name = "", inst = "" } = thing.loc;
      if (scope === "player") {
        player.objects = filterRemoveObjectInstance(
          player.objects,
          thing.isObject,
          thing.isInst
        );
      } else if (scope === "room") {
        const room = rooms[name];
        room.objects = filterRemoveObjectInstance(
          room.objects,
          thing.isObject,
          thing.isInst
        );
      } else if (scope === "object") {
        const objInst = objects[name].copies[inst];
        objInst.objects = filterRemoveObjectInstance(
          objInst.objects,
          thing.isObject,
          thing.isInst
        );
      }

      // set thing to empty
      delete thing.isObject;
      delete thing.isInst;
      thing.isEmpty = true;
      thing.objects = [];
      thing.desc = {};
      thing.loc = {};
      thing.vars = {};
      thing.hooks = {};

      return true;
    }

    if ("isPlayer" in thing) {
      // can only move a player into a room, not into an object or the player itself
      if ("isRoom" in destination && thing.loc !== destination.isRoom) {
        if (currentRoom.hooks.exit) {
          hooks.insert(
            "ROOM-ACT-EXIT",
            routines[currentRoom.hooks.exit],
            currentRoom,
            command.cmds
          );
        }
        if (player.hooks.exit) {
          hooks.insert(
            "PLAYER-ACT-EXIT",
            routines[player.hooks.exit],
            currentRoom,
            command.cmds
          );
        }

        currentRoom = rooms[destination.isRoom];

        if (player.hooks.enter) {
          hooks.insert(
            "PLAYER-ACT-ENTER",
            routines[player.hooks.enter],
            currentRoom,
            command.cmds
          );
        }
        if (currentRoom.hooks.enter) {
          hooks.insert(
            "ROOM-ACT-ENTER",
            routines[currentRoom.hooks.enter],
            currentRoom,
            command.cmds
          );
        }

        locals["currentRoom"] = currentRoom;
        return true;
      }
    } else if ("isObject" in thing) {
      // can move an object into a room, player, or another object
      let goodMove = false;
      const objHooks = objects[thing.isObject].hooks;
      const { scope = "", name = "", inst = "" } = thing.loc;

      const oldLocation = (() => {
        if (scope == "player") {
          return player;
        } else if (scope == "room") {
          return rooms[name];
        } else if (scope == "object") {
          return objects[name].copies[inst];
        } else {
          return { objects: {} };
        }
      })();

      if ("isPlayer" in destination && scope !== "player") {
        if (objHooks.enterPlayer) {
          hooks.insert(
            "OBJ-ACT-ADD",
            routines[objHooks.enterPlayer],
            currentRoom,
            [thing, ...command.cmds.slice(1)]
          );
        }

        // update object location
        thing.loc = { scope: "player" };

        goodMove = true;
      } else if (
        "isRoom" in destination &&
        (scope !== "room" || name !== destination.isRoom)
      ) {
        if (scope == "player" && objHooks.exitPlayer) {
          hooks.insert(
            "OBJ-ACT-REMOVE",
            routines[objHooks.exitPlayer],
            currentRoom,
            [thing, ...command.cmds.slice(1)]
          );
        }

        // update object location
        thing.loc = { scope: "room", name: destination.isRoom };

        goodMove = true;
      } else if (
        "isObject" in destination &&
        (scope !== "object" ||
          name !== destination.isObject ||
          inst !== destination.isInst)
      ) {
        if (scope == "player" && objHooks.exitPlayer) {
          hooks.insert(
            "OBJ-ACT-REMOVE",
            routines[objHooks.exitPlayer],
            currentRoom,
            [thing, ...command.cmds.slice(1)]
          );
        }

        // update object location
        thing.loc = {
          scope: "object",
          name: destination.isObject,
          inst: destination.isInst,
        };

        goodMove = true;
      }

      if (goodMove) {
        // remove object from old location
        oldLocation.objects = filterRemoveObjectInstance(
          oldLocation.objects,
          thing.isObject,
          thing.isInst
        );

        // add object to destination
        if (thing.isObject in destination.objects) {
          destination.objects[thing.isObject].push(thing.isInst);
        } else {
          destination.objects[thing.isObject] = [thing.isInst];
        }

        return true;
      }
    }

    return false;
  }

  // aka <INST ... >
  // get the first object instance of type objName in the scope
  // params: scope is an object instance, a room, or the player.
  // params: objName is a string, the name of the object
  // params: nested is a boolean
  // returns an instance or the emptyResource
  // if nested is false, only search the scope, not nested objects
  // if nested is true, search the scope and all nested objects, breadth-first
  function getInst(scope, objName, nested = true) {
    if (
      !scope ||
      !objName ||
      typeof scope !== "object" ||
      typeof objName !== "string"
    ) {
      return getEmptyResource();
    }

    if (!nested) {
      if (objName in scope.objects && scope.objects[objName].length > 0) {
        return objects[objName].copies[scope.objects[objName][0]];
      }
    }

    const searchable = [scope.objects];
    while (searchable.length > 0) {
      const nestedObjs = searchable.shift();
      for (const name of Object.keys(nestedObjs)) {
        if (name === objName) {
          return objects[name].copies[nestedObjs[name][0]];
        }
        searchable.push(
          ...nestedObjs[name].map((inst) => objects[name].copies[inst].objects)
        );
      }
    }

    return getEmptyResource();
  }

  // these functions are all exposed outside the closure
  // but only 'setLogger', 'start', and 'handleRawInput' should be called by non-generated code
  return {
    log,

    describe,

    // the player enters the game
    start() {
      if (player.hooks.enter) {
        hooks.insert(
          "PLAYER-ACT-ENTER",
          routines[player.hooks.enter],
          currentRoom,
          command.cmds
        );
      }
      if (currentRoom.hooks.enter) {
        hooks.insert(
          "ROOM-ACT-ENTER",
          routines[currentRoom.hooks.enter],
          currentRoom,
          command.cmds
        );
      }

      hooks.callAll();
    },

    setLogger(newLogger) {
      logger = newLogger;
    },

    handleGo,

    // parse user text input, expects a string, returns an object (see details below)
    handleRawInput(input) {
      let handled = false;
      if (is_over) {
        return {
          handled,
          syntax: {
            prsa: "",
            cmds: [],
          },
          goDirection: null,
          playerVars: { ...player.vars },
        };
      }

      let result = parseInput(input);
      command = result;

      // command.cmds should be a list of object instances that correspond to OBJECTs in the syntax
      // starts at 1, not 0, because game developers don't need to understand the historical basis for zero-based indexing
      // cmds[0] from the parser is always an empty object to faciliate this
      // but cmds[i > 0] from the parser is a { word, val } object, and the engine only cares about val
      command.cmds = result.cmds.map((cmd) =>
        cmd.val ? cmd.val : getEmptyResource()
      );

      if ("move" in result) {
        // user command is "GO <direction>"
        handled = handleGo(result.move);
      }

      if ("handle" in result) {
        // user command matches a syntax
        handled = true;

        // srh = syntax routine handlers
        const srh = handlers[result.prsa];

        let alreadySeen = [];
        for (const inst of command.cmds) {
          if (inst.isObject in srh.objHandlers) {
            if (alreadySeen.includes(inst.isObject)) {
              continue;
            } else {
              alreadySeen.push(inst.isObject);
            }

            const objHandle = srh.objHandlers[inst.isObject];
            if ("before" in objHandle) {
              hooks.insert(
                "BEFORE-ACTION",
                objHandle.before,
                currentRoom,
                command.cmds
              );
            }
          }
        }

        if ("func" in srh) {
          hooks.insert("SYNTAX-ACTION", srh.func, currentRoom, command.cmds);
        }

        alreadySeen = [];
        for (const inst of command.cmds) {
          if (inst.isObject in srh.objHandlers) {
            if (alreadySeen.includes(inst.isObject)) {
              continue;
            } else {
              alreadySeen.push(inst.isObject);
            }

            const objHandle = srh.objHandlers[inst.isObject];
            if ("after" in objHandle) {
              hooks.insert(
                "AFTER-ACTION",
                objHandle.after,
                currentRoom,
                command.cmds
              );
            }
          }
        }

        // run all queued hooks to ensure that future insertions of higher-priority hooks don't wipe these calls
        hooks.callAll();
      }

      // handle all OBJ-ACT-IN-ROOM hooks
      let nested = Object.keys(currentRoom.objects)
        .map((name) =>
          currentRoom.objects[name].map((inst) => objects[name].copies[inst])
        )
        .flat();

      while (nested.length > 0) {
        const inst = nested.shift();
        const object = objects[inst.isObject];
        if (object.hooks.inRoom) {
          hooks.insert(
            "OBJ-ACT-IN-ROOM",
            routines[object.hooks.inRoom],
            currentRoom,
            [inst, ...command.cmds.slice(1)]
          );
        }

        const children = Object.keys(inst.objects)
          .map((name) =>
            inst.objects[name].map((inst) => objects[name].copies[inst])
          )
          .flat();

        nested.push(...children);
      }

      // handle all OBJ-ACT-IN-PLAYER hooks
      nested = Object.keys(player.objects)
        .map((name) =>
          player.objects[name].map((inst) => objects[name].copies[inst])
        )
        .flat();

      while (nested.length > 0) {
        const inst = nested.shift();
        const object = objects[inst.isObject];
        if (object.hooks.inPlayer) {
          hooks.insert(
            "OBJ-ACT-IN-PLAYER",
            routines[object.hooks.inPlayer],
            currentRoom,
            [inst, ...command.cmds.slice(1)]
          );
        }

        const children = Object.keys(inst.objects)
          .map((name) =>
            inst.objects[name].map((inst) => objects[name].copies[inst])
          )
          .flat();

        nested.push(...children);
      }

      // run all queued hooks to ensure...
      hooks.callAll();

      if (currentRoom.hooks.always) {
        hooks.insert(
          "ROOM-ACT-ALWAYS",
          routines[currentRoom.hooks.always],
          currentRoom,
          command.cmds
        );
      }
      if (player.hooks.always) {
        hooks.insert(
          "PLAYER-ACT-ALWAYS",
          routines[player.hooks.always],
          currentRoom,
          command.cmds
        );
      }

      // run all queued hooks to ensure...
      hooks.callAll();

      // handled: was any zil code run? Doesn't mean that logs were written
      // syntax.prsa: the action (first word) of the command, after translated from synonyms
      // syntax.cmds: an array of objects, one for each OBJECT in the cmd
      // syntax.cmds[i].word: the actual word (a string) that was parsed, which should correspon to an object
      // syntax.cmds[i].hasVal: did this word match an object?
      // goDirection: if the command was "GO <direction>", this is the direction
      // playerVars: an object with all the player variables (these are all numeric). Good for determining if the player is dead and the game should end
      return {
        handled,
        syntax: {
          prsa: result?.prsa ?? "",
          // first elem is always an emptyResource, can remove it
          cmds: (result?.cmds || []).slice(1).map((cmd) => ({
            word: cmd.word,
            hasVal: !!cmd.val,
          })),
        },
        goDirection: result?.move ?? null,
        playerVars: { ...player.vars },
      };
    },

    // aka <EACH-VAL obj ... >
    // get variables of an object instance, a room, or the player, and technically would work with an object too
    // return an array of { name: string, val: number }
    // variable values are always numeric (at least the ones returned from this function)
    getVariablesOf(obj) {
      if (!obj || !("vars" in obj)) {
        return [];
      }

      return Object.keys(obj.vars).map((name) => ({
        name,
        val: obj.vars[name],
      }));
    },

    // aka <EACH-OBJ ... >
    // get all object instances in an object instance (aka nested), a room, or the player
    // OR get all instances of an object type
    // does not return deeper nested objects
    getObjectsIn(something) {
      if (!something || typeof something !== "object") {
        return [];
      }

      if ("isObject" in something && !("isInst" in something)) {
        // get all instances of an object type
        return Object.values(something.copies);
      }

      return Object.keys(something.objects)
        .map((name) => {
          return something.objects[name].map(
            (inst) => objects[name].copies[inst]
          );
        })
        .flat();
    },

    // used by the parser when it encounters an OBJECT in the syntax
    // must be able to find an object instance (of object type word) in the player or current room, but it can be nested
    // should only be used by the parser, as it returns emptyObj (not emptyResource)
    findObjectMatchingParsedWord(word) {
      const emptyObj = { objectVal: null };
      if (!word) {
        return emptyObj;
      }

      // matches is a string[]
      const matches = lookupObjectNamesBySynonym(word);
      if (!matches || matches.length === 0) {
        return emptyObj;
      }

      // objects are accessible if in current room or player, or nested within
      const objectsAccessible = [];
      const addInstances = (nestedObjs) => {
        const instances = Object.keys(nestedObjs)
          .map((name) =>
            nestedObjs[name].map((inst) => objects[name].copies[inst])
          )
          .flat();

        objectsAccessible.push(...instances);

        for (const obj of instances) {
          addInstances(obj.objects);
        }
      };

      addInstances(player.objects);
      addInstances(currentRoom.objects);

      // objectsMatching is an object[]
      const objectsMatching = objectsAccessible.filter((obj) =>
        matches.includes(obj.isObject)
      );

      if (objectsMatching.length > 0) {
        return { objectVal: objectsMatching[0] };
      } else {
        return emptyObj;
      }
    },

    // aka <LOC ... >
    // get the parent of an object instance
    // if !nested, returns an object instance, a room, or the player
    // if !nested and passed a room or the player, returns the room or the player
    // if nested, returns a room
    getParent(thing, nested = false) {
      if (!thing || typeof thing !== "object") {
        return getEmptyResource();
      }

      if (nested) {
        if ("isPlayer" in thing) {
          return currentRoom;
        } else if ("isRoom" in thing) {
          return thing;
        }

        let parent = thing;
        while (true) {
          if ("isObject" in parent && "isInst" in parent) {
            const { scope = "", name = "", inst = "" } = parent.loc;
            if (scope === "player") {
              return currentRoom;
            } else if (scope === "room" && !!name) {
              return rooms[name];
            } else if (scope === "object" && !!name && !!inst) {
              parent = objects[name].copies[inst];
            } else {
              return getEmptyResource();
            }
          } else {
            return getEmptyResource();
          }
        }
      }

      if ("isPlayer" in thing) {
        return player;
      } else if ("isRoom" in thing) {
        return thing;
      } else if ("isObject" in thing) {
        const { scope = "", name = "", inst = "" } = thing.loc;

        if (scope === "player") {
          return player;
        }

        if (scope === "room") {
          return rooms[name];
        }

        if (scope === "object") {
          return objects[name].copies[inst];
        }
      }

      return player;
    },

    // aka <IS-IN ... >
    // container: an object instance, a room, or the player
    // thing: an object instance, the player, OR an object
    // returns a boolean
    isInLocation(cRoom, container, thing, nested) {
      // if thing == location && !nested -> return true
      // if thing == location && nested -> return false

      // if !nested -> thing.loc == container
      // if nested -> any parent of thing == container

      if (
        !thing ||
        !container ||
        typeof thing !== "object" ||
        typeof container !== "object"
      ) {
        return false;
      }

      if ("isPlayer" in thing) {
        if ("isPlayer" in container) {
          return !nested;
        } else if ("isRoom" in container) {
          return cRoom.isRoom === container.isRoom;
        }
      } else if ("isRoom" in thing) {
        if ("isRoom" in container) {
          return thing.isRoom === container.isRoom && !nested;
        }
      } else if ("isObject" in thing) {
        if (!("loc" in thing)) {
          return !getInst(container, thing.isObject, nested).isEmpty;
        }

        const { scope = "", name = "", inst = "" } = thing.loc;

        if (!nested) {
          if ("isPlayer" in container) {
            return scope === "player";
          } else if ("isRoom" in container) {
            return scope === "room" && name === container.isRoom;
          } else if ("isObject" in container) {
            return (
              (scope === "object" &&
                name === container.isObject &&
                inst === container.isInst) ||
              (scope === "object" &&
                name === thing.isObject &&
                inst === thing.isInst)
            );
          }
        }

        let parent = thing.loc;
        while (true) {
          if ("isPlayer" in container) {
            if (parent.scope === "player") {
              return true;
            }
          } else if ("isRoom" in container) {
            if (parent.scope === "room" && parent.name === container.isRoom) {
              return true;
            }
          } else if ("isObject" in container) {
            if (
              parent.scope === "object" &&
              parent.name === container.isObject &&
              parent.inst === container.isInst
            ) {
              return true;
            }
          }

          if (parent.scope !== "object") {
            break;
          } else {
            parent = objects[parent.name].copies[parent.inst].loc;
          }
        }
      }

      return false;
    },

    move,

    // create a new instance of an object
    // thing can be an object instance or an object
    // destination can be an object instance, a room, or the player
    // returns a boolean
    copyMove(thing, destination) {
      if (!thing || typeof thing !== "object") {
        return false;
      }

      // does not copy nested objects
      const newId = gen_inst_id();
      const copy = {
        isInst: newId,
        isObject: thing.isObject,
        vars: { ...thing.vars },
        loc: {},
        objects: {},
      };

      // add new copy to the parent object
      objects[thing.isObject].copies[newId] = copy;

      // we can't copy the player, which means the current room won't change, which means we can pass {} as locals
      return move({}, copy, destination);
    },

    getInst,

    // compare two or more values
    // values can be: object instance, object, room, or the player
    // objects and object instances can be compared, but otherwise values must be all of the same type if they have any hope of returning true
    // returns a boolean
    isEqual(...args) {
      if (!args || !Array.isArray(args) || args.length < 2) {
        return false;
      }

      const normalized = args.map((arg) => {
        if (typeof arg === "object") {
          if ("isPlayer" in arg) {
            return "player";
          } else if ("isRoom" in arg) {
            return "r-" + arg.isRoom;
          } else if ("isObject" in arg) {
            return "o-" + arg.isObject;
          } else {
            return "unknown";
          }
        } else {
          return "unknown";
        }
      });

      return [...new Set(normalized)].length === 1;
    },

    close() {
      is_over = true;
    },
  };
};

// create a game instance, so generated code can get access to it
export const game = newGame();
