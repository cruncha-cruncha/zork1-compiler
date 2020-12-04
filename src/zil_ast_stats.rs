use std::collections::HashSet;

use crate::zil_ast::*;

// <X ... >
pub fn stat_a(root: &Node, collector: &mut HashSet<String>) {
    if root.is_routine() &&
       root.has_children() {
        collector.insert(String::from(&root.children[0].tokens[0].value));
    }
    for n in root.children.iter() {
        stat_a(n, collector);
    }
}

// <ROUTINE X ... >
pub fn stat_b(root: &Node, collector: &mut HashSet<String>) {
    if root.is_routine() &&
       root.has_children() &&
       root.children[0].is_word() &&
       root.children[0].tokens[0].value == String::from("ROUTINE") && 
       root.children.len() >= 2 {
        collector.insert(String::from(&root.children[1].tokens[0].value));
    }
    for n in root.children.iter() {
        stat_b(n, collector);
    }
}

// <GLOBAL X ... >
pub fn stat_c(root: &Node, collector: &mut HashSet<String>) {
    if root.is_routine() &&
       root.has_children() &&
       root.children[0].is_word() &&
       root.children[0].tokens[0].value == String::from("GLOBAL") && 
       root.children.len() >= 2 {
        collector.insert(String::from(&root.children[1].tokens[0].value));
    }
    for n in root.children.iter() {
        stat_c(n, collector);
    }
}

// <OBJECT X ... >
pub fn stat_d(root: &Node, collector: &mut HashSet<String>) {
    if root.is_routine() &&
       root.has_children() &&
       root.children[0].is_word() &&
       root.children[0].tokens[0].value == String::from("OBJECT") && 
       root.children.len() >= 2 {
        collector.insert(String::from(&root.children[1].tokens[0].value));
    }
    for n in root.children.iter() {
        stat_d(n, collector);
    }
}

// (X ... )
pub fn stat_e(root: &Node, collector: &mut HashSet<String>) {
    if root.is_grouping() &&
       root.has_children() {
        collector.insert(String::from(&root.children[0].tokens[0].value));
    }
    for n in root.children.iter() {
        stat_e(n, collector);
    }
}

// <COND ... (X ... ) ... >
pub fn stat_f(root: &Node, collector: &mut HashSet<String>) {
    if root.is_routine() &&
       root.has_children() &&
       root.children[0].is_word() &&
       root.children[0].tokens[0].value == String::from("COND") {
        for n in root.children.iter() {
            if n.is_grouping() &&
               n.has_children() {
                collector.insert(String::from(&n.children[0].tokens[0].value));
            }
        }
    }
    for n in root.children.iter() {
        stat_f(n, collector);
    }
}

// <OBJECT ... (X ... ) ... >
pub fn stat_g(root: &Node, collector: &mut HashSet<String>) {
    if root.is_routine() &&
       root.has_children() &&
       root.children[0].is_word() &&
       root.children[0].tokens[0].value == String::from("OBJECT") {
        for n in root.children.iter() {
            if n.is_grouping() &&
               n.has_children() {
                collector.insert(String::from(&n.children[0].tokens[0].value));
            }
        }
    }
    for n in root.children.iter() {
        stat_g(n, collector);
    }
}

// all WORDs that start with "."
pub fn stat_h(root: &Node, collector: &mut HashSet<String>) {
    if root.is_word() &&
       root.tokens[0].value.starts_with(".") {
        collector.insert(String::from(&root.tokens[0].value));
    }
    for n in root.children.iter() {
        stat_h(n, collector);
    }
}

// all WORDs that start with ","
pub fn stat_i(root: &Node, collector: &mut HashSet<String>) {
    if root.is_word() &&
       root.tokens[0].value.starts_with(",") {
        collector.insert(String::from(&root.tokens[0].value));
    }
    for n in root.children.iter() {
        stat_i(n, collector);
    }
}

