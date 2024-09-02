use std::collections::{HashMap, HashSet};

use crate::{
    stats::{cross_ref::Codex, helpers::get_token_as_word},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use crate::stats::cross_ref::Populator;

use regex::Regex;

use once_cell::sync::Lazy;

pub static ILLEGAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"[\.\,\?\!\"\'\}\{\[\]\|\_<>]"#).unwrap());

static VERB: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^V-(?P<raw>.*)$"#).unwrap());

static PRE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^PRE-(?P<raw>.*)$"#).unwrap());

static MAX_CMD_LENGTH: usize = 16;

// https://medium.com/swlh/zork-the-great-inner-workings-b68012952bdc
// PRSA = action
// PRSO = direct object
// PRSI = indirect object
// execute actions in what order?

// have to know our LOC (location) to know what synonyms + adjectives we can apply

// all commands are of the form:
// <SYNTAX (PRSA) ...>
// <SYNTAX (PRSA) ... (PRSO)>
// <SYNTAX (PRSA) ... (PRSO) ... (PRSI) ...>
// first word (or group of words?) is always the action
// first OBJECT is always PRSO
// second OBJECT is always PRSI

pub struct SyntaxStats<'a> {
    basis: Vec<&'a ZilNode>,
    pub all_syntax: HashMap<String, Vec<Vec<SyntaxType>>>,
    pub all_verbs: HashSet<String>,
    pub all_pres: HashSet<String>,
    pub prepositions: HashSet<String>,
    pub firsts: HashSet<String>,
}

impl<'a> SyntaxStats<'a> {
    pub fn new() -> SyntaxStats<'a> {
        SyntaxStats {
            basis: Vec::new(),
            all_syntax: HashMap::new(),
            all_verbs: HashSet::new(),
            all_pres: HashSet::new(),
            prepositions: HashSet::new(),
            firsts: HashSet::new(),
        }
    }

    pub fn validate_actions(&self, routines: &impl Codex) -> Result<(), String> {
        for v in self.all_verbs.iter() {
            let routine = format!("V-{}", v);
            if routines.lookup(&routine).is_none() {
                return Err(format!("Verb {} doesn't correspond to a ROUTINE", routine));
            }
        }

        for v in self.all_pres.iter() {
            let routine = format!("PRE-{}", v);
            if routines.lookup(&routine).is_none() {
                return Err(format!("Pre {} doesn't correspond to a ROUTINE", routine));
            }
        }

        Ok(())
    }
}

impl<'a> Populator<'a> for SyntaxStats<'a> {
    fn add_node(&mut self, node: &'a ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        let mut intermediate_parse: Vec<WhatWhat> = Vec::new();
        for n in self.basis.iter() {
            if n.children.len() < 4 {
                return Err(format!(
                    "Syntax node has less than four children\n{}",
                    format_file_location(n)
                ));
            }

            if n.children[1].node_type != ZilNodeType::Token(TokenType::Word) {
                return Err(format!(
                    "Syntax node has non-word second child\n{}",
                    format_file_location(&&n.children[1])
                ));
            }

            let first = get_token_as_word(&n.children[1]).unwrap();
            self.firsts.insert(first.clone());

            if n.children[n.children.len() - 1].node_type != ZilNodeType::Token(TokenType::Word) {
                return Err(format!(
                    "Syntax node has non-word last child\n{}",
                    format_file_location(&&n.children[1])
                ));
            }

            let mut equality_index: Option<usize> = None;
            let mut object_count = 0;
            for (i, c) in n
                .children
                .iter()
                .skip(2)
                .take(n.children.len() - 3)
                .enumerate()
            {
                match c.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let name = get_token_as_word(c).unwrap();
                        if name == "=" {
                            if equality_index.is_some() {
                                return Err(format!(
                                    "Multiple '=' in Synonym node\n{}",
                                    format_file_location(&c)
                                ));
                            }

                            equality_index = Some(i + 2);
                        } else if name == "OBJECT" {
                            object_count += 1;
                        }
                    }
                    ZilNodeType::Group => {
                        for gc in c.children.iter() {
                            if gc.node_type != ZilNodeType::Token(TokenType::Word) {
                                return Err(format!(
                                    "Synonym node has non-word child in group\n{}",
                                    format_file_location(&gc)
                                ));
                            }
                        }
                    }
                    _ => {
                        return Err(format!(
                            "Syntax node has child which is not a word nor group\n{}",
                            format_file_location(&c)
                        ));
                    }
                }
            }

            if equality_index.is_none() {
                return Err(format!(
                    "Syntax node has no '='\n{}",
                    format_file_location(n)
                ));
            }

            if object_count > 2 {
                return Err(format!(
                    "Syntax node has more than two OBJECTs\n{}",
                    format_file_location(n)
                ));
            }

            let what_what = WhatWhat {
                cmd: n.children[1..equality_index.unwrap()].iter().collect(),
                action: n.children[(equality_index.unwrap() + 1)..n.children.len()]
                    .iter()
                    .collect(),
            };

            intermediate_parse.push(what_what);
        }

        match build(&intermediate_parse) {
            Ok(built) => {
                self.all_syntax = built.all_syntax;
                self.all_verbs = built.all_verbs;
                self.all_pres = built.all_pres;
                self.prepositions = built.prepositions;
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn validate(&self, cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
        self.validate_actions(&cross_ref.routines)?;

        Ok(())
    }
}

pub struct WhatWhat<'a> {
    cmd: Vec<&'a ZilNode>,
    action: Vec<&'a ZilNode>,
}

