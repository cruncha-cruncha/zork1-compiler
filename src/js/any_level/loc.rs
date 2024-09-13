use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::loc::Location,
};

impl CanWriteOutput for Location {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("game.getLocation(", false)?;

        self.scope.write_output(formatter)?;

        formatter.write(")", false)?;

        Ok(())
    }
}
