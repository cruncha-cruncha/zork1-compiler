use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{
        each_var::EachVar,
        set_var::{Scope, VarWordType},
    },
};

impl CanWriteOutput for EachVar {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("for (let varItem of getVariablesOf(", true)?;

        match self.scope {
            Scope::Object(VarWordType::Literal(ref s)) => {
                formatter.write(&format!("objects['{}']", Formatter::safe_case(s)), false)?
            }
            Scope::Object(VarWordType::Indirect(ref s)) => {
                formatter.write(&format!("objects[{}]", Formatter::safe_case(s)), false)?
            }
            Scope::Room(ref s) => {
                formatter.write(&format!("rooms['{}']", Formatter::safe_case(s)), false)?
            }
            Scope::Location(ref s) => {
                formatter.write(&format!("locals['{}']", Formatter::safe_case(s)), false)?
            }
            Scope::LOC(ref s) => s.write_output(formatter)?,
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
