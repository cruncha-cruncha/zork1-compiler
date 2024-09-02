use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Disable {}

impl HasZilName for Disable {
    fn zil_name(&self) -> &'static str {
        "DISABLE"
    }
}

impl CanValidate for Disable {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
