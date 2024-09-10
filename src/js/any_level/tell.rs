use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::tell::Tell,
};

impl CanWriteOutput for Tell {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("log(", true)?;

        for (i, node) in self.text.iter().enumerate() {
            node.write_output(formatter)?;

            if i < self.text.len() - 1 {
                formatter.write(", ", false)?;
            }
        }

        if self.cr {
            if self.text.len() > 0 {
                formatter.write(", ", false)?;
            }

            formatter.write("'\\n'", false)?;
        }

        formatter.write(");", false)?;

        Ok(())
    }
}
