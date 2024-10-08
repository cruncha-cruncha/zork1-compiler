use crate::{
    js::write_output::OutputNode,
    stats::{
        helpers::num_children,
        routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    },
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

pub struct Not {
    pub value: OutputNode,
}

impl Not {
    pub fn new() -> Self {
        Self {
            value: OutputNode::TBD,
        }
    }
}

impl HasReturnType for Not {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::Boolean]
    }
}

impl CanValidate for Not {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children(n, 2)?;

        v.expect_val(ReturnValType::Boolean);

        match n.children[1].node_type {
            ZilNodeType::Cluster => match v.validate_cluster(&n.children[1]) {
                Ok(_) => match v.take_last_writer() {
                    Some(w) => self.value = OutputNode::Writer(w),
                    None => unreachable!(),
                },
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "Expected word or cluster, found {}\n{}",
                    n.children[1].node_type,
                    format_file_location(&n)
                ));
            }
        }

        Ok(())
    }
}
