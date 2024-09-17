import { game } from './game.js';

export const buzz = ["A", "THE", "AGAIN", "SOME", "OF", "ALL"];

export const directions = ["DOWN", "EAST", "IN", "NORTH", "OUT", "SOUTH", "UP", "WEST"];

export const parseInput = (rawString) => {
  const words = rawString.split(" ").map(w => w.toUpperCase()).filter(w => !buzz.includes(w));
  if ((words.length == 2) && (words[0] == "GO")) {
    return { move: words[1], prsa: translateAction(words[0]) };
  }

  const prsa = translateAction(words[0]);
  let prso = {};
  let prsi = {};

  switch (words[0]) {
  case "WHERE":
    switch (words[1]) {
    case "AM":
      switch (words[2]) {
      case "I":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {routine: 'vRoomDetail', prsa, prso, prsi };
          }
          return { prsa, prso, prsi };
        }
      default:
        return { prsa, prso, prsi };
      }
    case "CAN":
      switch (words[2]) {
      case "I":
        switch (words[3]) {
        case "GO":
          switch (words[4]) {
          default:
            if (words.length == 4) {
              return {routine: 'vWhereToGo', prsa, prso, prsi };
            }
            return { prsa, prso, prsi };
          }
        default:
          return { prsa, prso, prsi };
        }
      default:
        return { prsa, prso, prsi };
      }
    default:
      return { prsa, prso, prsi };
    }
  case "LOOK":
    switch (words[1]) {
    case "AROUND":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {routine: 'vDescObjectsInRoom', prsa, prso, prsi };
        }
        return { prsa, prso, prsi };
      }
    default:
      return { prsa, prso, prsi };
    }
  case "INVENTORY":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {routine: 'vInventory', prsa, prso, prsi };
      }
      return { prsa, prso, prsi };
    }
  case "WEATHER":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {routine: 'weatherReport', prsa, prso, prsi };
      }
      return { prsa, prso, prsi };
    }
  case "SLEEP":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {routine: 'vSleep', prsa, prso, prsi };
      }
      return { prsa, prso, prsi };
    }
  case "EXAMINE":
  case "INSPECT":
  case "READ":
  case "INVESTIGATE":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: []}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {routine: 'vExamine', prsa, prso, prsi };
          }
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "TAKE":
  case "GATHER":
  case "GET":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: []}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {routine: 'vTake', prsa, prso, prsi };
          }
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "UNPACK":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: []}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {routine: 'vTakeOut', prsa, prso, prsi };
          }
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "DROP":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: []}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {routine: 'vDrop', prsa, prso, prsi };
          }
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "PUT":
  case "PLACE":
  case "POUR":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: ["canContain"]}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        case "INTO":
          switch (words[3]) {
          default:
            const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[3], [{withVars: []}]);
            if (prso.val && !prsi.val) { prsi = { word: words[3], val: objectVal }; } else if (!prso.val) { prso = { word: words[3], val: objectVal }; }
            switch (objectNum) {
            case 1:
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {routine: 'vPutIn', prsa, prso, prsi };
                }
                return { prsa, prso, prsi };
              }
            }
            return { prsa, prso, prsi };
          }
        default:
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "FILL":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: ["canContain"]}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        case "WITH":
          switch (words[3]) {
          default:
            const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[3], [{withVars: []}]);
            if (prso.val && !prsi.val) { prsi = { word: words[3], val: objectVal }; } else if (!prso.val) { prso = { word: words[3], val: objectVal }; }
            switch (objectNum) {
            case 1:
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {routine: 'vPutIn', prsa, prso, prsi };
                }
                return { prsa, prso, prsi };
              }
            }
            return { prsa, prso, prsi };
          }
        default:
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "OPEN":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: []}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {routine: 'vOpen', prsa, prso, prsi };
          }
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "CLOSE":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: []}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {routine: 'vClose', prsa, prso, prsi };
          }
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "HIT":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: []}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        case "WITH":
          switch (words[3]) {
          default:
            const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[3], [{withVars: ["maxDamage"]}]);
            if (prso.val && !prsi.val) { prsi = { word: words[3], val: objectVal }; } else if (!prso.val) { prso = { word: words[3], val: objectVal }; }
            switch (objectNum) {
            case 1:
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {routine: 'vHitWith', prsa, prso, prsi };
                }
                return { prsa, prso, prsi };
              }
            }
            return { prsa, prso, prsi };
          }
        default:
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "COOK":
  case "BOIL":
  case "ROAST":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: []}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {routine: 'vCook', prsa, prso, prsi };
          }
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "EAT":
  case "TASTE":
  case "LICK":
  case "DRINK":
  case "IMBIBE":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: ["edible"]}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {routine: 'vEat', prsa, prso, prsi };
          }
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "WORK":
    switch (words[1]) {
    default:
      const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[1], [{withVars: []}]);
      if (prso.val && !prsi.val) { prsi = { word: words[1], val: objectVal }; } else if (!prso.val) { prso = { word: words[1], val: objectVal }; }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        case "WITH":
          switch (words[3]) {
          default:
            const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[3], [{withVars: ["maxHealth"]}]);
            if (prso.val && !prsi.val) { prsi = { word: words[3], val: objectVal }; } else if (!prso.val) { prso = { word: words[3], val: objectVal }; }
            switch (objectNum) {
            case 1:
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {routine: 'vWorkWith', prsa, prso, prsi };
                }
                return { prsa, prso, prsi };
              }
            }
            return { prsa, prso, prsi };
          }
        default:
          return { prsa, prso, prsi };
        }
      }
      return { prsa, prso, prsi };
    }
  case "SPARK":
    switch (words[1]) {
    case "FLINT":
      switch (words[2]) {
      case "AT":
        switch (words[3]) {
        default:
          const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[3], [{withVars: []}]);
          if (prso.val && !prsi.val) { prsi = { word: words[3], val: objectVal }; } else if (!prso.val) { prso = { word: words[3], val: objectVal }; }
          switch (objectNum) {
          case 1:
            switch (words[4]) {
            default:
              if (words.length == 4) {
                return {routine: 'vSparkAt', prsa, prso, prsi };
              }
              return { prsa, prso, prsi };
            }
          }
          return { prsa, prso, prsi };
        }
      default:
        return { prsa, prso, prsi };
      }
    default:
      return { prsa, prso, prsi };
    }
  case "TALK":
    switch (words[1]) {
    case "TO":
      switch (words[2]) {
      default:
        const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[2], [{withVars: []}]);
        if (prso.val && !prsi.val) { prsi = { word: words[2], val: objectVal }; } else if (!prso.val) { prso = { word: words[2], val: objectVal }; }
        switch (objectNum) {
        case 1:
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {routine: 'vTalkTo', prsa, prso, prsi };
            }
            return { prsa, prso, prsi };
          }
        }
        return { prsa, prso, prsi };
      }
    default:
      return { prsa, prso, prsi };
    }
  case "PEE":
    switch (words[1]) {
    case "ON":
      switch (words[2]) {
      default:
        const { objectNum, objectVal } = game.findObjectMatchingParsedWord(words[2], [{withVars: []}]);
        if (prso.val && !prsi.val) { prsi = { word: words[2], val: objectVal }; } else if (!prso.val) { prso = { word: words[2], val: objectVal }; }
        switch (objectNum) {
        case 1:
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {routine: 'vPeeOn', prsa, prso, prsi };
            }
            return { prsa, prso, prsi };
          }
        }
        return { prsa, prso, prsi };
      }
    default:
      return { prsa, prso, prsi };
    }
  case "WRITE":
    switch (words[1]) {
    case "NOTE":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {routine: 'vWriteNote', prsa, prso, prsi };
        }
        return { prsa, prso, prsi };
      }
    default:
      return { prsa, prso, prsi };
    }
  case "SWIM":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {routine: 'vSwim', prsa, prso, prsi };
      }
      return { prsa, prso, prsi };
    }
  case "DIVE":
    switch (words[1]) {
    case "IN":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {routine: 'vSwim', prsa, prso, prsi };
        }
        return { prsa, prso, prsi };
      }
    default:
      return { prsa, prso, prsi };
    }
  case "JUMP":
    switch (words[1]) {
    case "IN":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {routine: 'vJump', prsa, prso, prsi };
        }
        return { prsa, prso, prsi };
      }
    case "DOWN":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {routine: 'vJump', prsa, prso, prsi };
        }
        return { prsa, prso, prsi };
      }
    default:
      return { prsa, prso, prsi };
    }
  default:
    return { prsa, prso, prsi };
  }
}

