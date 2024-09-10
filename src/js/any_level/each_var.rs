use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{each_var::EachVar, set_var::Scope},
};

impl CanWriteOutput for EachVar {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("for (let varItem of getVariablesOf(", true)?;

        match self.scope {
            Scope::Local(ref name) => {
                formatter.write(&format!("locals['{}']", Formatter::safe_case(name)), false)?
            }
            Scope::Object(ref name) => {
                formatter.write(&format!("objects[{}]", Formatter::safe_case(name)), false)?
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

        formatter.writeln(&format!(
            "locals['{}'] = varItem.name;",
            Formatter::safe_case(&self.name_var)
        ))?;
        formatter.write(
            &format!(
                "locals['{}'] = varItem.val;",
                Formatter::safe_case(&self.value_var)
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
