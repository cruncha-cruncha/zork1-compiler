import { rooms } from "./rooms.js";
import { player } from "./globals.js";
import {
  objects,
  translateSynonym as lookupObjectNamesBySynonym,
} from "./objects.js";
import { parseInput } from "./parser.js";
import { routines, handlers } from "./routines.js";
import { newHooks } from "./hooks.js";

export const getEmptyResource = () => ({
  isEmpty: true,
  objects: [],
  loc: {},
  vars: {},
  hooks: {},
});

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
  let command = { prsa: "", cmds: [] };

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
      hooks.callDescription(routines[desc.routine], currentRoom, [
        getEmptyResource(),
        ...command.cmds,
      ]);
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

      if (currentRoom.hooks.enter) {
        hooks.insert(
          "ROOM-ACT-ENTER",
          routines[currentRoom.hooks.enter],
          currentRoom,
          command.cmds
        );
      }
      if (player.hooks.enter) {
        hooks.insert(
          "PLAYER-ACT-ENTER",
          routines[player.hooks.enter],
          currentRoom,
          command.cmds
        );
      }
    } else if ("routine" in result) {
      hooks.insert(
        "SYNTAX-ACTION",
        routines[result.routine],
        currentRoom,
        command.cmds
      );
    }

    return true;
  }

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

      for (const copy in delCopies) {
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
    }

    if ("isPlayer" in thing) {
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

        if (currentRoom.hooks.enter) {
          hooks.insert(
            "ROOM-ACT-ENTER",
            routines[currentRoom.hooks.enter],
            currentRoom,
            command.cmds
          );
        }
        if (player.hooks.enter) {
          hooks.insert(
            "PLAYER-ACT-ENTER",
            routines[player.hooks.enter],
            currentRoom,
            command.cmds
          );
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
          hooks.insert(
            "OBJ-ACT-ADD",
            objHooks.enterPlayer,
            currentRoom,
            command.cmds
          );
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
          hooks.insert(
            "OBJ-ACT-REMOVE",
            objHooks.exitPlayer,
            currentRoom,
            command.cmds
          );
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
          hooks.insert(
            "OBJ-ACT-REMOVE",
            objHooks.exitPlayer,
            currentRoom,
            command.cmds
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

  function getInst(scope, objName, nested = true) {
    if (
      !scope ||
      !objName ||
      typeof scope !== "object" ||
      typeof objName !== "string"
    ) {
      console.error("Bad argument passed to engine.getInst", {
        scope,
        objName,
      });
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

  return {
    log,

    describe,

    start() {
      if (currentRoom.hooks.enter) {
        hooks.insert(
          "ROOM-ACT-ENTER",
          routines[currentRoom.hooks.enter],
          currentRoom,
          command.cmds
        );
      }
      if (player.hooks.enter) {
        hooks.insert(
          "PLAYER-ACT-ENTER",
          routines[player.hooks.enter],
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

    handleRawInput(input) {
      let handled = false;
      let result = parseInput(input);
      command = result;
      command.cmds = result.cmds.map((cmd) =>
        cmd.val ? cmd.val : getEmptyResource()
      );

      if ("move" in result) {
        handled = handleGo(result.move);
      }

      if ("handle" in result) {
        handled = true;

        // srh = syntax routine handlers
        const srh = handlers[result.prsa];

        for (const inst of command.cmds) {
          console.log("inst", inst);
          if (inst.isObject in srh.objHandlers) {
            const objHandle = srh.objHandlers[inst.isObject];
            console.log("objHandle", objHandle);
            if ("before" in objHandle) {
              console.log("before", objHandle.before);
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

        for (const inst of command.cmds) {
          if (inst.isObject in srh.objHandlers) {
            const objHandle = srh.objHandlers[inst.isObject];
            if ("after" in objHandle) {
              hooks.insert(
                "AFTER-ACTION",
                objHandle.before,
                currentRoom,
                command.cmds
              );
            }
          }
        }

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
            command.cmds
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
            command.cmds
          );
        }

        const children = Object.keys(inst.objects)
          .map((name) =>
            inst.objects[name].map((inst) => objects[name].copies[inst])
          )
          .flat();

        nested.push(...children);
      }

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

      hooks.callAll();

      return {
        handled,
        syntax: {
          prsa: result.prsa,
          cmds: result.cmds.map((cmd) => ({
            word: cmd.word,
            hasVal: !!cmd.val,
          })),
        },
        goDirection: result.move ?? null,
        playerVars: { ...player.vars },
      };
    },

    getSyntaxCmd() {
      const { cmds } = command;
      return cmds.map((cmd) => (cmd.val ? cmd.val : getEmptyResource()));
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

      if ("isObject" in something && !("isInst" in something)) {
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

    findObjectMatchingParsedWord(word, params) {
      if (!word) {
        return getEmptyResource();
      }

      // matches is a string[]
      const matches = lookupObjectNamesBySynonym(word);
      if (!matches || matches.length === 0) {
        return getEmptyResource();
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

      return getEmptyResource();
    },

    getParent(thing) {
      if (!thing || typeof thing !== "object") {
        console.error("Bad argument passed to engine.getParent", { thing });
        return getEmptyResource();
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

    copyMove(locals, thing, destination) {
      if (!thing || typeof thing !== "object") {
        console.error("Bad argument passed to engine.copyMove", { thing });
        return getEmptyResource();
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
