use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::{TokenType, ZilNode, ZilNodeType},
};

// <VERB? CLOSE>
// <VERB? TAKE RAISE LOWER>
// <VERB? WALK-AROUND>
// <VERB? SGIVE>

pub struct VerbQ {}

impl HasZilName for VerbQ {
    fn zil_name(&self) -> &'static str {
        "VERB?"
    }
}

impl CanValidate for VerbQ {
    fn validate(&self, n: &ZilNode, _v: &Validator) -> Result<(), String> {
        if n.children.len() < 2 {
            return Err(format!(
                "VERB? node does not have at least two children\n{}",
                n
            ));
        }

        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Token(TokenType::Word) => (),
                _ => {
                    return Err(format!("Child of VERB? node is not a word\n{}", child));
                }
            }
        }

        Ok(())
    }
}
