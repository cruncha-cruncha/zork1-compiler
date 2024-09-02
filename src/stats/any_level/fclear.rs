use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <FCLEAR .OBJ ,OPENBIT>
// <FCLEAR ,ATTIC-TABLE ,NDESCBIT>
// <FCLEAR .X ,INVISIBLE>

pub struct Fclear {}

impl HasZilName for Fclear {
    fn zil_name(&self) -> &'static str {
        "FCLEAR"
    }
}

impl CanValidate for Fclear {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 3 {
            return Err(format!(
                "FCLEAR node does not have three children\n{}",
                format_file_location(&n)
            ));
        }

        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Token(TokenType::Word) => (),
                _ => {
                    return Err(format!(
                        "Child of FCLEAR node is not a word\n{}",
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
