use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::node::ZilNode,
};

//

pub struct Mod {}

impl HasZilName for Mod {
    fn zil_name(&self) -> &'static str {
        "MOD"
    }
}

impl CanValidate for Mod {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        Ok(())
    }
}