pub enum SyntaxType {
    Action(Action),
    Cmd(Cmd),
    Object(Object),
}

#[derive(Clone)]
pub struct Cmd {
    pub name: String,
}

#[derive(Clone)]
pub struct Object {
    pub find: Option<String>,
    pub restrictions: Vec<String>,
}

#[derive(Clone)]
pub struct Action {
    pub verb: String,
    pub pre: Option<String>,
}

fn build<'a>(whats: &Vec<WhatWhat<'a>>) -> Result<Built, String> {
    let mut all_syntax: HashMap<String, Vec<Vec<SyntaxType>>> = HashMap::new();
    let mut all_verbs: HashSet<String> = HashSet::new();
    let mut all_pres: HashSet<String> = HashSet::new();
    let mut prepositions: HashSet<String> = HashSet::new();

    for what in whats.iter() {
        // check length
        if what.cmd.len() > MAX_CMD_LENGTH {
            return Err(format!(
                "Syntax node has too many children\n{}",
                format_file_location(&what.cmd[0])
            ));
        }

        // sort out the action
        let verb = get_token_as_word(what.action[0]).unwrap();
        let pre = match what.action.len() {
            1 => None,
            2 => Some(get_token_as_word(what.action[1]).unwrap()),
            _ => {
                return Err(format!(
                    "Syntax has more than two action children\n{}",
                    format_file_location(&what.action[0])
                ));
            }
        };

        // validate action
        let raw_verb = match VERB.captures(&verb) {
            Some(cap) => String::from(cap.name("raw").unwrap().as_str()),
            None => {
                return Err(format!(
                    "Syntax node has verb with no V- prefix\n{}",
                    format_file_location(&what.action[0])
                ));
            }
        };
        all_verbs.insert(raw_verb.clone());

        if pre.is_some() {
            match PRE.captures(&pre.clone().unwrap()) {
                Some(cap) => {
                    let raw_pre = String::from(cap.name("raw").unwrap().as_str());
                    all_pres.insert(raw_pre.clone());
                    // don't worry about if raw_pre == raw_verb
                }
                None => {
                    return Err(format!(
                        "Syntax node has pre with no PRE- prefix\n{}",
                        format_file_location(&what.action[0])
                    ));
                }
            }
        }

        // for use below
        let bad_group_order = || {
            return Err(format!(
                "Syntax node has group out of order\n{}",
                format_file_location(&what.cmd[0])
            ));
        };

        // validate group order in cmd (groups must be after a group or OBJECT)
        for (i, cmd) in what.cmd.iter().skip(1).enumerate() {
            if cmd.node_type == ZilNodeType::Group {
                let before = what.cmd[i];
                match before.node_type {
                    ZilNodeType::Group => (),
                    ZilNodeType::Token(TokenType::Word) => {
                        if get_token_as_word(before).unwrap() != "OBJECT" {
                            return bad_group_order();
                        }
                    }
                    _ => {
                        return bad_group_order();
                    }
                }
            }
        }

        // start building this syntax
        let mut commands: Vec<SyntaxType> = Vec::new();
        commands.push(SyntaxType::Action(Action {
            verb: verb,
            pre: pre,
        }));

        // build cmd
        let mut find: Option<String> = None;
        let mut restrictions: Vec<String> = Vec::new();
        for i in (1..what.cmd.len()).rev() {
            let child = what.cmd[i];
            match child.node_type {
                ZilNodeType::Group => {
                    if get_token_as_word(&child.children[0]).unwrap() == "FIND" {
                        if find.is_some() {
                            return Err(format!(
                                "Multiple FIND groups on the same OBJECT in syntax\n{}",
                                format_file_location(&child)
                            ));
                        }

                        if child.children.len() == 2 {
                            find = Some(get_token_as_word(&child.children[1]).unwrap());
                        } else {
                            return Err(format!(
                                "FIND group with bad number of children in syntax\n{}",
                                format_file_location(&child)
                            ));
                        }
                    } else {
                        if restrictions.len() > 0 {
                            return Err(format!(
                                "Multiple restriction groups on the same OBJECT in syntax\n{}",
                                format_file_location(&child)
                            ));
                        }

                        for c in child.children.iter() {
                            restrictions.push(get_token_as_word(c).unwrap());
                        }
                    }
                }
                ZilNodeType::Token(TokenType::Word) => {
                    let name = get_token_as_word(child).unwrap();
                    if name == "OBJECT" {
                        commands.insert(
                            0,
                            SyntaxType::Object(Object {
                                find: find,
                                restrictions: restrictions,
                            }),
                        );

                        find = None;
                        restrictions = Vec::new();
                    } else {
                        if ILLEGAL.is_match(&name) {
                            return Err(format!(
                                "Syntax node has cmd ({}) with an illegal char\n{}",
                                &name,
                                format_file_location(&child)
                            ));
                        }

                        prepositions.insert(name.clone());
                        commands.insert(0, SyntaxType::Cmd(Cmd { name: name }));
                    }
                }
                _ => unreachable!(),
            }
        }

        match what.cmd[0].node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let name = get_token_as_word(what.cmd[0]).unwrap();
                if name == "OBJECT" {
                    return Err(format!(
                        "Syntax node has OBJECT as first child\n{}",
                        format_file_location(&what.cmd[0])
                    ));
                }

                commands.insert(0, SyntaxType::Cmd(Cmd { name: name }));
            }
            _ => unreachable!(),
        }

        // save to all_syntax
        match &commands[0] {
            SyntaxType::Cmd(_) => {
                let key = get_command_key(&commands);
                if all_syntax.contains_key(&key) {
                    all_syntax.get_mut(&key).unwrap().push(commands);
                } else {
                    all_syntax.insert(key, vec![commands]);
                }
            }
            _ => {
                return Err(format!(
                    "Syntax node doesn't have a command as first child\n{}",
                    format_file_location(&what.cmd[0])
                ));
            }
        }
    }

    Ok(Built {
        all_syntax: all_syntax,
        all_verbs: all_verbs,
        all_pres: all_pres,
        prepositions: prepositions,
    })
}

pub struct Built {
    all_syntax: HashMap<String, Vec<Vec<SyntaxType>>>,
    all_verbs: HashSet<String>,
    all_pres: HashSet<String>,
    prepositions: HashSet<String>,
}

fn get_command_key(commands: &Vec<SyntaxType>) -> String {
    let mut out = String::new();
    for cmd in commands.iter() {
        match cmd {
            SyntaxType::Action(_) => (),
            SyntaxType::Cmd(cmd) => {
                out.push_str(&format!("_{}", cmd.name));
            }
            SyntaxType::Object(_) => {
                out.push_str("_OBJECT");
            }
        }
    }

    out
}

impl Action {
    fn base(&self) -> String {
        VERB.captures(&self.verb)
            .unwrap()
            .name("raw")
            .unwrap()
            .as_str()
            .to_string()
    }
}
