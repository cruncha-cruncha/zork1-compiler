import { lookupBySynonym as lookupObjectBySynonym } from "./objects.js";

const buzz = ["IS", "NO", ",", "ONE", "OOPS", "YES", "HERE", "G", "AN", "EXCEPT", "A", ".", "Y", "THE", "AGAIN", "OF", "ALL", "BUT", "THEN", "AND", "\""];

const findObjectMatchingParsedWord = (word, params) => {
  const object = lookupObjectBySynonym(word);
  if (!object) { return { objectNum: 0, objectVal: null }; }
  // TODO: verify object matches one of the params, and if so, which one
  return { objectNum: 1, objectVal: object };
}

export const parseInput = (rawString) => {
  const words = rawString.split(" ").map(w => w.toUpperCase()).filter(w => !buzz.includes(w));
  let prso = null;
  let prsi = null;
  
  switch (words[0]) {
  case "BURN":
  case "INCINERATE":
  case "IGNITE":
    switch (words[1]) {
    case "DOWN":
      switch (words[2]) {
      default:
        const { objectNum, objectVal } = findObjectMatchingParsedWord(words[2], [{find: "burnbit", restrictions: ["HELD", "CARRIED", "ON-GROUND", "IN-ROOM"]}]);
        if (objectVal) { if (prso && !prsi) { prsi = { word: words[2], val: objectVal }; } else if (!prso) { prso = { word: words[2], val: objectVal }; } }
        switch (objectNum) {
        case 1:
          switch (words[3]) {
          case "WITH":
            switch (words[4]) {
            default:
              const { objectNum, objectVal } = findObjectMatchingParsedWord(words[4], [{find: "flamebit", restrictions: ["HELD", "CARRIED", "ON-GROUND", "IN-ROOM", "HAVE"]}]);
              if (objectVal) { if (prso && !prsi) { prsi = { word: words[4], val: objectVal }; } else if (!prso) { prso = { word: words[4], val: objectVal }; } }
              switch (objectNum) {
              case 1:
                switch (words[5]) {
                default:
                  if (words.length == 5) {
                    return {action: {verb: "vBurn", pre: "preBurn"}, prsa: words[0], prso, prsi };
                  }
                  return null;
                }
              }
              return null;
            }
          default:
            return null;
          }
        }
        return null;
      }
    default:
      const { objectNum, objectVal } = findObjectMatchingParsedWord(words[1], [{find: "burnbit", restrictions: ["HELD", "CARRIED", "ON-GROUND", "IN-ROOM"]}]);
      if (objectVal) { if (prso && !prsi) { prsi = { word: words[1], val: objectVal }; } else if (!prso) { prso = { word: words[1], val: objectVal }; } }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        case "WITH":
          switch (words[3]) {
          default:
            const { objectNum, objectVal } = findObjectMatchingParsedWord(words[3], [{find: "flamebit", restrictions: ["HELD", "CARRIED", "ON-GROUND", "IN-ROOM", "HAVE"]}]);
            if (objectVal) { if (prso && !prsi) { prsi = { word: words[3], val: objectVal }; } else if (!prso) { prso = { word: words[3], val: objectVal }; } }
            switch (objectNum) {
            case 1:
              switch (words[4]) {
              default:
                if (words.length == 4) {
                  return {action: {verb: "vBurn", pre: "preBurn"}, prsa: words[0], prso, prsi };
                }
                return null;
              }
            }
            return null;
          }
        default:
          return null;
        }
      }
      return null;
    }
  case "CHOMP":
  case "LOSE":
  case "BARF":
    switch (words[1]) {
    default:
      if (words.length == 1) {
        return {action: {verb: "vChomp"}, prsa: words[0], prso, prsi };
      }
      return null;
    }
  case "CLIMB":
  case "SIT":
    switch (words[1]) {
    case "UP":
      switch (words[2]) {
      default:
        const { objectNum, objectVal } = findObjectMatchingParsedWord(words[2], [{find: "rmungbit"}, {find: "climbbit", restrictions: ["ON-GROUND", "IN-ROOM"]}]);
        if (objectVal) { if (prso && !prsi) { prsi = { word: words[2], val: objectVal }; } else if (!prso) { prso = { word: words[2], val: objectVal }; } }
        switch (objectNum) {
        case 1:
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {action: {verb: "vClimbUp"}, prsa: words[0], prso, prsi };
            }
            return null;
          }
        case 2:
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {action: {verb: "vClimbUp"}, prsa: words[0], prso, prsi };
            }
            return null;
          }
        }
        return null;
      }
    case "DOWN":
      switch (words[2]) {
      default:
        const { objectNum, objectVal } = findObjectMatchingParsedWord(words[2], [{find: "rmungbit"}, {find: "climbbit", restrictions: ["ON-GROUND", "IN-ROOM"]}]);
        if (objectVal) { if (prso && !prsi) { prsi = { word: words[2], val: objectVal }; } else if (!prso) { prso = { word: words[2], val: objectVal }; } }
        switch (objectNum) {
        case 1:
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {action: {verb: "vClimbDown"}, prsa: words[0], prso, prsi };
            }
            return null;
          }
        case 2:
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {action: {verb: "vClimbDown"}, prsa: words[0], prso, prsi };
            }
            return null;
          }
        }
        return null;
      }
    case "IN":
      switch (words[2]) {
      default:
        const { objectNum, objectVal } = findObjectMatchingParsedWord(words[2], [{find: "vehbit", restrictions: ["ON-GROUND", "IN-ROOM"]}]);
        if (objectVal) { if (prso && !prsi) { prsi = { word: words[2], val: objectVal }; } else if (!prso) { prso = { word: words[2], val: objectVal }; } }
        switch (objectNum) {
        case 1:
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {action: {verb: "vBoard", pre: "preBoard"}, prsa: words[0], prso, prsi };
            }
            return null;
          }
        }
        return null;
      }
    case "ON":
      switch (words[2]) {
      default:
        const { objectNum, objectVal } = findObjectMatchingParsedWord(words[2], [{find: "vehbit", restrictions: ["ON-GROUND", "IN-ROOM"]}]);
        if (objectVal) { if (prso && !prsi) { prsi = { word: words[2], val: objectVal }; } else if (!prso) { prso = { word: words[2], val: objectVal }; } }
        switch (objectNum) {
        case 1:
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {action: {verb: "vClimbOn"}, prsa: words[0], prso, prsi };
            }
            return null;
          }
        }
        return null;
      }
    case "WITH":
      switch (words[2]) {
      default:
        const { objectNum, objectVal } = findObjectMatchingParsedWord(words[2], [{}]);
        if (objectVal) { if (prso && !prsi) { prsi = { word: words[2], val: objectVal }; } else if (!prso) { prso = { word: words[2], val: objectVal }; } }
        switch (objectNum) {
        case 1:
          switch (words[3]) {
          default:
            if (words.length == 3) {
              return {action: {verb: "vThrough"}, prsa: words[0], prso, prsi };
            }
            return null;
          }
        }
        return null;
      }
    default:
      const { objectNum, objectVal } = findObjectMatchingParsedWord(words[1], [{find: "climbbit", restrictions: ["ON-GROUND", "IN-ROOM"]}]);
      if (objectVal) { if (prso && !prsi) { prsi = { word: words[1], val: objectVal }; } else if (!prso) { prso = { word: words[1], val: objectVal }; } }
      switch (objectNum) {
      case 1:
        switch (words[2]) {
        default:
          if (words.length == 2) {
            return {action: {verb: "vClimbFoo"}, prsa: words[0], prso, prsi };
          }
          return null;
        }
      }
      return null;
    }
  default:
    return null;
  }
}
