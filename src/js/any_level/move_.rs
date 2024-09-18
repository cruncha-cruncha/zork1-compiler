use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::move_::Move,
};

impl CanWriteOutput for Move {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("game.move(locals, ", true)?;

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
