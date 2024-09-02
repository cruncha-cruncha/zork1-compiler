use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Getp {}

impl HasZilName for Getp {
    fn zil_name(&self) -> &'static str {
        "GETP"
    }
}

impl CanValidate for Getp {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
