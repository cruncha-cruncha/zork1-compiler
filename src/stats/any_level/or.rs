use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <OR ,LOUD-FLAG <AND <NOT ,GATES-OPEN> ,LOW-TIDE>>
// <OR <VERB? LEAP> <AND <VERB? PUT> <EQUAL? ,PRSO ,ME>>>
// <OR <FSET? .O ,FIGHTBIT> <APPLY <GETP .O ,P?ACTION> ,F-FIRST?>>

pub struct Or {}

impl HasZilName for Or {
    fn zil_name(&self) -> &'static str {
        "OR"
    }
}

impl CanValidate for Or {
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