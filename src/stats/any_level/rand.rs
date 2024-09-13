use crate::{
    stats::routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    zil::{file_table::format_file_location, node::ZilNode},
};

pub struct Rand {}

impl Rand {
    pub fn new() -> Self {
        Self {}
    }
}

impl HasReturnType for Rand {
    fn return_type(&self) -> ReturnValType {
        // returns an integer from 0 to 99 (inclusive)
        ReturnValType::Number
    }
}

impl CanValidate for Rand {
    fn validate(&mut self, _v: &mut Validator, n: &ZilNode) -> Result<(), String> {
        if n.children.len() != 1 {
            return Err(format!(
                "Expected exactly 1 child, found {}\n{}",
                n.children.len(),
                format_file_location(&n)
            ));
        }

        Ok(())
    }
}
