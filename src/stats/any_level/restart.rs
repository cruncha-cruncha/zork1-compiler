use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Restart {}

impl HasZilName for Restart {
    fn zil_name(&self) -> &'static str {
        "RESTART"
    }
}

impl CanValidate for Restart {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
