use std::{fs, path::Path};

extern crate once_cell;
extern crate regex;

use js::write_output::write_output;

use crate::zil::ast;

mod js;
mod stats;
mod zil;

fn main() {
    let mut thread_pool = stats::weaver::Sigourney::new(4);

    let mut files_lookup = get_files_lookup();
    println!("{}", files_lookup);

    let tree = build_tree(&mut files_lookup);
    // ast::print(&tree.root);

    let mut lookup = stats::cross_ref::CrossRef::new(tree);
    lookup.add_nodes();
    match lookup.crunch_top_level(&mut thread_pool) {
        Ok(_) => println!("lookups crunched"),
        Err(e) => {
            let mut out = String::from("ERROR while crunching lookups\n");
            if e.len() > 10 {
                for i in 0..10 {
                    out.push_str(&format!("{}\n", e[i]));
                }
                out.push_str("...\n");
            } else {
                for err in e {
                    out.push_str(&format!("{}\n", err));
                }
            }
            panic!("{}", out);
        }
    }

    match lookup.validate_unique_names() {
        Ok(_) => println!("names are unique"),
        Err(e) => panic!("ERROR while validating names\n{}", e),
    }

    let mut validator = match lookup.validate_routines_recursive() {
        Ok(v) => {
            println!("routines are valid");
            v
        }
        Err(e) => panic!("ERROR while validating routines\n{}", e),
    };

    match write_output(&lookup, &mut validator) {
        Ok(_) => println!("output written"),
        Err(e) => panic!("ERROR while writing output\n{}", e),
    }
}

fn get_files_lookup() -> zil::file_table::FileTable {
    let mut files_lookup = zil::file_table::FileTable::new();

    let paths = fs::read_dir(Path::new(".").join("input-files")).unwrap();

    for path in paths {
        files_lookup.add(path.unwrap().path());
    }

    files_lookup
}

fn build_tree(mut files_lookup: &mut zil::file_table::FileTable) -> zil::ast::Tree {
    let mut char_gen = zil::char_gen::new(&mut files_lookup);
    let mut token_gen = zil::token_gen::new(&mut char_gen);

    let tree = match ast::build_tree(&mut token_gen) {
        Ok(t) => t,
        Err(e) => panic!("\nERROR\n{}", e),
    };

    tree
}

// #[allow(non_snake_case)]
// pub fn get_CustomBufWriter(file_path: &Path) -> Option<crate::js::custom_buf_writer::CustomBufWriter<File>> {
//   match File::create(file_path) {
//     Ok(f) => Some(crate::js::custom_buf_writer::CustomBufWriter::new(f)),
//     Err(e) => {
//       println!("Failed to create file {}", file_path.to_str().unwrap());
//       println!("{}", e);
//       None
//     },
//   }
// }
