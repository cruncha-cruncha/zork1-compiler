use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::cond::Cond,
};

impl CanWriteOutput for Cond {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        for (i, branch) in self.branches.iter().enumerate() {
            formatter.write("if ", i == 0)?;
            branch.condition.write_output(formatter)?;
            formatter.write(" {", false)?;

            formatter.indent();
            for node in branch.body.iter() {
                node.write_output(formatter)?;
                formatter.write(";", false)?;
            }
            formatter.outdent();

            formatter.newline()?;
            if i < self.branches.len() - 1 {
                formatter.write("} else ", true)?;
            } else {
                formatter.write("}", true)?;
            }
        }

        Ok(())
    }
}
