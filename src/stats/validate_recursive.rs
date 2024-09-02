use std::collections::HashMap;

use crate::zil::{
    file_table::format_file_location,
    node::{ZilNode, ZilNodeType},
};

use super::{
    cross_ref::{Codex, CrossRef},
    helpers::get_nth_child_as_word,
};

// how can we rearrange the tree?
// A[B,C] -> A[B,C]  // no change
// A[B,C] -> A[C,B]  // swap children
// A[B,C] -> B[A,C]  // swap child with parent
// A[B,C] -> A[C],B  // elevate first child

pub trait HasZilName {
    fn zil_name(&self) -> &'static str;
}

pub trait CanValidate: HasZilName {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String>;
}

pub struct Validator<'a> {
    pub cross_ref: &'a CrossRef,
    pub router: HashMap<&'static str, Box<dyn CanValidate>>,
}

impl<'a> Validator<'a> {
    pub fn new(cross_ref: &'a CrossRef) -> Validator<'a> {
        Validator {
            cross_ref,
            router: Self::build_router(),
        }
    }

    pub fn build_router() -> HashMap<&'static str, Box<dyn CanValidate>> {
        let mut router: HashMap<&'static str, Box<dyn CanValidate>> = HashMap::new();

        let v = super::any_level::and::And {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::apply::Apply {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::asterisk_symbol::AsteriskSymbol {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::cond::Cond {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::crlf::Crlf {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::disable::Disable {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::double_equal_sign_q::DoubleEqualSignQ {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::enable::Enable {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::equal_q::EqualQ {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::fclear::Fclear {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::first_q::FirstQ {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::forward_slash_symbol::ForwardSlashSymbol {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::fset_q::FsetQ {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::fset::Fset {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::g_q::GQ {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::get::Get {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::getb::Getb {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::getp::Getp {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::in_q::InQ {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::int::Int {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::length_q::LengthQ {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::length::Length {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::lkp::Lkp {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::loc::Loc {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::minus_sign::MinusSign {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::mod_::Mod {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::move_::Move {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::next_q::NextQ {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::not::Not {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::one_q::OneQ {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::or::Or {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::perform::Perform {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::plus_sign::PlusSign {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::primtype::Primtype {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::prob::Prob {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::put::Put {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::putb::Putb {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::putp::Putp {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::queue::Queue {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::random::Random {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::remove::Remove {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::repeat::Repeat {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::restart::Restart {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::return_::Return {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::rfalse::Rfalse {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::rtrue::Rtrue {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::set::Set {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::setg::SetG {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::tell::Tell {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::verb_q::VerbQ {};
        router.insert(v.zil_name(), Box::new(v));

        let v = super::any_level::zero_q::ZeroQ {};
        router.insert(v.zil_name(), Box::new(v));

        router
    }

    pub fn validate_cluster(&self, n: &ZilNode) -> Result<(), String> {
        if n.node_type != ZilNodeType::Cluster {
            return Err(format!(
                "Node is not a cluster in validate_cluster\n{}",
                format_file_location(&n)
            ));
        }

        let name = match get_nth_child_as_word(0, n) {
            Some(name) => name,
            None => {
                return Err(format!(
                    "Cluster node has no name\n{}",
                    format_file_location(&n)
                ));
            }
        };

        match self.router.get(name.as_str()) {
            Some(v) => {
                return v.validate(n, self);
            }
            None => (),
        }

        if self.cross_ref.routines.lookup(&name).is_some() {
            // verify the arguments
        } else {
            return Err(format!(
                "Unknown cluster name {} in validate_cluster\n{}",
                name,
                format_file_location(&n)
            ));
        }

        Ok(())
    }

    pub fn validate_cluster_or_null(&self, n: &ZilNode) -> Result<(), String> {
        if n.children.len() == 0 {
            return Ok(());
        }

        self.validate_cluster(n)
    }
}
