use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <==? ,HERE ,KITCHEN>
// <==? .V ,M-FATAL>
// <==? ,ZORK-NUMBER 3>
// <==? .A ,V?WALK>
// <==? ,HERE ,RESERVOIR ,IN-STREAM>
// <==? <LENGTH .L> 3>

pub struct DoubleEqualSignQ {}

impl HasZilName for DoubleEqualSignQ {
    fn zil_name(&self) -> &'static str {
        "==?"
    }
}

impl CanValidate for DoubleEqualSignQ {
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
