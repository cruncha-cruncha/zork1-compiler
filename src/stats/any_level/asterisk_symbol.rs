use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <* .CNT ,FUMBLE-PROB>
// <* ,CURE-WAIT <- .WD 1>>
// <* <GETB ,P-LEXV ,P-LEXWORDS> ,P-WORDLEN>

pub struct AsteriskSymbol {}

impl HasZilName for AsteriskSymbol {
    fn zil_name(&self) -> &'static str {
        "*"
    }
}

impl CanValidate for AsteriskSymbol {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 3 {
            return Err(format!(
                "Expected 3 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Token(TokenType::Number) | ZilNodeType::Token(TokenType::Word) => (),
                ZilNodeType::Cluster => v.validate_cluster(child)?,
                _ => {
                    return Err(format!(
                        "Expected number, token, token bunch, or cluster, found {}\n{}",
                        child.node_type,
                        format_file_location(&n)
                    ));
                }
            }
        }

        Ok(())
    }
}
