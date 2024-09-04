use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Getpt {}

impl HasZilName for Getpt {
    fn zil_name(&self) -> &'static str {
        "GETPT"
    }
}

impl CanValidate for Getpt {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
