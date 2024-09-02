use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Put {}

impl HasZilName for Put {
    fn zil_name(&self) -> &'static str {
        "PUT"
    }
}

impl CanValidate for Put {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
