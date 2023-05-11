use std::fs::File;
use std::io::{BufReader};
use std::path::Path;

mod stats;
mod zil;
// mod js;
// mod inter;
// #[cfg(test)]
// mod tests;

fn main() {
    let mut files_lookup = zil::file_table::FileTable::new();

    let mut file_path = Path::new("..").join("dummy-data").join("test1.zil");
    files_lookup.add(file_path);
    file_path = Path::new("..").join("dummy-data").join("test2.zil");
    files_lookup.add(file_path);
    println!("{}", files_lookup);

    let char_gen = zil::char_gen::new(&mut files_lookup);
    let mut token_gen = zil::token_gen::new(char_gen);

    let tree = match zil::ast::build_tree(&mut token_gen) {
        Ok(t) => t,
        Err(e) => panic!("\nERROR\n{}", e),
    };

    tree.print();

    // let mut lookup = stats::top_level::CrossRef::new(&tree);
    // lookup.populate();

    // for k in lookup.rooms.keys() {
    //     println!("{}", k);
    // }

    // let mut rooms = stats::rooms::RoomCodex::new(&lookup.rooms);
    // rooms.populate();

    // for v in rooms.subgroups.iter() {
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
