// re-organize tree (like "AUX" in ROUTINE params, IN vs IN TO in OBJECTs and ROOMs)
// override/substitute words

// can compare enum references, is this more performant?

// new hypothesis
// dot prefix can be ignored
// comma prefix cannot. It means "dereference this pointer". Can simulate pointer by with an array of length 1
// will have to make extensive use of (() => { ... })() aka Immediately Invoked Function Expressions (IIFE) as everything returns something

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
  refactor_params(&mut root);
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

// for every ROUTINE, take the second child (which should be a grouping)
// for everything after "AUX", move into a <SET> in the children of the ROUTINE

pub fn refactor_params(root: &mut InterNode) {
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
    refactor_params(&mut root.children[i]);
  }
}