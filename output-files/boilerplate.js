import { rooms } from "./rooms.js";
import { currentRoom } from "./player.js";
import { lookupBySynonym as lookupObjectBySynonym } from "./objects.js";

export const findObjectMatchingParsedWord = (word, params) => {
  const object = lookupObjectBySynonym(word);
  if (!object) {
    return { objectNum: 0, objectVal: null };
  }

  // TODO

  // should not be able to interact with objects that are not in the same room or on the player
  // params[0].restrictions is just a list of variable names that must be present on the object, doesn't matter what their value is

  return { objectNum: 1, objectVal: object };
};

export const handleMove = (direction) => {
  let roomName = currentRoom.name;
  let actualRoom = rooms[roomName];

  let result = actualRoom.move[direction];

  if (!result) {
    console.log("You can't go that way.");
    return;
  }

  if ("text" in result) {
    console.log(result.text);
  } else if ("room" in result) {
    currentRoom.name = result.room;
    actualRoom = rooms[result.room];
    console.log(actualRoom.desc);
  } else if ("action" in result) {
    console.log("Do some action before we can move", result);
  }
};
