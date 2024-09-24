use std::collections::{BTreeMap, BTreeSet};

use crate::stats::cross_ref::{CrossRef, Populator};
use crate::stats::helpers::{get_token_as_word, num_children_more_than, ValidationResult};
use crate::zil::node::ZilNodeType;
use crate::zil::{file_table::format_file_location, node::ZilNode};

use crate::stats::cross_ref::Codex;

pub struct RoutineStats {
    basis: Vec<ZilNode>,
    all_routines: BTreeMap<String, RoutineInfo>,
    all_handlers: BTreeMap<String, Vec<HandlerInfo>>,
}

// routines don't get any arguments
// they must define all their variable names at the top

pub struct RoutineInfo {
    index: usize,
    name: String,
    var_names: BTreeSet<String>,
}

pub struct HandlerInfo {
    index: usize,
    pub action: String,
    pub object: Option<String>,
    pub before: bool, // if true: ACTION OBJ (), if false: ACTION () OBJ
    pub var_names: BTreeSet<String>,
}

impl HandlerInfo {
    pub fn get_key(&self) -> String {
        return HandlerInfo::format_key(&self.action, self.object.as_ref(), self.before);
    }

    pub fn format_key(action: &str, object: Option<&String>, before: bool) -> String {
        if object.is_none() {
            return String::from(action);
        }

        if before {
            return format!("{}_{}", object.unwrap(), action);
        } else {
            return format!("{}_{}", action, object.unwrap());
        }
    }
}

impl RoutineStats {
    pub fn new() -> RoutineStats {
        RoutineStats {
            basis: Vec::new(),
            all_routines: BTreeMap::new(),
            all_handlers: BTreeMap::new(),
        }
    }

    pub fn as_codex(&self) -> RoutineCodex {
        RoutineCodex {
            index: 0,
            basis: &self.basis,
            all_routines: &self.all_routines,
        }
    }

    pub fn iter_actions(&self) -> HandlerIter {
        HandlerIter {
            index: 0,
            basis: &self.basis,
            all_handlers: &self.all_handlers,
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

            let first_word = get_token_as_word(&node.children[0]).unwrap();
            if first_word == "ROUTINE" {
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
            } else {
                let group_index: usize;
                if node.children[1].node_type == ZilNodeType::Group {
                    group_index = 1;
                } else if node.children[2].node_type == ZilNodeType::Group {
                    group_index = 2;
                } else {
                    errors.push(format!(
                        "Possible handler node has non-group second and third children\n{}",
                        format_file_location(&node)
                    ));
                    continue;
                }

                let mut object_name: Option<String> = None;
                if group_index == 1 {
                    object_name = match get_token_as_word(&node.children[2]) {
                        Ok(v) => Some(v),
                        _ => None,
                    };
                } else if group_index == 2 {
                    object_name = match get_token_as_word(&node.children[1]) {
                        Ok(v) => Some(v),
                        Err(e) => {
                            errors.push(e);
                            continue;
                        }
                    };
                }

                let var_names = match RoutineStats::crunch_var_names(&node.children[group_index]) {
                    Ok(v) => v,
                    Err(e) => {
                        routine_errors.push(e);
                        BTreeSet::new()
                    }
                };

                let skip = if object_name.is_none() { 2 } else { 3 };
                for c in node.children.iter().skip(skip) {
                    if c.node_type != ZilNodeType::Cluster {
                        routine_errors.push(format!(
                            "Possible handler node has non-cluster body child\n{}",
                            format_file_location(&node)
                        ));
                    }
                }

                let val = HandlerInfo {
                    index: i,
                    action: first_word,
                    object: object_name,
                    before: group_index == 2,
                    var_names,
                };

                let key = val.get_key();

                if self.all_handlers.contains_key(&val.action) {
                    let action = self.all_handlers.get_mut(&val.action).unwrap();
                    let found = action.iter().any(|x| {
                        x.action == val.action && x.object == val.object && x.before == val.before
                    });
                    if found {
                        routine_errors.push(format!(
                            "Duplicate handler: {}\n{}",
                            key,
                            format_file_location(&node)
                        ));
                    } else {
                        action.push(val);
                    }
                } else {
                    self.all_handlers.insert(val.action.clone(), vec![val]);
                }
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

    fn validate(&self, cross_ref: &CrossRef) -> ValidationResult<()> {
        let object_codex = cross_ref.objects.as_codex();
        let syntax_codex = cross_ref.syntax.as_codex();
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

        for (key, val) in self.all_handlers.iter() {
            if syntax_codex.lookup(key).is_none() {
                errors.push(format!("Handler references unknown syntax: {}", key));
                continue;
            }

            for handle in val.iter() {
                for var in handle.var_names.iter() {
                    if CrossRef::name_is_illegal(var) {
                        errors.push(format!("Illegal variable name: {}", var));
                    }
                }

                if handle.object.is_some() {
                    if object_codex
                        .lookup(handle.object.as_ref().unwrap())
                        .is_none()
                    {
                        errors.push(format!(
                            "Handler references unknown object: {}\n{}",
                            handle.object.as_ref().unwrap(),
                            format_file_location(&self.basis[handle.index])
                        ));
                    }
                }
            }
        }

        // TODO: routine names cannot be sytax actions

        // deeper recursive validation is performed later (impl CanValidate for RoutineRoot)

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
            let mut info: Option<&RoutineInfo>;
            loop {
                if self.index >= self.basis.len() {
                    return None;
                }

                let node = &self.basis[self.index];
                let key = get_token_as_word(&node.children[1]).unwrap_or_default();
                info = self.all_routines.get(&key);

                if info.is_some() {
                    break;
                } else {
                    self.index += 1;
                }
            }
            let info = info.unwrap();
            self.index += 1;

            Some(RoutineCodexValue {
                name: &info.name,
                var_names: &info.var_names,
                node: &self.basis[info.index],
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

pub struct HandlerIter<'a> {
    index: usize,
    basis: &'a Vec<ZilNode>,
    all_handlers: &'a BTreeMap<String, Vec<HandlerInfo>>,
}

pub struct HandlerIterValue<'a> {
    pub name: String,
    pub handlers: &'a Vec<HandlerInfo>,
    pub nodes: Vec<&'a ZilNode>,
}

impl<'a> Iterator for HandlerIter<'a> {
    type Item = HandlerIterValue<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.all_handlers.len() {
            None
        } else {
            let key = self.all_handlers.keys().nth(self.index).unwrap();
            let list = self.all_handlers.get(key).unwrap();
            self.index += 1;

            Some(HandlerIterValue {
                name: key.clone(),
                handlers: list,
                nodes: list.iter().map(|v| &self.basis[v.index]).collect(),
            })
        }
    }
}
