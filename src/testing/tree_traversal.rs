use std::path::Path;
use std::collections::HashMap;
use std::mem::discriminant;

use crate::tokens_and_nodes::*;
use crate::parse_tree_generator::*;

macro_rules! matching {
    ($a: expr, $b: pat) => {
        match $a {
            $b => true,
            _ => false
        }
    }
}

#[allow(dead_code)]
// returns true as long as all the NodeWrappers in fake are identical to the corresponding NodeWrappers in real
// (real can be larger than fake and this function may still return true)
pub fn tree_compare(real: &NodeWrapper, fake: &NodeWrapper) -> bool {
    if real.is_node() && fake.is_node() {
        if discriminant(&real.borrow_node().name) == discriminant(&fake.borrow_node().name) {
            let fake_len = fake.borrow_node().children.len();
                if real.borrow_node().children.len() >= fake_len {
                    for i in 0..fake_len {
                        if !tree_compare(&real.borrow_node().children[i], &fake.borrow_node().children[i]) {
                            return false;
                        }
                    }
                    return true;
                }
        } else {
            return false;
        }
    } else if real.is_token() && fake.is_token() {
        if discriminant(&real.borrow_token().name) == discriminant(&fake.borrow_token().name) {
            return real.borrow_token().value == fake.borrow_token().value;
        } else {
            return false;
        }
    }

    false
}

#[allow(dead_code)]
pub fn combine_files(root: NodeWrapper) -> NodeWrapper {
    let input_path = Path::new(".").join("src").join("testing").join("insert-file.zil");
    let mut fake = read_file_to_tree(&input_path).unwrap();
    fake = fake.remove_child(0);

    combine_recursive(root, &fake)
}

#[allow(dead_code)]
// does not guard against cyclic INSERT-FILEs
fn combine_recursive(mut root: NodeWrapper, fake: &NodeWrapper) -> NodeWrapper {
    match root.data {
        TokenOrNode::Node(mut n) => {
            let mut recycle = Vec::new();
            while n.children.len() > 0 {
                let mut child = n.children.remove(0);
                loop {
                    if tree_compare(&child, fake) {
                        let file_name = format!("{}{}", child.borrow_node().children[1].borrow_node().children[0].borrow_token().value, ".zil");
                        //let new_input_path = Path::new(".").join("edited-zork").join(file_name);
                        let new_input_path = Path::new(".").join("zork1-master").join(file_name);
                        let new_tree = read_file_to_tree(&new_input_path).unwrap();
                        match new_tree.data {
                            TokenOrNode::Node(mut new_n) => {
                                match new_n.name {
                                    NodeType::FunkyBunch => {
                                        child = new_n.children.remove(0);
                                        if new_n.children.len() > 0 {
                                            for i in (0..new_n.children.len()).rev() {
                                                n.children.insert(0, new_n.children.remove(i));
                                            }
                                        }
                                    },
                                    _ => panic!()
                                };
                            },
                            _ => panic!()
                        };
                    } else {
                        break;
                    }
                }
                recycle.push(combine_recursive(child, fake));
            }
            root.data = TokenOrNode::Node(Node { name: n.name, children: recycle });
        },
        TokenOrNode::Token(_) => {}
    };

    root
}

#[allow(dead_code)]
pub fn get_functions(root: &NodeWrapper) -> HashMap<String, usize> {
    let out = HashMap::new();
    find_functions_recursively(root, out)

    /*
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
    */
}

#[allow(dead_code)]
fn find_functions_recursively(nw: &NodeWrapper, mut out: HashMap<String, usize>) -> HashMap<String, usize> {
    if nw.is_node() {
        match &nw.borrow_node().name {
            NodeType::PointyFunc => {
                if nw.borrow_node().children.len() == 0 {
                    let key = "<empty>".to_string();
                    match out.get(&key) {
                        Some(_) => {
                            *out.get_mut(&key).unwrap() += 1;
                        },
                        None => {
                            out.insert(key, 1);
                        }
                    }
                } else {
                    let tmp_node = nw.borrow_node().children[0].borrow_node();
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
                }
            },
            _ => {}
        }
    }

    if nw.is_node() {
        match &nw.borrow_node().name {
            NodeType::FullComment => {},
            _ => {
                for nw in &nw.borrow_node().children {
                    out = find_functions_recursively(nw, out);
                }
            }
        }
    }

    out
}

#[allow(dead_code)]
pub fn get_routines(root: &NodeWrapper) -> HashMap<String, usize> {
    let input_path = Path::new(".").join("src").join("testing").join("routine.zil");
    let mut fake = read_file_to_tree(&input_path).unwrap();
    fake = fake.remove_child(0);

    let out = HashMap::new();
    find_routines_recursively(root, &fake, out)

    /*
    let mut out = HashMap::new();
    out = find_routines_recursively(root, &fake, out);

    struct RoutineOccur {
        routine: String,
        occur: usize
    }

    let mut sorted = Vec::new();
    for (k, v) in out.iter() {
        sorted.push(RoutineOccur{ routine: k.to_string(), occur: *v });
    }
    out.clear();

    println!("found {} routines", sorted.len());
    println!("");

    sorted.sort_by(|a, b| b.occur.cmp(&a.occur));
    for kv in &sorted {
        println!("{} {}", kv.occur, kv.routine);
    }
    */
}

#[allow(dead_code)]
fn find_routines_recursively(nw: &NodeWrapper, fake: &NodeWrapper, mut out: HashMap<String, usize>) -> HashMap<String, usize> {
    if nw.is_node() {
        if tree_compare(&nw, fake) {
            let tmp_node = nw.borrow_node().children[1].borrow_node();
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
        }
    }

    if nw.is_node() {
        match &nw.borrow_node().name {
            NodeType::FullComment => {},
            _ => {
                for nw in &nw.borrow_node().children {
                    out = find_routines_recursively(nw, fake, out);
                }
            }
        }
    }

    out
}

