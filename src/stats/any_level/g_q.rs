use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct GQ {}

impl HasZilName for GQ {
    fn zil_name(&self) -> &'static str {
        "G?"
    }
}

impl CanValidate for GQ {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
