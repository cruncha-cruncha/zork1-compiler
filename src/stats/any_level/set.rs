use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

// <SET W ,PRSO>
// <SET PI? <>>
// <SET HERE? <EQUAL? ,HERE ,MAINTENANCE-ROOM>>

pub struct Set {}

impl HasZilName for Set {
    fn zil_name(&self) -> &'static str {
        "SET"
    }
}

impl CanValidate for Set {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 3 {
            return Err(format!(
                "SET node does not have three children\n{}",
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Token(_) => (),
            _ => {
                return Err(format!(
                    "Second child of SET node is not a word\n{}",
                    format_file_location(&n)
                ));
            }
        }

        match n.children[2].node_type {
            ZilNodeType::Token(_) => (),
            ZilNodeType::Cluster => v.validate_cluster_or_null(&n.children[2])?,
            _ => {
                return Err(format!(
                    "Third child of SET node is not a word or cluster\n{}",
                    format_file_location(&n)
                ));
            }
        }

        Ok(())
    }
}
