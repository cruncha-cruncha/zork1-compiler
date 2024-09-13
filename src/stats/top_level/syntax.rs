use std::collections::HashSet;

use crate::{
    stats::{cross_ref::Codex, helpers::get_token_as_word},
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

    fn validate_routines<T>(&self, routines: &impl Codex<T>) -> Result<(), String> {
        for v in self.all_routine_names.iter() {
            if routines.lookup(&v).is_none() {
                return Err(format!(
                    "Can't find definition for routine {} (in some syntax)",
                    v
                ));
            }
        }

        Ok(())
    }
}

impl Populator for SyntaxStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        for line in self.basis.iter() {
            let mut steps: Vec<SyntaxItem> = Vec::new();
            let mut obj_count = 0;

            if line.children.len() < 4 {
                return Err(format!(
                    "Possible syntax node doesn't have enough children\n{}",
                    format_file_location(&line)
                ));
            }

            let first_word = get_token_as_word(&line.children[0]).unwrap_or_default();
            if first_word != "SYNTAX" {
                unreachable!();
            }

            let second_word = get_token_as_word(&line.children[1]);
            if second_word.is_none() {
                return Err(format!(
                    "Syntax node has non-word second child\n{}",
                    format_file_location(&line)
                ));
            }

            let second_word = second_word.unwrap();
            if second_word == "OBJECT" {
                return Err(format!(
                    "Syntax node's second child cannot be OBJECT\n{}",
                    format_file_location(&line)
                ));
            } else if second_word == "GO" {
                return Err(format!(
                    "Syntax node's second child cannot be GO (this action is reserved)\n{}",
                    format_file_location(&line)
                ));
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
                                return Err(format!(
                                    "Syntax has group node, but previous word is not OBJECT\n{}",
                                    format_file_location(&n)
                                ));
                            }
                        };

                        let mut restrictions: Vec<String> = Vec::new();
                        for c in n.children.iter() {
                            let word = get_token_as_word(c);
                            if word.is_none() {
                                return Err(format!(
                                    "Syntax has group node with non-word child\n{}",
                                    format_file_location(&n)
                                ));
                            } else {
                                restrictions.push(word.unwrap());
                            }
                        }

                        step.restrictions = restrictions;
                    }
                    _ => {
                        return Err(format!(
                            "Syntax has child which is not word or cluster, is:{}\n{}",
                            n.node_type,
                            format_file_location(&n)
                        ));
                    }
                }
            }

            if obj_count > 2 {
                return Err(format!(
                    "Syntax has too many variables (allowed at most two OBJECTs)\n{}",
                    format_file_location(&line)
                ));
            }

            let second_last_word =
                get_token_as_word(&line.children[line.children.len() - 2]).unwrap_or_default();
            if second_last_word != "=" {
                return Err(format!(
                    "Syntax node's second-last child must be '='\n{}",
                    format_file_location(&line)
                ));
            }

            let last_word = get_token_as_word(line.children.last().unwrap());
            if last_word.is_none() {
                return Err(format!(
                    "Syntax node's last child must be a word\n{}",
                    format_file_location(&line)
                ));
            }

            let last_word = last_word.unwrap();
            steps.push(SyntaxItem::Action(Action {
                routine: last_word.clone(),
            }));

            self.all_syntax.push(steps);
            self.all_routine_names.insert(last_word);
        }

        Ok(())
    }

    fn validate(&self, cross_ref: &crate::stats::cross_ref::CrossRef) -> Result<(), String> {
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
