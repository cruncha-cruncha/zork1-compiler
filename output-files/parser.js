import { findObjectMatchingParsedWord } from "./main.js";

export const buzz = [];

export const directions = ["WEST", "NORTH", "SOUTH", "EAST"];

export const parseInput = (rawString) => {
  const words = rawString.split(" ").map(w => w.toUpperCase()).filter(w => !buzz.includes(w));
  if ((words.length == 2) && (words[0] == "GO")) {
    return { move: words[1] };
  }
  
  let prso = null;
  let prsi = null;
  
  switch (words[0]) {
  case "WHERE":
    switch (words[1]) {
    case "AM":
      switch (words[2]) {
      case "I":
        switch (words[3]) {
        default:
          if (words.length == 3) {
            return {action: {routine: "vDescRoom"}, prsa: words[0], prso, prsi };
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
