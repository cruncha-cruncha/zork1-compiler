use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <EQUAL? .RARG ,M-LOOK>
// <EQUAL? ,HERE ,KITCHEN ,LIVING-ROOM ,ATTIC>
// <EQUAL? ,PRSO ,KNIFE ,SWORD ,AXE>
// <EQUAL? <LOC ,GARLIC> ,WINNER ,HERE>
// <EQUAL? <GET <GET ,P-ITBL ,P-NC1> 0> ,W?ALL>
pub struct EqualQ {}

impl HasZilName for EqualQ {
    fn zil_name(&self) -> &'static str {
        "EQUAL?"
    }
}

impl CanValidate for EqualQ {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() < 3 {
            return Err(format!(
                "EQUAL? node does not have at least three children\n{}",
                format_file_location(&n)
            ));
        }

        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Token(TokenType::Word) => (),
                ZilNodeType::Cluster => v.validate_cluster(child)?,
                _ => {
                    return Err(format!(
                        "Child of EQUAL? node is not a word or cluster\n{}",
                        format_file_location(&child)
                    ));
                }
            }
        }

        Ok(())
    }
}
