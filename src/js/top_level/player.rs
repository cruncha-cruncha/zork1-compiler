use crate::{
    js::{formatter::Formatter, write_output::CanWriteOutput},
    stats::top_level::player::PlayerStats,
};

impl CanWriteOutput for PlayerStats {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("import { rooms } from \"./rooms.js\";")?;
        formatter.writeln("")?;

        formatter.writeln("export const currentRoom = {")?;
        formatter.indent();

        match self.get_room() {
            Some(room) => {
                formatter.writeln(&format!("name: '{}',", Formatter::safe_case(&room)))?;
            }
            None => {
                formatter.writeln("name: Object.keys(rooms)[0],")?;
            }
        }

        formatter.outdent();
        formatter.writeln("};")?;

        formatter.flush()?;

        Ok(())
    }
}
