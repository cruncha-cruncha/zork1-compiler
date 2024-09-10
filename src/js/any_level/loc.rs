use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{
        loc::Location,
        set_var::{Scope, VarWordType},
    },
};

impl CanWriteOutput for Location {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("loc(", false)?;

        match self.scope {
            Scope::Object(VarWordType::Literal(ref s)) => {
                formatter.write(&format!("objects['{}']", s), false)?
            }
            Scope::Object(VarWordType::Indirect(ref s)) => {
                formatter.write(&format!("objects[{}]", s), false)?
            }
            Scope::Location(ref s) => formatter.write(&format!("{}", s), false)?,
            Scope::LOC(ref s) => s.write_output(formatter)?,
            _ => panic!("IDK"),
        }

        formatter.write(")", false)?;

        Ok(())
    }
}
