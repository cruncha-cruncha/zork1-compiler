use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::subtract::Subtract,
};

impl CanWriteOutput for Subtract {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("(", false)?;

        self.minuend.write_output(formatter)?;
        formatter.write(" - ", false)?;
        self.subtrahend.write_output(formatter)?;

        formatter.write(")", false)?;

        Ok(())
    }
}
