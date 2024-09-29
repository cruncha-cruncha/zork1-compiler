import { game, getEmptyResource } from './engine.js';

export const buzz = ["A", "AGAIN", "ALL", "MY", "OF", "SOME", "THE"];

export const directions = ["DOWN", "EAST", "NORTH", "SOUTH", "UP", "WEST"];

export const parseInput = (rawString) => {
  if (!rawString || typeof rawString !== 'string') { return { prsa: '', cmds: [] }; }
  const words = rawString.split(" ").map(w => w.toUpperCase()).filter(w => !!w && !buzz.includes(w));
  if (words.length == 0) { return { prsa: '', cmds: [{}] }; }
  let cmds = [{}];

  if ((words.length == 2) && (words[0] == "GO")) {
    return { move: words[1], prsa: 'GO', cmds: [] };
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
            return {handle: '', prsa: 'where', cmds };
          }
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    case "CAN":
      switch (words[2]) {
      case "I":
        switch (words[3]) {
        case "GO":
          switch (words[4]) {
          default:
            if (words.length == 4) {
              return {handle: '', prsa: 'where', cmds };
            }
            return { prsa: '', cmds };
          }
        default:
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "WHAT":
    switch (words[1]) {
    case "CAN":
      switch (words[2]) {
      case "I":
        switch (words[3]) {
        case "DO":
          switch (words[4]) {
          default:
            if (words.length == 4) {
              return {handle: '', prsa: 'where', cmds };
            }
            return { prsa: '', cmds };
          }
        default:
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    case "IS":
      switch (words[2]) {
      case "HERE":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {handle: '', prsa: 'what', cmds };
          }
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "TIME":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {handle: '', prsa: 'time', cmds };
      }
      return { prsa: '', cmds };
    }
  case "LOOK":
    switch (words[1]) {
    case "AROUND":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa: 'look', cmds };
        }
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "INVENTORY":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {handle: '', prsa: 'inventory', cmds };
      }
      return { prsa: '', cmds };
    }
  case "WEATHER":
    switch (words[1]) {
    case "REPORT":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa: 'weather', cmds };
        }
        return { prsa: '', cmds };
      }
    default:
      if (words.length == 1) {
        return {handle: '', prsa: 'weather', cmds };
      }
      return { prsa: '', cmds };
    }
  case "SLEEP":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {handle: '', prsa: 'sleep', cmds };
      }
      return { prsa: '', cmds };
    }
  case "CHEAT":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa: 'cheat', cmds };
          }
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "DEBUG":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {handle: '', prsa: 'debug', cmds };
      }
      return { prsa: '', cmds };
    }
  case "EXAMINE":
  case "INSPECT":
  case "READ":
  case "INVESTIGATE":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa: 'examine', cmds };
          }
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "TAKE":
  case "GATHER":
  case "GET":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa: 'take', cmds };
          }
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "DROP":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa: 'drop', cmds };
          }
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "EMPTY":
  case "UNPACK":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa: 'empty', cmds };
          }
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "ADD":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        case "TO":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa: 'add', cmds };
                }
                return { prsa: '', cmds };
              }
            return { prsa: '', cmds };
          }
        default:
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "PUT":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        case "IN":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa: 'add', cmds };
                }
                return { prsa: '', cmds };
              }
            return { prsa: '', cmds };
          }
        default:
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "HIT":
  case "SMASH":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        case "WITH":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa: 'hit', cmds };
                }
                return { prsa: '', cmds };
              }
            return { prsa: '', cmds };
          }
        default:
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "WORK":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        case "WITH":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa: 'work', cmds };
                }
                return { prsa: '', cmds };
              }
            return { prsa: '', cmds };
          }
        default:
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
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
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa: 'eat', cmds };
          }
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "OPEN":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {handle: '', prsa: 'open', cmds };
          }
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "SPARK":
    switch (words[1]) {
    default:
      const { objectVal } = game.findObjectMatchingParsedWord(words[1]);
      cmds.push({ word: words[1], val: objectVal });
        switch (words[2]) {
        case "AT":
          switch (words[3]) {
          default:
            const { objectVal } = game.findObjectMatchingParsedWord(words[3]);
            cmds.push({ word: words[3], val: objectVal });
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {handle: '', prsa: 'spark', cmds };
                }
                return { prsa: '', cmds };
              }
            return { prsa: '', cmds };
          }
        default:
          return { prsa: '', cmds };
        }
      return { prsa: '', cmds };
    }
  case "TALK":
    switch (words[1]) {
    case "TO":
      switch (words[2]) {
      default:
        const { objectVal } = game.findObjectMatchingParsedWord(words[2]);
        cmds.push({ word: words[2], val: objectVal });
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {handle: '', prsa: 'talk', cmds };
            }
            return { prsa: '', cmds };
          }
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "PEE":
    switch (words[1]) {
    case "ON":
      switch (words[2]) {
      default:
        const { objectVal } = game.findObjectMatchingParsedWord(words[2]);
        cmds.push({ word: words[2], val: objectVal });
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {handle: '', prsa: 'pee', cmds };
            }
            return { prsa: '', cmds };
          }
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "WRITE":
    switch (words[1]) {
    case "NOTE":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa: 'write', cmds };
        }
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "ENTER":
    switch (words[1]) {
    case "CABIN":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa: 'enter', cmds };
        }
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "EXIT":
    switch (words[1]) {
    case "CABIN":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa: 'exit', cmds };
        }
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "SWIM":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {handle: '', prsa: 'swim', cmds };
      }
      return { prsa: '', cmds };
    }
  case "JUMP":
    switch (words[1]) {
    case "IN":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa: 'jump', cmds };
        }
        return { prsa: '', cmds };
      }
    case "DOWN":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {handle: '', prsa: 'jump', cmds };
        }
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "GREAT":
    switch (words[1]) {
    case "BALL":
      switch (words[2]) {
      case "FIRE":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {handle: '', prsa: 'great', cmds };
          }
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "EUNICE":
    switch (words[1]) {
    case "BROKE":
      switch (words[2]) {
      case "HEART":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {handle: '', prsa: 'eunice', cmds };
          }
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "SCHOOL":
    switch (words[1]) {
    case "IS":
      switch (words[2]) {
      case "FOR":
        switch (words[3]) {
        case "ME":
          switch (words[4]) {
          default:
            if (words.length == 4) {
              return {handle: '', prsa: 'school', cmds };
            }
            return { prsa: '', cmds };
          }
        default:
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "BUG":
    switch (words[1]) {
    case "THAT":
      switch (words[2]) {
      case "BIG":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {handle: '', prsa: 'bug', cmds };
          }
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "EVER":
    switch (words[1]) {
    case "PLAY":
      switch (words[2]) {
      case "LOTTERY":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {handle: '', prsa: 'ever', cmds };
          }
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "LISTENING":
    switch (words[1]) {
    case "TO":
      switch (words[2]) {
      case "RIVER":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {handle: '', prsa: 'listening', cmds };
          }
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "GROWING":
    switch (words[1]) {
    case "UP":
      switch (words[2]) {
      case "BACK":
        switch (words[3]) {
        case "HOME":
          switch (words[4]) {
          default:
            if (words.length == 4) {
              return {handle: '', prsa: 'growing', cmds };
            }
            return { prsa: '', cmds };
          }
        default:
          return { prsa: '', cmds };
        }
      default:
        return { prsa: '', cmds };
      }
    default:
      return { prsa: '', cmds };
    }
  case "YES":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {handle: '', prsa: 'yes', cmds };
      }
      return { prsa: '', cmds };
    }
  case "NO":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {handle: '', prsa: 'no', cmds };
      }
      return { prsa: '', cmds };
    }
  default:
    return { prsa: '', cmds };
  }
}
