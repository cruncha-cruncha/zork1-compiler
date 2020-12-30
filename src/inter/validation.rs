use crate::inter::node::*;

pub fn validate(n: &InterNode) -> Result<(), InterErr> {
  if n.kind == InterNodeType::Routine {
    match n.value() {
      "ROUTINE" => {
        // at least 3 children
        // first must be word
        // second must be grouping -> validate
        // the rest must be routines

        if n.children.len() < 3 ||
           n.children[0].kind != InterNodeType::Word || 
           n.children[1].kind != InterNodeType::Grouping {
          return Err(InterErr::origin(format!("Invalid ROUTINE ({}); could be a couple things", n)));
        }

        match validate_param_grouping(&n.children[1]) {
          Ok(_) => (),
          Err(e) => return Err(InterErr::wrap(e, format!("From ROUTINE\n{}", n)))
        };

        for i in 2..n.children.len() {
          if n.children[i].kind != InterNodeType::Routine {
            return Err(InterErr::origin(format!("Invalid ROUTINE ({}); not all children are Routines", n)));
          }
        }
      },
      "OBJECT" => {
        // at least 1 child
        // first must be word
        // the rest must be groupings -> validate

        if n.children.len() < 1 ||
           n.children[0].kind != InterNodeType::Word {
          return Err(InterErr::origin(format!("Invalid OBJECT ({}); could be a couple things", n)));
        }

        for i in 1..n.children.len() {
          match validate_object_grouping(&n.children[i]) {
            Ok(_) => (),
            Err(e) => return Err(InterErr::wrap(e, format!("{}", n)))
          };
        }
      },
      "ROOM" => {
        // at least 1 child
        // first must be a word
        // the rest must be groupings -> validate

        if n.children.len() < 1 ||
           n.children[0].kind != InterNodeType::Word {
          return Err(InterErr::origin(format!("Invalid ROOM ({}); could be a couple things", n)));
        }

        for i in 1..n.children.len() {
          match validate_room_grouping(&n.children[i]) {
            Ok(_) => (),
            Err(e) => return Err(InterErr::wrap(e, format!("{}", n)))
          };
        }
      },
      "GLOBAL" | "SET" => {
        // exactly two children
        // first child must be a word
        // second cannot be a grouping

        if n.children.len() != 2 ||
           n.children[0].kind != InterNodeType::Word ||
           n.children[1].kind == InterNodeType::Grouping {
          return Err(InterErr::origin(format!("Invalid ({}); could be a couple things", n)));
        }
      },
      "TELL" => {
        // at least one child
        // all must be routine, text, or word

        if n.children.len() < 1 {
          return Err(InterErr::origin(format!("Invalid TELL ({}); could be a couple things", n)));
        }

        for i in 0..n.children.len() {
          if n.children[i].kind != InterNodeType::Routine &&
             n.children[i].kind != InterNodeType::Text &&
             n.children[i].kind != InterNodeType::Word {
            return Err(InterErr::origin(format!("Invalid TELL ({}); something with the children, could be a couple things", n)));
          }
        }
      },
      "REPEAT" => {
        // at least 2 children
        // first child must be grouping with 0 children
        // the rest must be routines

        if n.children.len() < 2 ||
           n.children[0].kind != InterNodeType::Grouping ||
           n.children[0].children.len() > 0 {
          return Err(InterErr::origin(format!("Invalid ROUTINE ({}); could be a couple things", n)));
        }

        for i in 1..n.children.len() {
          if n.children[i].kind != InterNodeType::Routine {
            return Err(InterErr::origin(format!("Invalid ROUTINE ({}) children", n)));
          }
        }
      }
      "COND" => {
        // at least 1 child
        // all must be groupings -> validate

        if n.children.len() < 1 {
          return Err(InterErr::origin(format!("Invalid COND ({}); doesn't have enough children", n)));
        }

        for i in 1..n.children.len() {
          match validate_cond_grouping(&n.children[i]) {
            Ok(_) => (),
            Err(e) => return Err(InterErr::wrap(e, format!("From COND\n{}", n)))
          };
        }
      },
      "NOT" => {
        // exactly one child
        // either a word or a routine

        if n.children.len() != 1 ||
           (n.children[0].kind != InterNodeType::Routine &&
            n.children[0].kind != InterNodeType::Word) {
          return Err(InterErr::origin(format!("Invalid NOT ({})", n)));
        }
      },
      "AND" | "OR" | "+" | "*" => {
        // at least two children
        // all either a word or routine

        if n.children.len() < 1 {
          return Err(InterErr::origin(format!("Invalid AND/OR/+/* ({})", n)));
        }

        for i in 0..n.children.len() {
          if n.children[i].kind != InterNodeType::Routine &&
             n.children[i].kind != InterNodeType::Word {
            return Err(InterErr::origin(format!("Invalid AND/OR/+/* ({}) child", n)));
          }
        }
      },
      "-" | "/" => {
        // exactly two children
        // all either a word or routine

        if n.children.len() != 2 {
          return Err(InterErr::origin(format!("Invalid -// ({}); bad child len()", n)));
        }

        for i in 0..n.children.len() {
          if n.children[i].kind != InterNodeType::Routine &&
             n.children[i].kind != InterNodeType::Word {
            return Err(InterErr::origin(format!("Invalid -// ({}); bad children", n)));
          }
        }
      },
      _ => ()
    };
  }

  for i in 0..n.children.len() {
    match validate(&n.children[i]) {
      Ok(()) => (),
      Err(e) => return Err(InterErr::wrap(e, format!("Invalid children, at {}", n)))
    };
  }

  Ok(())
}

