use crate::{
    js::{
        formatter::Formatter,
        write_output::{CanWriteOutput, OutputNode},
    },
    stats::{
        any_level::{set_var::Scope, tell::Tell},
        routine_tracker::ReturnValType,
    },
};

impl CanWriteOutput for Tell {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("game.log(", true)?;

        for (i, node) in self.text.iter().enumerate() {
            let return_type = self.text_types[i];
            match node {
                OutputNode::Number(n) => formatter.write(&format!("{}.toString()", n), false)?,
                OutputNode::Text(t) => formatter.write(&format!("\"{}\"", t), false)?,
                OutputNode::Variable(Scope::Local(local_var)) => {
                    formatter.write(
                        &format!("locals['{}']", Formatter::safe_case(local_var)),
                        false,
                    )?;
                    match return_type {
                        ReturnValType::Text => (),
                        _ => formatter.write(".toString()", false)?,
                    }
                }
                OutputNode::Variable(Scope::Global(name)) => formatter.write(
                    &format!("globals['{}'].toString()", Formatter::safe_case(name)),
                    false,
                )?,
                OutputNode::Writer(ref w) => {
                    w.write_output(formatter)?;
                    formatter.write(".toString()", false)?;
                }
                _ => panic!("IDK"),
            }

            if i < self.text.len() - 1 {
                formatter.write(", ", false)?;
            }
        }

        if self.cr {
            if self.text.len() > 0 {
                formatter.write(", ", false)?;
            }

            formatter.write("'\\n'", false)?;
        }

        formatter.write(");", false)?;

        Ok(())
    }
}
