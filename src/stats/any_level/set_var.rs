use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

pub struct SetVar {}

impl HasZilName for SetVar {
    fn zil_name(&self) -> &'static str {
        "SET-VAR"
    }
}

impl CanValidate for SetVar {
    fn validate(&self, v: &mut Validator, n: &ZilNode) -> Result<(), String> {
        if n.children.len() != 3 && n.children.len() != 4 {
            return Err(format!(
                "Expected 3 or 4 children, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        // TODO

        Ok(())
    }
}
