use std::fs::File;
use std::path::Path;
use std::io::BufReader;

mod tokens_and_nodes;
mod tokenizer;
mod parse_tree_generator;
mod testing;

use crate::tokens_and_nodes::*;
use crate::tokenizer::*;
use crate::parse_tree_generator::*;
use crate::testing::tree_traversal::*;

fn main() {
    let token_map = TokenType::get_map();
    
    let input_path = Path::new(".").join("edited-zork").join("test.txt");
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(_) => panic!("could not open file")
    };

    let reader = BufReader::new(file);

    let mut token_list = tokenize(reader, &token_map);

    let mut root = parse(&mut token_list);

    root = clean_tree(root);

    //root.describe(String::new());

    //print_functions(&root);
    combine_files(&root);

    // https://doc.rust-lang.org/1.30.0/book/second-edition/ch16-00-concurrency.html
}

