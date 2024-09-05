use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::top_level::objects::ObjectStats,
};

impl CanWriteOutput for ObjectStats {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("export const objects = {")?;
        formatter.indent();

        for info in self.as_codex() {
            formatter.writeln(&format!("{}: {{}},", Formatter::safe_case(&info.name)))?;
        }

        formatter.outdent();
        formatter.writeln("};")?;
        formatter.writeln("")?;

        formatter.writeln("export const lookupBySynonym = (word) => {")?;
        formatter.indent();

        formatter.writeln("switch (word) {")?;

        for info in self.as_codex() {
            for syn in info.synonyms.iter() {
                formatter.writeln(&format!("case '{}':", syn))?;
            }

            formatter.indent();
            formatter.writeln(&format!(
                "return objects['{}'];",
                Formatter::safe_case(&info.name)
            ))?;
            formatter.outdent();
        }

        formatter.writeln("default:")?;
        formatter.indent();
        formatter.writeln("return null;")?;
        formatter.outdent();

        formatter.writeln("}")?;

        formatter.outdent();
        formatter.writeln("}")?;

        Ok(())
    }
}
