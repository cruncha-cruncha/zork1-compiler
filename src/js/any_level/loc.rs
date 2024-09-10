use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{loc::Location, set_var::Scope},
};

impl CanWriteOutput for Location {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("loc(", false)?;

        match self.scope {
            Scope::Local(ref name) => {
                formatter.write(&format!("locals[{}]", Formatter::safe_case(name)), false)?
            }
            Scope::Object(ref name) => {
                formatter.write(&format!("objects[{}]", Formatter::safe_case(name)), false)?
            }
            Scope::LOC(ref w) => w.write_output(formatter)?,
            _ => panic!("IDK"),
        }

        formatter.write(")", false)?;

        Ok(())
    }
}
