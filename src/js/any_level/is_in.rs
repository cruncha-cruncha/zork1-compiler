use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::is_in::IsIn,
};

impl CanWriteOutput for IsIn {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("(game.isInLocation(locals['cRoom'], ", false)?;

        self.container.write_output(formatter)?;

        formatter.write(", ", false)?;

        self.item.write_output(formatter)?;

        if self.nested {
            formatter.write(", true))", false)?;
        } else {
            formatter.write(", false))", false)?;
        }

        Ok(())
    }
}
