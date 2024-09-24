use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::each_val::EachVal,
};

impl CanWriteOutput for EachVal {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        if self.iterate {
            formatter.write("for (let i of Array.from(Array(", true)?;
            self.scope.write_output(formatter)?;
            formatter.write(").keys())) {", false)?;
            formatter.indent();
            formatter.newline()?;
            formatter.write(
                &format!("locals['{}'] = i;", Formatter::safe_case(&self.first_var)),
                true,
            )?;
        } else {
            formatter.write("for (let varItem of game.getVariablesOf(", true)?;
            self.scope.write_output(formatter)?;
            formatter.write(")) {", false)?;
            formatter.newline()?;
            formatter.indent();

            formatter.writeln(&format!(
                "locals['{}'] = varItem.name;",
                Formatter::safe_case(&self.first_var)
            ))?;
            formatter.write(
                &format!(
                    "locals['{}'] = varItem.val;",
                    Formatter::safe_case(&self.second_var)
                ),
                true,
            )?;
        }

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