fn validate_param_grouping(n: &InterNode) -> Result<(), InterErr> {
  for c in n.children.iter() {
    match c.kind {
      InterNodeType::Grouping => {
        if c.children.len() != 2 ||
           c.children[0].kind != InterNodeType::Word ||
           c.children[1].kind == InterNodeType::Grouping {
          return Err(InterErr::origin(format!("Invalid param grouping grouping ({})", c)));
        }
      },
      InterNodeType::Text => {
        match c.value() {
          "OPTIONAL" => (),
          "AUX" => (),
          _ => return Err(InterErr::origin(format!("Unknown text in param grouping: {}", c)))
        };
      },
      InterNodeType::Word => (),
      _ => return Err(InterErr::origin(format!("Unknown InterNodeType in param grouping ({})", c)))
    };
  }

  Ok(())
}

fn validate_object_grouping(n: &InterNode) -> Result<(), InterErr> {
  if n.children.len() < 2 ||
     n.children[0].kind != InterNodeType::Word {
    return Err(InterErr::origin(format!("Invalid object grouping ({})", n)));
  }

  match n.children[0].value() {
    "IN" => {
      return validate_object_IN_grouping(&n);
    }
    "ACTION" | "DESCFCN" => {
      // exactly two children
      // second must be word
      if n.children.len() != 2 ||
         n.children[1].kind != InterNodeType::Word {
        return Err(InterErr::origin(format!("Bad ({}) in object grouping", n.children[0])));
      }
    },
    "TEXT" | "DESC" | "LDESC" | "FDESC" => {
      // exactly two children
      // second must be text
      if n.children.len() != 2 ||
         n.children[1].kind != InterNodeType::Text {
        return Err(InterErr::origin(format!("Bad ({}) in object grouping", n.children[0])));
      }
    },
    "VTYPE" | "FLAGS" | "SYNONYM" | "ADJECTIVE" => {
      // at least two children
      // all must be words
      if n.children.len() < 2 {
        return Err(InterErr::origin(format!("({}) doesn't have enough siblings, in object grouping", n.children[0])));
      }
      for i in 1..n.children.len() {
        if n.children[i].kind != InterNodeType::Word {
          return Err(InterErr::origin(format!("({}) has a non-word sibling, in object grouping", n.children[0])));
        }
      }
    },
    "VALUE" | "TVALUE" | "CAPACITY" | "SIZE" | "STRENGTH" => {
      // exactly two children
      // second must be Int
      if n.children.len() != 2 ||
         n.children[1].kind != InterNodeType::Int {
        return Err(InterErr::origin(format!("Bad ({}) in object grouping", n.children[0])));
      }
    },
    _ => return Err(InterErr::origin(format!("Unknown object grouping: ({})", n.children[0])))
  };

  Ok(())
}

#[allow(non_snake_case)]
fn validate_object_IN_grouping(n: &InterNode) -> Result<(), InterErr> {
  if n.children.len() != 2 ||
     n.children[1].kind != InterNodeType::Word {
    return Err(InterErr::origin(format!("Invalid object IN grouping ({})", n)));
  }

  Ok(())
}

