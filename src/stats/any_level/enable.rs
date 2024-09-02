use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Enable {}

impl HasZilName for Enable {
    fn zil_name(&self) -> &'static str {
        "ENABLE"
    }
}

impl CanValidate for Enable {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
