use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{
        desc::Description,
        set_var::{Scope, VarWordType},
    },
};

impl CanWriteOutput for Description {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        match self.scope {
            Scope::Object(VarWordType::Literal(ref object)) => {
                formatter.write(
                    &format!("describe(objects['{}']);", Formatter::safe_case(object)),
                    true,
                )?;
            }
            Scope::Object(VarWordType::Indirect(ref object)) => {
                formatter.write(
                    &format!("describe(objects[{}]);", Formatter::safe_case(object)),
                    true,
                )?;
            }
            Scope::Room(ref room) => {
                formatter.write(
                    &format!("describe(rooms['{}']);", Formatter::safe_case(room)),
                    true,
                )?;
            }
            Scope::Location(ref location) => {
                formatter.write(
                    &format!("describe(locals['{}']);", Formatter::safe_case(location)),
                    true,
                )?;
            }
            Scope::LOC(ref w) => {
                formatter.write("describe(", true)?;
                w.write_output(formatter)?;
                formatter.write(");", false)?;
            }
            _ => panic!("IDK"),
        }

        Ok(())
    }
}
