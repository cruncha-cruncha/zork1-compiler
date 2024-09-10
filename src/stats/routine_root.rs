use std::collections::HashSet;

use crate::{
    js::{
        formatter::Formatter,
        write_output::{CanWriteOutput, OutputNode},
    },
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

use super::{
    routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    top_level::routines::RoutineCodexValue,
};

pub struct RoutineRoot {
    pub name: String,
    pub var_names: HashSet<String>,
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
}

impl HasReturnType for RoutineRoot {
    fn return_type(&self) -> ReturnValType {
        // returns 0 if no explicit return called
        ReturnValType::Number
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

        for child in n.children.iter().skip(3) {
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
    fn return_type(&self) -> ReturnValType {
        // returns 0 if no explicit return called
        ReturnValType::Number
    }
}

impl CanWriteOutput for RoutineStub {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write(
            &format!("routines[{}].func(player, prsa, prso, prsi)", self.name),
            false,
        )?;

        Ok(())
    }
}
