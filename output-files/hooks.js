const getPriority = (hookType) => {
  // higher number = higher priority = called earlier
  // adding a hook of priority N will remove all hooks of priority N-1 or lower
  switch (hookType) {
    case "BEFORE-ACTION":
      return 13;
    case "GO-ACTION":
    case "SYNTAX-ACTION":
      return 12;
    case "AFTER-ACTION":
      return 11;
    // hooks 10 and 9 are only ever fired by <MOVE ... >
    case "OBJ-ACT-REMOVE":
      return 10;
    case "OBJ-ACT-ADD":
      return 9;
    // hooks 8, 7, 6, and 5 are fired by GO DIRECTION or <MOVE PLAYER ... >
    case "ROOM-ACT-EXIT":
      return 8;
    case "PLAYER-ACT-EXIT":
      return 7;
    case "ROOM-ACT-ENTER":
      return 6;
    case "PLAYER-ACT-ENTER":
      return 5;
    // hooks 4 and 3 are called with the currentRoom at this moment in time
    // they're fired at most once per object per command
    case "OBJ-ACT-IN-ROOM":
      return 4;
    case "OBJ-ACT-IN-PLAYER":
      return 3;
    // hooks 2 and 1 are called with the currentRoom at this moment in time
    // they're fired at most once per command
    case "ROOM-ACT-ALWAYS":
      return 2;
    case "PLAYER-ACT-ALWAYS":
      return 1;
    default:
      return 0;
  }
};

// a giant closure
export const newHooks = () => {
  // the hooks to be called
  // lower index = higher priority
  let buffer = [];

  // fire the next hook in the buffer (starts from 0)
  function callNext() {
    if (buffer.length === 0) {
      return null;
    }

    let next = buffer.shift();
    next.func(next.cRoom, next.cmds);

    return next;
  }

  return {
    // buffer a new hook to be called later
    // respects priority
    insert(hookType, func, cRoom, cmds) {
      const priority = getPriority(hookType);

      if (buffer.length === 0) {
        buffer.push({
          priority,
          func,
          cRoom,
          cmds,
        });
        return;
      }

      let low = 0;
      let high = buffer.length;
      let mid;

      while (low != high) {
        mid = Math.floor((low + high) / 2);
        if (buffer[mid].priority >= priority) {
          low = mid + 1;
        } else {
          high = mid;
        }
      }

      buffer.splice(low, buffer.length - low, {
        priority,
        func,
        cRoom,
        cmds,
      });
    },

    // aka immediately call this hook
    callDescription(func, cRoom, cmds) {
      func(cRoom, cmds);
    },

    callNext,

    // empty the buffer
    callAll() {
      while (buffer.length > 0) {
        callNext();
      }
    },

    length() {
      return buffer.length;
    },

    clear() {
      buffer = [];
    },
  };
};
