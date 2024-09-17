use crate::stats::{
    cross_ref::Codex,
    helpers::contains_same_elements,
    top_level::{
        buzzi::BuzzStats,
        directions::DirectionStats,
        synonyms::SynonymStats,
        syntax::{SyntaxItem, SyntaxStats},
    },
};

use super::{
    formatter::Formatter,
    write_output::{CanWriteOutput, ToJs},
};

pub struct ParseTree {
    branches: Vec<SyntaxStep>,
    buzz: Vec<String>,
    directions: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum SyntaxStep {
    Cmd(Cmd),
    Object(Object),
    Action(Action),
}

#[derive(Clone, Debug)]
pub struct Cmd {
    pub name: String,
    pub synonyms: Vec<String>,
    pub children: Vec<SyntaxStep>,
}

#[derive(Clone, Debug)]
pub struct Object {
    pub restrictions: Vec<String>,
    pub children: Vec<SyntaxStep>,
}

#[derive(Clone, Debug)]
pub struct Action {
    pub routine: String,
    pub children: Vec<SyntaxStep>,
}

// TODO: if player tries a command that starts with a known action but doesn't quite match the rest of the syntax,
// suggest the syntax. Like if player tries 'SPREAD OVER BOARDS', suggest 'SPREAD OBJECT ON OBJECT'.

impl ParseTree {
    pub fn new() -> ParseTree {
        ParseTree {
            branches: Vec::new(),
            buzz: Vec::new(),
            directions: Vec::new(),
        }
    }

    pub fn parse_syntax(&mut self, syntax_stats: &SyntaxStats) {
        let mut branches: Vec<SyntaxStep> = Vec::new();

        for line in syntax_stats.as_iter() {
            let first_word = match line.first() {
                Some(SyntaxItem::Cmd(cmd)) => cmd.name.clone(),
                _ => unreachable!(),
            };

            let mut branch = match branches.iter_mut().find(|branch| match branch {
                SyntaxStep::Cmd(cmd) => cmd.name == first_word,
                _ => false,
            }) {
                Some(branch) => branch,
                None => {
                    let new_branch = SyntaxStep::Cmd(Cmd {
                        name: first_word,
                        synonyms: Vec::new(),
                        children: Vec::new(),
                    });

                    branches.push(new_branch);
                    branches.last_mut().unwrap()
                }
            };

            for syntax_type in line.iter().skip(1) {
                let index = branch.add_child(syntax_type);
                branch = &mut branch.get_children_mut()[index];
            }
        }

        self.branches = branches;
    }

    pub fn add_synonyms(&mut self, synonyms: &SynonymStats) {
        Self::add_synonyms_recursive(synonyms, &mut self.branches.iter_mut());
    }

    pub fn add_buzzi(&mut self, buzzi: &BuzzStats) {
        self.buzz = buzzi.get_vals();
    }

    pub fn add_directions(&mut self, directions: &DirectionStats) {
        self.directions = directions.get_vals();
    }

    fn add_synonyms_recursive(
        synonyms: &SynonymStats,
        children: &mut std::slice::IterMut<SyntaxStep>,
    ) {
        for child in children {
            match child {
                SyntaxStep::Cmd(cmd) => {
                    if let Some(synonyms) = synonyms.lookup(&cmd.name) {
                        cmd.synonyms = synonyms;
                    }
                }
                _ => (),
            }

            Self::add_synonyms_recursive(synonyms, &mut child.get_children_mut().iter_mut());
        }
    }

    fn write_output_cmd(
        &self,
        formatter: &mut Formatter,
        cmd: &Cmd,
        depth: usize,
    ) -> Result<(), std::io::Error> {
        for name in cmd.get_names() {
            formatter.writeln(&format!("case \"{}\":", name))?;
        }

        self.write_output_recursive(formatter, &cmd.children, depth)?;

        Ok(())
    }

