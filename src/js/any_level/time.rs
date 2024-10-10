use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::time::Time,
};

impl CanWriteOutput for Time {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("Math.round(new Date().getTime() / 1000)", false)?;

        Ok(())
    }
}
