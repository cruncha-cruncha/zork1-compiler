use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

//mod tokenize;
//mod zil_ast;
//mod zil_ast_stats;
//mod tokenizer;
//mod parse_tree_generator;
//mod testing;

mod zil;

use crate::zil::tokenize::*;
use crate::zil::ast::*;
use crate::zil::ast_stats::*;

//use crate::tokenize::*;
//use crate::zil_ast::*;
//use crate::zil_ast_stats::*;
//use crate::parse_tree_generator::*;
//use crate::testing::tree_traversal::*;

pub struct FileNameTable {
    key: u32,
    table: HashMap<u32, String>,
}

impl FileNameTable {
    pub fn new() -> FileNameTable {
        FileNameTable {key: 0, table: HashMap::new()}
    }

    pub fn insert(&mut self, v: String) -> u32 {
        self.key += 1;
        self.table.insert(
            self.key,
            v,
        );
        self.key
    }

    pub fn get(&mut self, k: u32) -> Option<String> {
        match self.table.get(&k) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub fn find_key(&mut self, v: String) -> Option<u32> {
        for (key, value) in self.table.iter() {
            if *value == v {
                return Some(*key);
            }
        }

        None
    }
}

// two passes through the tree?
// #1 collect info
// #2 print

fn main() {
    let mut files_lookup = FileNameTable::new();

    let file_path = Path::new(".").join("dummy-data").join("testing.zil");
    let file_key = files_lookup.insert(file_path.to_str().unwrap().to_string());

    let mut generator = match TokenGenerator::new(file_key, &file_path) {
        Some(v) => v,
        None => return,
    };

    let mut root = Node::new();
    build_tree(&mut generator, &mut root);
    retain_child_routines(&mut root);
    remove_comments(&mut root);

    match validate_tree(&root, 0, &mut files_lookup) {
        Ok(()) => println!("ok"),
        Err(()) => {
            println!("ERROR");
            //print_tree(&root, 0);
            return;
        },
    }

    //run_stats(&root);
    dot_comma_stats(&root);
}

/*
<OBJECT ATTIC-TABLE
	(IN ATTIC)
	(SYNONYM TABLE)
	(DESC "table")
	(FLAGS NDESCBIT CONTBIT OPENBIT SURFACEBIT)
    (CAPACITY 40)>

// (IN ATTIC) and (SYNONTM TABLE) are handled elsewhere?

class ATTIC-TABLE:
  ndescbit = false
  contbit = false
  openbit = false
  surfacebit = false
  def describe():
    return "table"
  def capacity():
    40

<COND (<EQUAL? .NG .G> <RFALSE>)
      (<EQUAL? .NG 2> <TELL "Your sword has begun to glow very brightly." CR>)
      (<0? .NG> <TELL "Your sword is no longer glowing." CR>)>
      
if EQUAL?(.NG, .G):
  RFALSE()
elif EQUAL?(.NG, 2):
  TELL("Your sword has begun to glow very brightly.", CR)
elif 0?(.NG):
  TELL("Your sword is no longer glowing.", CR)
*/
