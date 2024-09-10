use crate::{
    js::{
        formatter::Formatter,
        write_output::{CanWriteOutput, OutputNode, OutputVariable},
    },
    stats::any_level::set_var::{Scope, SetVar, VarWordType},
};

impl CanWriteOutput for SetVar {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        match self.var.scope {
            Scope::Local => {
                formatter.write("locals[", true)?;
            }
            Scope::Global => {
                formatter.write("globals[", true)?;
            }
            Scope::Room(ref room) => {
                formatter.write(
                    &format!("rooms['{}'].vars[", Formatter::safe_case(room)),
                    true,
                )?;
            }
            Scope::Object(VarWordType::Literal(ref object)) => {
                formatter.write(
                    &format!("objects['{}'].vars[", Formatter::safe_case(object)),
                    true,
                )?;
            }
            Scope::Object(VarWordType::Indirect(ref object)) => {
                formatter.write(
                    &format!("objects[{}].vars[", Formatter::safe_case(object)),
                    true,
                )?;
            }
            Scope::Location(ref location) => {
                formatter.write(&format!("setVar({}, ", location), true)?;
            }
            Scope::LOC(ref w) => {
                formatter.write("setVar(", true)?;
                w.write_output(formatter)?;
                formatter.write(", ", false)?;
            }
            _ => panic!("IDK"),
        }

        match self.var {
            OutputVariable {
                scope: Scope::Location(_) | Scope::LOC(_),
                name: VarWordType::Literal(ref word),
            } => {
                formatter.write(&format!("'{}', ", word), false)?;
            }
            OutputVariable {
                scope: Scope::Location(_) | Scope::LOC(_),
                name: VarWordType::Indirect(ref word),
            } => {
                formatter.write(&format!("{}, ", word), false)?;
            }
            OutputVariable {
                scope: _,
                name: VarWordType::Literal(ref word),
            } => {
                formatter.write(&format!("'{}'] = ", Formatter::safe_case(word)), false)?;
            }
            OutputVariable {
                scope: _,
                name: VarWordType::Indirect(ref word),
            } => {
                formatter.write(&format!("{}] = ", Formatter::safe_case(word)), false)?;
            }
            _ => panic!("IDK"),
        }

        match self.value {
            OutputNode::Number(n) => {
                formatter.write(&format!("{}", n), false)?;
            }
            OutputNode::Variable(OutputVariable {
                scope: Scope::Local,
                name: VarWordType::Literal(ref name),
            }) => {
                formatter.write(&format!("locals['{}']", Formatter::safe_case(name)), false)?;
            }
            OutputNode::Variable(OutputVariable {
                scope: Scope::Local,
                name: VarWordType::Indirect(ref name),
            }) => {
                formatter.write(&format!("locals[{}]", Formatter::safe_case(name)), false)?;
            }
            OutputNode::Variable(OutputVariable {
                scope: Scope::Global,
                name: VarWordType::Literal(ref name),
            }) => {
                formatter.write(&format!("globals['{}']", Formatter::safe_case(name)), false)?;
            }
            OutputNode::Writer(ref w) => {
                w.write_output(formatter)?;
            }
            _ => panic!("IDK"),
        }

        match self.var.scope {
            Scope::Local | Scope::Global | Scope::Room(_) | Scope::Object(_) => {
                formatter.write(";", false)?;
            }
            Scope::Location(_) | Scope::LOC(_) => {
                formatter.write(");", false)?;
            }
            _ => panic!("IDK"),
        }

        Ok(())
    }
}
