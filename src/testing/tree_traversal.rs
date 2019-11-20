use std::fs::File;
use std::collections::HashMap;

use crate::tokens_and_nodes::*;

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
// TokenType::LeftArrow, NodeType::FullWord (TokenType::Word .value == "INSERT-FILE"), NodeType::FullQuote, NodeType::FullWord (TokenType::Word .value == "T"), TokenType::RightArrow

pub fn combine_files(root: &NodeWrapper) {
    if !root.is_node() {
        return;
    }

    let children = &root.borrow_node().children;

    for child in children {
        if child.is_node() {
            let tmp_node = child.borrow_node();
            if matching!(&tmp_node.name, NodeType::PointyFunc) {
                for i in 0..tmp_node.children.len() {
                    if tmp_node.children[i].is_node() {
                        let tmp_node = tmp_node.children[i].borrow_node();
                        if matching!(&tmp_node.name, NodeType::FullWord) {
                            let tmp_token = tmp_node.children[0].borrow_token();
                            if tmp_token.value == "INSERT-FILE" {
                                //
                            }
                        }
                    }
                }
            }
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

