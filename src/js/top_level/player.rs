use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::top_level::player::PlayerStats,
};

impl CanWriteOutput for PlayerStats {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("export const player = {")?;
        formatter.indent();

        formatter.writeln("isPlayer: true,")?;

        let info = self.get_info();

        match &info.room {
            Some(room) => {
                formatter.writeln(&format!("startRoom: '{}',", Formatter::safe_case(room)))?;
            }
            None => (),
        }

        formatter.writeln("vars: {")?;
        formatter.indent();
        for (var, val) in info.vars.iter() {
            formatter.writeln(&format!("{}: {},", Formatter::safe_case(&var), val))?;
        }
        formatter.outdent();
        formatter.writeln("},")?;

        formatter.writeln("objects: [")?;
        formatter.indent();
        for obj in info.objects.iter() {
            formatter.writeln(&format!("'{}',", Formatter::safe_case(obj)))?;
        }
        formatter.outdent();
        formatter.writeln("],")?;

        formatter.writeln("hooks: {")?;
        formatter.indent();
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
        formatter.writeln("};")?;
        formatter.newline()?;

        formatter.flush()?;

        Ok(())
    }
}
