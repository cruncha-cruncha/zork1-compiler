use crate::{
    stats::routine_tracker::{CanValidate, HasReturnType, ReturnValType, Validator},
    zil::{file_table::format_file_location, node::ZilNode},
};

pub struct End {}

impl End {
    pub fn new() -> Self {
        Self {}
    }
}

impl HasReturnType for End {
    fn return_type(&self) -> Vec<ReturnValType> {
        vec![ReturnValType::None]
    }
}

impl CanValidate for End {
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
