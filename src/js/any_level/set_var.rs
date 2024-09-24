use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::set_var::{Scope, SetVar},
};

impl CanWriteOutput for SetVar {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("", true)?;

        if self.object.is_some() {
            formatter.write("game.getInst(", false)?;
        }

        if self.scope.is_some() {
            self.scope.as_ref().unwrap().write_output(formatter)?;
        }

        if self.object.is_some() {
            formatter.write(
                &format!(
                    ", '{}')",
                    Formatter::safe_case(self.object.as_ref().unwrap())
                ),
                false,
            )?;
        }

        if self.scope.is_some() {
            formatter.write(".vars[", false)?;
        }

        match self.var {
            Scope::Object(ref hacky_var) => {
                formatter.write(
                    &format!("locals[locals['{}']]", Formatter::safe_case(hacky_var)),
                    false,
                )?;
            }
            _ => self.var.write_output(formatter)?,
        }

        if self.scope.is_some() {
            formatter.write("]", false)?;
        }

        formatter.write(" = ", false)?;

        self.value.write_output(formatter)?;

        Ok(())
    }
}
