use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io;

mod zil;
mod py;
mod file_table;

use crate::file_table::FileTable;

// two passes through the tree?
// #1 collect info
// #2 print

fn main() {
    let mut files_lookup = FileTable::new();

    let file_path = Path::new(".").join("dummy-data").join("testing.zil");
    let file_key = files_lookup.insert(file_path.to_str().unwrap().to_string());

    let reader = get_BufReader(&file_path).unwrap();

    let mut generator = match zil::tokenize::TokenGenerator::new(file_key, reader) {
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
            //zil::ast::print_tree(&root, 0);
            return;
        },
    }

    //zil::ast::print_tree(&root, 0);

    //zil::ast_stats::run_stats(&root);

    let output_file_path = Path::new(".").join("out").join("testing.py");
    let writer = get_BufWriter(&output_file_path).unwrap();
    py::parse::parse(&root, writer);
}


pub fn get_BufReader(file_path: &Path) -> Option<BufReader<File>> {
  match File::open(file_path) {
    Ok(f) => Some(BufReader::new(f)),
    Err(e) => {
      println!("Failed to open file {}", file_path.to_str().unwrap());
      println!("{:?}", e);
      None
    },
  }
}

pub fn get_BufWriter(file_path: &Path) -> Option<BufWriter<File>> {
  match File::create(file_path) {
    Ok(f) => Some(BufWriter::new(f)),
    Err(e) => {
      println!("Failed to create file {}", file_path.to_str().unwrap());
      println!("{:?}", e);
      None
    },
  }
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
