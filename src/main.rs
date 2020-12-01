use std::path::Path;

mod tokens_and_nodes;
//mod tokenizer;
//mod parse_tree_generator;
//mod testing;

use crate::tokens_and_nodes::*;
//use crate::parse_tree_generator::*;
//use crate::testing::tree_traversal::*;

// use Result for error handling

fn main() {
    let file_path = Path::new(".").join("dummy-data").join("1dungeon.zil");

    let generator = match TokenGenerator::new(&file_path) {
        Some(v) => v,
        None => return,
    };

    for mt in generator {
        let t = mt.unwrap();
        println!("{0: <10} {1}", t.kind.to_str(), t.value);
    }

    /*
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
    */

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
