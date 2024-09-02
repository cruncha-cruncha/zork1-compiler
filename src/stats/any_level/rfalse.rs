use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{file_table::format_file_location, node::ZilNode},
};

// <RFALSE>

pub struct Rfalse {}

impl HasZilName for Rfalse {
    fn zil_name(&self) -> &'static str {
        "RFALSE"
    }
}

impl CanValidate for Rfalse {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() > 1 {
            return Err(format!(
                "RFALSE node has more than one child\n{}",
                format_file_location(&n)
            ));
        }

        Ok(())
    }
}
