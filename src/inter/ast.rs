// override/substitute words

// can compare enum references, is this more performant?

// Hypotheses:
// - dot prefix can be ignored
// - comma prefix cannot. It means "dereference this pointer". Can simulate pointer by with an array of length 1
// - will have to make extensive use of (() => { ... })() aka Immediately Invoked Function Expressions (IIFE) as everything returns something
// - ,P?xxxx refers to property xxxx. Sometimes of the current ROOM, sometimes of a provided ROOM or OBJECT

/*
;"0 -> no next, 1 -> success, 2 -> failed move"
<ROUTINE GO-NEXT (TBL "AUX" VAL)
	 <COND
 		(<SET VAL <LKP ,HERE .TBL>> <COND 
			(<NOT <GOTO .VAL>> 2)
		      	(T 1)
		>)
	>
>
*/

// becomes

/*
let GO_NEXT = (TBL) => {
  let VAL;
  // ROUTINE always returns the last expressions
  // COND is always wrapped in an IIFE
  return (() => {
    if (VAL = LKP(HERE[0], TBL)) {
      // COND groupings always return the last expr
      // COND is always wrapped in an IIFE
      return (() => {
        if (!GOTO(VAL)) {
          return 2;
        } else {
          return 1;
        }
        // COND always returns 0 by default
        return 0;
      })();
    }
    // COND always returns 0 by default
    return 0;
  })();
}
*/

use crate::zil::node::*;
use crate::inter::node::*;
use crate::inter;

pub fn clone_zil_tree(root: &ZilNode) -> Result<InterNode, InterErr> {
  let mut root = InterNode::clone_zilnode(&root)?;
  inter::validation::validate(&root)?;
  refactor_routine_params(&mut root);
  refactor_room_nav(&mut root);
  replace_dashes(&mut root);
  Ok(root)
}

#[allow(dead_code)]
pub fn print_tree(root: &InterNode, depth: u64) {
    let spacer = String::from("  ");
    let mut out = String::new();
    for _ in 0..depth {
        out.push_str(&spacer);
    }
    out.push_str(&format!("({})", root));
    println!("{}", out);
    for n in root.children.iter() {
        print_tree(n, depth+1);
    }
}

fn replace_dashes(root: &mut InterNode) {
  match root.kind {
    InterNodeType::EmptyRoutine | InterNodeType::Text | InterNodeType::Int => return,
    InterNodeType::Unknown | InterNodeType::Grouping => (),
    _ => {
      root.value = root.value().replace("-", "_");
    }
  };
  for i in 0..root.children.len() {
    replace_dashes(&mut root.children[i]);
  }
}

