use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

pub struct Tell {}

impl HasZilName for Tell {
    fn zil_name(&self) -> &'static str {
        "TELL"
    }
}

impl CanValidate for Tell {
    fn validate(&self, v: &mut Validator, n: &ZilNode) -> Result<(), String> {
        if n.children.len() < 2 {
            return Err(format!(
                "Expected at least 2 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Token(TokenType::Word) | ZilNodeType::Token(TokenType::Text) => (),
                ZilNodeType::Cluster => v.validate_cluster(child)?,
                _ => {
                    return Err(format!(
                        "Expected word, text, or cluster, found {}\n{}",
                        child.node_type,
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
