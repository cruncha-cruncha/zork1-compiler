use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Remove {}

impl HasZilName for Remove {
    fn zil_name(&self) -> &'static str {
        "REMOVE"
    }
}

impl CanValidate for Remove {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