fn validate_room_grouping(n: &InterNode) -> Result<(), InterErr> {
  if n.children.len() < 2 ||
     n.children[0].kind != InterNodeType::Word {
    return Err(InterErr::origin(format!("Invalid room grouping ({})", n)));
  }

  match validate_object_grouping(&n) {
    Ok(v) => return Ok(v),
    Err(_) => ()
  };

  match n.children[0].value() {
    "PSEUDO" => {
      // at least 3 children
      // alternating word, text, word, text, etc.
      // like (PSEUDO "DOOR" DOOR-PSEUDO "PAINT" PAINT-PSEUDO)
      if n.children.len() < 3 ||
         n.children.len()%2 != 1 {
        return Err(InterErr::origin(format!("({}) doesn't have enough siblings, in room grouping", n.children[0])));
      }

      for i in 0..(n.children.len()-1)/2 {
        if n.children[i*2+1].kind != InterNodeType::Text {
          return Err(InterErr::origin(format!("Odd child is not Text, in ({}), in room grouping", n.children[0])));
        }
        if n.children[i*2+2].kind != InterNodeType::Word {
          return Err(InterErr::origin(format!("Even child is not Word, in ({}), in room grouping", n.children[0])));
        }
      }
    },
    "GLOBAL" => {
      // at least 2 children
      // all must be words
      if n.children.len() < 2 {
        return Err(InterErr::origin(format!("({}) doesn't have enough siblings, in room grouping", n.children[0])));
      }
      for i in 1..n.children.len() {
        if n.children[i].kind != InterNodeType::Word {
          return Err(InterErr::origin(format!("({}) has a non-word sibling, in room grouping", n.children[0])));
        }
      }
    },
    "IN" | "NORTH" | "NE" | "EAST" | "SE" | "SOUTH" | "SW" | "WEST" | "NW" | "LAND" | "UP" | "DOWN" | "OUT" => {
      return validate_room_IN_grouping(&n);
    },
    _ => return Err(InterErr::origin(format!("Bad room grouping: {}", n.children[0])))
  };

  Ok(())
}

#[allow(non_snake_case)]
fn validate_room_IN_grouping(n: &InterNode) -> Result<(), InterErr> {
  // bit tricky to explain, can look like
  // (IN ROOMS)
  // (IN "The dam blocks your way.")
  // (IN TO SQUEEKY-ROOM)
  // (IN PER GRATING-EXIT)
  // (IN TO STONE-BARROW IF WON-FLAG)
  // (IN TO KITCHEN IF KITCHEN-WINDOW IS OPEN)
  // (IN TO RESERVOIR IF LOW-TIDE ELSE "You would drown.")
  // assume (IN TO X IF Y IS Z ELSE "Text") is possible

  if n.children.len() < 2 {
    return Err(InterErr::origin("IN grouping does not have enough children"));
  }

  if n.children[1].kind == InterNodeType::Text {
    if n.children.len() != 2 {
      return Err(InterErr::origin("IN grouping's second child is Text but len() is longer than 2"));
    }
  } else if n.children[1].kind == InterNodeType::Word {
    if n.children[1].value() == "PER" {
      if n.children.len() != 3 || n.children[2].kind != InterNodeType::Word {
        return Err(InterErr::origin("Bad PER in IN grouping"));
      }
    } else if n.children[1].value() == "TO" {
      match n.children.len() {
        3 => {
          if n.children[2].kind != InterNodeType::Word {
            return Err(InterErr::origin("Bad IN grouping: (IN TO should-be-a-word-but-its-not)"));
          }
        },
        5 => {
          if n.children[2].kind != InterNodeType::Word ||
             n.children[3].kind != InterNodeType::Word ||
             n.children[3].value() != "IF" ||
             n.children[4].kind != InterNodeType::Word {
            return Err(InterErr::origin("Bad (IN TO X IF Y) type IN grouping (len() 5)"));
          }
        },
        7 => {
          if n.children[2].kind != InterNodeType::Word ||
             n.children[3].kind != InterNodeType::Word ||
             n.children[3].value() != "IF" ||
             n.children[4].kind != InterNodeType::Word ||
             n.children[5].kind != InterNodeType::Word ||
             ((n.children[5].value() != "IS" ||
               n.children[6].kind != InterNodeType::Word) &&
              (n.children[5].value() != "ELSE" ||
               n.children[6].kind != InterNodeType::Text)) {
            return Err(InterErr::origin("Bad IN grouping of len() 7"));
          }
        },
        9 => {
          if n.children[2].kind != InterNodeType::Word ||
             n.children[3].kind != InterNodeType::Word ||
             n.children[3].value() != "IF" ||
             n.children[4].kind != InterNodeType::Word || 
             n.children[5].kind != InterNodeType::Word ||
             n.children[5].value() != "IS" ||
             n.children[6].kind != InterNodeType::Word || 
             n.children[7].kind != InterNodeType::Word ||
             n.children[7].value() != "ELSE" || 
             n.children[8].kind != InterNodeType::Text {
            return Err(InterErr::origin("Bad IN grouping of len() 9"));
          }
        },
        _ => return Err(InterErr::origin("Bad IN grouping len()"))
      };
    } else {
      return Err(InterErr::origin("Bad IN grouping; second child is word, but not PER or TO"));
    }
  } else {
    return Err(InterErr::origin("Unknown InterNodeType for second child in IN grouping"));
  }

  Ok(())
}

fn validate_cond_grouping(n: &InterNode) -> Result<(), InterErr> {
  if n.children.len() < 2 {
    return Err(InterErr::origin("COND grouping has no children"));
  }

  for i in 0..n.children.len() {
    if n.children[i].kind == InterNodeType::Grouping {
      return Err(InterErr::origin("Child in COND grouping is a grouping"));
    }
  }

  Ok(())
}