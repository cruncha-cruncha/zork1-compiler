use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{file_table::format_file_location, node::ZilNode},
};

// <CRLF>

pub struct Crlf {}

impl HasZilName for Crlf {
    fn zil_name(&self) -> &'static str {
        "CRLF"
    }
}

impl CanValidate for Crlf {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() != 1 {
            return Err(format!(
                "CRLF node does not have one child\n{}",
                format_file_location(&n)
            ));
        }

        Ok(())
    }
}
