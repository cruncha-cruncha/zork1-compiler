use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <NOT <FSET? ,PRSO ,ACTORBIT>>
// <NOT <FIRST? ,BOTTLE>>
// <NOT ,KITCHEN-WINDOW-FLAG>
// <NOT .F>
// <NOT <IN? .V ,HERE>>
// <NOT ,THIEF-HERE>

pub struct Not {}

impl HasZilName for Not {
    fn zil_name(&self) -> &'static str {
        "NOT"
    }
}

impl CanValidate for Not {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 2 {
            return Err(format!(
                "NOT node does not have two children\n{}",
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => (),
            ZilNodeType::Cluster => v.validate_cluster(&n.children[1])?,
            _ => {
                return Err(format!(
                    "Second child of NOT node is not a word or cluster\n{}",
                    format_file_location(&n)
                ));
            }
        }

        Ok(())
    }
}
