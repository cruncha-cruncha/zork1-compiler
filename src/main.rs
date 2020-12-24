use std::path::Path;
use std::fs::File;
use std::io::BufReader;

mod zil;
mod js;
mod inbetween;
#[cfg(test)]
mod tests;

// two passes through the tree?
// #1 collect info
// #2 print

fn main() {
    let mut files_lookup = zil::file_table::FileTable::new();

    let file_path = Path::new(".").join("dummy-data").join("testing.zil");
    let file_key = files_lookup.insert(file_path.to_str().unwrap().to_string());
    println!("{}", files_lookup);

    let reader = get_BufReader(&file_path).unwrap();

    let mut generator = zil::tokenize::TokenGenerator::new(file_key, reader);

    let mut root = zil::ast::Node::new();
    
    match zil::ast::build_tree(&mut generator, &mut root) {
      Ok(()) => println!("build tree ok"),
      Err(e) => {
        println!("\nERROR\n{}", e);
        zil::ast::print_tree(&root, 0);
        return;
      }
    };

    //crate::inbetween::ast_stats::run_all(&root);

    let output_file_path = Path::new(".").join("out").join("testing.js");
    let writer = get_CustomBufWriter(&output_file_path).unwrap();
    match js::parse::parse(&root, writer) {
      Ok(()) => println!("handle js ok"),
      Err(e) => {
        println!("\nERROR\n{}", e);
        zil::ast::print_tree(&root, 0);
        return;
      }
    };
}

#[allow(non_snake_case)]
pub fn get_BufReader(file_path: &Path) -> Option<BufReader<File>> {
  match File::open(file_path) {
    Ok(f) => Some(BufReader::new(f)),
    Err(e) => {
      println!("Failed to open file {}", file_path.to_str().unwrap());
      println!("{}", e);
      None
    },
  }
}

#[allow(non_snake_case)]
pub fn get_CustomBufWriter(file_path: &Path) -> Option<crate::js::custom_buf_writer::CustomBufWriter<File>> {
  match File::create(file_path) {
    Ok(f) => Some(crate::js::custom_buf_writer::CustomBufWriter::new(f)),
    Err(e) => {
      println!("Failed to create file {}", file_path.to_str().unwrap());
      println!("{}", e);
      None
    },
  }
}
