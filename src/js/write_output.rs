use std::path::Path;

use crate::stats::{any_level::set_var::Scope, cross_ref::CrossRef, routine_tracker::Validator};

use super::{build_parser::ParseTree, formatter::Formatter, top_level::routines::RoutineToots};

pub trait ToJs {
    fn to_js(&self) -> String;
}

pub trait CanWriteOutput {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error>;
}

pub enum OutputNode {
    TBD,
    Number(i32),
    Text(String),
    Variable(Scope),
    Writer(Box<dyn CanWriteOutput>),
}

pub fn write_output(cross_ref: &CrossRef, validator: &mut Validator) -> Result<(), std::io::Error> {
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

    let mut formatter = Formatter::new(&Path::new(".").join("output-files").join("routines.js"));
    cross_ref.routines.write_output(&mut formatter)?;
    let routines_recursive = RoutineToots::from(validator);
    routines_recursive.write_output(&mut formatter)?;

    let mut formatter = Formatter::new(&Path::new(".").join("output-files").join("globals.js"));
    cross_ref.player.write_output(&mut formatter)?;
    cross_ref.globals.write_output(&mut formatter)?;

    let mut parse_tree = ParseTree::new();
    parse_tree.parse_syntax(&cross_ref.syntax);
    parse_tree.add_synonyms(&cross_ref.synonyms);
    parse_tree.add_buzzi(&cross_ref.buzzi);
    parse_tree.add_directions(&cross_ref.directions);

    let mut formatter = Formatter::new(&Path::new(".").join("output-files").join("parser.js"));
    parse_tree.write_output(&mut formatter)?;

    Ok(())
}

impl CanWriteOutput for OutputNode {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        match self {
            OutputNode::TBD => panic!("TBD"),
            OutputNode::Variable(scope) => scope.write_output(formatter),
            OutputNode::Text(s) => formatter.write(&format!("\"{}\"", s), false),
            OutputNode::Number(n) => formatter.write(&format!("{}", n), false),
            OutputNode::Writer(w) => w.write_output(formatter),
        }
    }
}

impl CanWriteOutput for Scope {
    fn write_output<'a>(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        match self {
            Scope::TBD => panic!("TBD"),
            Scope::Player => formatter.write("player", false),
            Scope::Bare(ref name) => {
                formatter.write(&format!("'{}'", Formatter::safe_case(name)), false)
            }
            Scope::Local(ref local_var) => formatter.write(
                &format!("locals['{}']", Formatter::safe_case(local_var)),
                false,
            ),
            Scope::Global(ref name) => {
                formatter.write(&format!("globals['{}']", Formatter::safe_case(name)), false)
            }
            Scope::Object(ref name) => {
                formatter.write(&format!("objects['{}']", Formatter::safe_case(name)), false)
            }
            Scope::Room(ref name) => {
                formatter.write(&format!("rooms['{}']", Formatter::safe_case(name)), false)
            }
            Scope::Writer(ref w) => w.write_output(formatter),
        }
    }
}
