use std::collections::{HashMap, HashSet};

use crate::stats::cross_ref::Populator;
use crate::stats::helpers::{get_nth_child_as_word, get_token_as_number, get_token_as_word};
use crate::stats::validate_recursive::Validator;
use crate::zil::node::{TokenType, ZilNodeType};
use crate::zil::{file_table::format_file_location, node::ZilNode};

use crate::stats::cross_ref::Codex;

use regex::Regex;

use once_cell::sync::Lazy;

pub static ILLEGAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[\;\.\,\"\'\%\}\{\[\]\|\_<>]"#).unwrap());

pub struct RoutineStats<'a> {
    basis: HashMap<String, &'a ZilNode>,
    pub sub_names: HashSet<String>,
    pub args_as_routines: HashSet<String>,
    pub aux_as_routines: HashSet<String>,
    pub info: HashMap<String, RoutineInfo<'a>>,
}

pub struct RoutineInfo<'a> {
    pub name: String,
    pub args: Vec<RoutineArg<'a>>,
    pub optional: Vec<RoutineArg<'a>>,
    pub aux: Vec<RoutineArg<'a>>,
    pub body: Vec<&'a ZilNode>,
}

pub struct RoutineArg<'a> {
    pub name: String,
    pub value: Option<ArgValue<'a>>,
}

pub enum ArgValue<'a> {
    Empty,
    Routine(&'a ZilNode),
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

impl<'a> RoutineStats<'a> {
    pub fn new() -> RoutineStats<'a> {
        RoutineStats {
            basis: HashMap::new(),
            sub_names: HashSet::new(),
            args_as_routines: HashSet::new(),
            aux_as_routines: HashSet::new(),
            info: HashMap::new(),
        }
    }

    pub fn validate_recursive(&self, v: &Validator) -> Result<(), String> {
        for (_k, n) in self.basis.iter() {
            for (i, c) in n.children.iter().skip(3).enumerate() {
                // TODO: most children should be a cluster, except for:
                // #DECL ( ... )
                // %< ... >
                // ... T>

                // I don't think we should support #DECL, but the other two need to be handled

                if c.node_type == ZilNodeType::Token(TokenType::Word) {
                    let name = get_token_as_word(c).unwrap();

                    match name.as_str() {
                        "#DECL" => {
                            return Err(format!(
                                "Routine node has #DECL child. This is not currently supported\n{}",
                                format_file_location(&c)
                            ));
                        }
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

    // also finds args_as_routines and aux_as_routines
    // we're missing routines used as default values in an arg, but should be good enough
    fn find_sub_names(&mut self) -> Result<(), String> {
        let mut top_sub_names: HashSet<String> = HashSet::new();

        for (k, n) in self.basis.iter() {
            let info = self.info.get(k).unwrap();

            for c in n.children.iter().skip(3) {
                let sub_names = recurse_sub_names(c)?;

                for arg in info.args.iter() {
                    if sub_names.contains(&arg.name) {
                        self.args_as_routines.insert(arg.name.clone());
                    }
                }

                for optional in info.optional.iter() {
                    if sub_names.contains(&optional.name) {
                        self.args_as_routines.insert(optional.name.clone());
                    }
                }

                for aux in info.aux.iter() {
                    if sub_names.contains(&aux.name) {
                        self.aux_as_routines.insert(aux.name.clone());
                    }
                }

                top_sub_names.extend(sub_names);
            }
        }

        for s in top_sub_names.clone().iter() {
            if self.basis.contains_key(s) {
                top_sub_names.remove(s);
            }
        }

        self.sub_names = top_sub_names;

        Ok(())
    }
}

impl<'a> Populator<'a> for RoutineStats<'a> {
    fn add_node(&mut self, node: &'a ZilNode) {
        let name = get_nth_child_as_word(1, node);
        match name {
            Some(name) => {
                if ILLEGAL.is_match(&name) {
                    panic!("Routine node has illegal name {}", &name);
                }

                if self.basis.insert(name, node).is_some() {
                    panic!(
                        "Routine node has duplicate name {}",
                        get_nth_child_as_word(1, node).unwrap()
                    );
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
                body: Vec::new(),
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

        // self.find_sub_names()?;

        Ok(())
    }

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        Ok(())
    }
}

impl<'a> Codex for RoutineStats<'a> {
    fn lookup(&self, word: &str) -> Option<&ZilNode> {
        self.basis.get(word).map(|n| *n)
    }
}

enum ArgMode {
    Regular,
    Optional,
    Aux,
}

struct ParsedArgs<'a> {
    regular: Vec<RoutineArg<'a>>,
    optional: Vec<RoutineArg<'a>>,
    aux: Vec<RoutineArg<'a>>,
}

fn parse_args<'a>(node: &'a ZilNode) -> Result<ParsedArgs<'a>, String> {
    let mut mode = ArgMode::Regular;
    let mut out = ParsedArgs {
        regular: Vec::new(),
        optional: Vec::new(),
        aux: Vec::new(),
    };

    let mut add_arg_to_out = |mode: &ArgMode, arg: RoutineArg<'a>| match mode {
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
                                value: Some(ArgValue::Routine(&c.children[1])),
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
