use std::collections::{HashMap, HashSet};

use crate::stats::cross_ref::{CrossRef, Populator};
use crate::stats::helpers::get_token_as_word;
use crate::zil::node::{TokenType, ZilNodeType};
use crate::zil::{file_table::format_file_location, node::ZilNode};

use crate::stats::cross_ref::Codex;

pub struct RoutineStats {
    basis: Vec<ZilNode>,
    all_routines: HashMap<String, RoutineInfo>,
}

// routines don't get any arguments
// they must define all their variable names at the top

pub struct RoutineInfo {
    index: usize,
    name: String,
    var_names: HashSet<String>,
}

impl RoutineStats {
    pub fn new() -> RoutineStats {
        RoutineStats {
            basis: Vec::new(),
            all_routines: HashMap::new(),
        }
    }

    pub fn as_codex(&self) -> RoutineCodex {
        RoutineCodex {
            index: 0,
            basis: &self.basis,
            all_routines: &self.all_routines,
        }
    }

    fn validate_var_group(node: &ZilNode) -> Result<HashSet<String>, String> {
        let mut out: HashSet<String> = HashSet::new();
        for c in node.children.iter() {
            if c.node_type != ZilNodeType::Token(TokenType::Word) {
                return Err(format!(
                    "Routine node has non-word child in vars definition group\n{}",
                    format_file_location(&node)
                ));
            }

            let word = get_token_as_word(c).unwrap();
            if out.contains(&word) {
                return Err(format!(
                    "Routine node has duplicate variable name in vars definition group\n{}",
                    format_file_location(&node)
                ));
            }

            out.insert(word);
        }

        Ok(out)
    }
}

impl Populator for RoutineStats {
    fn add_node(&mut self, node: ZilNode) {
        self.basis.push(node);
    }

    fn crunch(&mut self) -> Result<(), String> {
        for (i, node) in self.basis.iter().enumerate() {
            if node.children.len() < 3 {
                return Err(format!(
                    "Possible routine node doesn't have enough children\n{}",
                    format_file_location(&node)
                ));
            }

            let first_word = get_token_as_word(&node.children[0]).unwrap_or_default();
            if first_word != "ROUTINE" {
                unreachable!();
            }

            let second_word = get_token_as_word(&node.children[1]);
            if second_word.is_none() {
                return Err(format!(
                    "Routine node has non-word second child\n{}",
                    format_file_location(&node)
                ));
            }

            if node.children[2].node_type != ZilNodeType::Group {
                return Err(format!(
                    "Routine node has non-group third child\n{}",
                    format_file_location(&node)
                ));
            }

            let var_names = match RoutineStats::validate_var_group(&node.children[2]) {
                Ok(vars) => vars,
                Err(e) => {
                    return Err(e);
                }
            };

            for c in node.children.iter().skip(3) {
                if c.node_type != ZilNodeType::Cluster {
                    return Err(format!(
                        "Routine node has non-cluster body child\n{}",
                        format_file_location(&node)
                    ));
                }
            }

            let name = second_word.unwrap();
            match self.all_routines.insert(
                name.clone(),
                RoutineInfo {
                    index: i,
                    name,
                    var_names,
                },
            ) {
                Some(old_val) => {
                    return Err(format!(
                        "Duplicate routine name: {}\n{}",
                        old_val.name,
                        format_file_location(&node)
                    ));
                }
                None => {}
            }
        }

        Ok(())
    }

    fn validate(&self, _cross_ref: &CrossRef) -> Result<(), String> {
        for (key, val) in self.all_routines.iter() {
            if CrossRef::name_is_illegal(key) {
                return Err(format!("Illegal routine name: {}", key));
            }

            for var in val.var_names.iter() {
                if CrossRef::name_is_illegal(var) {
                    return Err(format!("Illegal variable name: {}", var));
                }
            }
        }

        // deeper recursive validation is performed later (see CanValidate)

        Ok(())
    }
}

pub struct RoutineCodex<'a> {
    index: usize,
    basis: &'a Vec<ZilNode>,
    all_routines: &'a HashMap<String, RoutineInfo>,
}
pub struct RoutineCodexValue<'a> {
    pub name: &'a String,
    pub var_names: &'a HashSet<String>,
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
