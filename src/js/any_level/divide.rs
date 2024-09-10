use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::divide::Divide,
};

impl CanWriteOutput for Divide {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("Math.floor(", false)?;

        self.dividend.write_output(formatter)?;
        formatter.write(" / ", false)?;
        self.divisor.write_output(formatter)?;

        formatter.write(")", false)?;

        Ok(())
    }
}
