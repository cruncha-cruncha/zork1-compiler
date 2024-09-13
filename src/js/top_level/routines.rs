use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::{
        routine_root::RoutineRoot, routine_tracker::Validator, top_level::routines::RoutineStats,
    },
};

impl CanWriteOutput for RoutineStats {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("import { game } from './game.js';")?;
        formatter.writeln("import { player, globals } from './globals.js';")?;
        formatter.writeln("import { objects } from './objects.js';")?;
        formatter.writeln("import { rooms } from './rooms.js';")?;
        formatter.newline()?;

        formatter.writeln("export const routines = {")?;
        formatter.indent();

        for info in self.as_codex() {
            let name = Formatter::safe_case(&info.name);
            formatter.writeln(&format!("{}: {{", &name))?;
            formatter.indent();

            formatter.writeln(&format!("isRoutine: '{}',", &name))?;
            formatter.writeln(&format!("func: {},", &name))?;

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
        match v.root.take() {
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
            "function {}(currentRoom, cmdPrsa, cmdPrso, cmdPrsi) {{",
            Formatter::safe_case(&self.name)
        ))?;
        formatter.indent();

        formatter.write(
            "const locals = {currentRoom, cmdPrsa, cmdPrso, cmdPrsi, ",
            true,
        )?;
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
        }

        formatter.newline()?;
        formatter.writeln("return 0;")?;
        formatter.outdent();
        formatter.writeln("}")?;
        formatter.newline()?;

        Ok(())
    }
}
