use crate::{
    js::write_output::{CanWriteOutput, OutputNode},
    stats::{
        helpers::{get_token_as_number, get_token_as_word},
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// if len 3: set global / local variable to a number
// if len 4: set object / room / player variable to a number

pub struct SetVar {
    pub scope: Option<Scope>,
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
    Local(LocalVar),
    // globals['global_name'] ...
    Global(String),
    // rooms['room_name'] ...
    Room(String),
    // objects['object_name'] ...
    Object(String),
    // any cluster with ReturnValType::Location corresponds to a zil LOC call
    // the result is an actual object in js
    // setVar(<writer>, ... )
    LOC(Box<dyn CanWriteOutput>), // setValue(<writer>, ...)
}

pub struct LocalVar {
    pub name: String,
    pub return_type: ReturnValType,
}

impl SetVar {
    pub fn new() -> Self {
        Self {
            scope: None,
            var: Scope::TBD,
            value: OutputNode::TBD,
        }
    }
}

impl HasReturnType for SetVar {
    fn return_type(&self) -> ReturnValType {
        ReturnValType::None
    }
}

impl CanValidate for SetVar {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        if n.children.len() != 3 && n.children.len() != 4 {
            return Err(format!(
                "Expected 3 or 4 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        if n.children.len() == 3 {
            let second_word = get_token_as_word(&n.children[1]);
            if second_word.is_none() {
                return Err(format!(
                    "Expected word, found {}\n{}",
                    n.children[1].node_type,
                    format_file_location(&n)
                ));
            }
            let second_word: String = second_word.unwrap();

            if let Some(var_type) = v.has_local_var(&second_word) {
                if var_type == ReturnValType::Number || var_type == ReturnValType::VarName {
                    self.var = Scope::Local(LocalVar {
                        name: second_word.clone(),
                        return_type: var_type,
                    });
                } else {
                    return Err(format!(
                        "Variable {} is not a number of local variable\n{}",
                        second_word,
                        format_file_location(&n.children[1])
                    ));
                }
            } else if v.is_global(&second_word) {
                self.var = Scope::Global(second_word.clone());
            } else {
                return Err(format!(
                    "Variable {} not found in local or global symbol table\n{}",
                    second_word,
                    format_file_location(&n.children[1])
                ));
            }
        }

        if n.children.len() == 4 {
            let second_child = &n.children[1];
            let mut found_scope = false;

            if second_child.node_type == ZilNodeType::Cluster {
                v.expect_val(ReturnValType::Location);
                match v.validate_cluster(&second_child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.scope = Some(Scope::LOC(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                }
                found_scope = true;
            }

            if second_child.node_type == ZilNodeType::Token(TokenType::Word) {
                let second_word = get_token_as_word(&second_child).unwrap();

                if second_word == "PLAYER" {
                    self.scope = Some(Scope::Player);
                    found_scope = true;
                }

                if let Some(var_type) = v.has_local_var(&second_word) {
                    match var_type {
                        ReturnValType::Location => {
                            self.scope = Some(Scope::Local(LocalVar {
                                name: second_word.clone(),
                                return_type: var_type,
                            }));
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not player, a room, or an object\n{}",
                                second_word,
                                format_file_location(&second_child)
                            ));
                        }
                    }
                    found_scope = true;
                }

                if let Some(_object) = v.get_object(&second_word) {
                    self.scope = Some(Scope::Object(second_word.clone()));
                    found_scope = true;
                }

                if let Some(_room) = v.get_room(&second_word) {
                    self.scope = Some(Scope::Room(second_word.clone()));
                    found_scope = true;
                }
            }

            if !found_scope {
                return Err(format!(
                    "Expected cluster, PLAYER, object, or room, found {}\n{}",
                    second_child.node_type,
                    format_file_location(&n)
                ));
            }

            let third_word = get_token_as_word(&n.children[2]);
            if third_word.is_none() {
                return Err(format!(
                    "Expected word, found {}\n{}",
                    n.children[2].node_type,
                    format_file_location(&n)
                ));
            }
            let third_word: String = third_word.unwrap();

            match v.has_local_var(&third_word) {
                Some(rt) => match rt {
                    ReturnValType::VarName | ReturnValType::Number => {
                        self.var = Scope::Local(LocalVar {
                            name: third_word.clone(),
                            return_type: rt,
                        });
                    }
                    _ => {
                        return Err(format!(
                            "Variable {} is not a local numeric variable\n{}",
                            third_word,
                            format_file_location(&n.children[2])
                        ));
                    }
                },
                _ => {
                    self.var = Scope::Bare(third_word.clone());
                }
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
                let word = get_token_as_word(&last_child).unwrap();
                if let Some(var_type) = v.has_local_var(&word) {
                    match var_type {
                        ReturnValType::Number | ReturnValType::VarName => {
                            self.value = OutputNode::Variable(Scope::Local(LocalVar {
                                name: word.clone(),
                                return_type: var_type,
                            }));
                        }
                        _ => {
                            return Err(format!(
                                "Variable {} is not a local numeric variable\n{}",
                                word,
                                format_file_location(&n.children[2])
                            ));
                        }
                    }
                } else if v.is_global(&word) {
                    self.value = OutputNode::Variable(Scope::Global(word.clone()));
                } else {
                    return Err(format!(
                        "Variable {} not found in local or global symbol table\n{}",
                        word,
                        format_file_location(&last_child)
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
                    "Expected number, found {}\n{}",
                    n.children.last().unwrap().node_type,
                    format_file_location(&n.children.last().unwrap())
                ));
            }
        }

        Ok(())
    }
}