use crate::{
    js::write_output::OutputNode,
    stats::routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

pub struct Or {
    pub values: Vec<OutputNode>,
}

impl Or {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl HasReturnType for Or {
    fn return_type(&self) -> ReturnValType {
        ReturnValType::Boolean
    }
}

impl CanValidate for Or {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        if n.children.len() < 3 {
            return Err(format!(
                "Expected at least 3 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        v.expect_val(ReturnValType::Boolean);

        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Cluster => match v.validate_cluster(&child) {
                    Ok(_) => match v.take_last_writer() {
                        Some(w) => self.values.push(OutputNode::Writer(w)),
                        None => unreachable!(),
                    },
                    Err(e) => return Err(e),
                },
                _ => {
                    return Err(format!(
                        "Expected word or cluster, found {}\n{}",
                        child.node_type,
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
