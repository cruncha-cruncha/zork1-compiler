use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Putb {}

impl HasZilName for Putb {
    fn zil_name(&self) -> &'static str {
        "PUTB"
    }
}

impl CanValidate for Putb {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
