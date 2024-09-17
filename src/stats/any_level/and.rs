use crate::{
    js::write_output::OutputNode,
    stats::{
        helpers::num_children_more_than,
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

pub struct And {
    pub values: Vec<OutputNode>,
}

impl And {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl HasReturnType for And {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::Boolean]
    }
}

impl CanValidate for And {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_more_than(n, 2)?;

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
