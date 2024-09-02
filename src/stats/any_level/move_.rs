use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <MOVE ,AXE ,TROLL>
// <MOVE ,PRSO ,GRATING-ROOM>
// <MOVE ,PRSO <LOC ,WINNER>>

pub struct Move {}

impl HasZilName for Move {
    fn zil_name(&self) -> &'static str {
        "MOVE"
    }
}

impl CanValidate for Move {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 3 {
            return Err(format!(
                "MOVE node does not have three children\n{}",
                format_file_location(&n)
            ));
        }

        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Token(TokenType::Word) => (),
                ZilNodeType::Cluster => v.validate_cluster(child)?,
                _ => {
                    return Err(format!(
                        "Child of MOVE node is not a word or cluster\n{}",
                        format_file_location(&child)
                    ));
                }
            }
        }

        Ok(())
    }
}
