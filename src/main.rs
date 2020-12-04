use std::path::Path;
use std::collections::HashMap;

mod tokenize;
mod zil_ast;
//mod tokenizer;
//mod parse_tree_generator;
//mod testing;

use crate::tokenize::*;
use crate::zil_ast::*;
//use crate::parse_tree_generator::*;
//use crate::testing::tree_traversal::*;

pub struct FileNameTable {
    key: u32,
    table: HashMap<u32, String>,
}

impl FileNameTable {
    pub fn new() -> FileNameTable {
        FileNameTable {key: 0, table: HashMap::new()}
    }

    pub fn insert(&mut self, v: String) -> u32 {
        self.key += 1;
        self.table.insert(
            self.key,
            v,
        );
        self.key
    }

    pub fn get(&mut self, k: u32) -> Option<String> {
        match self.table.get(&k) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub fn find_key(&mut self, v: String) -> Option<u32> {
        for (key, value) in self.table.iter() {
            if *value == v {
                return Some(*key);
            }
        }

        None
    }
}

fn main() {
    let mut files_lookup = FileNameTable::new();

    let file_path = Path::new(".").join("dummy-data").join("1dungeon.zil");
    let file_key = files_lookup.insert(file_path.to_str().unwrap().to_string());

    let mut generator = match TokenGenerator::new(file_key, &file_path) {
        Some(v) => v,
        None => return,
    };

    let mut root = Node::new();
    build_tree(&mut generator, &mut root);

    match validate_tree(&root, 0, &mut files_lookup) {
        Ok(()) => println!("ok"),
        Err(()) => {
            println!("ERROR");
            print_tree(&root, 0);
        },
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
