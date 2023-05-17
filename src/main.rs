use std::{path::Path, sync::{Arc, Condvar, Mutex}, thread, time};

use crate::{
    stats::top_level::Codex,
    zil::{ast, mess::Huh},
};

mod stats;
mod zil;

fn main() {
    let mut files_lookup = zil::file_table::FileTable::new();

    let mut file_path = Path::new("..").join("dummy-data").join("test1.zil");
    files_lookup.add(file_path);
    // file_path = Path::new("..").join("dummy-data").join("test2.zil");
    // files_lookup.add(file_path);
    println!("{}", files_lookup);

    let mut char_gen = zil::char_gen::new(&mut files_lookup);
    let mut token_gen = zil::token_gen::new(&mut char_gen);

    let tree = match ast::build_tree(&mut token_gen) {
        Ok(t) => t,
        Err(e) => panic!("\nERROR\n{}", e),
    };

    //ast::print(tree.get_root());

    let mut lookup = stats::top_level::CrossRef::new(&tree);
    lookup.find_stuff();


    let mut weaver = stats::weaver::Sigourney::new();
    let recv = weaver.run_fn(|| lookup.globals.crunch());

    recv.recv().unwrap().unwrap();

    for n in lookup.globals.into_iter() {
        println!("{}", n);
    }

    // for v in rooms.subgroup_names.iter() {
    //     println!("{}", v);
    // }

    // //inter::ast_stats::run_all(&root);

    // let root = match inter::ast::clone_zil_tree(&root) {
    //   Ok(v) => {
    //     println!("built inter tree");
    //     v
    //   },
    //   Err(e) => {
    //     println!("\nERROR\n{}", e);
    //     zil::ast::print_tree(&root, 0);
    //     return;
    //   }
    // };

    // //inter::ast::print_tree(&root, 0);

    // let root = js::node::JSNode::clone_internode(&root);

    // let output_file_path = Path::new(".").join("out").join("testing.js");
    // let writer = get_CustomBufWriter(&output_file_path).unwrap();
    // match js::parse::parse(&root, writer) {
    //   Ok(_) => println!("output ok"),
    //   Err(_) => {
    //     println!("\nBAD OUTPUT\n");
    //     return;
    //   }
    // };
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