    fn write_output_objects(
        &self,
        formatter: &mut Formatter,
        objects: Vec<&Object>,
        depth: usize,
    ) -> Result<(), std::io::Error> {
        if objects.len() == 0 {
            return Ok(());
        }

        let objects_str = objects
            .iter()
            .map(|obj| obj.to_js())
            .collect::<Vec<String>>()
            .join(", ");

        formatter.writeln(&format!(
            "const {{ objectNum, objectVal }} = game.findObjectMatchingParsedWord(words[{}], [{}]);",
            depth - 1,
            objects_str
        ))?;

        formatter.writeln(&format!(
            "if (prso.val && !prsi.val) {{ prsi = {{ word: words[{}], val: objectVal }}; }} else if (!prso.val) {{ prso = {{ word: words[{}], val: objectVal }}; }}",
            depth - 1,
            depth - 1,
        ))?;

        formatter.writeln("switch (objectNum) {")?;

        for (i, object) in objects.iter().enumerate() {
            formatter.writeln(&format!("case {}:", i + 1))?;
            self.write_output_recursive(formatter, &object.children, depth)?;
        }

        formatter.writeln("}")?;

        Ok(())
    }

    fn write_output_action(
        &self,
        formatter: &mut Formatter,
        action: Option<&Action>,
        depth: usize,
    ) -> Result<(), std::io::Error> {
        if action.is_none() {
            return Ok(());
        }

        formatter.writeln(&format!("if (words.length == {}) {{", depth - 1))?;
        formatter.indent();

        formatter.writeln(&format!(
            "return {{routine: '{}', prsa, prso, prsi }};",
            Formatter::safe_case(&action.unwrap().routine)
        ))?;

        formatter.outdent();
        formatter.writeln("}")?;

        Ok(())
    }

    fn write_output_recursive(
        &self,
        formatter: &mut Formatter,
        children: &Vec<SyntaxStep>,
        depth: usize,
    ) -> Result<(), std::io::Error> {
        formatter.indent();
        formatter.writeln(&format!("switch (words[{}]) {{", depth))?;

        for child in SyntaxStep::get_cmd_children(children) {
            self.write_output_cmd(formatter, child, depth + 1)?;
        }

        formatter.writeln("default:")?;
        formatter.indent();

        let action_child = SyntaxStep::get_action_child(children);
        self.write_output_action(formatter, action_child, depth + 1)?;

        let object_children = SyntaxStep::get_object_children(children);
        self.write_output_objects(formatter, object_children, depth + 1)?;

        formatter.writeln("return { prsa, prso, prsi };")?;
        formatter.outdent();

        formatter.writeln("}")?;
        formatter.outdent();

        Ok(())
    }
}

impl CanWriteOutput for ParseTree {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("import { game } from './game.js';")?;
        formatter.newline()?;

        formatter.writeln(&format!(
            "export const buzz = [{}];",
            self.buzz
                .iter()
                .map(|w| format!("\"{}\"", str::replace(w, "\"", "\\\"")))
                .collect::<Vec<String>>()
                .join(", ")
        ))?;
        formatter.newline()?;

        formatter.writeln(&format!(
            "export const directions = [{}];",
            self.directions
                .iter()
                .map(|w| format!("\"{}\"", w))
                .collect::<Vec<String>>()
                .join(", ")
        ))?;
        formatter.newline()?;

        formatter.writeln("export const parseInput = (rawString) => {")?;
        formatter.indent();

        formatter
            .writeln("const words = rawString.split(\" \").map(w => w.toUpperCase()).filter(w => !buzz.includes(w));")?;
        formatter.writeln("if ((words.length == 2) && (words[0] == \"GO\")) {")?;
        formatter.indent();
        formatter.writeln("return { move: words[1], prsa: translateAction(words[0]) };")?;
        formatter.outdent();
        formatter.writeln("}")?;
        formatter.newline()?;

        formatter.writeln("const prsa = translateAction(words[0]);")?;
        formatter.writeln("let prso = {};")?;
        formatter.writeln("let prsi = {};")?;
        formatter.newline()?;

        formatter.outdent();
        self.write_output_recursive(formatter, &self.branches, 0)?;

        formatter.writeln("}")?;

        formatter.newline()?;
        formatter.writeln("export const translateAction = (actionWord) => {")?;
        formatter.indent();
        formatter.writeln("switch (actionWord) {")?;
        formatter.indent();

