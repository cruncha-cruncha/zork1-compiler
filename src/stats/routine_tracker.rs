use std::{collections::HashMap, hash::Hash};

use crate::{
    js::write_output::CanWriteOutput,
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

use super::{
    cross_ref::{Codex, CrossRef},
    helpers::get_token_as_word,
    routine_root::{RoutineRoot, RoutineStub},
    top_level::{
        globals::GlobalCodex,
        objects::{ObjectCodex, ObjectCodexValue},
        player::PlayerInfo,
        rooms::{RoomCodex, RoomCodexValue},
        routines::RoutineCodex,
    },
};

pub trait HasReturnType {
    fn return_type(&self) -> ReturnValType;
}

pub trait CanValidate {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String>;
}

pub struct Validator<'a> {
    pub player: &'a PlayerInfo,
    routine_codex: RoutineCodex<'a>,
    object_codex: ObjectCodex<'a>,
    room_codex: RoomCodex<'a>,
    global_codex: GlobalCodex<'a>,
    stack: Vec<StackFrame>,
    pub root: Option<Vec<RoutineRoot>>,
    last_writer: Option<Box<dyn CanWriteOutput>>,
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    vars: HashMap<String, ReturnValType>,
    expect_vals: Vec<ReturnValType>,
    return_type: ReturnValType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReturnValType {
    Unknown,
    Number,     // an integer in js
    Boolean,    // true/false in js
    Text,       // " ... " in js
    ObjectName, // a key in const objects
    VarName,    // a variable name, usually ' ... ' in js
    CmdWord,    // an all-uppercase part of the original command
    Location,   // either PLAYER, a ROOM, or an OBJECT. Is always a full object in js
    None,
    Any,
}

impl<'a> Validator<'a> {
    pub fn new(cross_ref: &'a CrossRef) -> Validator<'a> {
        let mut vars: HashMap<String, ReturnValType> = HashMap::new();
        vars.insert("PLAYER".to_string(), ReturnValType::Location);
        vars.insert("CURRENT-ROOM".to_string(), ReturnValType::Location);
        vars.insert("CMD-ACTION".to_string(), ReturnValType::CmdWord);
        vars.insert("CMD-PRSO".to_string(), ReturnValType::Location);
        vars.insert("CMD-PRSI".to_string(), ReturnValType::Location);

        let base_stack = StackFrame {
            vars,
            expect_vals: vec![ReturnValType::Any],
            return_type: ReturnValType::None,
        };

        Validator {
            player: cross_ref.player.get_info(),
            routine_codex: cross_ref.routines.as_codex(),
            object_codex: cross_ref.objects.as_codex(),
            room_codex: cross_ref.rooms.as_codex(),
            global_codex: cross_ref.globals.as_codex(),
            last_writer: None,
            stack: vec![base_stack],
            root: Some(Vec::new()),
        }
    }

    fn push_stack(&mut self) {
        self.stack.push(StackFrame {
            vars: HashMap::new(),
            expect_vals: Vec::new(),
            return_type: ReturnValType::Unknown,
        });
    }

    pub fn add_local_var(&mut self, var: String, return_type: ReturnValType) {
        if let Some(frame) = self.stack.last_mut() {
            frame.vars.insert(var, return_type);
        }
    }

    pub fn expect_vals(&mut self, vals: Vec<ReturnValType>) {
        if let Some(frame) = self.stack.last_mut() {
            frame.expect_vals = vals;
        }
    }

    #[allow(dead_code)]
    pub fn add_expect_val(&mut self, val: ReturnValType) {
        if let Some(frame) = self.stack.last_mut() {
            frame.expect_vals.push(val);
        }
    }

    pub fn expect_val(&mut self, val: ReturnValType) {
        if let Some(frame) = self.stack.last_mut() {
            frame.expect_vals = vec![val];
        }
    }

    pub fn get_prev_expect_vals(&self) -> Vec<&ReturnValType> {
        if self.stack.len() > 1 {
            return self.stack[self.stack.len() - 2]
                .expect_vals
                .iter()
                .collect();
        }

        vec![&ReturnValType::Any]
    }

    fn set_return_type(&mut self, val: ReturnValType) {
        if let Some(frame) = self.stack.last_mut() {
            frame.return_type = val;
        }
    }

    fn get_return_type(&self) -> ReturnValType {
        if let Some(frame) = self.stack.last() {
            return frame.return_type;
        }

        ReturnValType::Unknown
    }

    fn pop_stack(&mut self) {
        self.stack.pop();
    }

    pub fn take_last_writer(&mut self) -> Option<Box<dyn CanWriteOutput>> {
        self.last_writer.take()
    }

    pub fn is_object(&self, name: &str) -> bool {
        self.object_codex.lookup(name).is_some()
    }

    pub fn get_object(&self, name: &str) -> Option<ObjectCodexValue> {
        self.object_codex.lookup(name)
    }

    pub fn is_room(&self, name: &str) -> bool {
        self.room_codex.lookup(name).is_some()
    }

    pub fn get_room(&self, name: &str) -> Option<RoomCodexValue> {
        self.room_codex.lookup(name)
    }

    pub fn is_global(&self, name: &str) -> bool {
        self.global_codex.lookup(name).is_some()
    }

    pub fn has_local_var(&self, name: &String) -> Option<ReturnValType> {
        for frame in self.stack.iter().rev() {
            if frame.vars.contains_key(name) {
                return Some((*frame.vars.get(name).unwrap()).clone());
            }
        }

        None
    }

    pub fn validate_cluster(&mut self, n: &'a ZilNode) -> Result<(), String> {
        self.push_stack();

        let result = self.validate(n);
        if result.is_err() {
            return result;
        }

        let got_val = &self.get_return_type();
        let prev_want = self.get_prev_expect_vals();

        let mut found_want = false;
        for want in prev_want.iter() {
            if (*want == &ReturnValType::Any) || (*want == got_val) {
                found_want = true;
                break;
            }
        }

        if !found_want {
            return Err(format!(
                "Expected {:?}, found {:?}\n{}",
                prev_want,
                self.get_return_type(),
                format_file_location(&n)
            ));
        }

        self.pop_stack();

        result
    }

    fn validate(&mut self, n: &'a ZilNode) -> Result<(), String> {
        if n.node_type != ZilNodeType::Cluster {
            return Err(format!(
                "Expected cluster, found {}\n{}",
                n.node_type,
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

        if name == "ROUTINE" && n.children.len() > 1 {
            let second_word = get_token_as_word(&n.children[1]).unwrap_or_default();
            let codex_value = self.routine_codex.lookup(&second_word).unwrap();
            let mut routine_root = RoutineRoot::from(&codex_value);

            self.expect_val(ReturnValType::Number);

            match routine_root.validate(self, n) {
                Ok(_) => {
                    self.root.as_mut().unwrap().push(routine_root);
                    return Ok(());
                }
                Err(e) => return Err(e),
            }
        }

        if self.routine_codex.lookup(&name).is_some() {
            if n.children.len() > 1 {
                return Err(format!(
                    "Custom routine called with more than one child\n{}",
                    format_file_location(&n)
                ));
            }

            let stub = RoutineStub::new(&name);
            self.set_return_type(stub.return_type());
            self.last_writer = Some(Box::new(stub));
            return Ok(());
        }

        // DRY?

        match name.as_str() {
            "ADD" => {
                let mut v = super::any_level::add::Add::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "AND" => {
                let mut v = super::any_level::and::And::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "COND" => {
                let mut v = super::any_level::cond::Cond::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "DESC" => {
                let mut v = super::any_level::desc::Description::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "DIVIDE" => {
                let mut v = super::any_level::divide::Divide::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "EACH-OBJ" => {
                let mut v = super::any_level::each_obj::EachObj::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "EACH-VAR" => {
                let mut v = super::any_level::each_var::EachVar::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "GET-VAR" => {
                let mut v = super::any_level::get_var::GetVar::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "IS-ASC" => {
                let mut v = super::any_level::is_asc::IsAsc::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "IS-DES" => {
                let mut v = super::any_level::is_des::IsDes::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "IS-EQUAL" => {
                let mut v = super::any_level::is_equal::IsEqual::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "LOC" => {
                let mut v = super::any_level::loc::Location::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "MOVE" => {
                let mut v = super::any_level::move_::Move::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "MULTIPLY" => {
                let mut v = super::any_level::multiply::Multiply::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "NOT" => {
                let mut v = super::any_level::not::Not::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "OR" => {
                let mut v = super::any_level::or::Or::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "RAND" => {
                let mut v = super::any_level::rand::Rand::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "RETURN" => {
                let mut v = super::any_level::return_::Return::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "SET-VAR" => {
                let mut v = super::any_level::set_var::SetVar::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "SUBTRACT" => {
                let mut v = super::any_level::subtract::Subtract::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "TELL" => {
                let mut v = super::any_level::tell::Tell::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            _ => (),
        }

        return Err(format!(
            "Unknown cluster name {} in validate_cluster\n{}",
            name,
            format_file_location(&n)
        ));
    }
}
