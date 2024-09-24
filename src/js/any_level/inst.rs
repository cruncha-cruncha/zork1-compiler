use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::inst::Inst,
};

impl CanWriteOutput for Inst {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("game.getInst(", false)?;
        self.scope.write_output(formatter)?;
        formatter.write(", ", false)?;
        formatter.write(
            &format!("'{}'", Formatter::safe_case(&self.object_name)),
            false,
        )?;
        formatter.write(", ", false)?;
        if self.nested {
            formatter.write("true", false)?;
        } else {
            formatter.write("false", false)?;
        }
        formatter.write(")", false)?;

        Ok(())
    }
}
