use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::{any_level::is_equal::IsEqual, routine_tracker::ReturnValType},
};

impl CanWriteOutput for IsEqual {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        let use_primitive = match self.val_type {
            ReturnValType::Text | ReturnValType::Boolean | ReturnValType::Number => true,
            _ => false,
        };

        if use_primitive {
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
        } else {
            formatter.write("game.isEqual(", false)?;

            for (i, val) in self.values.iter().enumerate() {
                val.write_output(formatter)?;

                if i < self.values.len() - 1 {
                    formatter.write(", ", false)?;
                }
            }

            formatter.write(")", false)?;
        }

        Ok(())
    }
}
