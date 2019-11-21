use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::collections::HashMap;

use crate::tokens_and_nodes::*;
use crate::tokenizer::*;
use crate::parse_tree_generator::*;

macro_rules! matching {
    ($a: expr, $b: pat) => {
        match $a {
            $b => true,
            _ => false
        }
    }
}

// root
// NodeType::PointyFunc
// NodeType::FullWord (TokenType::Word .value == "INSERT-FILE"), NodeType::FullQuote, NodeType::FullWord (TokenType::Word .value == "T")

// returns true as long as all the NodeWrappers in fake are identical to the corresponding NodeWrappers in real
// (real can be larger than fake and this function may still return true)
pub fn tree_compare(real: &NodeWrapper, fake: &NodeWrapper) -> bool {
    if real.is_node() && fake.is_node() {
        let tmp_name = &fake.borrow_node().name;
        match &real.borrow_node().name {
            tmp_name => {
                let fake_len = fake.borrow_node().children.len();
                if real.borrow_node().children.len() >= fake_len {
                    for i in 0..fake_len {
                        if !tree_compare(&real.borrow_node().children[i], &fake.borrow_node().children[i]) {
                            return false;
                        }
                    }
                    return true;
                }
            }, 
            _ => return false
        };
    } else if real.is_token() && fake.is_token() {
        let tmp_name = &fake.borrow_token().name;
        match &real.borrow_token().name {
            tmp_name => return (real.borrow_token().value == fake.borrow_token().value),
            _ => return false
        };
    }

    false
}

pub fn combine_files(root: &NodeWrapper) {
    let token_map = TokenType::get_map();
    let input_path = Path::new(".").join("src").join("testing").join("insert-file.zil");
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(_) => panic!()
    };
    let reader = BufReader::new(file);
    let mut token_list = tokenize(reader, &token_map);
    let mut fake = clean_tree(parse(&mut token_list));
    fake = fake.remove_child(0);

    combine_recursive(root, &fake);
}

fn combine_recursive(root: &NodeWrapper, fake: &NodeWrapper) {
    if tree_compare(root, fake) {
        // will have to track line and character numbers
        root.borrow_node().children[1].borrow_node().describe(String::new());
    } else if root.is_node() {
        for i in 0..root.borrow_node().children.len() {
            combine_recursive(&root.borrow_node().children[i], &fake);
        }
    }
}

// need to find functions defined within the source files, by looking
// at the FullWords that come after ROUTINE

pub fn print_functions(root: &NodeWrapper) {
    let mut out = HashMap::new();
    out = find_functions_recursively(root, out);

    struct FuncOccur {
        func: String,
        occur: usize
    }

    let mut sorted = Vec::new();
    for (k, v) in out.iter() {
        sorted.push(FuncOccur{ func: k.to_string(), occur: *v });
    }
    out.clear();

    println!("found {} functions", sorted.len());
    println!("");

    sorted.sort_by(|a, b| b.occur.cmp(&a.occur));
    for kv in &sorted {
        println!("{} {}", kv.occur, kv.func);
    }
}

fn find_functions_recursively(nw: &NodeWrapper, mut out: HashMap<String, usize>) -> HashMap<String, usize> {
    if nw.is_node() {
        let tmp_node = nw.borrow_node();
        if matching!(&tmp_node.name, NodeType::PointyFunc) {
            for i in 0..tmp_node.children.len() {
                if tmp_node.children[i].is_node() {
                    let tmp_node = tmp_node.children[i].borrow_node();
                    if matching!(&tmp_node.name, NodeType::FullWord) {
                        let mut key = String::new();
                        for nw in &tmp_node.children {
                            let tmp_token = nw.borrow_token();
                            key.push_str(&tmp_token.value);
                        }
                        match out.get(&key) {
                            Some(_) => {
                                *out.get_mut(&key).unwrap() += 1;
                            },
                            None => {
                                out.insert(key, 1);
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    if nw.is_node() {
        let children = &nw.borrow_node().children;
        for nw in children {
            out = find_functions_recursively(nw, out);
        }
    }

    out
}

