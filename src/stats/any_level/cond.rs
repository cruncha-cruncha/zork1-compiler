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

// starts a conditional block
// each group child is a branch
// first node in the group is the condition
// takes whichever branch who's condition is true first

pub struct Cond {
    pub branches: Vec<Branch>,
}

pub struct Branch {
    pub condition: OutputNode,
    pub body: Vec<OutputNode>,
}

impl Cond {
    pub fn new() -> Self {
        Self {
            branches: Vec::new(),
        }
    }
}

impl HasReturnType for Cond {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::None]
    }
}

impl CanValidate for Cond {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String> {
        num_children_more_than(n, 1)?;

        for child in n.children.iter().skip(1) {
            let mut branch = Branch {
                condition: OutputNode::TBD,
                body: Vec::new(),
            };

            if child.node_type != ZilNodeType::Group {
                return Err(format!(
                    "Child of COND node is not a group\n{}",
                    format_file_location(&child)
                ));
            }

            if child.children.len() == 0 {
                return Err(format!(
                    "Group node of COND has no children\n{}",
                    format_file_location(&child)
                ));
            }

            for (i, gc) in child.children.iter().enumerate() {
                if i == 0 {
                    v.expect_val(ReturnValType::Boolean);
                } else {
                    v.expect_val(ReturnValType::Any);
                }

                match gc.node_type {
                    ZilNodeType::Cluster => match v.validate_cluster(&gc) {
                        Ok(_) => match v.take_last_writer() {
                            Some(w) => {
                                if i == 0 {
                                    branch.condition = OutputNode::Writer(w);
                                } else {
                                    branch.body.push(OutputNode::Writer(w));
                                }
                            }
                            None => unreachable!(),
                        },
                        Err(e) => return Err(e),
                    },
                    _ => {
                        return Err(format!(
                            "Child of COND group is not a cluster\n{}",
                            format_file_location(&gc)
                        ));
                    }
                }
            }

            self.branches.push(branch);
        }

        return Ok(());
    }
}