        for branch in &self.branches {
            match branch {
                SyntaxStep::Cmd(cmd) => {
                    for name in cmd.get_names() {
                        formatter.writeln(&format!("case \"{}\":", name))?;
                    }

                    formatter.indent();
                    formatter.writeln(&format!("return \"{}\";", cmd.name))?;
                    formatter.outdent();
                }
                _ => unreachable!(),
            }
        }

        formatter.writeln("default:")?;
        formatter.indent();
        formatter.writeln("return actionWord;")?;
        formatter.outdent();
        formatter.writeln("}")?;
        formatter.outdent();
        formatter.writeln("}")?;

        Ok(())
    }
}

impl SyntaxStep {
    pub fn get_children(&self) -> &Vec<SyntaxStep> {
        match self {
            SyntaxStep::Cmd(cmd) => &cmd.children,
            SyntaxStep::Object(obj) => &obj.children,
            SyntaxStep::Action(action) => &action.children,
        }
    }

    pub fn get_cmd_children(children: &Vec<SyntaxStep>) -> Vec<&Cmd> {
        children
            .iter()
            .filter_map(|child| match child {
                SyntaxStep::Cmd(cmd) => Some(cmd),
                _ => None,
            })
            .collect()
    }

    pub fn get_object_children(children: &Vec<SyntaxStep>) -> Vec<&Object> {
        children
            .iter()
            .filter_map(|child| match child {
                SyntaxStep::Object(obj) => Some(obj),
                _ => None,
            })
            .collect()
    }

    pub fn get_action_child(children: &Vec<SyntaxStep>) -> Option<&Action> {
        children
            .iter()
            .filter_map(|child| match child {
                SyntaxStep::Action(action) => Some(action),
                _ => None,
            })
            .collect::<Vec<&Action>>()
            .last()
            .copied()
    }

    pub fn get_children_mut(&mut self) -> &mut Vec<SyntaxStep> {
        match self {
            SyntaxStep::Cmd(cmd) => &mut cmd.children,
            SyntaxStep::Object(obj) => &mut obj.children,
            SyntaxStep::Action(action) => &mut action.children,
        }
    }

    pub fn add_child(&mut self, new_child: &SyntaxItem) -> usize {
        for (i, c) in self.get_children_mut().iter_mut().enumerate() {
            match c {
                SyntaxStep::Cmd(cmd) => {
                    if let SyntaxItem::Cmd(ref new_cmd) = new_child {
                        if cmd.name == new_cmd.name {
                            return i;
                        }
                    }
                }
                SyntaxStep::Object(obj) => {
                    if let SyntaxItem::Object(ref new_obj) = new_child {
                        if contains_same_elements(&obj.restrictions, &new_obj.restrictions) {
                            return i;
                        }
                    }
                }
                SyntaxStep::Action(action) => {
                    if let SyntaxItem::Action(ref new_action) = new_child {
                        if action.routine == new_action.routine {
                            return i;
                        }
                    }
                }
            }
        }

        let new_child = match new_child {
            SyntaxItem::Cmd(cmd) => SyntaxStep::Cmd(Cmd {
                name: cmd.name.clone(),
                synonyms: Vec::new(),
                children: Vec::new(),
            }),
            SyntaxItem::Object(obj) => SyntaxStep::Object(Object {
                restrictions: obj.restrictions.clone(),
                children: Vec::new(),
            }),
            SyntaxItem::Action(action) => SyntaxStep::Action(Action {
                routine: action.routine.clone(),
                children: Vec::new(),
            }),
        };

        self.get_children_mut().push(new_child);

        self.get_children().len() - 1
    }
}

impl Cmd {
    pub fn get_names(&self) -> Vec<String> {
        let mut names = vec![self.name.clone()];
        names.extend(self.synonyms.clone());
        names
    }
}

impl ToJs for Object {
    fn to_js(&self) -> String {
        let mut out = String::from("{");

        out.push_str("withVars: [");

        for (i, restriction) in self.restrictions.iter().enumerate() {
            out.push_str(&format!("\"{}\"", Formatter::safe_case(restriction)));

            if i < self.restrictions.len() - 1 {
                out.push_str(", ");
            }
        }

        out.push_str("]");

        out.push_str("}");

        out
    }
}
