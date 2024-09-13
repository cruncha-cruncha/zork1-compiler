use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::multiply::Multiply,
};

impl CanWriteOutput for Multiply {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("(", false)?;

        for (i, val) in self.values.iter().enumerate() {
            val.write_output(formatter)?;

            if i < self.values.len() - 1 {
                formatter.write(" * ", false)?;
            }
        }

        formatter.write(")", false)?;

        Ok(())
    }
}
