use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Perform {}

impl HasZilName for Perform {
    fn zil_name(&self) -> &'static str {
        "PERFORM"
    }
}

impl CanValidate for Perform {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
