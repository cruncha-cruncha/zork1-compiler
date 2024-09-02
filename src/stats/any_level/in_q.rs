use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <IN? ,PRSI ,WINNER>
// <IN? .W .AV>
// <IN? .OBJ ,GLOBAL-OBJECTS>
// <IN? <SET O <GET .OO ,V-VILLAIN>> ,HERE>

pub struct InQ {}

impl HasZilName for InQ {
    fn zil_name(&self) -> &'static str {
        "IN?"
    }
}

impl CanValidate for InQ {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 3 {
            return Err(format!(
                "Expected 3 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Token(TokenType::Word) => (),
            ZilNodeType::Cluster => v.validate_cluster(&n.children[1])?,
            _ => {
                return Err(format!(
                    "Expected word, found {}\n{}",
                    n.children[1].node_type,
                    format_file_location(&n)
                ));
            }
        }

        Ok(())
    }
}
