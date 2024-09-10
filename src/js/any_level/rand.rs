use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::rand::Rand,
};

impl CanWriteOutput for Rand {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("Math.floor(Math.random() * 101)", false)?;

        Ok(())
    }
}
