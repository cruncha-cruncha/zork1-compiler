use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::each_obj::EachObj,
};

impl CanWriteOutput for EachObj {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("for (let object of game.getObjectsIn(", true)?;

        self.scope.write_output(formatter)?;

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
            formatter.write(";", false)?;
        }

        formatter.outdent();
        formatter.newline()?;
        formatter.write("}", true)?;

        Ok(())
    }
}
