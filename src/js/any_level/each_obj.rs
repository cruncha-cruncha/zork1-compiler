use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{each_obj::EachObj, set_var::Scope},
};

impl CanWriteOutput for EachObj {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("for (let object of getObjectsIn(", true)?;

        match self.scope {
            Scope::Local(ref name) => {
                formatter.write(&format!("locals['{}']", Formatter::safe_case(name)), false)?
            }
            Scope::Object(ref name) => {
                formatter.write(&format!("objects['{}']", Formatter::safe_case(name)), false)?
            }
            Scope::Room(ref name) => {
                formatter.write(&format!("rooms['{}']", Formatter::safe_case(name)), false)?
            }
            Scope::LOC(ref w) => w.write_output(formatter)?,
            _ => panic!("IDK"),
        }

        formatter.write(")) {", false)?;
        formatter.newline()?;
        formatter.indent();

        formatter.write(
            &format!(
                "locals['{}'] = object;",
                Formatter::safe_case(&self.name_var)
            ),
            true,
        )?;

        for node in self.body.iter() {
            node.write_output(formatter)?;
        }

        formatter.outdent();
        formatter.newline()?;
        formatter.write("}", true)?;

        Ok(())
    }
}
