use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{desc::Description, set_var::Scope},
};

impl CanWriteOutput for Description {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        match self.scope {
            Scope::Local(ref name) => {
                formatter.write(
                    &format!("describe(locals['{}']);", Formatter::safe_case(name)),
                    true,
                )?;
            }
            Scope::Object(ref name) => {
                formatter.write(
                    &format!("describe(objects['{}']);", Formatter::safe_case(name)),
                    true,
                )?;
            }
            Scope::Room(ref name) => {
                formatter.write(
                    &format!("describe(rooms['{}']);", Formatter::safe_case(name)),
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
