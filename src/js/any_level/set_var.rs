use crate::{
    js::{
        formatter::Formatter,
        write_output::{CanWriteOutput, OutputNode},
    },
    stats::any_level::set_var::{Scope, SetVar},
};

impl CanWriteOutput for SetVar {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        if self.scope.is_some() {
            match self.scope.as_ref().unwrap() {
                Scope::Local(ref name) => {
                    formatter.write(
                        &format!("locals['{}'].vars[", Formatter::safe_case(name)),
                        true,
                    )?;
                }
                Scope::Room(ref name) => {
                    formatter.write(
                        &format!("rooms['{}'].vars[", Formatter::safe_case(name)),
                        true,
                    )?;
                }
                Scope::Object(ref name) => {
                    formatter.write(
                        &format!("objects['{}'].vars[", Formatter::safe_case(name)),
                        true,
                    )?;
                }
                Scope::LOC(ref w) => {
                    formatter.write("setVar(", true)?;
                    w.write_output(formatter)?;
                    formatter.write(", ", false)?;
                }
                _ => panic!("IDK"),
            }
        }

        match self.var {
            Scope::Bare(ref name) => {
                formatter.write(
                    &format!("'{}'", Formatter::safe_case(name)),
                    self.scope.is_none(),
                )?;
            }
            Scope::Local(ref name) => {
                formatter.write(
                    &format!("locals['{}']", Formatter::safe_case(name)),
                    self.scope.is_none(),
                )?;
            }
            Scope::Global(ref name) => {
                formatter.write(
                    &format!("globals['{}']", Formatter::safe_case(name)),
                    self.scope.is_none(),
                )?;
            }
            Scope::LOC(ref w) => {
                w.write_output(formatter)?;
            }
            _ => panic!("IDK"),
        }

        if self.scope.is_some() {
            match self.scope.as_ref().unwrap() {
                Scope::LOC(_) => {
                    formatter.write(", ", false)?;
                }
                _ => {
                    formatter.write("] = ", false)?;
                }
            }
        } else {
            formatter.write(" = ", false)?;
        }

        match self.value {
            OutputNode::Number(n) => {
                formatter.write(&format!("{}", n), false)?;
            }
            OutputNode::Variable(Scope::Local(ref name)) => {
                formatter.write(&format!("locals['{}']", Formatter::safe_case(name)), false)?;
            }
            OutputNode::Variable(Scope::Global(ref name)) => {
                formatter.write(&format!("globals[{}]", Formatter::safe_case(name)), false)?;
            }
            OutputNode::Writer(ref w) => {
                w.write_output(formatter)?;
            }
            _ => panic!("IDK"),
        }

        if self.scope.is_some() {
            match self.scope.as_ref().unwrap() {
                Scope::LOC(_) => {
                    formatter.write(");", false)?;
                }
                _ => (),
            }
        }
        formatter.write(";", false)?;

        Ok(())
    }
}
