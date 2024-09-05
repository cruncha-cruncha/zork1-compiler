use crate::{
    js::{
        formatter::Formatter,
        write_output::{CanWriteOutput, ToJs},
    },
    stats::top_level::rooms::RoomStats,
};

impl CanWriteOutput for RoomStats {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("export const rooms = {")?;
        formatter.indent();

        for info in self.as_codex() {
            let name = Formatter::safe_case(&info.name);
            formatter.writeln(&format!("{}: {{", &name))?;
            formatter.indent();

            formatter.writeln(&format!("isRoom: \"{}\",", &name))?;

            if info.desc.is_some() {
                formatter.writeln(&format!("desc: \"{}\",", info.desc.as_ref().unwrap()))?;
            }

            // pub vars: &'a HashMap<String, VarVal>,

            if info.action.is_some() {
                formatter.writeln(&format!(
                    "action: \"{}\",",
                    Formatter::safe_case_option(&info.action)
                ))?;
            }

            formatter.writeln("move: {")?;
            formatter.indent();
            for (dir, dest) in info.directions.iter() {
                formatter.writeln(&format!("{}: {},", dir, dest.to_js()))?;
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

impl ToJs for crate::stats::top_level::rooms::Direction {
    fn to_js(&self) -> String {
        match self.kind {
            crate::stats::top_level::rooms::DirectionType::TEXT => {
                format!("{{ text: \"{}\" }}", self.thing)
            }
            crate::stats::top_level::rooms::DirectionType::ROUTINE => {
                format!("{{ routine: \"{}\" }}", Formatter::safe_case(&self.thing))
            }
            crate::stats::top_level::rooms::DirectionType::ROOM => {
                format!("{{ room: \"{}\" }}", Formatter::safe_case(&self.thing))
            }
        }
    }
}
