use crate::{
    js::write_output::OutputNode,
    stats::{
        helpers::{
            get_token_as_number, get_token_as_text, get_token_as_word, num_children_more_than,
        },
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::Scope;

// TODO
// handle OBJ == INST

pub struct IsEqual {
    pub val_type: ReturnValType,
    pub values: Vec<OutputNode>,
}

impl IsEqual {
    pub fn new() -> Self {
        Self {
            val_type: ReturnValType::Unknown,
            values: Vec::new(),
        }
    }
}

impl HasReturnType for IsEqual {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::Boolean]
    }
}

impl CanValidate for IsEqual {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_more_than(n, 2)?;

        let mut expect: ReturnValType = ReturnValType::Unknown;
        v.expect_val(ReturnValType::Any);

        let first_child = &n.children[1];
        match first_child.node_type {
            ZilNodeType::Token(TokenType::Text) => {
                let text = get_token_as_text(&first_child).unwrap();
                self.values.push(OutputNode::Text(text));
                expect = ReturnValType::Text;
            }
            ZilNodeType::Token(TokenType::Number) => {
                let num = get_token_as_number(&first_child).unwrap();
                self.values.push(OutputNode::Number(num));
                expect = ReturnValType::Number;
            }
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(&first_child).unwrap();
                if let Some(return_type) = v.has_local_var(&word) {
                    self.values.push(OutputNode::Variable(Scope::Local(word)));
                    expect = *return_type;
                } else if v.is_global(&word) {
                    self.values.push(OutputNode::Variable(Scope::Global(word)));
                    expect = ReturnValType::Number;
                } else if word == "PLAYER" {
                    self.values.push(OutputNode::Variable(Scope::Player));
                    expect = ReturnValType::RP;
                } else if v.is_room(&word) {
                    self.values.push(OutputNode::Variable(Scope::Room(word)));
                    expect = ReturnValType::RP;
                } else if v.is_object(&word) {
                    self.values.push(OutputNode::Variable(Scope::Object(word)));
                    // this is not an object instance, but it'll work for this command
                    expect = ReturnValType::Inst;
                } else {
                    return Err(format!(
                        "Word {} is not player, and not found in locals, globals, or rooms, or objects\n{}",
                        word,
                        format_file_location(&first_child)
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(&first_child) {
                Ok(return_type) => match v.take_last_writer() {
                    Some(w) => {
                        self.values.push(OutputNode::Writer(w));
                        expect = return_type;
                    }
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected number, text, word, or cluster, found {}\n{}",
                    first_child.node_type,
                    format_file_location(&n)
                ));
            }
        }

        v.expect_val(expect);

        for child in n.children.iter().skip(2) {
            match child.node_type {
                ZilNodeType::Token(TokenType::Text) => {
                    if expect != ReturnValType::Text {
                        return Err(format!(
                            "Got text, but expected {:?}\n{}",
                            expect,
                            format_file_location(&child)
                        ));
                    }
                    let text = get_token_as_text(&child).unwrap();
                    self.values.push(OutputNode::Text(text));
                }
                ZilNodeType::Token(TokenType::Number) => {
                    if expect != ReturnValType::Number {
                        return Err(format!(
                            "Got number, but expected {:?}\n{}",
                            expect,
                            format_file_location(&child)
                        ));
                    }
                    let num = get_token_as_number(&child).unwrap();
                    self.values.push(OutputNode::Number(num));
                }
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&child).unwrap();
                    if let Some(return_type) = v.has_local_var(&word) {
                        if return_type != &expect {
                            return Err(format!(
                                "Variable {} is not a {:?}\n{}",
                                word,
                                expect,
                                format_file_location(&child)
                            ));
                        }
                        self.values.push(OutputNode::Variable(Scope::Local(word)));
                    } else if v.is_global(&word) {
                        if expect != ReturnValType::Number {
                            return Err(format!(
                                "Variable {} is not a number\n{}",
                                word,
                                format_file_location(&child)
                            ));
                        }
                        self.values.push(OutputNode::Variable(Scope::Global(word)));
                    } else if word == "PLAYER" {
                        if expect != ReturnValType::RP {
                            return Err(format!(
                                "Variable {} is not a player\n{}",
                                word,
                                format_file_location(&child)
                            ));
                        }
                        self.values.push(OutputNode::Variable(Scope::Player));
                    } else if v.is_room(&word) {
                        if expect != ReturnValType::RP {
                            return Err(format!(
                                "Variable {} is not a room\n{}",
                                word,
                                format_file_location(&child)
                            ));
                        }
                        self.values.push(OutputNode::Variable(Scope::Room(word)));
                    } else if v.is_object(&word) {
                        if expect != ReturnValType::Inst {
                            return Err(format!(
                                "Variable {} is not an object\n{}",
                                word,
                                format_file_location(&child)
                            ));
                        }
                        self.values.push(OutputNode::Variable(Scope::Object(word)));
                    } else {
                        return Err(format!(
                            "Word {} is not player, and not found in locals, globals, rooms, or objects\n{}",
                            word,
                            format_file_location(&child)
                        ));
                    }
                }
                ZilNodeType::Cluster => match v.validate_cluster(&child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.values.push(OutputNode::Writer(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Expected number, text, word, or cluster, found {}\n{}",
                        child.node_type,
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
