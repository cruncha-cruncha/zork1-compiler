use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{file_table::format_file_location, node::ZilNode},
};

// <RFATAL>

pub struct Rfatal {}

impl HasZilName for Rfatal {
    fn zil_name(&self) -> &'static str {
        "RFATAL"
    }
}

impl CanValidate for Rfatal {
    fn validate(&self, n: &ZilNode, _v: &Validator) -> Result<(), String> {
        if n.children.len() > 1 {
            return Err(format!(
                "RFATAL node has more than one child\n{}",
                format_file_location(&n)
            ));
        }

        Ok(())
    }
}
