use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

pub struct Return {}

impl HasZilName for Return {
    fn zil_name(&self) -> &'static str {
        "RETURN"
    }
}

impl CanValidate for Return {
    fn validate(&self, v: &mut Validator, n: &ZilNode) -> Result<(), String> {
        for child in n.children.iter().skip(1) {
            match child.node_type {
                ZilNodeType::Cluster => v.validate_cluster(child)?,
                ZilNodeType::Group => {
                    // TODO
                }
                _ => (),
            }
        }

        Ok(())
    }
}
