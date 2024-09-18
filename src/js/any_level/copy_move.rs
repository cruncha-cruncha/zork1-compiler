use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::copy_move::CopyMove,
};

impl CanWriteOutput for CopyMove {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("game.copyMove(locals, ", true)?;

        if self.item_name.is_some() {
            formatter.write("game.getInst(", false)?;
        }

        self.item_scope.write_output(formatter)?;

        if self.item_name.is_some() {
            formatter.write(
                &format!(
                    ", '{}')",
                    Formatter::safe_case(self.item_name.as_ref().unwrap())
                ),
                false,
            )?;
        }

        formatter.write(", ", false)?;

        if self.destination_name.is_some() {
            formatter.write("game.getInst(", false)?;
        }

        self.destination_scope.write_output(formatter)?;

        if self.destination_name.is_some() {
            formatter.write(
                &format!(
                    ", '{}')",
                    Formatter::safe_case(self.destination_name.as_ref().unwrap())
                ),
                false,
            )?;
        }

        formatter.write(")", false)?;

        Ok(())
    }
}
