use crate::{
    js::{
        formatter::Formatter,
        write_output::{CanWriteOutput, ToJs},
    },
    stats::top_level::rooms::{Direction, DirectionType, RoomStats},
};

impl CanWriteOutput for RoomStats {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("export const rooms = {")?;
        formatter.indent();

        for info in self.as_codex() {
            let name = Formatter::safe_case(&info.name);
            formatter.writeln(&format!("{}: {{", &name))?;
            formatter.indent();

            formatter.writeln(&format!("isRoom: '{}',", &name))?;

            if info.desc.is_some() {
                formatter.writeln(&format!("desc: {},", info.desc.as_ref().unwrap().to_js()))?;
            }

            formatter.writeln("objects: [")?;
            formatter.indent();
            for obj in info.objects.iter() {
                formatter.writeln(&format!("'{}',", Formatter::safe_case(obj)))?;
            }
            formatter.outdent();
            formatter.writeln("],")?;

            formatter.writeln("vars: {")?;
            formatter.indent();
            for (var, val) in info.vars.iter() {
                formatter.writeln(&format!("{}: {},", Formatter::safe_case(var), val))?;
            }
            formatter.outdent();
            formatter.writeln("},")?;

            formatter.writeln("move: {")?;
            formatter.indent();
            for (dir, dest) in info.directions.iter() {
                formatter.writeln(&format!("{}: {},", dir, dest.to_js()))?;
            }
            formatter.outdent();
            formatter.writeln("},")?;

            formatter.writeln("hooks: {")?;
            formatter.indent();
            if info.actions.first_enter.is_some() {
                formatter.writeln(&format!(
                    "firstEnter: '{}',",
                    Formatter::safe_case(info.actions.first_enter.as_ref().unwrap())
                ))?;
            }
            if info.actions.enter.is_some() {
                formatter.writeln(&format!(
                    "enter: '{}',",
                    Formatter::safe_case(info.actions.enter.as_ref().unwrap())
                ))?;
            }
            if info.actions.exit.is_some() {
                formatter.writeln(&format!(
                    "exit: '{}',",
                    Formatter::safe_case(info.actions.exit.as_ref().unwrap())
                ))?;
            }
            if info.actions.always.is_some() {
                formatter.writeln(&format!(
                    "always: '{}',",
                    Formatter::safe_case(info.actions.always.as_ref().unwrap())
                ))?;
            }
            formatter.outdent();
            formatter.writeln("},")?;

            formatter.outdent();
            formatter.writeln("},")?;
        }

        formatter.outdent();
        formatter.writeln("};")?;
        formatter.flush()?;

        Ok(())
    }
}

impl ToJs for Direction {
    fn to_js(&self) -> String {
        match self.kind {
            DirectionType::TEXT => {
                format!("{{ text: \"{}\" }}", self.thing)
            }
            DirectionType::ROUTINE => {
                format!("{{ routine: '{}' }}", Formatter::safe_case(&self.thing))
            }
            DirectionType::ROOM => {
                format!("{{ room: '{}' }}", Formatter::safe_case(&self.thing))
            }
        }
    }
}
