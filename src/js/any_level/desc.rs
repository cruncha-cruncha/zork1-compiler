use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::desc::Description,
};

impl CanWriteOutput for Description {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("game.describe(", true)?;

        self.scope.write_output(formatter)?;

        formatter.write(");", false)?;

        Ok(())
    }
}
