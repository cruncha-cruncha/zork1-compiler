// re-organize tree (like "AUX" in ROUTINE params, IN vs IN TO in OBJECTs and ROOMs)
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

/*
<ROUTINE PATIO (V "AUX" E)
  <SET E <>>
  <SET E V>
>
*/

use crate::zil::tokenize::*;
use crate::zil::contracts::*;
use crate::inter::contracts::*;
use crate::inter;

pub fn clone_zil_tree(root: &ZilNode) -> Result<InterNode, InterErr> {
  let mut root = inter::contracts::InterNode::clone_zilnode(&root)?;
  inter::validation::validate(&root)?;
  refactor_routine_params(&mut root);
  refactor_room_nav(&mut root);
  Ok(root)
}

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
pub fn refactor_room_nav(root: &mut InterNode) {
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
            let mut tmp_InterNode = root.children[i].children.remove(0);
            #[allow(non_snake_case)]
            let mut tmp_Token = tmp_InterNode.token.unwrap();
            tmp_Token.value = String::from(format!("{}_TO", &tmp_Token.value));
            tmp_InterNode.token = Some(tmp_Token);
            root.children[i].children.insert(0, tmp_InterNode);

            match root.children.len() {
              5 => {
                // (IN_TO STONE-BARROW IF WON-FLAG) -> (IN_TO <COND (,WON-FLAG STONE-BARROW)>)
                let tmp_cond = InterNode {
                  kind: InterNodeType::Routine,
                  token: Some(Token::fake(TokenType::Word, "COND")),
                  children: vec![InterNode{
                    kind: InterNodeType::Grouping,
                    token: None,
                    children: vec![InterNode {
                      kind: InterNodeType::Word,
                      token: Some(Token::fake(TokenType::Word, format!(",{}", root.children[i].children[1].value()))),
                      children: Vec::new()
                    }, InterNode {
                      kind: InterNodeType::Word,
                      token: Some(Token::fake(TokenType::Word, root.children[i].children[4].value())),
                      children: Vec::new()
                    }]
                  }]
                };
                root.children[i].children = vec![root.children[i].children.remove(0), tmp_cond];
              },
              7 => {
                if root.children[5].value() == "IS" {
                  // (IN_TO KITCHEN IF KITCHEN-WINDOW IS OPEN) -> (IN_TO <COND (<FSET? ,KITCHEN-WINDOW ,OPENBIT> KITCHEN)>)
                  let tmp_cond = InterNode {
                    kind: InterNodeType::Routine,
                    token: Some(Token::fake(TokenType::Word, "COND")),
                    children: vec![InterNode {
                      kind: InterNodeType::Grouping,
                      token: None,
                      children: vec![InterNode {
                        kind: InterNodeType::Routine,
                        token: Some(Token::fake(TokenType::Word, "FSET?")),
                        children: vec![InterNode {
                          kind: InterNodeType::Word,
                          token: Some(Token::fake(TokenType::Word, format!(",{}", root.children[i].children[3].value()))),
                          children: Vec::new()
                        }, InterNode {
                          kind: InterNodeType::Word,
                          token: Some(Token::fake(TokenType::Word, format!(",{}BIT", root.children[i].children[5].value()))),
                          children: Vec::new()
                        }]
                      }, InterNode {
                        kind: InterNodeType::Word,
                        token: Some(Token::fake(TokenType::Word, root.children[i].children[1].value())),
                        children: Vec::new()
                      }]
                    }]
                  };
                  root.children[i].children = vec![root.children[i].children.remove(0), tmp_cond];
                } else {
                  // (IN_TO RESERVOIR IF LOW-TIDE ELSE "You would drown.") -> (IN_TO <COND (,LOW-TIDE RESERVOIR) (T "You would drown")>)
                  let tmp_cond = InterNode {
                    kind: InterNodeType::Routine,
                    token: Some(Token::fake(TokenType::Word, "COND")),
                    children: vec![InterNode {
                      kind: InterNodeType::Grouping,
                      token: None,
                      children: vec![InterNode {
                          kind: InterNodeType::Word,
                          token: Some(Token::fake(TokenType::Word, format!(",{}", root.children[i].children[3].value()))),
                          children: Vec::new()
                        }, InterNode {
                          kind: InterNodeType::Word,
                          token: Some(Token::fake(TokenType::Word, format!(",{}BIT", root.children[i].children[1].value()))),
                          children: Vec::new()
                        }]
                    }, InterNode {
                      kind: InterNodeType::Grouping,
                      token: None,
                      children: vec![InterNode {
                          kind: InterNodeType::Word,
                          token: Some(Token::fake(TokenType::Word, "T")),
                          children: Vec::new()
                        }, InterNode {
                          kind: InterNodeType::Text,
                          token: Some(Token::fake(TokenType::Text, root.children[i].children[5].value())),
                          children: Vec::new()
                        }]
                    }]
                  };
                  root.children[i].children = vec![root.children[i].children.remove(0), tmp_cond];
                }
              },
              9 => {
                // (IN_TO X IF Y IS Z ELSE "Text") -> (IN_TO <COND (<FSET? ,Y ,Z> X) (T "Text")>)
              },
              _ => panic!()
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

// for <ROUTINE X ( ... ) ... >, take the params grouping (children[1])
// if there is an "AUX" child in the params grouping:
// convert every child after to a <SET> routine in the main ROUTINE body
// remove those params from the grouping
pub fn refactor_routine_params(root: &mut InterNode) {
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
            root.children.insert(2, InterNode {
              kind: InterNodeType::Routine,
              token: Some(Token::fake(TokenType::Word, "SET")),
              children: vec![InterNode {
                kind: InterNodeType::Word,
                token: Some(Token::fake(TokenType::Word, root.children[1].children[i].children[0].value())),
                children: Vec::new()
              }, InterNode {
                kind: root.children[1].children[i].children[1].kind,
                token: match root.children[1].children[i].children[1].kind {
                  InterNodeType::EmptyRoutine => None,
                  InterNodeType::Word | InterNodeType::Int => Some(Token::fake(TokenType::Word, root.children[1].children[i].children[1].value())),
                  _ => panic!()
                },
                children: Vec::new()
              }]
            });
          },
          InterNodeType::Word => {
            root.children.insert(2, InterNode{
              kind: InterNodeType::Routine,
              token: Some(Token::fake(TokenType::Word, "SET")),
              children: vec![InterNode {
                kind: InterNodeType::Word,
                token: Some(Token::fake(TokenType::Word, root.children[1].children[i].value())),
                children: Vec::new()
              }, InterNode {
                kind: InterNodeType::EmptyRoutine,
                token: None,
                children: Vec::new()
              }]
            });
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