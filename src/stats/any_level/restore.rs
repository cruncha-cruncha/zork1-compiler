use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{file_table::format_file_location, node::ZilNode},
};

// <RESTORE>

pub struct Restore {}

impl HasZilName for Restore {
    fn zil_name(&self) -> &'static str {
        "RESTORE"
    }
}

impl CanValidate for Restore {
    fn validate(&self, n: &ZilNode, _v: &Validator) -> Result<(), String> {
        if n.children.len() > 1 {
            return Err(format!(
                "RESTORE node has more than one child\n{}",
                format_file_location(&n)
            ));
        }

        Ok(())
    }
}
