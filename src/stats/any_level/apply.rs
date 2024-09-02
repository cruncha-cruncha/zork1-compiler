use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Apply {}

impl HasZilName for Apply {
    fn zil_name(&self) -> &'static str {
        "APPLY"
    }
}

impl CanValidate for Apply {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
