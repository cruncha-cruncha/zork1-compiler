use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

mod zil;
mod file_table;

use crate::file_table::FileTable;

// two passes through the tree?
// #1 collect info
// #2 print

fn main() {
    let mut files_lookup = FileTable::new();

    let file_path = Path::new(".").join("dummy-data").join("testing.zil");
    let file_key = files_lookup.insert(file_path.to_str().unwrap().to_string());

    let mut generator = match zil::tokenize::TokenGenerator::new(file_key, &file_path) {
        Some(v) => v,
        None => return,
    };

    let mut root = zil::ast::Node::new();
    zil::ast::build_tree(&mut generator, &mut root);
    zil::ast::retain_child_routines(&mut root);
    zil::ast::remove_comments(&mut root);

    match zil::ast::validate_tree(&root, 0) {
        Ok(()) => println!("ok"),
        Err(()) => {
            println!("ERROR");
            //print_tree(&root, 0);
            return;
        },
    }

    //run_stats(&root);
    zil::ast_stats::dot_comma_stats(&root);
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
