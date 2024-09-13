use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::move_::Move,
};

impl CanWriteOutput for Move {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("game.move(locals, ", true)?;

        self.item.write_output(formatter)?;

        formatter.write(", ", false)?;

        self.destination.write_output(formatter)?;

        formatter.write(")", false)?;

        Ok(())
    }
}
