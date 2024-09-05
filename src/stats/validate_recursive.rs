use std::{collections::HashMap, rc::Rc};

use crate::zil::{
    file_table::format_file_location,
    node::{ZilNode, ZilNodeType},
};

use super::{
    cross_ref::{Codex, CrossRef},
    helpers::get_token_as_word,
};

pub trait HasZilName {
    fn zil_name(&self) -> &'static str;
}

pub trait CanValidate: HasZilName {
    fn validate(&self, v: &mut Validator, n: &ZilNode) -> Result<(), String>;
}

pub struct Validator<'a> {
    pub cross_ref: &'a CrossRef,
    pub router: HashMap<&'static str, Rc<dyn CanValidate>>,
}

impl<'a> Validator<'a> {
    pub fn new(cross_ref: &'a CrossRef) -> Validator<'a> {
        Validator {
            cross_ref,
            router: Self::build_router(),
        }
    }

    pub fn build_router() -> HashMap<&'static str, Rc<dyn CanValidate>> {
        let mut router: HashMap<&'static str, Rc<dyn CanValidate>> = HashMap::new();

        let v = super::any_level::add::Add {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::and::And {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::cond::Cond {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::divide::Divide {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::each::Each {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::get_val::GetVal {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::is_asc::IsAsc {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::is_desc::IsDesc {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::is_equal::IsEqual {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::multiply::Multiply {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::not::Not {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::or::Or {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::pop::Pop {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::push::Push {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::return_::Return {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::set_var::SetVar {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::subtract::Subtract {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::tell::Tell {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::typeof_::Typeof {};
        router.insert(v.zil_name(), Rc::new(v));

        let v = super::any_level::unset_var::UnsetVar {};
        router.insert(v.zil_name(), Rc::new(v));

        router
    }

    pub fn validate_cluster(&mut self, n: &ZilNode) -> Result<(), String> {
        if n.node_type != ZilNodeType::Cluster {
            return Err(format!(
                "Node is not a cluster in validate_cluster\n{}",
                format_file_location(&n)
            ));
        }

        let name = match get_token_as_word(&n.children[0]) {
            Some(name) => name,
            None => {
                return Err(format!(
                    "Cluster node has no name\n{}",
                    format_file_location(&n)
                ));
            }
        };

        match self.router.get(name.as_str()) {
            Some(v) => return v.clone().validate(self, n),
            None => (),
        }

        if self.cross_ref.routines.as_codex().lookup(&name).is_some() {
            return Ok(());
        }

        return Err(format!(
            "Unknown cluster name {} in validate_cluster\n{}",
            name,
            format_file_location(&n)
        ));
    }
}
