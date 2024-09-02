use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct LengthQ {}

impl HasZilName for LengthQ {
    fn zil_name(&self) -> &'static str {
        "LENGTH?"
    }
}

impl CanValidate for LengthQ {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
