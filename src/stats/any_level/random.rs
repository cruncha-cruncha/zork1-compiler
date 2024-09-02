use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Random {}

impl HasZilName for Random {
    fn zil_name(&self) -> &'static str {
        "RANDOM"
    }
}

impl CanValidate for Random {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
