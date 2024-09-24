use std::collections::BTreeMap;

use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::{
        routine_root::{RoutineRoot, RoutineStub},
        routine_tracker::Validator,
        top_level::routines::{HandlerInfo, RoutineStats},
    },
};

impl CanWriteOutput for RoutineStats {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("import { game, getEmptyResource } from './engine.js';")?;
        formatter.writeln("import { player, globals } from './globals.js';")?;
        formatter.writeln("import { objects } from './objects.js';")?;
        formatter.writeln("import { rooms } from './rooms.js';")?;
        formatter.newline()?;

        formatter.writeln("export const routines = {")?;
        formatter.indent();

        for info in self.as_codex() {
            let name = Formatter::safe_case(&info.name);
            formatter.writeln(&format!("{}: {},", &name, &name))?;
        }

        formatter.outdent();
        formatter.writeln("};")?;
        formatter.newline()?;

        formatter.flush()?;

        formatter.writeln("export const handlers = {")?;
        formatter.indent();

        for action in self.iter_actions() {
            let name = Formatter::safe_case(&action.name);
            let mut info = ActionJsInfo {
                has_func: false,
                obj_handlers: BTreeMap::new(),
            };

            for handle in action.handlers.iter() {
                if handle.object.is_none() {
                    info.has_func = true;
                } else {
                    let obj_name = Formatter::safe_case(handle.object.as_ref().unwrap());
                    let obj_handle =
                        info.obj_handlers
                            .entry(obj_name.clone())
                            .or_insert(ObjectHandler {
                                before: false,
                                after: false,
                            });

                    if handle.before {
                        obj_handle.before = true;
                    } else {
                        obj_handle.after = true;
                    }
                }
            }

            formatter.writeln(&format!("{}: {{", &name))?;
            formatter.indent();

            if info.has_func {
                formatter.writeln(&format!("func: {},", &name))?;
            }

            formatter.writeln("objHandlers: {")?;
            formatter.indent();

            for (obj_name, obj_handle) in info.obj_handlers.iter() {
                formatter.writeln(&format!("{}: {{", &obj_name))?;
                formatter.indent();

                if obj_handle.before {
                    formatter.writeln(&format!(
                        "before: {},",
                        HandlerInfo::format_key(&name, Some(obj_name), true)
                    ))?;
                }

                if obj_handle.after {
                    formatter.writeln(&format!(
                        "after: {},",
                        HandlerInfo::format_key(&name, Some(obj_name), false)
                    ))?;
                }

                formatter.outdent();
                formatter.writeln("},")?;
            }

            formatter.outdent();
            formatter.writeln("},")?;

            formatter.outdent();
            formatter.writeln("},")?;
        }

        formatter.outdent();
        formatter.writeln("};")?;
        formatter.newline()?;

        formatter.flush()?;

        Ok(())
    }
}

pub struct RoutineToots {
    pub root: Vec<RoutineRoot>,
}

impl RoutineToots {
    pub fn from(v: &mut Validator) -> Self {
        match v.roots.take() {
            Some(n) => Self { root: n },
            None => Self { root: Vec::new() },
        }
    }
}

impl CanWriteOutput for RoutineToots {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        for func in self.root.iter() {
            func.write_output(formatter)?;
        }

        Ok(())
    }
}

impl CanWriteOutput for RoutineRoot {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln(&format!(
            "function {}(cRoom, cmd) {{",
            Formatter::safe_case(&self.name)
        ))?;
        formatter.indent();

        formatter.write("const locals = {cRoom, cmd, ", true)?;
        let locals = self
            .var_names
            .iter()
            .map(|x| format!("{}: 0", Formatter::safe_case(x)))
            .collect::<Vec<String>>()
            .join(",");
        formatter.write(&locals, false)?;
        formatter.write("};", false)?;

        for item in self.body.iter() {
            item.write_output(formatter)?;
            formatter.write(";", false)?;
        }

        formatter.newline()?;
        formatter.writeln("return 0;")?;
        formatter.outdent();
        formatter.writeln("}")?;
        formatter.newline()?;

        Ok(())
    }
}

impl CanWriteOutput for RoutineStub {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;
        formatter.write(
            &format!(
                "routines['{}'](locals['cRoom'], locals['cmd'], locals['prso'], locals['prsi'])",
                Formatter::safe_case(&self.name)
            ),
            true,
        )?;

        Ok(())
    }
}

pub struct ActionJsInfo {
    has_func: bool,
    obj_handlers: BTreeMap<String, ObjectHandler>,
}

pub struct ObjectHandler {
    before: bool,
    after: bool,
}
