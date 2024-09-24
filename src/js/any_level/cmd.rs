use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::cmd::Cmd,
};

impl CanWriteOutput for Cmd {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.write("(locals['cmd']?.[", false)?;
        self.num.write_output(formatter)?;
        formatter.write("] ?? getEmptyResource())", false)?;

        Ok(())
    }
}
