use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{get_var::GetVar, set_var::Scope},
};

impl CanWriteOutput for GetVar {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        if self.scope.is_some() {
            match self.scope.as_ref().unwrap() {
                Scope::Local(ref name) => {
                    formatter.write(
                        &format!("(locals['{}'].vars[", Formatter::safe_case(name)),
                        false,
                    )?;
                }
                Scope::Object(ref name) => {
                    formatter.write(
                        &format!("(objects['{}'].vars[", Formatter::safe_case(name)),
                        false,
                    )?;
                }
                Scope::Room(ref name) => {
                    formatter.write(
                        &format!("(rooms['{}'].vars[", Formatter::safe_case(name)),
                        false,
                    )?;
                }
                Scope::LOC(ref w) => {
                    formatter.write("getVar(", false)?;
                    w.write_output(formatter)?;
                    formatter.write(", ", false)?;
                }
                _ => panic!("IDK"),
            }
        }

        match self.var {
            Scope::Bare(ref name) => {
                formatter.write(&format!("'{}'", Formatter::safe_case(name)), false)?;
            }
            Scope::Local(ref name) => {
                formatter.write(&format!("locals['{}']", Formatter::safe_case(name)), false)?;
            }
            Scope::Global(ref name) => {
                formatter.write(&format!("globals['{}']", Formatter::safe_case(name)), false)?;
            }
            Scope::LOC(ref w) => {
                w.write_output(formatter)?;
            }
            _ => panic!("IDK"),
        }

        if self.scope.is_some() {
            match self.scope.as_ref().unwrap() {
                Scope::LOC(_) => {
                    formatter.write(")", false)?;
                }
                _ => {
                    formatter.write("]", false)?;
                }
            }
        }

        Ok(())
    }
}
