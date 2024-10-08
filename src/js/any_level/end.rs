use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::end::End,
};

impl CanWriteOutput for End {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("game.close()", false)?;

        Ok(())
    }
}
