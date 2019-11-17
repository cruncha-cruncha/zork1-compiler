use std::fs::File;
use std::path::Path;
use std::io::BufReader;

mod tokens_and_nodes;
mod tokenizer;
mod parse_tree_generator;

use crate::tokens_and_nodes::*;
use crate::tokenizer::*;
use crate::parse_tree_generator::*;

fn main() {
    let token_map = get_token_type_map();
    
    let input_path = Path::new(".").join("edited-zork").join("test.txt");
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(_) => panic!("could not open file")
    };

    let reader = BufReader::new(file);

    let mut token_list = tokenize(reader, &token_map);

    /*
    for t in &token_list {
        println!("{}", t.value);  
    }
    */

    let root = parse(&mut token_list);

    //root.describe(String::new());

    // https://doc.rust-lang.org/1.30.0/book/second-edition/ch16-00-concurrency.html
}

