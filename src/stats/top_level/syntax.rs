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
// <SYNTAX ACTION ... >
// <SYNTAX ACTION ... OBJECT ... >
// <SYNTAX ACTION ... OBJECT ... OBJECT ... >
// etc.

pub struct SyntaxStats {
    basis: Vec<ZilNode>,
    all_syntax: Vec<Vec<SyntaxItem>>,
}

#[derive(Clone, Debug)]
pub enum SyntaxItem {
    Cmd(Cmd),
    Object(Object),
}

#[derive(Clone, Debug)]
pub struct Cmd {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Object {
    pub restrictions: Vec<String>,
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

            match num_children_more_than(line, 0) {
                Ok(_) => {}
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            }

            let second_word = match get_token_as_word(&line.children[1]) {
                Ok(v) => v,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            if second_word == "OBJECT" {
                errors.push(format!(
                    "Syntax node's second child cannot be OBJECT\n{}",
                    format_file_location(&line)
                ));
                continue;
            } else if CrossRef::name_is_illegal(&second_word) {
                errors.push(format!(
                    "Syntax node's second child is illegal: {}\n{}",
                    second_word,
                    format_file_location(&line)
                ));
                continue;
            }

            steps.push(SyntaxItem::Cmd(Cmd { name: second_word }));

            for i in 2..line.children.len() {
                let n = &line.children[i];
                match n.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let word = get_token_as_word(&n).unwrap();

                        if word == "OBJECT" {
                            steps.push(SyntaxItem::Object(Object {
                                restrictions: Vec::new(),
                            }));
                        } else {
                            steps.push(SyntaxItem::Cmd(Cmd { name: word }));
                        }
                    }
                    ZilNodeType::Group => {
                        let step = match steps.last_mut().unwrap() {
                            SyntaxItem::Object(obj) => obj,
                            _ => {
                                syntax_errors.push(format!(
                                    "Syntax has group node, but previous word is not OBJECT\n{}",
                                    format_file_location(&n)
                                ));
                                continue;
                            }
                        };

                        let mut restrictions: Vec<String> = Vec::new();
                        for c in n.children.iter() {
                            let word = match get_token_as_word(c) {
                                Ok(v) => Some(v),
                                Err(e) => {
                                    syntax_errors.push(e);
                                    continue;
                                }
                            };

                            restrictions.push(word.unwrap());
                        }

                        step.restrictions = restrictions;
                    }
                    _ => {
                        syntax_errors.push(format!(
                            "Syntax has child which is not word or group, is:{}\n{}",
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
        Ok(())
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

impl<'a> Codex<&'a Vec<SyntaxItem>> for SyntaxCodex<'a> {
    fn lookup(&self, word: &str) -> Option<&'a Vec<SyntaxItem>> {
        for syntax in self.all_syntax.iter() {
            match syntax.first().unwrap() {
                SyntaxItem::Cmd(cmd) => {
                    if cmd.name == word {
                        return Some(syntax);
                    }
                }
                _ => {}
            }
        }

        return None;
    }
}
