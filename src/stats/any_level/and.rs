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

pub struct And {}

impl HasZilName for And {
    fn zil_name(&self) -> &'static str {
        "AND"
    }
}

impl CanValidate for And {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() < 3 {
            return Err(format!(
                "Expected at least 3 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Token(TokenType::Word) => (),
                ZilNodeType::Cluster => v.validate_cluster(child)?,
                _ => {
                    return Err(format!(
                        "Expected word, found {}\n{}",
                        child.node_type,
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
