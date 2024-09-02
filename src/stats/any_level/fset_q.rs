use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <FSET? .OBJ ,OPENBIT>
// <FSET? ,HERE ,ONBIT>
// <FSET? <LOC ,WINNER> ,VEHBIT>>

pub struct FsetQ {}

impl HasZilName for FsetQ {
    fn zil_name(&self) -> &'static str {
        "FSET?"
    }
}

impl CanValidate for FsetQ {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 3 {
            return Err(format!(
                "FSET? node does not have three children\n{}",
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => (),
            ZilNodeType::Cluster => v.validate_cluster(&n.children[1])?,
            _ => {
                return Err(format!(
                    "Second child of FSET? node is not a word or cluster\n{}",
                    format_file_location(&n)
                ));
            }
        }

        match n.children[2].node_type {
            ZilNodeType::Token(TokenType::Word) => {
                // TODO: word must start with "," (aka is global)
            }
            _ => {
                return Err(format!(
                    "Third child of FSET? node is not a word\n{}",
                    format_file_location(&n)
                ));
            }
        }

        Ok(())
    }
}
