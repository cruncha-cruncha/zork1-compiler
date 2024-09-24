use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::not::Not,
};

impl CanWriteOutput for Not {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("(!", false)?;

        self.value.write_output(formatter)?;

        formatter.write(")", false)?;

        Ok(())
    }
}
