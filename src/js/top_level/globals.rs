use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::top_level::globals::GlobalStats,
};

impl CanWriteOutput for GlobalStats {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("export const globals = {")?;
        formatter.indent();

        for info in self.as_codex() {
            formatter.writeln(&format!(
                "{}: {},",
                Formatter::safe_case(info.name),
                info.val
            ))?;
        }

        formatter.outdent();
        formatter.writeln("};")?;

        formatter.flush()?;

        Ok(())
    }
}
