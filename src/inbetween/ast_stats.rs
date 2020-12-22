use std::collections::HashSet;

use crate::zil::ast::*;

pub trait TreeStat {
    fn desc(&self) -> String;
    fn calc(&self, root: &Node, collector: &mut HashSet<String>);
}

#[allow(dead_code)]
fn set_to_str(collector: &HashSet<String>) -> String {
    let mut out = String::new();
    for s in collector.iter() {
        out.push_str(s);
        out.push_str(", ");
    }
    out
}

#[allow(dead_code)]
pub fn run_all(root: &Node) {
    let mut stats: Vec<Box<dyn TreeStat>> = Vec::new();
    stats.push(Box::new(Stat01{}));
    stats.push(Box::new(Stat02{}));
    stats.push(Box::new(Stat03{}));
    stats.push(Box::new(Stat12{}));
    stats.push(Box::new(Stat04{}));
    stats.push(Box::new(Stat05{}));
    stats.push(Box::new(Stat06{}));
    stats.push(Box::new(Stat07{}));
    stats.push(Box::new(Stat08{}));
    stats.push(Box::new(Stat09{}));
    stats.push(Box::new(Stat11{}));

    let mut collector: HashSet<String>;
    for b in stats.iter() {
        collector = HashSet::new();
        println!("\n{}", b.desc());
        b.calc(&root, &mut collector);
        println!("{}", set_to_str(&collector));
    }
}

#[allow(dead_code)]
pub fn dot_comma(root: &Node) {
    let dot_stats = Stat09{};
    println!("\n{}", dot_stats.desc());
    let mut dot_collector: HashSet<String> = HashSet::new();
    dot_stats.calc(&root, &mut dot_collector);
    println!("{}", set_to_str(&dot_collector));

    let comma_stats = Stat11{};
    println!("\n{}", comma_stats.desc());
    let mut comma_collector: HashSet<String> = HashSet::new();
    comma_stats.calc(&root, &mut comma_collector);
    println!("{}", set_to_str(&comma_collector));

    println!("\nIntersection");
    println!("{}", set_to_str(&dot_collector.intersection(&comma_collector).cloned().collect()));
}

pub struct Stat01;
impl TreeStat for Stat01 {
    fn desc(&self) -> String {
        String::from("<X ... >")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
        if root.is_routine() &&
           root.has_children() {
            collector.insert(String::from(&root.children[0].tokens[0].value));
        }
        for n in root.children.iter() {
            self.calc(n, collector);
        }
    }
}

pub struct Stat02;
impl TreeStat for Stat02 {
    fn desc(&self) -> String {
        String::from("<ROUTINE X ... >")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
        if root.is_routine() &&
           root.has_children() &&
           root.children[0].is_word() &&
           root.children[0].tokens[0].value == String::from("ROUTINE") && 
           root.children.len() >= 2 {
            collector.insert(String::from(&root.children[1].tokens[0].value));
        }
        for n in root.children.iter() {
            self.calc(n, collector);
        }
    }
}

pub struct Stat03;
impl TreeStat for Stat03 {
    fn desc(&self) -> String {
        String::from("<GLOBAL X ... >")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
        if root.is_routine() &&
           root.has_children() &&
           root.children[0].is_word() &&
           root.children[0].tokens[0].value == String::from("GLOBAL") && 
           root.children.len() >= 2 {
            collector.insert(String::from(&root.children[1].tokens[0].value));
        }
        for n in root.children.iter() {
            self.calc(n, collector);
        }
    }
}

pub struct Stat04;
impl TreeStat for Stat04 {
    fn desc(&self) -> String {
        String::from("<OBJECT X ... >")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
        if root.is_routine() &&
           root.has_children() &&
           root.children[0].is_word() &&
           root.children[0].tokens[0].value == String::from("OBJECT") && 
           root.children.len() >= 2 {
            collector.insert(String::from(&root.children[1].tokens[0].value));
        }
        for n in root.children.iter() {
            self.calc(n, collector);
        }
    }
}

pub struct Stat05;
impl TreeStat for Stat05 {
    fn desc(&self) -> String {
        String::from("(X ... )")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
        if root.is_grouping() &&
           root.has_children() {
            collector.insert(String::from(&root.children[0].tokens[0].value));
        }
        for n in root.children.iter() {
            self.calc(n, collector);
        }
    }
}

pub struct Stat06;
impl TreeStat for Stat06 {
    fn desc(&self) -> String {
        String::from("<COND ... (X ... ) ... >")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
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
            self.calc(n, collector);
        }
    }
}

pub struct Stat07;
impl TreeStat for Stat07 {
    fn desc(&self) -> String {
        String::from("(X ... ) but not <COND ... (X ... ) ... >")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
        if root.has_children() {
            if !root.children[0].is_word() ||
               root.children[0].tokens[0].value != String::from("COND") {
                for n in root.children.iter() {
                    if n.is_grouping() &&
                    n.has_children() {
                        collector.insert(String::from(&n.children[0].tokens[0].value));
                    }
                }
            }
        }

        for n in root.children.iter() {
            self.calc(n, collector);
        }
    }
}

pub struct Stat08;
impl TreeStat for Stat08 {
    fn desc(&self) -> String {
        String::from("<OBJECT ... (X ... ) ... >")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
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
            self.calc(n, collector);
        }
    }
}

pub struct Stat09;
impl TreeStat for Stat09 {
    fn desc(&self) -> String {
        String::from("WORDs that start with \".\"")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
        if root.is_word() &&
           root.tokens[0].value.starts_with(".") {
            collector.insert(String::from(&root.tokens[0].value[1..]));
        }
        for n in root.children.iter() {
            self.calc(n, collector);
        }
    }
}

pub struct Stat11;
impl TreeStat for Stat11 {
    fn desc(&self) -> String {
        String::from("WORDs that start with \",\"")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
        if root.is_word() &&
           root.tokens[0].value.starts_with(",") {
            collector.insert(String::from(&root.tokens[0].value[1..]));
        }
        for n in root.children.iter() {
            self.calc(n, collector);
        }
    }
}

pub struct Stat12;
impl TreeStat for Stat12 {
    fn desc(&self) -> String {
        String::from("<CONSTANT X ... >")
    }

    fn calc(&self, root: &Node, collector: &mut HashSet<String>) {
        if root.is_routine() &&
           root.has_children() &&
           root.children[0].is_word() &&
           root.children[0].tokens[0].value == String::from("CONSTANT") && 
           root.children.len() >= 2 {
            collector.insert(String::from(&root.children[1].tokens[0].value));
        }
        for n in root.children.iter() {
            self.calc(n, collector);
        }
    }
}

