use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::is_equal::IsEqual,
};

impl CanWriteOutput for IsEqual {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("(", false)?;

        for i in 0..self.values.len() - 1 {
            let val = &self.values[i];
            let next_val = &self.values[i + 1];

            val.write_output(formatter)?;
            formatter.write(" === ", false)?;
            next_val.write_output(formatter)?;

            if i < self.values.len() - 2 {
                formatter.write(" && ", false)?;
            }
        }

        formatter.write(")", false)?;

        Ok(())
    }
}
