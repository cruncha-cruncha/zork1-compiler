use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::each_var::EachVar,
};

impl CanWriteOutput for EachVar {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("for (let varItem of game.getVariablesOf(", true)?;

        self.scope.write_output(formatter)?;

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
