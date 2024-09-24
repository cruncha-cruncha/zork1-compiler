use crate::{
    js::write_output::{CanWriteOutput, OutputNode},
    stats::{
        helpers::{get_token_as_number, get_token_as_word, num_children_between},
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// if len 3: set global / local variable to a number
// if len 4: set IRP variable to a number
// if len 5: set variable on first object instance in IRP to a number
// IRP = object instance, or room, or player

pub struct SetVar {
    pub scope: Option<Scope>,
    pub object: Option<String>,
    pub var: Scope,
    pub value: OutputNode,
}
pub enum Scope {
    // illegal
    TBD,
    // player ...
    Player,
    // 'bare' ...
    Bare(String),
    // locals['local_name'] ...
    Local(String),
    // globals['global_name'] ...
    Global(String),
    // rooms['room_name'] ...
    Room(String),
    // objects['object_name'] ...
    Object(String),
    // setVar(<writer>, ... )
    Writer(Box<dyn CanWriteOutput>), // setValue(<writer>, ...)
}

impl SetVar {
    pub fn new() -> Self {
        Self {
            scope: None,
            object: None,
            var: Scope::TBD,
            value: OutputNode::TBD,
        }
    }
}

impl HasReturnType for SetVar {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::None]
    }
}

impl CanValidate for SetVar {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_between(n, 3, 5)?;

        let second_last_child = &n.children[n.children.len() - 2];
        // has to be a word, can be a local variable of type text
        match second_last_child.node_type {
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(second_last_child).unwrap();
                if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Text => {
                            // this is a hack
                            // this scope, on this particular var, means locals[locals['word']]
                            // aka Scope::Local(Scope::Local(word))
                            self.var = Scope::Object(word);
                        }
                        ReturnValType::Number => {
                            self.var = Scope::Local(word);
                        }
                        _ => {
                            return Err(format!(
                                "Local variable {} is not a number\n{}",
                                word,
                                format_file_location(second_last_child)
                            ));
                        }
                    }
                } else if v.is_global(&word) {
                    self.var = Scope::Global(word);
                } else if n.children.len() > 3 {
                    self.var = Scope::Bare(word);
                } else {
                    return Err(format!(
                        "Word {} is not local or global\n{}",
                        word,
                        format_file_location(second_last_child)
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "Expected text, number, or cluster, found {}\n{}",
                    second_last_child.node_type,
                    format_file_location(second_last_child)
                ));
            }
        }

        v.expect_val(ReturnValType::Number);

        let last_child = n.children.last().unwrap();
        match last_child.node_type {
            ZilNodeType::Token(TokenType::Number) => {
                let num = get_token_as_number(last_child).unwrap();
                self.value = OutputNode::Number(num);
            }
            ZilNodeType::Token(TokenType::Word) => {
                let word = get_token_as_word(last_child).unwrap();
                if let Some(return_type) = v.has_local_var(&word) {
                    match return_type {
                        ReturnValType::Number => {
                            self.value = OutputNode::Variable(Scope::Local(word));
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not a number\n{}",
                                word,
                                format_file_location(last_child)
                            ));
                        }
                    }
                } else if v.is_global(&word) {
                    self.value = OutputNode::Variable(Scope::Global(word));
                } else {
                    return Err(format!(
                        "Word {} is not local or globals\n{}",
                        word,
                        format_file_location(last_child)
                    ));
                }
            }
            ZilNodeType::Cluster => match v.validate_cluster(last_child) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.value = OutputNode::Writer(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected text, number, or cluster, found {}\n{}",
                    n.children.last().unwrap().node_type,
                    format_file_location(&n.children.last().unwrap())
                ));
            }
        }

        if n.children.len() > 3 {
            let second_child = &n.children[1];
            v.expect_vals(vec![ReturnValType::Inst, ReturnValType::RP]);

            match second_child.node_type {
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&second_child).unwrap();
                    if word == "PLAYER" {
                        self.scope = Some(Scope::Player);
                    } else if v.is_room(&word) {
                        self.scope = Some(Scope::Room(word));
                    } else if let Some(return_type) = v.has_local_var(&word) {
                        match return_type {
                            ReturnValType::Inst => {
                                self.scope = Some(Scope::Local(word));
                            }
                            _ => {
                                return Err(format!(
                                    "Variable {} is not an object\n{}",
                                    word,
                                    format_file_location(&second_child)
                                ));
                            }
                        }
                    } else {
                        return Err(format!(
                            "Variable {} is not player or room and not found in locals\n{}",
                            word,
                            format_file_location(&second_child)
                        ));
                    }
                }
                ZilNodeType::Cluster => match v.validate_cluster(&second_child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.scope = Some(Scope::Writer(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Expected a word or cluster, found {}\n{}",
                        second_child.node_type,
                        format_file_location(&second_child)
                    ));
                }
            }
        }

        if n.children.len() > 4 {
            let third_child = &n.children[2];
            match third_child.node_type {
                ZilNodeType::Token(TokenType::Word) => {
                    let word = get_token_as_word(&third_child).unwrap();
                    if v.is_object(&word) {
                        self.object = Some(word);
                    } else {
                        return Err(format!(
                            "Word {} not found in objects\n{}",
                            word,
                            format_file_location(&third_child)
                        ));
                    }
                }
                _ => {
                    return Err(format!(
                        "Expected a word, found {}\n{}",
                        third_child.node_type,
                        format_file_location(&third_child)
                    ));
                }
            }
        }

        Ok(())
    }
}
