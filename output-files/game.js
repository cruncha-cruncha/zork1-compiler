import { rooms } from "./rooms.js";
import { player } from "./globals.js";
import {
  objects,
  translateSynonym as lookupObjectNamesBySynonym,
} from "./objects.js";
import { parseInput } from "./parser.js";
import { newHooks } from "./hooks.js";

function gen_inst_id() {
  const characters = "abcdefghijklmnopqrstuvwxyz";
  let result = "";
  for (let i = 0; i < 12; i++) {
    result += characters.charAt(Math.floor(Math.random() * 26));
  }
  return "inst_" + result;
}

const newGame = () => {
  let logger = console.log;
  const hooks = newHooks();
  let currentRoom = player.startRoom
    ? rooms[player.startRoom]
    : Object.values(rooms)[0];
  let command = {};

  function log(...args) {
    logger(...args);
  }

  function describe(fakePrso) {
    if (!fakePrso || typeof fakePrso !== "object") {
      return;
    }

    let desc = null;
    if ("isObject" in fakePrso && "isInst" in fakePrso) {
      desc = objects[fakePrso.isObject].desc;
    } else {
      desc = fakePrso.desc;
    }

    if ("text" in desc) {
      log(desc.text);
    } else if ("routine" in desc) {
      hooks.callDescription(desc.routine, currentRoom, fakePrso);
    }
  }

  function handleGo(direction) {
    let result = currentRoom.move[direction];

    if (!result) {
      return false;
    }

    if ("text" in result) {
      log(result.text, "\n");
    } else if ("room" in result) {
      if (currentRoom.hooks.exit) {
        hooks.insert("ROOM-ACT-EXIT", currentRoom.hooks.exit, currentRoom);
      }
      if (player.hooks.exit) {
        hooks.insert("PLAYER-ACT-EXIT", player.hooks.exit, currentRoom);
      }

      currentRoom = rooms[result.room];

      if (currentRoom.hooks.enter) {
        hooks.insert("ROOM-ACT-ENTER", currentRoom.hooks.enter, currentRoom);
      }
      if (player.hooks.enter) {
        hooks.insert("PLAYER-ACT-ENTER", player.hooks.enter, currentRoom);
      }
    } else if ("routine" in result) {
      hooks.insert("SYNTAX-ACTION", result.routine, currentRoom);
    }

    return true;
  }

  function move(locals, thing, destination) {
    if (
      !thing ||
      !destination ||
      typeof thing !== "object" ||
      typeof destination !== "object"
    ) {
      return false;
    }

    if ("isPlayer" in thing) {
      if ("isRoom" in destination && thing.loc !== destination.isRoom) {
        if (currentRoom.hooks.exit) {
          hooks.insert("ROOM-ACT-EXIT", currentRoom.hooks.exit, currentRoom);
        }
        if (player.hooks.exit) {
          hooks.insert("PLAYER-ACT-EXIT", player.hooks.exit, currentRoom);
        }

        currentRoom = rooms[destination.isRoom];

        if (currentRoom.hooks.enter) {
          hooks.insert("ROOM-ACT-ENTER", currentRoom.hooks.enter, currentRoom);
        }
        if (player.hooks.enter) {
          hooks.insert("PLAYER-ACT-ENTER", player.hooks.enter, currentRoom);
        }

        locals["currentRoom"] = currentRoom;
        return true;
      }
    } else if ("isObject" in thing) {
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
          hooks.insert("OBJ-ACT-ADD", objHooks.enterPlayer, currentRoom);
        }

        // update object location
        thing.loc = { scope: "player" };

        goodMove = true;
      } else if (
        "isRoom" in destination &&
        scope !== "room" &&
        name !== destination.isRoom
      ) {
        if (scope == "player" && objHooks.exitPlayer) {
          hooks.insert("OBJ-ACT-REMOVE", objHooks.exitPlayer, currentRoom);
        }

        // update object location
        thing.loc = { scope: "room", name: destination.isRoom };

        goodMove = true;
      } else if (
        "isObject" in destination &&
        scope !== "object" &&
        name !== destination.isObject &&
        inst !== destination.isInst
      ) {
        if (scope == "player" && objHooks.exitPlayer) {
          hooks.insert("OBJ-ACT-REMOVE", objHooks.exitPlayer, currentRoom);
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
        oldLocation.objects = Object.keys(oldLocation.objects).reduce(
          (acc, name) => {
            let list = oldLocation.objects[name];
            if (name === thing.isObject) {
              list = oldLocation.objects[name].filter(
                (inst) => inst !== thing.isInst
              );
            }

            if (list.length > 0) {
              return { ...acc, [name]: list };
            } else {
              return acc;
            }
          },
          {}
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

  function getInst(scope, objName, nested = true) {
    if (
      !scope ||
      !objName ||
      typeof scope !== "object" ||
      typeof objName !== "string"
    ) {
      return null;
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

    return null;
  }

  return {
    log,

    describe,

    start() {
      if (currentRoom.hooks.enter) {
        hooks.insert("ROOM-ACT-ENTER", currentRoom.hooks.enter, currentRoom);
      }
      if (player.hooks.enter) {
        hooks.insert("PLAYER-ACT-ENTER", player.hooks.enter, currentRoom);
      }

      hooks.callAll();
    },

    setLogger(newLogger) {
      logger = newLogger;
    },

    handleGo,

    handleRawInput(input) {
      let handled = false;
      let result = parseInput(input);
      command = result;

      if ("move" in result) {
        handled = handleGo(result.move);
      }

      if ("routine" in result) {
        handled = true;

        if (command.prsi.val) {
          const objHooks = objects[command.prsi.val.isObject].hooks;
          if (objHooks.prsi) {
            hooks.insert("OBJ-ACT-PRSI", objHooks.prsi, currentRoom);
          }
        }
        if (command.prso.val) {
          const objHooks = objects[command.prso.val.isObject].hooks;
          if (objHooks.prso) {
            hooks.insert("OBJ-ACT-PRSO", objHooks.prso, currentRoom);
          }
        }

        hooks.insert("SYNTAX-ACTION", command.routine, currentRoom);

        hooks.callAll();
      }

      for (const objectName of Object.keys(currentRoom.objects)) {
        let nested = [objectName];
        while (nested.length > 0) {
          const object = objects[nested.shift()];
          if (object.hooks.inRoom) {
            hooks.insert("OBJ-ACT-IN-ROOM", object.hooks.inRoom, currentRoom);
          }

          const children = [
            ...new Set(
              Object.values(object.copies)
                .map((copy) => Object.keys(copy.objects))
                .flat()
            ),
          ];

          nested.push(...children);
        }
      }

      for (const objectName of Object.keys(player.objects)) {
        let nested = [objectName];
        while (nested.length > 0) {
          const object = objects[nested.shift()];
          if (object.hooks.inPlayer) {
            hooks.insert(
              "OBJ-ACT-IN-PLAYER",
              object.hooks.inPlayer,
              currentRoom
            );
          }

          const children = [
            ...new Set(
              Object.values(object.copies)
                .map((copy) => Object.keys(copy.objects))
                .flat()
            ),
          ];

          nested.push(...children);
        }
      }

      hooks.callAll();

      if (currentRoom.hooks.always) {
        hooks.insert("ROOM-ACT-ALWAYS", currentRoom.hooks.always, currentRoom);
      }
      if (player.hooks.always) {
        hooks.insert("PLAYER-ACT-ALWAYS", player.hooks.always, currentRoom);
      }

      hooks.callAll();

      return {
        handled,
        gameOver: false,
        prsa: result.prsa,
        prso: (result.prso && result.prso.word) ?? null,
        foundPrso: result.prso && result.prso.val ? true : false,
        prsi: (result.prsi && result.prsi.word) ?? null,
        foundPrsi: result.prsi && result.prsi.val ? true : false,
        goDirection: result.move ?? null,
      };
    },

    getRoutineCommandArgs() {
      const emptyObject = {
        isObject: null,
        objects: [],
        loc: "player",
        vars: {},
        hooks: {},
      };

      const { prsa, prso, prsi } = command;
      return [
        prsa,
        prso ? prso.val : emptyObject,
        prsi ? prsi.val : emptyObject,
      ];
    },

    getVariablesOf(obj) {
      if (!obj || !("vars" in obj)) {
        return [];
      }

      return Object.keys(obj.vars).map((name) => ({
        name,
        val: obj.vars[name],
      }));
    },

    getObjectsIn(something) {
      if (!something || typeof something !== "object") {
        return [];
      }

      return Object.keys(something.objects)
        .map((name) => {
          return something.objects[name].map(
            (inst) => objects[name].copies[inst]
          );
        })
        .flat();
    },

    findObjectMatchingParsedWord(word, params) {
      const emptyResult = { objectNum: 0, objectVal: null };
      if (!word) {
        return emptyResult;
      }

      // matches is a string[]
      const matches = lookupObjectNamesBySynonym(word);
      if (!matches || matches.length === 0) {
        return emptyResult;
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

      addInstances(currentRoom.objects);
      addInstances(player.objects);

      // objectsMatching is an object[]
      const objectsMatching = objectsAccessible.filter((obj) =>
        matches.includes(obj.isObject)
      );

      for (let i = 0; i < params.length; i++) {
        const param = params[i];

        let goodObject = objectsMatching.find((obj) =>
          param.withVars.every(
            (varName) => varName in obj.vars && obj.vars[varName] !== 0
          )
        );

        if (goodObject) {
          return { objectNum: i + 1, objectVal: goodObject };
        }
      }

      return emptyResult;
    },

    getLocation(thing) {
      if (!thing || typeof thing !== "object") {
        // fallback to player
        return player;
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
          return getInst(container, thing.isObject, nested) !== null;
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

    copyMove(locals, thing, destination) {
      if (!thing || typeof thing !== "object") {
        return null;
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

      return move(locals, copy, destination);
    },

    getInst,

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
  };
};

export const game = newGame();
