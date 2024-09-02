use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Lkp {}

impl HasZilName for Lkp {
    fn zil_name(&self) -> &'static str {
        "LKP"
    }
}

impl CanValidate for Lkp {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