/*
(IN ROOMS) -> (IN ROOMS)
(IN "The dam blocks your way.") -> (IN_TO "The dam blocks your way.")
(IN TO SQUEEKY-ROOM) -> (IN_TO SQUEEKY-ROOM)
(IN PER GRATING-EXIT) -> (IN_TO PER GRATING-EXIT) // execute GRATING-EXIT
(IN TO STONE-BARROW IF WON-FLAG) -> (IN_TO <COND (,WON-FLAG STONE-BARROW)>)
(IN TO KITCHEN IF KITCHEN-WINDOW IS OPEN) -> (IN_TO <COND (<FSET? ,KITCHEN-WINDOW ,OPENBIT> KITCHEN)>)
(IN TO RESERVOIR IF LOW-TIDE ELSE "You would drown.") -> (IN_TO <COND (,LOW-TIDE RESERVOIR) (T "You would drown")>)
(IN TO X IF Y IS Z ELSE "Text") -> (IN_TO <COND (<FSET? ,Y ,Z> X) (T "Text")>)
*/
fn refactor_room_nav(root: &mut InterNode) {
  if root.kind == InterNodeType::Routine && root.value() == "ROOM" {
    for i in 1..root.children.len() {
      if root.children[i].children[0].value() == "IN" && root.children[i].children[1].value() == "ROOMS" {
        // do nothing
      } else {
        match root.children[i].children[0].value() {
          "IN" | "NORTH" | "NE" | "EAST" | "SE" | "SOUTH" | "SW" | "WEST" | "NW" | "LAND" | "UP" | "DOWN" | "OUT" => {
            if root.children[i].children[1].value() == "TO" {
              root.children[i].children.remove(1);
            }

            #[allow(non_snake_case)]
            let tmp_InterNode = root.children[i].children.remove(0);
            root.children[i].children.insert(0, InterNode::no_token(
              InterNodeType::Word, format!("{}_TO", &tmp_InterNode.value), vec![]));

            match root.children[i].children.len() {
              2 | 3 => (),
              4 => {
                // (IN_TO STONE-BARROW IF WON-FLAG) -> (IN_TO <COND (,WON-FLAG STONE-BARROW)>)
                let tmp_cond = InterNode::no_token(
                  InterNodeType::Routine, "COND",
                  vec![InterNode::no_token(
                    InterNodeType::Grouping, "",
                    vec![
                      InterNode::no_token(InterNodeType::Word, root.children[i].children[1].value(), vec![]),
                      InterNode::no_token(InterNodeType::Word, root.children[i].children[3].value(), vec![])]
                  )]
                );
                root.children[i].children = vec![root.children[i].children.remove(0), tmp_cond];
              },
              6 => {
                if root.children[i].children[4].value() == "IS" {
                  // (IN_TO KITCHEN IF KITCHEN-WINDOW IS OPEN) -> (IN_TO <COND (<FSET? ,KITCHEN-WINDOW ,OPENBIT> KITCHEN)>)
                  let tmp_cond = InterNode::no_token(
                    InterNodeType::Routine, "COND",
                    vec![InterNode::no_token(
                      InterNodeType::Grouping, "",
                      vec![InterNode::no_token(
                        InterNodeType::Routine, "FSET?",
                        vec![
                          InterNode::no_token(InterNodeType::Word, format!(",{}", root.children[i].children[3].value()), vec![]),
                          InterNode::no_token(InterNodeType::Word, format!(",{}BIT", root.children[i].children[5].value()), vec![])]
                      ), 
                      InterNode::no_token(InterNodeType::Word, root.children[i].children[1].value(), vec![])]
                    )]
                  );
                  root.children[i].children = vec![root.children[i].children.remove(0), tmp_cond];
                } else {
                  // (IN_TO RESERVOIR IF LOW-TIDE ELSE "You would drown.") -> (IN_TO <COND (,LOW-TIDE RESERVOIR) (T "You would drown")>)
                  let tmp_cond = InterNode::no_token(
                    InterNodeType::Routine, "COND",
                    vec![InterNode::no_token(
                      InterNodeType::Grouping, "",
                      vec![
                        InterNode::no_token(InterNodeType::Word, format!(",{}", root.children[i].children[3].value()), vec![]),
                        InterNode::no_token(InterNodeType::Word, format!(",{}BIT", root.children[i].children[1].value()), vec![])]
                    ), InterNode::no_token(
                      InterNodeType::Grouping, "",
                      vec![
                        InterNode::no_token(InterNodeType::Word, "T", vec![]),
                        InterNode::no_token(InterNodeType::Text, root.children[i].children[5].value(), vec![])]
                    )]
                  );
                  root.children[i].children = vec![root.children[i].children.remove(0), tmp_cond];
                }
              },
              8 => {
                // (IN_TO X IF Y IS Z ELSE "Text") -> (IN_TO <COND (<FSET? ,Y ,Z> X) (T "Text")>)
                let tmp_cond = InterNode::no_token(
                  InterNodeType::Routine, "COND",
                  vec![InterNode::no_token(
                    InterNodeType::Grouping, "",
                    vec![
                      InterNode::no_token(
                        InterNodeType::Routine, "FSET?",
                        vec![
                          InterNode::no_token(InterNodeType::Word, format!(",{}", root.children[i].children[3].value()), vec![]),
                          InterNode::no_token(InterNodeType::Word, format!(",{}BIT", root.children[i].children[5].value()), vec![])]), 
                      InterNode::no_token(InterNodeType::Word, root.children[i].children[1].value(), vec![])] 
                  ), InterNode::no_token(
                    InterNodeType::Grouping, "",
                    vec![
                      InterNode::no_token(InterNodeType::Word, "T", vec![]),
                      InterNode::no_token(InterNodeType::Text, root.children[i].children[7].value(), vec![])]
                  )]
                );
                root.children[i].children = vec![root.children[i].children.remove(0), tmp_cond];
              },
              x => { println!("{}", x); panic!() }
            };
          },
          _ => panic!()
        };
      }
    }
  }
  for i in 0..root.children.len() {
    refactor_room_nav(&mut root.children[i]);
  }
}

// <ROUTINE R (X Y "AUX" Z) ... > becomes <ROUTINE R (X Y) <LET Z> ... >
fn refactor_routine_params(root: &mut InterNode) {
  if root.kind == InterNodeType::Routine && root.value() == "ROUTINE" {
    let mut aux_index: Option<usize> = None;
    for i in 0..root.children[1].children.len() {
      if root.children[1].children[i].value() == "AUX" {
        aux_index = Some(i);
        break;
      }
    }
    if aux_index != None {
      for i in ((aux_index.unwrap()+1)..root.children[1].children.len()).rev() {
        match root.children[1].children[i].kind {
          InterNodeType::Grouping => {
            root.children.insert(2, InterNode::no_token(
              InterNodeType::Routine, "LET",
              vec![
                InterNode::no_token(InterNodeType::Word, root.children[1].children[i].children[0].value(), vec![]),
                InterNode::no_token(
                  root.children[1].children[i].children[1].kind,
                  match root.children[1].children[i].children[1].kind {
                    InterNodeType::EmptyRoutine => String::from(""),
                    InterNodeType::Word | InterNodeType::Int => String::from(root.children[1].children[i].children[1].value()),
                    _ => panic!()
                  },
                  vec![])]
            ));
          },
          InterNodeType::Word => {
            root.children.insert(2, InterNode::no_token(
              InterNodeType::Routine, "LET",
              vec![
                InterNode::no_token(InterNodeType::Word, root.children[1].children[i].value(), vec![]),
                InterNode::no_token(InterNodeType::EmptyRoutine, "", vec![])]
            ));
          },
          _ => panic!()
        };
      }
      root.children[1].children = root.children[1].children[..aux_index.unwrap()].to_vec();
    }
  }
    
  for i in 0..root.children.len() {
    refactor_routine_params(&mut root.children[i]);
  }
}