use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io;

mod tokens_and_nodes;
//mod tokenizer;
//mod parse_tree_generator;
//mod testing;

use crate::tokens_and_nodes::*;
//use crate::parse_tree_generator::*;
//use crate::testing::tree_traversal::*;

// use Result for error handling

fn main() {
    let input_path = Path::new(".").join("dummy-data").join("1dungeon.zil");

    let reader = match open_file(&input_path) {
        Ok(v) => v,
        Err(e) => {
            println!("Error opening input_path file: {:?}", e);
            return
        }
    };

    let tokens = match tokenize(reader) {
        Ok(v) => v,
        Err(e) => {
            println!("Error tokenizing file, {:?}", e);
            return
        }
    };

    for t in tokens.iter() {
        println!("{0: <10} {1}", t.kind.to_str(), t.value);
    }
    println!();

    /*

    root = combine_files(root);

    let functions = get_functions(&root);
    let routines = get_routines(&root);

    struct FuncOccur {
        func: String,
        occur: usize
    }

    let mut sorted = Vec::new();

    for (k, v) in &functions {
        //println!("{}", k);
        match routines.get(k) {
            Some(_) => {},
            None => sorted.push(FuncOccur{ func: k.to_string(), occur: *v })
        }
    }

    println!("found {} functions", sorted.len());
    println!("");

    sorted.sort_by(|a, b| b.occur.cmp(&a.occur));
    for kv in &sorted {
        println!("{} {}", kv.occur, kv.func);
    }

    //root.describe(String::new());
    */
}

pub fn open_file(input_path: &Path) -> Result<BufReader<File>, io::Error> {
    let file = File::open(input_path)?;
    return Ok(BufReader::new(file));
}
