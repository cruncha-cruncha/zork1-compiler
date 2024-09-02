use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Putp {}

impl HasZilName for Putp {
    fn zil_name(&self) -> &'static str {
        "PUTP"
    }
}

impl CanValidate for Putp {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
