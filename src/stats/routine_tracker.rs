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
    helpers::parse_token_as_word,
    routine_root::{RoutineRoot, RoutineStub},
    top_level::{
        globals::GlobalCodex, objects::ObjectCodex, player::PlayerInfo, rooms::RoomCodex,
        routines::RoutineCodex,
    },
};

pub trait HasReturnType {
    fn return_type(&self) -> Vec<ReturnValType>;
}

pub trait CanValidate {
    fn validate<'a>(&mut self, v: &mut Validator<'a>, n: &'a ZilNode) -> Result<(), String>;
}

pub struct Validator<'a> {
    #[allow(dead_code)]
    pub player: &'a PlayerInfo,
    routine_codex: RoutineCodex<'a>,
    object_codex: ObjectCodex<'a>,
    room_codex: RoomCodex<'a>,
    global_codex: GlobalCodex<'a>,
    stack: Vec<StackFrame>,
    pub roots: Option<Vec<RoutineRoot>>,
    last_writer: Option<Box<dyn CanWriteOutput>>,
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    vars: HashMap<String, ReturnValType>,
    expect_vals: Vec<ReturnValType>,
    return_type: Vec<ReturnValType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReturnValType {
    Unknown,
    None,
    Any,
    Boolean,
    Number,
    Text,
    Inst, // an object instance
    RP,   // a Room or Player
}

impl<'a> Validator<'a> {
    pub fn new(cross_ref: &'a CrossRef) -> Validator<'a> {
        let mut vars: HashMap<String, ReturnValType> = HashMap::new();
        vars.insert("C-ROOM".to_string(), ReturnValType::RP);
        vars.insert("CMD".to_string(), ReturnValType::Text);

        let base_stack = StackFrame {
            vars,
            expect_vals: vec![ReturnValType::Any],
            return_type: Vec::new(),
        };

        Validator {
            player: cross_ref.player.get_info(),
            routine_codex: cross_ref.routines.as_codex(),
            object_codex: cross_ref.objects.as_codex(),
            room_codex: cross_ref.rooms.as_codex(),
            global_codex: cross_ref.globals.as_codex(),
            last_writer: None,
            stack: vec![base_stack],
            roots: Some(Vec::new()),
        }
    }

    pub fn push_stack(&mut self) {
        self.stack.push(StackFrame {
            vars: HashMap::new(),
            expect_vals: Vec::new(),
            return_type: Vec::new(),
        });
    }

    pub fn add_local_var(&mut self, name: String, val_type: ReturnValType) {
        if let Some(frame) = self.stack.last_mut() {
            frame.vars.insert(name, val_type);
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

    fn set_return_type(&mut self, vals: Vec<ReturnValType>) {
        if let Some(frame) = self.stack.last_mut() {
            frame.return_type = vals;
        }
    }

    fn get_return_type(&self) -> Vec<ReturnValType> {
        if let Some(frame) = self.stack.last() {
            return frame.return_type.clone();
        }

        Vec::new()
    }

    pub fn pop_stack(&mut self) {
        self.stack.pop();
    }

    pub fn take_last_writer(&mut self) -> Option<Box<dyn CanWriteOutput>> {
        self.last_writer.take()
    }

    pub fn is_object(&self, name: &str) -> bool {
        self.object_codex.lookup(name).is_some()
    }

    pub fn is_room(&self, name: &str) -> bool {
        self.room_codex.lookup(name).is_some()
    }

    pub fn is_global(&self, name: &str) -> bool {
        self.global_codex.lookup(name).is_some()
    }

    pub fn has_local_var(&self, name: &String) -> Option<&ReturnValType> {
        for frame in self.stack.iter().rev() {
            if frame.vars.get(name).is_some() {
                return frame.vars.get(name);
            }
        }

        None
    }

    pub fn validate_cluster(&mut self, n: &'a ZilNode) -> Result<ReturnValType, String> {
        self.push_stack();

        let result = self.validate(n);
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let got_val = &self.get_return_type();
        let prev_want = self.get_prev_expect_vals();

        let mut found_want: Option<ReturnValType> = None;
        for want in prev_want.iter() {
            if *want == &ReturnValType::Any {
                if got_val.len() > 0 && got_val.len() < 2 {
                    found_want = Some(got_val[0]);
                    break;
                } else {
                    found_want = Some(ReturnValType::Unknown);
                    break;
                }
            } else if got_val.contains(want) {
                found_want = Some(**want);
                break;
            }
        }

        if found_want.is_none() {
            return Err(format!(
                "Expected {:?}, found {:?}\n{}",
                prev_want,
                self.get_return_type(),
                format_file_location(&n)
            ));
        }

        self.pop_stack();

        Ok(found_want.unwrap())
    }

    fn validate(&mut self, n: &'a ZilNode) -> Result<(), String> {
        if n.node_type != ZilNodeType::Cluster {
            return Err(format!(
                "Expected cluster, found {}\n{}",
                n.node_type,
                format_file_location(&n)
            ));
        }

        let name = match parse_token_as_word(&n.children[0]) {
            Some(name) => name,
            None => {
                return Err(format!(
                    "Cluster node has no name\n{}",
                    format_file_location(&n)
                ));
            }
        };

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
            "CMD" => {
                let mut v = super::any_level::cmd::Cmd::new();
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
            "COPY-MOVE" => {
                let mut v = super::any_level::copy_move::CopyMove::new();
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
            "END" => {
                let mut v = super::any_level::end::End::new();
                match v.validate(self, n) {
                    Ok(_) => {
                        self.set_return_type(v.return_type());
                        self.last_writer = Some(Box::new(v));
                        return Ok(());
                    }
                    Err(e) => return Err(e),
                }
            }
            "EACH-VAL" => {
                let mut v = super::any_level::each_val::EachVal::new();
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
            "INST" => {
                let mut v = super::any_level::inst::Inst::new();
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
            "IS-IN" => {
                let mut v = super::any_level::is_in::IsIn::new();
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
