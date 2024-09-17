use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{get_var::GetVar, set_var::Scope},
};

impl CanWriteOutput for GetVar {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("(", false)?;
        match self.scope {
            Scope::Local(ref local_var) => {
                formatter.write(
                    &format!("locals['{}'].vars[", Formatter::safe_case(local_var)),
                    false,
                )?;
            }
            Scope::Object(ref name) => {
                formatter.write(
                    &format!("objects['{}'].vars[", Formatter::safe_case(name)),
                    false,
                )?;
            }
            Scope::Room(ref name) => {
                formatter.write(
                    &format!("rooms['{}'].vars[", Formatter::safe_case(name)),
                    false,
                )?;
            }
            Scope::Writer(ref w) => {
                w.write_output(formatter)?;
                formatter.write(".vars[", false)?;
            }
            _ => panic!("IDK"),
        }

        self.var.write_output(formatter)?;

        formatter.write("] || 0)", false)?;

        Ok(())
    }
}