export const translateAction = (actionWord) => {
  switch (actionWord) {
    case "WHERE":
      return "WHERE";
    case "LOOK":
      return "LOOK";
    case "INVENTORY":
      return "INVENTORY";
    case "WEATHER":
      return "WEATHER";
    case "SLEEP":
      return "SLEEP";
    case "EXAMINE":
    case "INSPECT":
    case "READ":
    case "INVESTIGATE":
      return "EXAMINE";
    case "TAKE":
    case "GATHER":
    case "GET":
      return "TAKE";
    case "UNPACK":
      return "UNPACK";
    case "DROP":
      return "DROP";
    case "PUT":
    case "PLACE":
    case "POUR":
      return "PUT";
    case "FILL":
      return "FILL";
    case "OPEN":
      return "OPEN";
    case "CLOSE":
      return "CLOSE";
    case "HIT":
      return "HIT";
    case "COOK":
    case "BOIL":
    case "ROAST":
      return "COOK";
    case "EAT":
    case "TASTE":
    case "LICK":
    case "DRINK":
    case "IMBIBE":
      return "EAT";
    case "WORK":
      return "WORK";
    case "SPARK":
      return "SPARK";
    case "TALK":
      return "TALK";
    case "PEE":
      return "PEE";
    case "WRITE":
      return "WRITE";
    case "SWIM":
      return "SWIM";
    case "DIVE":
      return "DIVE";
    case "JUMP":
      return "JUMP";
    default:
      return actionWord;
    }
  }
