use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

pub struct Pop {}

impl HasZilName for Pop {
    fn zil_name(&self) -> &'static str {
        "POP"
    }
}

impl CanValidate for Pop {
    fn validate(&self, v: &mut Validator, n: &ZilNode) -> Result<(), String> {
        if n.children.len() != 2 {
            return Err(format!(
                "Expected exactly 2 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => (),
            ZilNodeType::Cluster => v.validate_cluster(&n.children[1])?,
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
