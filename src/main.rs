use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io;

mod zil;
mod js;
mod inbetween;

// need regression tests for ast parser

// two passes through the tree?
// #1 collect info
// #2 print

fn main() {
    let mut files_lookup = zil::file_table::FileTable::new();

    let file_path = Path::new(".").join("dummy-data").join("testing.zil");
    let file_key = files_lookup.insert(file_path.to_str().unwrap().to_string());

    let reader = get_BufReader(&file_path).unwrap();

    let mut generator = zil::tokenize::TokenGenerator::new(file_key, reader);

    let mut root = zil::ast::Node::new();
    
    match zil::ast::build_tree(&mut generator, &mut root) {
      Ok(()) => {
        println!("build tree ok");
        //zil::ast::print_tree(&root, 0);
        return;
      },
      Err(e) => {
        println!("\nERROR\n{}", e);
        zil::ast::print_tree(&root, 0);
        return;
      }
    };

    /*
    let output_file_path = Path::new(".").join("out").join("testing.js");
    let writer = get_BufWriter(&output_file_path).unwrap();
    match js::parse::parse(&root, writer) {
      Ok(()) => println!("ok"),
      Err(()) => {
        println!("ERROR");
        return;
      }
    };
    */
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
