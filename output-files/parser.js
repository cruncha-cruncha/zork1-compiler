import { game, getEmptyResource } from './engine.js';

export const buzz = ["A", "AGAIN", "ALL", "MY", "OF", "SOME", "THE"];

export const directions = ["DOWN", "EAST", "NORTH", "SOUTH", "UP", "WEST"];

export const parseInput = (rawString) => {
  if (!rawString || typeof rawString !== 'string') { return { prsa: '', cmds: [] }; }
  const words = rawString.split(" ").map(w => w.toUpperCase()).filter(w => !buzz.includes(w));
  if (words.length == 0) { return { prsa: '', cmds: [{}] }; }
  const prsa = translateAction(words[0]);
  let cmds = [{}];

  if ((words.length == 2) && (words[0] == "GO")) {
    return { move: words[1], prsa, cmds: [] };
  }

  switch (words[0]) {
  case "WHERE":
    switch (words[1]) {
    case "AM":
      switch (words[2]) {
      case "I":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {handle: '', prsa, cmds };
          }
          return { prsa, cmds };
        }
      default:
        return { prsa, cmds };
      }
    case "CAN":
      switch (words[2]) {
      case "I":
        switch (words[3]) {
        case "GO":
          switch (words[4]) {
          default:
            if (words.length == 4) {
              return {handle: '', prsa, cmds };
            }
            return { prsa, cmds };
          }
        default:
          return { prsa, cmds };
        }
      default:
        return { prsa, cmds };
      }
    default:
      return { prsa, cmds };
    }
  case "WHAT":
    switch (words[1]) {
    case "IS":
      switch (words[2]) {
      case "HERE":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {handle: '', prsa, cmds };
          }
          return { prsa, cmds };
        }
      default:
        return { prsa, cmds };
      }
    case "CAN":
      switch (words[2]) {
      case "I":
        switch (words[3]) {
        case "DO":
          switch (words[4]) {
          default:
            if (words.length == 4) {
              return {handle: '', prsa, cmds };
            }
            return { prsa, cmds };
          }
        default:
          return { prsa, cmds };
        }
      default:
        return { prsa, cmds };
      }
    default:
      return { prsa, cmds };
    }
  case "LOOK":
    switch (words[1]) {
    case "AROUND":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa, cmds };
        }
        return { prsa, cmds };
      }
    default:
      return { prsa, cmds };
    }
  case "INVENTORY":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {handle: '', prsa, cmds };
      }
      return { prsa, cmds };
    }
  case "WEATHER":
    switch (words[1]) {
    case "REPORT":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa, cmds };
        }
        return { prsa, cmds };
      }
    default:
      if (words.length == 1) {
        return {handle: '', prsa, cmds };
      }
      return { prsa, cmds };
    }
  case "SLEEP":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {handle: '', prsa, cmds };
      }
      return { prsa, cmds };
    }
  case "EXAMINE":
  case "INSPECT":
  case "READ":
  case "INVESTIGATE":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa, cmds };
          }
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "TAKE":
  case "GATHER":
  case "GET":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa, cmds };
          }
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "DROP":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa, cmds };
          }
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "EMPTY":
  case "UNPACK":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa, cmds };
          }
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "ADD":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        case "TO":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
            if (objectVal) {
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa, cmds };
                }
                return { prsa, cmds };
              }
            }
            return { prsa, cmds };
          }
        default:
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "PUT":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        case "IN":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
            if (objectVal) {
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa, cmds };
                }
                return { prsa, cmds };
              }
            }
            return { prsa, cmds };
          }
        default:
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "FILL":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        case "WITH":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
            if (objectVal) {
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa, cmds };
                }
                return { prsa, cmds };
              }
            }
            return { prsa, cmds };
          }
        default:
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "POUR":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        case "ON":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
            if (objectVal) {
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa, cmds };
                }
                return { prsa, cmds };
              }
            }
            return { prsa, cmds };
          }
        default:
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "HIT":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        case "WITH":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
            if (objectVal) {
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa, cmds };
                }
                return { prsa, cmds };
              }
            }
            return { prsa, cmds };
          }
        default:
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "WORK":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        case "WITH":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
            if (objectVal) {
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa, cmds };
                }
                return { prsa, cmds };
              }
            }
            return { prsa, cmds };
          }
        default:
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "EAT":
  case "TASTE":
  case "LICK":
  case "DRINK":
  case "IMBIBE":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa, cmds };
          }
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "OPEN":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa, cmds };
          }
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "SPARK":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
      if (objectVal) {
        switch (words[2]) {
        case "AT":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
            if (objectVal) {
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa, cmds };
                }
                return { prsa, cmds };
              }
            }
            return { prsa, cmds };
          }
        default:
          return { prsa, cmds };
        }
      }
      return { prsa, cmds };
    }
  case "TALK":
    switch (words[1]) {
    case "TO":
      switch (words[2]) {
      default:
        const { objectVal } = game.findObjectMatchingParsedWord(words[2]);
        cmds.push({ word: words[2], val: objectVal });
        if (objectVal) {
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {handle: '', prsa, cmds };
            }
            return { prsa, cmds };
          }
        }
        return { prsa, cmds };
      }
    default:
      return { prsa, cmds };
    }
  case "PEE":
    switch (words[1]) {
    case "ON":
      switch (words[2]) {
      default:
        const { objectVal } = game.findObjectMatchingParsedWord(words[2]);
        cmds.push({ word: words[2], val: objectVal });
        if (objectVal) {
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {handle: '', prsa, cmds };
            }
            return { prsa, cmds };
          }
        }
        return { prsa, cmds };
      }
    default:
      return { prsa, cmds };
    }
  case "WRITE":
    switch (words[1]) {
    case "NOTE":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa, cmds };
        }
        return { prsa, cmds };
      }
    default:
      return { prsa, cmds };
    }
  case "SWIM":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {handle: '', prsa, cmds };
      }
      return { prsa, cmds };
    }
  case "JUMP":
    switch (words[1]) {
    case "IN":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa, cmds };
        }
        return { prsa, cmds };
      }
    case "DOWN":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa, cmds };
        }
        return { prsa, cmds };
      }
    default:
      return { prsa, cmds };
    }
  default:
    return { prsa, cmds };
  }
}

export const translateAction = (actionWord) => {
  switch (actionWord) {
    case "WHERE":
      return "where";
    case "WHAT":
      return "what";
    case "LOOK":
      return "look";
    case "INVENTORY":
      return "inventory";
    case "WEATHER":
      return "weather";
    case "SLEEP":
      return "sleep";
    case "EXAMINE":
    case "INSPECT":
    case "READ":
    case "INVESTIGATE":
      return "examine";
    case "TAKE":
    case "GATHER":
    case "GET":
      return "take";
    case "DROP":
      return "drop";
    case "EMPTY":
    case "UNPACK":
      return "empty";
    case "ADD":
      return "add";
    case "PUT":
      return "put";
    case "FILL":
      return "fill";
    case "POUR":
      return "pour";
    case "HIT":
      return "hit";
    case "WORK":
      return "work";
    case "EAT":
    case "TASTE":
    case "LICK":
    case "DRINK":
    case "IMBIBE":
      return "eat";
    case "OPEN":
      return "open";
    case "SPARK":
      return "spark";
    case "TALK":
      return "talk";
    case "PEE":
      return "pee";
    case "WRITE":
      return "write";
    case "SWIM":
      return "swim";
    case "JUMP":
      return "jump";
    default:
      return actionWord;
    }
  }
