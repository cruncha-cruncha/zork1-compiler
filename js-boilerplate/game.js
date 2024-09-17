import { rooms } from "./rooms.js";
import { player } from "./globals.js";
import {
  objects,
  translateSynonym as lookupObjectNamesBySynonym,
} from "./objects.js";
import { parseInput } from "./parser.js";
import { newHooks } from "./hooks.js";

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

  function describe(location) {
    if ("text" in location.desc) {
      log(location.desc.text);
    } else if ("routine" in location.desc) {
      hooks.call(location.desc.routine, currentRoom);
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

        if (command.prsi.val && command.prsi.val.hooks.prsi) {
          hooks.insert(
            "OBJ-ACT-PRSI",
            command.prsi.val.hooks.prsi,
            currentRoom
          );
        }
        if (command.prso.val && command.prso.val.hooks.prso) {
          hooks.insert(
            "OBJ-ACT-PRSO",
            command.prso.val.hooks.prso,
            currentRoom
          );
        }

        hooks.insert("SYNTAX-ACTION", command.routine, currentRoom);

        hooks.callAll();
      }

      for (const objectName of currentRoom.objects) {
        let nestedNames = [objectName];
        while (nestedNames.length > 0) {
          const object = objects[nestedNames.shift()];
          if (object.hooks.inRoom) {
            hooks.insert("OBJ-ACT-IN-ROOM", object.hooks.inRoom, currentRoom);
          }

          nestedNames.push(...object.objects);
        }
      }

      for (const objectName of player.objects) {
        let nestedNames = [objectName];
        while (nestedNames.length > 0) {
          const object = objects[nestedNames.shift()];
          if (object.hooks.inPlayer) {
            hooks.insert(
              "OBJ-ACT-IN-PLAYER",
              object.hooks.inPlayer,
              currentRoom
            );
          }

          nestedNames.push(...object.objects);
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
      if (!something || !("objects" in something)) {
        return [];
      }

      return something.objects.map((objName) => objects[objName]);
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

      const objectsAccessible = currentRoom.objects.map(
        (name) => objects[name]
      );

      // TODO: allow nested objects if in player inventory?
      objectsAccessible.push(...player.objects.map((name) => objects[name]));

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
        const name = thing.loc;

        if (name in objects) {
          return objects[name];
        }

        if (name in rooms) {
          return rooms[name];
        }
      }

      return player;
    },

    isInLocation(routineRoom, container, thing, nested) {
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
          return routineRoom.isRoom === container.isRoom;
        }
      } else if ("isRoom" in thing) {
        if ("isRoom" in container) {
          return thing.isRoom === container.isRoom && !nested;
        }
      } else if ("isObject" in thing) {
        let containerKey = "";
        if ("isPlayer" in container) {
          containerKey = "player";
        } else if ("isRoom" in container) {
          containerKey = container.isRoom;
        } else if ("isObject" in container) {
          containerKey = container.isObject;
        }

        if (!nested) {
          return thing.isObject === containerKey || thing.loc === containerKey;
        }

        let parent = thing.isObject;
        while (true) {
          if (parent in objects) {
            parent = objects[parent].loc;
          } else {
            break;
          }

          if (parent === containerKey) {
            return true;
          }
        }
      }

      return false;
    },

    move(locals, thing, destination) {
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
            hooks.insert(
              "ROOM-ACT-ENTER",
              currentRoom.hooks.enter,
              currentRoom
            );
          }
          if (player.hooks.enter) {
            hooks.insert("PLAYER-ACT-ENTER", player.hooks.enter, currentRoom);
          }

          locals["currentRoom"] = currentRoom;
          return true;
        }
      } else if ("isObject" in thing) {
        const oldLocation = (() => {
          if (thing.loc == "player") {
            return player;
          } else if (thing.loc in objects) {
            return objects[thing.loc];
          } else if (thing.loc in rooms) {
            return rooms[thing.loc];
          }
        })();

        if ("isPlayer" in destination && thing.loc !== "player") {
          if (thing.hooks.enterPlayer) {
            hooks.insert("OBJ-ACT-ADD", thing.hooks.enterPlayer, currentRoom);
          }
          oldLocation.objects = oldLocation.objects.filter(
            (name) => name !== thing.isObject
          );
          player.objects.push(thing.isObject);
          thing.loc = "player";
          return true;
        } else if (
          "isObject" in destination &&
          thing.loc !== destination.isObject
        ) {
          if (thing.loc == "player" && thing.hooks.exitPlayer) {
            hooks.insert("OBJ-ACT-REMOVE", thing.hooks.exitPlayer, currentRoom);
          }
          oldLocation.objects = oldLocation.objects.filter(
            (name) => name !== thing.isObject
          );
          destination.objects.push(thing.isObject);
          thing.loc = destination.isObject;
          return true;
        } else if (
          "isRoom" in destination &&
          thing.loc !== destination.isRoom
        ) {
          if (thing.loc == "player" && thing.hooks.exitPlayer) {
            hooks.insert("OBJ-ACT-REMOVE", thing.hooks.exitPlayer, currentRoom);
          }
          oldLocation.objects = oldLocation.objects.filter(
            (name) => name !== thing.isObject
          );
          destination.objects.push(thing.isObject);
          thing.loc = destination.isRoom;
          return true;
        }
      }

      return false;
    },
  };
};

export const game = newGame();
