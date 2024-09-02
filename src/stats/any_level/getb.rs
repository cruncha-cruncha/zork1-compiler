use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Getb {}

impl HasZilName for Getb {
    fn zil_name(&self) -> &'static str {
        "GETB"
    }
}

impl CanValidate for Getb {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
