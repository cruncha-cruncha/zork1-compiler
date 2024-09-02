use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

pub struct Prob {}

impl HasZilName for Prob {
    fn zil_name(&self) -> &'static str {
        "PROB"
    }
}

impl CanValidate for Prob {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() < 2 || n.children.len() > 3 {
            return Err(format!(
                "PROB node does not have two or three children\n{}",
                format_file_location(&n)
            ));
        }

        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Token(TokenType::Word) | ZilNodeType::Token(TokenType::Number) => (),
                ZilNodeType::Cluster => v.validate_cluster(child)?,
                _ => {
                    return Err(format!(
                        "Child of PROB node is not a word or cluster\n{}",
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
