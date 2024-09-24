use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::loc::Location,
};

impl CanWriteOutput for Location {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("game.getParent(", false)?;
        self.instance.write_output(formatter)?;
        formatter.write(", ", false)?;
        if self.nested {
            formatter.write("true", false)?;
        } else {
            formatter.write("false", false)?;
        }
        formatter.write(")", false)?;

        Ok(())
    }
}
