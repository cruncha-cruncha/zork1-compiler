use std::collections::{HashMap, HashSet};

use crate::stats::cross_ref::Populator;
use crate::stats::helpers::{get_nth_child_as_word, get_token_as_number, get_token_as_word};
use crate::stats::meta_handler::MetaHandler;
use crate::stats::validate_recursive::Validator;
use crate::zil::node::{TokenType, ZilNodeType};
use crate::zil::{file_table::format_file_location, node::ZilNode};

use crate::stats::cross_ref::Codex;

use regex::Regex;

use once_cell::sync::Lazy;

pub static ILLEGAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[\;\.\,\"\'\%\}\{\[\]\|\_<>]"#).unwrap());

pub struct RoutineStats {
    basis: HashMap<String, ZilNode>,
    pub sub_names: HashSet<String>,
    pub args_as_routines: HashSet<String>,
    pub aux_as_routines: HashSet<String>,
    pub info: HashMap<String, RoutineInfo>,
}

pub struct RoutineInfo {
    pub name: String,
    pub args: Vec<RoutineArg>,
    pub optional: Vec<RoutineArg>,
    pub aux: Vec<RoutineArg>,
}

pub struct RoutineArg {
    pub name: String,
    pub value: Option<ArgValue>,
}

pub enum ArgValue {
    Empty,
    Routine(ZilNode),
    Text(String),
    Word(String),
    Number(i32),
}

// <VERB? WORD1 WORD2 ...>
// corresponds to all syntax with = V-WORD1 or = V-WORD2 etc.

// <FSET? WORD1 WORD2>, <FCLEAR WORD1 WORD2>
// WORD1 is a room or object

// <QUEUE ...> is used to add events to CLOCKER

// any routine with (RARG) can only be called by a ROOM? or also an OBJECT?
// routine with ("AUX" ...) is just declaring variables
// routine with ("OPTIONAL" ...) declares optional arguments, and all must have a default value?
// order of args must be: regular, optional, aux

impl RoutineStats {
    pub fn new() -> RoutineStats {
        RoutineStats {
            basis: HashMap::new(),
            sub_names: HashSet::new(),
            args_as_routines: HashSet::new(),
            aux_as_routines: HashSet::new(),
            info: HashMap::new(),
        }
    }

    pub fn resolve_meta_code(&mut self, meta_handler: &MetaHandler) -> Result<bool, String> {
        // Meta code is not currently supported, just take the first option available

        let mut replaced_something = false;

        for (_k, n) in self.basis.iter_mut() {
            let status = meta_handler.replace_meta_recursively(n);

            if status.is_err() {
                return status;
            } else if status.unwrap_or_default() {
                replaced_something = true;
            }
        }

        Ok(replaced_something)
    }

    pub fn remove_decls(&mut self) {
        // Not supported and never will be, just remove them

        for (_k, n) in self.basis.iter_mut() {
            let mut i = 0;

            while i < n.children.len() {
                if n.children[i].node_type == ZilNodeType::Token(TokenType::Word) {
                    let name = get_token_as_word(&n.children[i]).unwrap();

                    if name == "#DECL" {
                        n.children.remove(i);

                        if (i + 1) < n.children.len()
                            && n.children[i].node_type == ZilNodeType::Group
                        {
                            n.children.remove(i);
                        }
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
        }
    }

    pub fn validate_recursive(&self, v: &Validator) -> Result<(), String> {
        for (_k, n) in self.basis.iter() {
            for (i, c) in n.children.iter().skip(3).enumerate() {
                if c.node_type == ZilNodeType::Token(TokenType::Word) {
                    let name = get_token_as_word(c).unwrap();

                    match name.as_str() {
                        "T" => {
                            if i + 1 != n.children.len() {
                                return Err(format!(
                                    "Routine node has T child that is not the last child\n{}",
                                    format_file_location(&c)
                                ));
                            }
                        }
                        _ => {
                            return Err(format!(
                                "Routine node has invalid child type\n{}",
                                format_file_location(&c)
                            ));
                        }
                    }
                }

                v.validate_cluster(c)?;
            }
        }

        Ok(())
    }
}

impl Populator for RoutineStats {
    fn add_node(&mut self, node: ZilNode) {
        let name = get_nth_child_as_word(1, &node);
        match name {
            Some(name) => {
                if ILLEGAL.is_match(&name) {
                    panic!("Routine node has illegal name {}", &name);
                }

                if self.basis.insert(name.clone(), node).is_some() {
                    panic!("Routine node has duplicate name {}", name);
                }
            }
            None => panic!("Routine node has no name\n{}", format_file_location(&node)),
        }
    }

    fn crunch(&mut self) -> Result<(), String> {
        for (k, n) in self.basis.iter() {
            let mut info = RoutineInfo {
                name: k.clone(),
                args: Vec::new(),
                optional: Vec::new(),
                aux: Vec::new(),
            };

            if n.children.len() <= 3 {
                return Err(format!(
                    "Routine node does not have enough children\n{}",
                    format_file_location(n)
                ));
            }

            match n.children[2].node_type {
                ZilNodeType::Group => match parse_args(&n.children[2]) {
                    Ok(args) => {
                        info.args = args.regular;
                        info.aux = args.aux;
                        info.optional = args.optional;
                    }
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Routine node has invalid third child type\n{}",
                        format_file_location(n)
                    ))
                }
            }

            self.info.insert(info.name.clone(), info);
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        Ok(())
    }
}

impl Codex for RoutineStats {
    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word)
    }
}

enum ArgMode {
    Regular,
    Optional,
    Aux,
}

struct ParsedArgs {
    regular: Vec<RoutineArg>,
    optional: Vec<RoutineArg>,
    aux: Vec<RoutineArg>,
}

fn parse_args(node: &ZilNode) -> Result<ParsedArgs, String> {
    let mut mode = ArgMode::Regular;
    let mut out = ParsedArgs {
        regular: Vec::new(),
        optional: Vec::new(),
        aux: Vec::new(),
    };

    let mut add_arg_to_out = |mode: &ArgMode, arg: RoutineArg| match mode {
        ArgMode::Regular => out.regular.push(arg),
        ArgMode::Optional => out.optional.push(arg),
        ArgMode::Aux => out.aux.push(arg),
    };

    for c in node.children.iter() {
        match c.node_type {
            ZilNodeType::Token(TokenType::Text) => {
                let val = c.get_first_token().unwrap().value.clone();
                match val.as_str() {
                    "OPTIONAL" => mode = ArgMode::Optional,
                    "AUX" => mode = ArgMode::Aux,
                    _ => {
                        return Err(format!(
                            "Routine node has invalid argument mode\n{}",
                            format_file_location(&c)
                        ));
                    }
                }
            }
            ZilNodeType::Token(TokenType::Word) => {
                let arg = RoutineArg {
                    name: get_token_as_word(c).unwrap(),
                    value: None,
                };

                add_arg_to_out(&mode, arg);
            }
            ZilNodeType::Group => {
                if c.children.len() != 2 {
                    return Err(format!(
                        "Routine node has invalid argument: group with not exactly two children\n{}",
                        format_file_location(&c)
                    ));
                }

                if c.children[0].node_type != ZilNodeType::Token(TokenType::Word) {
                    return Err(format!(
                        "Routine node has invalid argument: non-word first child\n{}",
                        format_file_location(&c)
                    ));
                }

                let name = get_token_as_word(&c.children[0]).unwrap();

                match c.children[1].node_type {
                    ZilNodeType::Token(TokenType::Text) => {
                        let arg = RoutineArg {
                            name: name,
                            value: Some(ArgValue::Text(
                                c.children[1].get_first_token().unwrap().value.clone(),
                            )),
                        };

                        add_arg_to_out(&mode, arg);
                    }
                    ZilNodeType::Token(TokenType::Word) => {
                        let arg = RoutineArg {
                            name: name,
                            value: Some(ArgValue::Word(get_token_as_word(&c.children[1]).unwrap())),
                        };

                        add_arg_to_out(&mode, arg);
                    }
                    ZilNodeType::Token(TokenType::Number) => {
                        let arg = RoutineArg {
                            name: name,
                            value: Some(ArgValue::Number(
                                get_token_as_number(&c.children[1]).unwrap(),
                            )),
                        };

                        add_arg_to_out(&mode, arg);
                    }
                    ZilNodeType::Cluster => {
                        let arg: RoutineArg;
                        if c.children[1].children.len() == 0 {
                            arg = RoutineArg {
                                name: name,
                                value: Some(ArgValue::Empty),
                            };
                        } else {
                            // TODO: parse more?
                            arg = RoutineArg {
                                name: name,
                                value: Some(ArgValue::Routine(c.children[1].clone())),
                            };
                        }

                        add_arg_to_out(&mode, arg);
                    }
                    _ => {
                        return Err(format!(
                            "Routine node has invalid argument\n{}",
                            format_file_location(&c)
                        ));
                    }
                }
            }
            _ => (),
        }
    }

    Ok(out)
}

// modify this to handle routine types more specifically?
fn recurse_sub_names(node: &ZilNode) -> Result<HashSet<String>, String> {
    let mut out: HashSet<String> = HashSet::new();

    if node.node_type != ZilNodeType::Cluster {
        // TODO: no! some routines have 'T' as their last child
        return Err(format!(
            "Routine node has non-cluster child in body\n{}",
            format_file_location(&node)
        ));
    }

    if node.children.len() == 0 {
        return Ok(out);
    }

    let name = match get_nth_child_as_word(0, node) {
        Some(name) => name,
        None => {
            return Err(format!(
                "Routine node has no name\n{}",
                format_file_location(&node)
            ));
        }
    };

    let mut iter = node.children.iter().skip(1);

    if name == "COND" {
        for c in node.children.iter().skip(1) {
            if c.node_type != ZilNodeType::Group {
                return Err(format!(
                    "Routine node has COND with non-group child\n{}",
                    format_file_location(&c)
                ));
            }

            // TODO: this doesn't work! we're inside a loop!
            iter = c.children.iter().skip(0);
        }
    } else if name == "REPEAT" {
        // first child has to be an empty group
        if node.children.len() < 2 {
            return Err(format!(
                "Routine node has REPEAT with too few children\n{}",
                format_file_location(&node)
            ));
        }

        if node.children[1].node_type != ZilNodeType::Group {
            return Err(format!(
                "Routine node has REPEAT with non-group second child\n{}",
                format_file_location(&&node.children[1])
            ));
        }

        if node.children[1].children.len() != 0 {
            return Err(format!(
                "Routine node has REPEAT with non-empty body\n{}",
                format_file_location(&&node.children[1])
            ));
        }
        iter = node.children.iter().skip(2);
    }

    for c in iter {
        match c.node_type {
            ZilNodeType::Cluster => match recurse_sub_names(c) {
                Ok(set) => {
                    out.extend(set);
                }
                Err(e) => return Err(e),
            },
            ZilNodeType::Token(_) => (),
            _ => {
                return Err(format!(
                    "Routine node has invalid child type\n{}",
                    format_file_location(&c)
                ))
            }
        }
    }

    Ok(out)
}
