use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <FIRST? .OBJ>
// <FIRST? ,HERE>

pub struct FirstQ {}

impl HasZilName for FirstQ {
    fn zil_name(&self) -> &'static str {
        "FIRST?"
    }
}

impl CanValidate for FirstQ {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 2 {
            return Err(format!(
                "FIRST? node does not have two children\n{}",
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => (),
            ZilNodeType::Cluster => v.validate_cluster(&n.children[1])?,
            _ => {
                return Err(format!(
                    "Second child of FIRST? node is not a word or cluster\n{}",
                    format_file_location(&n)
                ));
            }
        }

        Ok(())
    }
}
