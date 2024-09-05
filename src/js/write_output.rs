use std::path::Path;

use crate::stats::cross_ref;

use super::{build_parser::ParseTree, formatter::Formatter};

pub trait ToJs {
    fn to_js(&self) -> String;
}

pub trait CanWriteOutput {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error>;
}

pub fn write_output(cross_ref: &cross_ref::CrossRef) -> Result<(), std::io::Error> {
    {
        let boilerplate = Path::new(".").join("js-boilerplate");
        let output_files = Path::new(".").join("output-files");
        std::fs::create_dir_all(&output_files)?;

        let paths = std::fs::read_dir(&boilerplate)?;
        for path in paths {
            let path = path?.path();
            let file_name = path.file_name().unwrap();
            let dest = output_files.join(file_name);
            std::fs::copy(&path, &dest)?;
        }
    }

    let mut formatter = Formatter::new(&Path::new(".").join("output-files").join("rooms.js"));
    cross_ref.rooms.write_output(&mut formatter)?;

    let mut formatter = Formatter::new(&Path::new(".").join("output-files").join("objects.js"));
    cross_ref.objects.write_output(&mut formatter)?;

    let mut formatter = Formatter::new(&Path::new(".").join("output-files").join("player.js"));
    cross_ref.player.write_output(&mut formatter)?;

    let mut parseTree = ParseTree::new();
    parseTree.parse_syntax(&cross_ref.syntax);
    parseTree.add_synonyms(&cross_ref.synonyms);
    parseTree.add_buzzi(&cross_ref.buzzi);
    parseTree.add_directions(&cross_ref.directions);

    let mut formatter = Formatter::new(&Path::new(".").join("output-files").join("parser.js"));
    parseTree.write_output(&mut formatter)?;

    Ok(())
}
