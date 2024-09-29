use std::fmt;

use crate::stats::{
    cross_ref::Codex,
    top_level::{
        buzzi::BuzzStats,
        directions::DirectionStats,
        synonyms::SynonymStats,
        syntax::{SyntaxItem, SyntaxStats},
    },
};

use super::{formatter::Formatter, write_output::CanWriteOutput};

pub struct ParseTree {
    branches: Vec<SyntaxStep>,
    buzz: Vec<String>,
    directions: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum SyntaxStep {
    Cmd(Cmd),
    Object(Object),
    End(String),
}

#[derive(Clone, Debug)]
pub struct Cmd {
    pub name: String,
    pub synonyms: Vec<String>,
    pub children: Vec<SyntaxStep>,
}

#[derive(Clone, Debug)]
pub struct Object {
    pub children: Vec<SyntaxStep>,
}

impl ParseTree {
    pub fn new() -> ParseTree {
        ParseTree {
            branches: Vec::new(),
            buzz: Vec::new(),
            directions: Vec::new(),
        }
    }

    pub fn parse_syntax(&mut self, syntax_stats: &SyntaxStats) {
        let syntax_codex = syntax_stats.as_codex();
        let mut branches: Vec<SyntaxStep> = Vec::new();

        for line in syntax_codex {
            let mut is_ambiguous: Option<usize> = None;

            let action_word = match line.first() {
                Some(SyntaxItem::Action(act)) => act.clone(),
                _ => unreachable!(),
            };

            // if none, then the first word is OBJECT
            let first_word = match &line[1] {
                SyntaxItem::Cmd(cmd) => Some(cmd.clone()),
                SyntaxItem::Object => None,
                _ => unreachable!(),
            };

            let branch_index = match branches.iter().position(|branch| match branch {
                SyntaxStep::Cmd(cmd) => {
                    first_word.is_some() && &cmd.name == first_word.as_ref().unwrap()
                }
                SyntaxStep::Object(_) => first_word.is_none(),
                _ => false,
            }) {
                Some(i) => i,
                None => {
                    let new_branch = SyntaxStep::Cmd(Cmd {
                        name: first_word.unwrap_or_default(),
                        synonyms: Vec::new(),
                        children: Vec::new(),
                    });

                    branches.push(new_branch);
                    branches.len() - 1
                }
            };
            let mut branch = &mut branches[branch_index];

            for syntax_type in line.iter().skip(2) {
                let index = match branch.add_child(syntax_type) {
                    Ok(index) => index,
                    Err(index) => {
                        if is_ambiguous.is_none() {
                            is_ambiguous = Some(index);
                        }
                        index
                    }
                };
                branch = &mut branch.get_children_mut()[index];
            }

            match branch.add_end(action_word) {
                Ok(()) => (),
                Err(()) => is_ambiguous = Some(line.len()),
            }

            if is_ambiguous.is_some() {
                print!("WARNING: Redundant syntax: '");
                for (i, word) in line.iter().enumerate() {
                    print!("{}", word);
                    if i < line.len() - 1 {
                        print!(" ");
                    }
                }
                print!("' (discernable until word {})\n", is_ambiguous.unwrap() + 2);
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
                SyntaxStep::Object(_) => (),
                SyntaxStep::End(_) => return,
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

    fn write_output_object(
        &self,
        formatter: &mut Formatter,
        object: &Object,
        depth: usize,
    ) -> Result<(), std::io::Error> {
        formatter.writeln(&format!(
            "const {{ objectVal }} = game.findObjectMatchingParsedWord(words[{}]);",
            depth - 1,
        ))?;

        formatter.writeln(&format!(
            "cmds.push({{ word: words[{}], val: objectVal }});",
            depth - 1,
        ))?;

        // formatter.writeln("if (objectVal) {")?;
        self.write_output_recursive(formatter, &object.children, depth)?;
        // formatter.writeln("}")?;

        Ok(())
    }

    fn write_output_end(
        &self,
        formatter: &mut Formatter,
        action_word: String,
        depth: usize,
    ) -> Result<(), std::io::Error> {
        formatter.writeln(&format!("if (words.length == {}) {{", depth - 1))?;
        formatter.indent();

        formatter.writeln(&format!(
            "return {{handle: '', prsa: '{}', cmds }};",
            Formatter::safe_case(&action_word)
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

        if let Some(action_word) = SyntaxStep::has_end_child(children) {
            self.write_output_end(formatter, action_word, depth + 1)?;
        }

        if let Some(ref object) = SyntaxStep::has_object_child(children) {
            self.write_output_object(formatter, object, depth + 1)?;
        }

        formatter.writeln("return { prsa: '', cmds };")?;
        formatter.outdent();

        formatter.writeln("}")?;
        formatter.outdent();

        Ok(())
    }
}

impl CanWriteOutput for ParseTree {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("import { game, getEmptyResource } from './engine.js';")?;
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

        formatter.writeln(
            "if (!rawString || typeof rawString !== 'string') { return { prsa: '', cmds: [] }; }",
        )?;

        formatter
            .writeln("const words = rawString.split(\" \").map(w => w.toUpperCase()).filter(w => !!w && !buzz.includes(w));")?;
        formatter.writeln("if (words.length == 0) { return { prsa: '', cmds: [{}] }; }")?;
        formatter.writeln("let cmds = [{}];")?;
        formatter.newline()?;

        formatter.writeln("if ((words.length == 2) && (words[0] == \"GO\")) {")?;
        formatter.indent();
        formatter.writeln("return { move: words[1], prsa: 'GO', cmds: [] };")?;
        formatter.outdent();
        formatter.writeln("}")?;
        formatter.newline()?;

        formatter.outdent();
        self.write_output_recursive(formatter, &self.branches, 0)?;

        formatter.writeln("}")?;

        Ok(())
    }
}

impl SyntaxStep {
    pub fn get_children_len(&self) -> usize {
        match self {
            SyntaxStep::Cmd(cmd) => cmd.children.len(),
            SyntaxStep::Object(obj) => obj.children.len(),
            SyntaxStep::End(_) => 0,
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

    pub fn has_object_child(children: &Vec<SyntaxStep>) -> Option<&Object> {
        children
            .iter()
            .filter_map(|child| match child {
                SyntaxStep::Object(obj) => Some(obj),
                _ => None,
            })
            .collect::<Vec<&Object>>()
            .first()
            .copied()
    }

    pub fn has_end_child(children: &Vec<SyntaxStep>) -> Option<String> {
        for child in children.iter() {
            match child {
                SyntaxStep::End(word) => return Some(word.clone()),
                _ => (),
            }
        }
        None
    }

    pub fn get_children_mut(&mut self) -> &mut Vec<SyntaxStep> {
        match self {
            SyntaxStep::Cmd(cmd) => &mut cmd.children,
            SyntaxStep::Object(obj) => &mut obj.children,
            SyntaxStep::End(_) => panic!(),
        }
    }

    pub fn add_end(&mut self, action_word: String) -> Result<(), ()> {
        let children = self.get_children_mut();
        for child in children.iter() {
            match child {
                SyntaxStep::End(word) => {
                    if word == &action_word {
                        return Ok(());
                    } else {
                        return Err(());
                    }
                }
                _ => (),
            }
        }
        children.push(SyntaxStep::End(action_word));
        Ok(())
    }

    pub fn add_child(&mut self, new_child: &SyntaxItem) -> Result<usize, usize> {
        let children = self.get_children_mut();

        for (i, c) in children.iter_mut().enumerate() {
            match c {
                SyntaxStep::Cmd(cmd) => {
                    if let SyntaxItem::Cmd(ref new_cmd) = new_child {
                        if &cmd.name == new_cmd {
                            return Ok(i);
                        }
                    }
                }
                SyntaxStep::Object(_) => {
                    if let SyntaxItem::Object = new_child {
                        return Err(i);
                    }
                }
                SyntaxStep::End(_) => (),
            }
        }

        let new_child = match new_child {
            SyntaxItem::Cmd(cmd) => SyntaxStep::Cmd(Cmd {
                name: cmd.clone(),
                synonyms: Vec::new(),
                children: Vec::new(),
            }),
            SyntaxItem::Object => SyntaxStep::Object(Object {
                children: Vec::new(),
            }),
            SyntaxItem::Action(_) => unreachable!(),
        };

        children.push(new_child);

        Ok(self.get_children_len() - 1)
    }
}

impl Cmd {
    pub fn get_names(&self) -> Vec<String> {
        let mut names = vec![self.name.clone()];
        names.extend(self.synonyms.clone());
        names
    }
}

impl fmt::Display for SyntaxStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyntaxStep::Cmd(cmd) => write!(f, "{}", cmd.name),
            SyntaxStep::Object(_) => write!(f, "<object>"),
            SyntaxStep::End(act) => write!(f, "{}", act),
        }
    }
}
