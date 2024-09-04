use std::path::Path;

use crate::stats::cross_ref;

use super::{build_parser::ParseTree, formatter::Formatter};

pub trait HasJsName {
    fn js_name(&self) -> &'static str;
}

pub trait CanWriteOutput {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error>;
}

pub fn write_output(cross_ref: &cross_ref::CrossRef) {
    let mut formatter = Formatter::new(&Path::new(".").join("output-files").join("rooms.js"));
    cross_ref.rooms.write_output(&mut formatter);

    let mut formatter = Formatter::new(&Path::new(".").join("output-files").join("objects.js"));
    cross_ref.objects.write_output(&mut formatter);

    let mut parseTree = ParseTree::new();
    parseTree.parse_syntax(&cross_ref.syntax);
    parseTree.add_synonyms(&cross_ref.synonyms);
    parseTree.add_buzzi(&cross_ref.buzzi);

    let mut formatter = Formatter::new(&Path::new(".").join("output-files").join("parser.js"));
    parseTree.write_output(&mut formatter);
}
