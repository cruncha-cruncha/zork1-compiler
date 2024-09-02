use std::{fs, path::Path};

extern crate once_cell;
extern crate regex;

use crate::zil::ast;

mod js;
mod stats;
mod zil;

fn main() {
    let mut thread_pool = stats::weaver::Sigourney::new(4);

    let mut files_lookup = get_files_lookup();
    println!("{}", files_lookup);

    let tree = build_tree(&mut files_lookup);
    // ast::print(tree.get_root());

    let mut lookup = stats::cross_ref::CrossRef::new(&tree);
    lookup.add_nodes();
    match lookup.crunch_top_level(&mut thread_pool) {
        Ok(_) => println!("lookups crunched"),
        Err(e) => panic!("ERROR while crunching lookups\n{}", e),
    }

    println!("");

    match lookup.validate_routines() {
        Ok(_) => println!("lookups validated"),
        Err(e) => panic!("ERROR while validating lookups\n{}", e),
    }

    println!("");

    // write_rooms("./out/rooms.js", &lookup.rooms);
    // write_objects("./out/objects.js", &lookup.objects);
    // write_globals_and_constants(
    //     "./out/globals_and_constants.js",
    //     &lookup.constants,
    //     &lookup.globals,
    // );
    // write_syntax("./out/syntax.js", &lookup.syntax);
    // write_other_syntax(
    //     "./out/other_syntax.js",
    //     &lookup.syntax,
    //     &lookup.synonyms,
    //     &lookup.directions,
    //     &lookup.buzzi,
    // );

    // for n in lookup.leftovers.iter() {
    //     match n.node_type {
    //         zil::node::ZilNodeType::Cluster => {
    //             if n.children.len() >= 1 {
    //                 match stats::helpers::get_nth_child_name(0, n) {
    //                     Some(name) => println!("Cluster {}", name),
    //                     None => panic!("Cluster has no name"),
    //                 }
    //             } else {
    //                 println!("Empty cluster");
    //             }
    //         },
    //         zil::node::ZilNodeType::Token(x) => {
    //             match x {
    //                 zil::node::TokenType::Word => {
    //                     println!("Token bunch {}", stats::helpers::get_bunch_name(n).unwrap());
    //                 },
    //                 _ => println!("Token bunch type {:?}", x),
    //             }

    //         },
    //         _ => println!("Unkown leftover type {}", n.node_type),
    //     }
    // }

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
