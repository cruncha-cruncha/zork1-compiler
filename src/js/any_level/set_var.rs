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

        formatter.write("", true)?;

        if self.scope.is_some() {
            match self.scope.as_ref().unwrap() {
                Scope::Local(ref local_var) => {
                    formatter.write(
                        &format!("locals['{}'].vars[", Formatter::safe_case(local_var)),
                        false,
                    )?;
                }
                Scope::Room(ref name) => {
                    formatter.write(
                        &format!("rooms['{}'].vars[", Formatter::safe_case(name)),
                        false,
                    )?;
                }
                Scope::Object(ref name) => {
                    formatter.write(
                        &format!("objects['{}'].vars[", Formatter::safe_case(name)),
                        false,
                    )?;
                }
                Scope::Writer(ref w) => {
                    w.write_output(formatter)?;
                    formatter.write(".vars[", false)?;
                }
                _ => panic!("IDK"),
            }
        }

        self.var.write_output(formatter)?;

        if self.scope.is_some() {
            formatter.write("] = ", false)?;
        } else {
            formatter.write(" = ", false)?;
        }

        match self.value {
            OutputNode::Number(n) => {
                formatter.write(&format!("{}", n), false)?;
            }
            OutputNode::Variable(Scope::Local(ref local_var)) => {
                formatter.write(
                    &format!("locals['{}']", Formatter::safe_case(local_var)),
                    false,
                )?;
            }
            OutputNode::Variable(Scope::Global(ref name)) => {
                formatter.write(&format!("globals[{}]", Formatter::safe_case(name)), false)?;
            }
            OutputNode::Writer(ref w) => {
                w.write_output(formatter)?;
            }
            _ => panic!("IDK"),
        }

        formatter.write(";", false)?;

        Ok(())
    }
}
