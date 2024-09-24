use std::collections::BTreeSet;

use crate::{
    js::write_output::OutputNode,
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

use super::{
    routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    top_level::routines::{HandlerInfo, RoutineCodexValue},
};

pub struct RoutineRoot {
    pub name: String,
    pub var_names: BTreeSet<String>,
    pub body: Vec<OutputNode>,
}

impl RoutineRoot {
    pub fn from(codex_value: &RoutineCodexValue) -> Self {
        Self {
            name: codex_value.name.clone(),
            var_names: codex_value.var_names.clone(),
            body: Vec::new(),
        }
    }

    pub fn from_handler(value: &HandlerInfo) -> Self {
        let name = value.get_key();
        Self {
            name,
            var_names: value.var_names.clone(),
            body: Vec::new(),
        }
    }
}

impl HasReturnType for RoutineRoot {
    fn return_type(&self) -> Vec<ReturnValType> {
        // returns 0 if no explicit return called
        vec![ReturnValType::Number]
    }
}

impl CanValidate for RoutineRoot {
    fn validate<'b>(&mut self, v: &mut Validator<'b>, n: &'b ZilNode) -> Result<(), String> {
        for var in self.var_names.iter() {
            if v.is_global(&var) || v.has_local_var(&var).is_some() {
                return Err(format!(
                    "Variable {} already found in symbol table\n{}",
                    var,
                    format_file_location(&n)
                ));
            }

            v.add_local_var(var.clone(), ReturnValType::Number);
        }

        v.expect_val(ReturnValType::Any);

        // action () ...
        // ROUTINE name () ...
        // action obj () ...
        // action () obj ...
        let skip = match n.children[2].node_type {
            ZilNodeType::Group | ZilNodeType::Token(_) => 3,
            _ => 2,
        };

        for child in n.children.iter().skip(skip) {
            match child.node_type {
                ZilNodeType::Cluster => match v.validate_cluster(child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.body.push(OutputNode::Writer(w)),
                        None => {
                            return Err(format!("No writer found\n{}", format_file_location(child)))
                        }
                    },
                    Err(e) => return Err(e),
                },
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

pub struct RoutineStub {
    pub name: String,
}

impl RoutineStub {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl HasReturnType for RoutineStub {
    fn return_type(&self) -> Vec<ReturnValType> {
        // returns 0 if no explicit return called
        vec![ReturnValType::Number]
    }
}
