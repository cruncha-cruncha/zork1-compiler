use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{move_::Move, set_var::Scope},
};

impl CanWriteOutput for Move {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("move(", true)?;

        match self.thing {
            Scope::Local(ref name) => {
                formatter.write(&format!("locals['{}']", Formatter::safe_case(name)), false)?
            }
            Scope::Object(ref name) => {
                formatter.write(&format!("objects['{}']", Formatter::safe_case(name)), false)?
            }
            Scope::LOC(ref w) => w.write_output(formatter)?,
            _ => panic!("IDK"),
        }

        formatter.write(", ", false)?;

        match self.to {
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

        formatter.write(")", false)?;

        Ok(())
    }
}
