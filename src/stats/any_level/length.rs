use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Length {}

impl HasZilName for Length {
    fn zil_name(&self) -> &'static str {
        "LENGTH"
    }
}

impl CanValidate for Length {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
