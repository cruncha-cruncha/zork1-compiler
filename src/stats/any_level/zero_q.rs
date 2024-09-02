use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <0? ,CYCLOWRATH>
// <0? <GETB ,P-LEXV ,P-LEXWORDS>>
// <0? .TICK>
// <0? <MOD ,HS 10>>

pub struct ZeroQ {}

impl HasZilName for ZeroQ {
    fn zil_name(&self) -> &'static str {
        "0?"
    }
}

impl CanValidate for ZeroQ {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 2 {
            return Err(format!(
                "0? node does not have two children\n{}",
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => (),
            ZilNodeType::Cluster => v.validate_cluster(&n.children[1])?,
            _ => {
                return Err(format!(
                    "Second child of 0? node is not a word or cluster\n{}",
                    format_file_location(&n)
                ));
            }
        }

        Ok(())
    }
}
