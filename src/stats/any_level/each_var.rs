use crate::{
    js::write_output::OutputNode,
    stats::{
        helpers::get_token_as_word,
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

use super::set_var::{Scope, VarWordType};

// starts a loop
// the third child is a group of exactly two variable names
// first variable gets the name
// second variable gets the value
// cannot return early

// can loop over: object (it's vars), player (my vars), room (it's vars)

pub struct EachVar {
    pub scope: Scope,
    pub name_var: String,
    pub value_var: String,
    pub body: Vec<OutputNode>,
}

impl EachVar {
    pub fn new() -> Self {
        Self {
            scope: Scope::TBD,
            name_var: String::new(),
            value_var: String::new(),
            body: Vec::new(),
        }
    }
}

impl HasReturnType for EachVar {
    fn return_type(&self) -> ReturnValType {
        // returns 0 if no explicit return called
        ReturnValType::Number
    }
}

impl CanValidate for EachVar {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        if n.children.len() < 3 {
            return Err(format!(
                "Expected at least 3 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        let second_child = &n.children[1];
        if second_child.node_type == ZilNodeType::Cluster {
            v.expect_val(ReturnValType::Location);
            match v.validate_cluster(&second_child) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.scope = Scope::LOC(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            }
        }

        if second_child.node_type == ZilNodeType::Token(TokenType::Word) {
            let mut found_scope = false;
            let second_word = get_token_as_word(&second_child).unwrap();
            if let Some(var_type) = v.has_local_var(&second_word) {
                match var_type {
                    ReturnValType::ObjectName => {
                        self.scope = Scope::Object(VarWordType::Literal(second_word.clone()));
                    }
                    ReturnValType::Location => self.scope = Scope::Location(second_word.clone()),
                    _ => {
                        return Err(format!(
                            "Variable {} is local but not a Location\n{}",
                            second_word,
                            format_file_location(&second_child)
                        ));
                    }
                }
                found_scope = true;
            }

            if v.is_room(&second_word) {
                self.scope = Scope::Room(second_word.clone());
                found_scope = true;
            }

            if v.is_object(&second_word) {
                self.scope = Scope::Object(VarWordType::Literal(second_word.clone()));
                found_scope = true;
            }

            if !found_scope {
                return Err(format!(
                    "Unknown word {}\n{}",
                    second_word,
                    format_file_location(&second_child)
                ));
            }
        }

        let var_group = &n.children[2];

        if var_group.node_type != ZilNodeType::Group {
            return Err(format!(
                "Expected group, found {}\n{}",
                var_group.node_type,
                format_file_location(&n)
            ));
        } else if var_group.children.len() != 2 {
            return Err(format!(
                "Expected group to have two children, found {}\n{}",
                var_group.children.len(),
                format_file_location(&n)
            ));
        }

        for (i, grand_child) in var_group.children.iter().enumerate() {
            let name = get_token_as_word(&grand_child);
            if name.is_none() {
                return Err(format!(
                    "Var name in EACH-VAR group is not a word\n{}",
                    format_file_location(&n)
                ));
            }
            let name = name.unwrap();

            if v.is_global(&name) || v.has_local_var(&name).is_some() {
                return Err(format!(
                    "Variable {} already found in symbol table\n{}",
                    name,
                    format_file_location(&grand_child)
                ));
            }

            match i {
                0 => {
                    v.add_local_var(name.clone(), ReturnValType::VarName);
                    self.name_var = name;
                }
                1 => {
                    v.add_local_var(name.clone(), ReturnValType::Number);
                    self.value_var = name;
                }
                _ => unreachable!(),
            }
        }

        v.expect_val(ReturnValType::Any);

        for child in n.children.iter().skip(3) {
            match child.node_type {
                ZilNodeType::Cluster => match v.validate_cluster(&child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.body.push(OutputNode::Writer(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Expected cluster, found {}\n{}",
                        child.node_type,
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
