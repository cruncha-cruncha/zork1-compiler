use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

pub struct Cond {}

impl HasZilName for Cond {
    fn zil_name(&self) -> &'static str {
        "COND"
    }
}

impl CanValidate for Cond {
    fn validate(&self, v: &mut Validator, n: &ZilNode) -> Result<(), String> {
        if n.children.len() < 2 {
            return Err(format!(
                "COND node has less than two children\n{}",
                format_file_location(&n)
            ));
        }

        for child in n.children.iter().skip(1) {
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

            for c in child.children.iter() {
                match c.node_type {
                    ZilNodeType::Cluster => v.validate_cluster(&c)?,
                    _ => {
                        return Err(format!(
                            "Child of COND node is not a cluster\n{}",
                            format_file_location(&c)
                        ));
                    }
                }
            }
        }

        return Ok(());
    }
}
