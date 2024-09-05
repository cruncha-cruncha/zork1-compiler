use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <AND <VERB? BRUSH> <EQUAL? ,PRSO ,TEETH>>
// <AND <EQUAL? ,HERE ,LLD-ROOM> <NOT ,LLD-FLAG>>
// <AND ,XB <IN? ,CANDLES ,WINNER> <FSET? ,CANDLES ,ONBIT> <NOT ,XC>>
// <AND .HERE? <FSET? ,THIEF ,FIGHTBIT> <NOT <WINNING? ,THIEF>>>

pub struct Push {}

impl HasZilName for Push {
    fn zil_name(&self) -> &'static str {
        "PUSH"
    }
}

impl CanValidate for Push {
    fn validate(&self, v: &mut Validator, n: &ZilNode) -> Result<(), String> {
        if n.children.len() != 3 {
            return Err(format!(
                "Expected exactly 3 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => (),
            ZilNodeType::Cluster => v.validate_cluster(&n.children[1])?,
            _ => {
                return Err(format!(
                    "Expected word or cluster, found {}\n{}",
                    n.children[1].node_type,
                    format_file_location(&n)
                ));
            }
        }

        match n.children[2].node_type {
            ZilNodeType::Token(_) => (),
            ZilNodeType::Cluster => v.validate_cluster(&n.children[1])?,
            _ => {
                return Err(format!(
                    "Expected token or cluster, found {}\n{}",
                    n.children[1].node_type,
                    format_file_location(&n)
                ));
            }
        }

        Ok(())
    }
}
