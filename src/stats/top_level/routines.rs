use std::collections::{BTreeMap, BTreeSet};

use crate::stats::cross_ref::{CrossRef, Populator};
use crate::stats::helpers::{get_token_as_word, num_children_more_than, ValidationResult};
use crate::zil::node::ZilNodeType;
use crate::zil::{file_table::format_file_location, node::ZilNode};

use crate::stats::cross_ref::Codex;

pub struct RoutineStats {
    basis: Vec<ZilNode>,
    all_routines: BTreeMap<String, RoutineInfo>,
}

// routines don't get any arguments
// they must define all their variable names at the top

pub struct RoutineInfo {
    index: usize,
    name: String,
    var_names: BTreeSet<String>,
}

impl RoutineStats {
    pub fn new() -> RoutineStats {
        RoutineStats {
            basis: Vec::new(),
            all_routines: BTreeMap::new(),
        }
    }

    pub fn as_codex(&self) -> RoutineCodex {
        RoutineCodex {
            index: 0,
            basis: &self.basis,
            all_routines: &self.all_routines,
        }
    }

    fn crunch_var_names(node: &ZilNode) -> Result<BTreeSet<String>, String> {
        let mut out: BTreeSet<String> = BTreeSet::new();
        for c in node.children.iter() {
            let word = match get_token_as_word(c) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            out.insert(word);
        }

        Ok(out)
    }
}

impl Populator for RoutineStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> ValidationResult<()> {
        let mut errors: Vec<String> = Vec::new();
        for (i, node) in self.basis.iter().enumerate() {
            let mut routine_errors: Vec<String> = Vec::new();

            match num_children_more_than(node, 2) {
                Ok(_) => {}
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            }

            let routine_name = match get_token_as_word(&node.children[1]) {
                Ok(v) => v,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            if node.children[2].node_type != ZilNodeType::Group {
                routine_errors.push(format!(
                    "Routine node has non-group third child\n{}",
                    format_file_location(&node)
                ));
            }

            let var_names = match RoutineStats::crunch_var_names(&node.children[2]) {
                Ok(v) => v,
                Err(e) => {
                    routine_errors.push(e);
                    BTreeSet::new()
                }
            };

            for c in node.children.iter().skip(3) {
                if c.node_type != ZilNodeType::Cluster {
                    routine_errors.push(format!(
                        "Routine node has non-cluster body child\n{}",
                        format_file_location(&node)
                    ));
                }
            }

            match self.all_routines.insert(
                routine_name.clone(),
                RoutineInfo {
                    index: i,
                    name: routine_name,
                    var_names,
                },
            ) {
                Some(old_val) => {
                    routine_errors.push(format!(
                        "Duplicate routine name: {}\n{}",
                        old_val.name,
                        format_file_location(&node)
                    ));
                }
                None => {}
            }

            if routine_errors.len() > 0 {
                errors.append(&mut routine_errors);
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &CrossRef) -> ValidationResult<()> {
        let mut errors: Vec<String> = Vec::new();
        for (key, val) in self.all_routines.iter() {
            if CrossRef::name_is_illegal(key) {
                errors.push(format!("Illegal routine name: {}", key));
            }

            for var in val.var_names.iter() {
                if CrossRef::name_is_illegal(var) {
                    errors.push(format!("Illegal variable name: {}", var));
                }
            }
        }

        // deeper recursive validation is performed later (see CanValidate)

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(())
    }
}

pub struct RoutineCodex<'a> {
    index: usize,
    basis: &'a Vec<ZilNode>,
    all_routines: &'a BTreeMap<String, RoutineInfo>,
}
pub struct RoutineCodexValue<'a> {
    pub name: &'a String,
    pub var_names: &'a BTreeSet<String>,
    pub node: &'a ZilNode,
}

impl<'a> Iterator for RoutineCodex<'a> {
    type Item = RoutineCodexValue<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.basis.len() {
            None
        } else {
            self.index += 1;
            let node = &self.basis[self.index - 1];
            let key = get_token_as_word(&node.children[1]).unwrap();
            let info = self.all_routines.get(&key).unwrap();

            Some(RoutineCodexValue {
                name: &info.name,
                var_names: &info.var_names,
                node: &node,
            })
        }
    }
}

impl<'a> Codex<RoutineCodexValue<'a>> for RoutineCodex<'a> {
    fn lookup(&self, word: &str) -> Option<RoutineCodexValue<'a>> {
        let info = self.all_routines.get(word);

        if info.is_none() {
            return None;
        }

        let info = info.unwrap();

        return Some(RoutineCodexValue {
            name: &info.name,
            var_names: &info.var_names,
            node: &self.basis[info.index],
        });
    }
}
