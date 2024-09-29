use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::get_var::GetVar,
};

impl CanWriteOutput for GetVar {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("(", false)?;

        self.scope.write_output(formatter)?;

        formatter.write(".vars[", false)?;

        self.var.write_output(formatter)?;

        formatter.write("] || 0)", false)?;

        Ok(())
    }
}
