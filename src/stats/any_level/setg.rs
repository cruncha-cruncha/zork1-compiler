use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

// <SETG CYCLOWRATH <- .COUNT>
// <SETG LOUD-FLAG T>
// <SETG P-CONT <>>
// <SETG LIT <LIT? ,HERE>>
// <SETG LIGHT-SHAFT 0>
// <SETG PRSA ,V?PUT>

pub struct SetG {}

impl HasZilName for SetG {
    fn zil_name(&self) -> &'static str {
        "SETG"
    }
}

impl CanValidate for SetG {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 3 {
            return Err(format!(
                "SETG node does not have three children\n{}",
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Token(_) => (),
            _ => {
                return Err(format!(
                    "Second child of SETG node is not a word\n{}",
                    format_file_location(&n)
                ));
            }
        }

        match n.children[2].node_type {
            ZilNodeType::Token(_) => (),
            ZilNodeType::Cluster => {
                return v.validate_cluster_or_null(&n.children[2]);
            }
            _ => {
                return Err(format!(
                    "Third child of SETG node is not a word or cluster\n{}",
                    format_file_location(&n)
                ));
            }
        }

        return Ok(());
    }
}
