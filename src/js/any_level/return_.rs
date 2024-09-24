use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::return_::Return,
};

impl CanWriteOutput for Return {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("return ", true)?;

        self.value.write_output(formatter)?;

        Ok(())
    }
}
