use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Rest {}

impl HasZilName for Rest {
    fn zil_name(&self) -> &'static str {
        "REST"
    }
}

impl CanValidate for Rest {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
