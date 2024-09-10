import { findObjectMatchingParsedWord } from "./main.js";

export const buzz = [];

export const directions = ["UP", "EAST", "WEST"];

export const parseInput = (rawString) => {
  const words = rawString.split(" ").map(w => w.toUpperCase()).filter(w => !buzz.includes(w));
  if ((words.length == 2) && (words[0] == "GO")) {
    return { move: words[1] };
  }

  let prso = null;
  let prsi = null;

  switch (words[0]) {
  case "DESC":
    switch (words[1]) {
    case "ROOM":
      switch (words[2]) {
      default:
        if (words.length == 2) {
          return {routine: 'vDescRoom', prsa: words[0], prso, prsi };
        }
        return null;
      }
    default:
      return null;
    }
  case "LIST":
    switch (words[1]) {
    case "VARS":
      switch (words[2]) {
      case "IN":
        switch (words[3]) {
        default:
          const { objectNum, objectVal } = findObjectMatchingParsedWord(words[3], [{restrictions: []}]);
          if (objectVal) { if (prso && !prsi) { prsi = { word: words[3], val: objectVal }; } else if (!prso) { prso = { word: words[3], val: objectVal }; } }
          switch (objectNum) {
          case 1:
            switch (words[4]) {
            default:
              if (words.length == 4) {
                return {routine: 'vListObjectVars', prsa: words[0], prso, prsi };
              }
              return null;
            }
          }
          return null;
        }
      default:
        return null;
      }
    case "MY":
      switch (words[2]) {
      case "VARS":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {routine: 'vListMyVars', prsa: words[0], prso, prsi };
          }
          return null;
        }
      case "OBJECTS":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {routine: 'vListMyObjects', prsa: words[0], prso, prsi };
          }
          return null;
        }
      default:
        return null;
      }
    case "ROOM":
      switch (words[2]) {
      case "OBJECTS":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {routine: 'vListObjects', prsa: words[0], prso, prsi };
          }
          return null;
        }
      default:
        return null;
      }
    default:
      return null;
    }
  default:
    return null;
  }
}
