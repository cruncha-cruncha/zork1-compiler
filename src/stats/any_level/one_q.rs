use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <1? .CNT>
// <1? <GET <INT I-THIEF> ,C-ENABLED?>>
// <1? ,DEATHS>

pub struct OneQ {}

impl HasZilName for OneQ {
    fn zil_name(&self) -> &'static str {
        "1?"
    }
}

impl CanValidate for OneQ {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 2 {
            return Err(format!(
                "1? node does not have two children\n{}",
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => (),
            ZilNodeType::Cluster => v.validate_cluster(&n.children[1])?,
            _ => {
                return Err(format!(
                    "Second child of 1? node is not a word or cluster\n{}",
                    format_file_location(&n)
                ));
            }
        }

        Ok(())
    }
}
