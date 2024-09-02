use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <TELL "open.">
// <TELL <PICK-ONE ,DUMMY>>
// <TELL "The boards are securely fastened." CR>
// <TELL "It is hardly likely that the " D ,PRSO " is interested." CR>

pub struct Tell {}

impl HasZilName for Tell {
    fn zil_name(&self) -> &'static str {
        "TELL"
    }
}

impl CanValidate for Tell {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() < 2 {
            return Err(format!(
                "TELL node does not have enough children\n{}",
                format_file_location(&n)
            ));
        }

        for child in &n.children[1..] {
            match child.node_type {
                ZilNodeType::Token(TokenType::Word) | ZilNodeType::Token(TokenType::Text) => (),
                ZilNodeType::Cluster => v.validate_cluster(&child)?,
                _ => {
                    return Err(format!(
                        "Child of TELL node is not a word or cluster\n{}",
                        format_file_location(&child)
                    ));
                }
            }
        }

        Ok(())
    }
}
