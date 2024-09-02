use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Primtype {}

impl HasZilName for Primtype {
    fn zil_name(&self) -> &'static str {
        "PRIMTYPE"
    }
}

impl CanValidate for Primtype {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
