use std::path::Path;

mod tokens_and_nodes;
mod tokenizer;
mod parse_tree_generator;
mod testing;

use crate::tokens_and_nodes::*;
use crate::parse_tree_generator::*;
use crate::testing::tree_traversal::*;

// use Result for error handling

fn main() {
    let input_path = Path::new(".").join("edited-zork").join("test1.zil");
    let mut root = read_file_to_tree(&input_path).unwrap();

    //

    //print_functions(&root);
    root = combine_files(root);

    root.describe(String::new());
}

