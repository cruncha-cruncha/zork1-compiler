use std::collections::HashSet;

use crate::{
    stats::{
        cross_ref::Codex,
        helpers::{get_token_as_word, num_children_more_than, ValidationResult},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use crate::stats::cross_ref::Populator;

// PRSA = action
// PRSO = direct object
// PRSI = indirect object
// so named because of hysterical raisins

// first word is always the action
// first OBJECT is always PRSO
// second OBJECT is always PRSI

// all commands are of the form:
// <SYNTAX (PRSA) ... = (ROUTINE)>
// <SYNTAX (PRSA) ... (PRSO) ... = (ROUTINE)>
// <SYNTAX (PRSA) ... (PRSO) ... (PRSI) ... = (ROUTINE)>

pub struct SyntaxStats {
    basis: Vec<ZilNode>,
    all_syntax: Vec<Vec<SyntaxItem>>,
    all_routine_names: HashSet<String>,
}

#[derive(Clone, Debug)]
pub enum SyntaxItem {
    Cmd(Cmd),
    Object(Object),
    Action(Action),
}

#[derive(Clone, Debug)]
pub struct Cmd {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Object {
    pub restrictions: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Action {
    pub routine: String,
}

impl SyntaxStats {
    pub fn new() -> SyntaxStats {
        SyntaxStats {
            basis: Vec::new(),
            all_syntax: Vec::new(),
            all_routine_names: HashSet::new(),
        }
    }

    pub fn as_iter(&self) -> SyntaxIter {
        SyntaxIter {
            index: 0,
            all_syntax: &self.all_syntax,
        }
    }

    fn validate_routines<T>(&self, routines: &impl Codex<T>) -> ValidationResult<()> {
        let mut errors: Vec<String> = Vec::new();
        for v in self.all_routine_names.iter() {
            if routines.lookup(&v).is_none() {
                errors.push(format!(
                    "Can't find definition for routine {} (in some syntax)",
                    v
                ));
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(())
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
            let mut obj_count = 0;

            match num_children_more_than(line, 3) {
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
            } else if second_word == "GO" {
                errors.push(format!(
                    "Syntax node's second child cannot be GO (this action is reserved)\n{}",
                    format_file_location(&line)
                ));
                continue;
            }

            steps.push(SyntaxItem::Cmd(Cmd { name: second_word }));

            for i in 2..line.children.len() - 2 {
                let n = &line.children[i];
                match n.node_type {
                    ZilNodeType::Token(TokenType::Word) => {
                        let word = get_token_as_word(&n).unwrap();

                        if word == "OBJECT" {
                            steps.push(SyntaxItem::Object(Object {
                                restrictions: Vec::new(),
                            }));
                            obj_count += 1;
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
                            "Syntax has child which is not word or cluster, is:{}\n{}",
                            n.node_type,
                            format_file_location(&n)
                        ));
                        continue;
                    }
                }
            }

            if obj_count > 2 {
                syntax_errors.push(format!(
                    "Syntax has too many variables (allowed at most two OBJECTs)\n{}",
                    format_file_location(&line)
                ));
            }

            let second_last_word =
                get_token_as_word(&line.children[line.children.len() - 2]).unwrap_or_default();
            if second_last_word != "=" {
                syntax_errors.push(format!(
                    "Syntax node's second-last child must be '='\n{}",
                    format_file_location(&line)
                ));
            }

            let last_word = match get_token_as_word(line.children.last().unwrap()) {
                Ok(v) => Some(v),
                Err(e) => {
                    syntax_errors.push(e);
                    None
                }
            };

            if syntax_errors.len() > 0 {
                errors.append(&mut syntax_errors);
                continue;
            }

            let last_word = last_word.unwrap();
            steps.push(SyntaxItem::Action(Action {
                routine: last_word.clone(),
            }));

            self.all_syntax.push(steps);
            self.all_routine_names.insert(last_word);
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(())
    }

    fn validate(&self, cross_ref: &crate::stats::cross_ref::CrossRef) -> ValidationResult<()> {
        self.validate_routines(&cross_ref.routines.as_codex())?;

        Ok(())
    }
}

pub struct SyntaxIter<'a> {
    index: usize,
    all_syntax: &'a Vec<Vec<SyntaxItem>>,
}

impl<'a> Iterator for SyntaxIter<'a> {
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
