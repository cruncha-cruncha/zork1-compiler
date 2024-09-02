use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct NextQ {}

impl HasZilName for NextQ {
    fn zil_name(&self) -> &'static str {
        "NEXT?"
    }
}

impl CanValidate for NextQ {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
