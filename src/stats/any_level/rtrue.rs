use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{file_table::format_file_location, node::ZilNode},
};

// <RTRUE>

pub struct Rtrue {}

impl HasZilName for Rtrue {
    fn zil_name(&self) -> &'static str {
        "RTRUE"
    }
}

impl CanValidate for Rtrue {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() > 1 {
            return Err(format!(
                "RTRUE node has more than one child\n{}",
                format_file_location(&n)
            ));
        }

        Ok(())
    }
}
