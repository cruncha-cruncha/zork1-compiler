use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::copy_move::CopyMove,
};

impl CanWriteOutput for CopyMove {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("game.copyMove(", true)?;

        self.item.write_output(formatter)?;

        formatter.write(", ", false)?;

        self.destination.write_output(formatter)?;

        formatter.write(")", false)?;

        Ok(())
    }
}
