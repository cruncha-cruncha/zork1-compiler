use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::any_level::{
        move_::Move,
        set_var::{Scope, VarWordType},
    },
};

impl CanWriteOutput for Move {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.newline()?;

        formatter.write("move(", true)?;

        match self.thing {
            Scope::Player => formatter.write("player", false)?,
            Scope::Object(VarWordType::Literal(ref s)) => {
                formatter.write(&format!("objects['{}']", s), false)?
            }
            Scope::Object(VarWordType::Indirect(ref s)) => {
                formatter.write(&format!("objects[{}]", s), false)?
            }
            Scope::Location(ref s) => formatter.write(&format!("{}", s), false)?,
            Scope::LOC(ref s) => s.write_output(formatter)?,
            _ => panic!("IDK"),
        }

        formatter.write(", ", false)?;

        match self.to {
            Scope::Object(VarWordType::Literal(ref s)) => {
                formatter.write(&format!("objects['{}']", s), false)?
            }
            Scope::Object(VarWordType::Indirect(ref s)) => {
                formatter.write(&format!("objects[{}]", s), false)?
            }
            Scope::Room(ref s) => formatter.write(&format!("rooms['{}']", s), false)?,
            Scope::Location(ref s) => formatter.write(&format!("{}", s), false)?,
            Scope::LOC(ref s) => s.write_output(formatter)?,
            _ => panic!("IDK"),
        }

        formatter.write(")", false)?;

        Ok(())
    }
}
