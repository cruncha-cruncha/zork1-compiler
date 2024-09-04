use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Ptsize {}

impl HasZilName for Ptsize {
    fn zil_name(&self) -> &'static str {
        "PTSIZE"
    }
}

impl CanValidate for Ptsize {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
