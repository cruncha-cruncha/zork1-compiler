use crate::{
    js::{
        formatter::Formatter,
        write_output::{CanWriteOutput, OutputVariable},
    },
    stats::any_level::{
        get_var::GetVar,
        set_var::{Scope, VarWordType},
    },
};

impl CanWriteOutput for GetVar {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        match self.var.scope {
            Scope::Local => {
                formatter.write("(locals[", false)?;
            }
            Scope::Global => {
                formatter.write("(globals[", false)?;
            }
            Scope::Room(ref room) => {
                formatter.write(
                    &format!("(rooms['{}'].vars[", Formatter::safe_case(room)),
                    false,
                )?;
            }
            Scope::Object(VarWordType::Literal(ref object)) => {
                formatter.write(
                    &format!("(objects['{}'].vars[", Formatter::safe_case(object)),
                    false,
                )?;
            }
            Scope::Object(VarWordType::Indirect(ref object)) => {
                formatter.write(
                    &format!("(objects[{}].vars[", Formatter::safe_case(object)),
                    false,
                )?;
            }
            Scope::Location(ref location) => {
                formatter.write(
                    &format!("getVar({}, ", Formatter::safe_case(location)),
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

        match self.var {
            OutputVariable {
                scope: Scope::Location(_) | Scope::LOC(_),
                name: VarWordType::Literal(ref word),
            } => {
                formatter.write(&format!("'{}')", word), false)?;
            }
            OutputVariable {
                scope: Scope::Location(_) | Scope::LOC(_),
                name: VarWordType::Indirect(ref word),
            } => {
                formatter.write(&format!("{})", word), false)?;
            }
            OutputVariable {
                scope: _,
                name: VarWordType::Literal(ref word),
            } => {
                formatter.write(&format!("'{}'] || 0)", Formatter::safe_case(word)), false)?;
            }
            OutputVariable {
                scope: _,
                name: VarWordType::Indirect(ref word),
            } => {
                formatter.write(&format!("{}] || 0)", Formatter::safe_case(word)), false)?;
            }
            _ => panic!("IDK"),
        }

        Ok(())
    }
}
