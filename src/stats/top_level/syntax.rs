use std::fmt;

use crate::{
    stats::{
        cross_ref::{Codex, CrossRef},
        helpers::{get_token_as_word, num_children_more_than, ValidationResult},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use crate::stats::cross_ref::Populator;

// first word is always the action
// all commands are of the form:
// <SYNTAX (ACTION) ... >
// <SYNTAX (ACTION) ... OBJECT ... >
// <SYNTAX (ACTION) ... OBJECT ... OBJECT ... >
// etc.

pub struct SyntaxStats {
    basis: Vec<ZilNode>,
    all_syntax: Vec<Vec<SyntaxItem>>,
}

#[derive(Clone, Debug)]
pub enum SyntaxItem {
    Action(String),
    Cmd(String),
    Object,
}

impl SyntaxStats {
    pub fn new() -> SyntaxStats {
        SyntaxStats {
            basis: Vec::new(),
            all_syntax: Vec::new(),
        }
    }

    pub fn as_codex(&self) -> SyntaxCodex {
        SyntaxCodex {
            index: 0,
            basis: &self.basis,
            all_syntax: &self.all_syntax,
        }
    }
}

impl Populator for SyntaxStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> ValidationResult<()> {
        let mut errors: Vec<String> = Vec::new();
        for line in self.basis.iter() {
            let mut syntax_errors: Vec<String> = Vec::new();
            let mut steps: Vec<SyntaxItem> = Vec::new();

            match num_children_more_than(line, 2) {
                Ok(_) => {}
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            }

            let second_child = &line.children[1];
            if second_child.node_type != ZilNodeType::Group {
                errors.push(format!(
                    "Syntax node's action child is not type group\n{}",
                    format_file_location(&line)
                ));
                continue;
            } else if second_child.children.len() != 1 {
                errors.push(format!(
                    "Syntax node's action group does not have exactly one child\n{}",
                    format_file_location(&line)
                ));
                continue;
            }

            let action_word = match get_token_as_word(&second_child.children[0]) {
                Ok(v) => v,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            if action_word == "OBJECT" {
                errors.push(format!(
                    "Syntax node's action word cannot be OBJECT\n{}",
                    format_file_location(&line)
                ));
                continue;
            } else if CrossRef::name_is_illegal(&action_word) {
                errors.push(format!(
                    "Syntax node's action word is illegal: {}\n{}",
                    action_word,
                    format_file_location(&line)
                ));
                continue;
            }

            steps.push(SyntaxItem::Action(action_word));

            for i in 2..line.children.len() {
                let n = &line.children[i];
                match n.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let word = get_token_as_word(&n).unwrap();
                        if word == "OBJECT" {
                            steps.push(SyntaxItem::Object);
                        } else {
                            steps.push(SyntaxItem::Cmd(word));
                        }
                    }
                    _ => {
                        syntax_errors.push(format!(
                            "Syntax has child which is not a word, is:{}\n{}",
                            n.node_type,
                            format_file_location(&n)
                        ));
                        continue;
                    }
                }
            }

            if syntax_errors.len() > 0 {
                errors.append(&mut syntax_errors);
                continue;
            }

            self.all_syntax.push(steps);
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &crate::stats::cross_ref::CrossRef) -> ValidationResult<()> {
        // if A OBJECT B exists then A OBJECT C is not allowed
        // but it's computationally expensive to check for this
        // so check for it later in build_parser

        Ok(())
    }
}

impl fmt::Display for SyntaxItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyntaxItem::Action(act) => write!(f, "{}", act),
            SyntaxItem::Cmd(cmd) => write!(f, "{}", cmd),
            SyntaxItem::Object => write!(f, "<object>"),
        }
    }
}

pub struct SyntaxCodex<'a> {
    index: usize,
    #[allow(dead_code)]
    basis: &'a Vec<ZilNode>,
    all_syntax: &'a Vec<Vec<SyntaxItem>>,
}

impl<'a> Iterator for SyntaxCodex<'a> {
    type Item = &'a Vec<SyntaxItem>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.all_syntax.len() {
            None
        } else {
            self.index += 1;
            Some(&self.all_syntax[self.index - 1])
        }
    }
}

// bad codex, as key:value is 1:n, not 1:1
// but good enough
impl<'a> Codex<&'a Vec<SyntaxItem>> for SyntaxCodex<'a> {
    fn lookup(&self, word: &str) -> Option<&'a Vec<SyntaxItem>> {
        for syntax in self.all_syntax.iter() {
            match syntax.first().unwrap() {
                SyntaxItem::Action(act) => {
                    if act == word {
                        return Some(syntax);
                    }
                }
                _ => {}
            }
        }

        return None;
    }
}
