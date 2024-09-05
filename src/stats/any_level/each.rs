use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

pub struct Each {}

impl HasZilName for Each {
    fn zil_name(&self) -> &'static str {
        "EACH"
    }
}

impl CanValidate for Each {
    fn validate(&self, v: &mut Validator, n: &ZilNode) -> Result<(), String> {
        if n.children.len() < 3 {
            return Err(format!(
                "Expected at least 3 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        if n.children[1].node_type != ZilNodeType::Group {
            return Err(format!(
                "Expected group, found {}\n{}",
                n.children[1].node_type,
                format_file_location(&n)
            ));
        } else if n.children[1].children.len() != 2 {
            return Err(format!(
                "Expected group to have two children, found {}\n{}",
                n.children[1].children.len(),
                format_file_location(&n)
            ));
        }

        for grand_child in n.children[1].children.iter() {
            if grand_child.node_type != ZilNodeType::Token(TokenType::Word) {
                return Err(format!(
                    "Var name in EACH group is not a word\n{}",
                    format_file_location(&n)
                ));
            }
        }

        for child in n.children.iter().skip(2) {
            match child.node_type {
                ZilNodeType::Cluster => v.validate_cluster(child)?,
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
