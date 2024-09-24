use std::collections::BTreeMap;

use crate::{
    js::{
        formatter::Formatter,
        write_output::{CanWriteOutput, ToJs},
    },
    stats::{
        helpers::DescType,
        top_level::objects::{ObjectLocation, ObjectStats},
    },
};

impl CanWriteOutput for ObjectStats {
    fn write_output(&self, formatter: &mut Formatter) -> Result<(), std::io::Error> {
        formatter.writeln("export const objects = {")?;
        formatter.indent();

        for info in self.as_codex() {
            formatter.writeln(&format!("{}: {{", Formatter::safe_case(&info.name)))?;
            formatter.indent();

            formatter.writeln(&format!(
                "isObject: '{}',",
                Formatter::safe_case(&info.name)
            ))?;

            if info.desc.is_some() {
                formatter.writeln(&format!("desc: {},", info.desc.as_ref().unwrap().to_js()))?;
            }

            formatter.writeln("copies: {")?;
            formatter.indent();

            for obj in info.copies.iter() {
                formatter.writeln(&format!("{}: {{", Formatter::safe_case(&obj.id)))?;
                formatter.indent();

                formatter.writeln(&format!("isObject: '{}',", Formatter::safe_case(&obj.name)))?;

                formatter.writeln(&format!("isInst: '{}',", Formatter::safe_case(&obj.id)))?;

                match obj.loc {
                    ObjectLocation::Player => {
                        formatter.writeln("loc: { scope: 'player' },")?;
                    }
                    ObjectLocation::Room(ref name) => {
                        formatter.writeln(&format!(
                            "loc: {{ scope: 'room', name: '{}' }},",
                            Formatter::safe_case(name)
                        ))?;
                    }
                    ObjectLocation::Object(ref name, _index, ref id) => {
                        let id = id.as_ref().unwrap();
                        formatter.writeln(&format!(
                            "loc: {{ scope: 'object', name: '{}', inst: '{}' }},",
                            Formatter::safe_case(name),
                            id
                        ))?;
                    }
                }

                formatter.writeln("vars: {")?;
                formatter.indent();
                for (var, val) in obj.vars.iter() {
                    formatter.writeln(&format!("{}: {},", Formatter::safe_case(var), val))?;
                }
                formatter.outdent();
                formatter.writeln("},")?;

                formatter.writeln("objects: {")?;
                formatter.indent();

                for (key, vals) in obj.nested.iter() {
                    formatter.write(&format!("{}: [", Formatter::safe_case(key)), true)?;
                    for val in vals.iter() {
                        formatter.write(&format!("'{}',", Formatter::safe_case(val)), false)?;
                    }
                    formatter.write("],", false)?;
                    formatter.newline()?;
                }

                formatter.outdent();
                formatter.writeln("},")?;

                formatter.outdent();
                formatter.writeln("},")?;
            }

            formatter.outdent();
            formatter.writeln("},")?;

            formatter.writeln("vars: {")?;
            formatter.indent();
            for (var, val) in info.vars.iter() {
                formatter.writeln(&format!("{}: {},", Formatter::safe_case(var), val))?;
            }
            formatter.outdent();
            formatter.writeln("},")?;

            formatter.writeln("hooks: {")?;
            formatter.indent();
            if info.actions.in_room.is_some() {
                formatter.writeln(&format!(
                    "inRoom: '{}',",
                    Formatter::safe_case(info.actions.in_room.as_ref().unwrap())
                ))?;
            }
            if info.actions.in_player.is_some() {
                formatter.writeln(&format!(
                    "inPlayer: '{}',",
                    Formatter::safe_case(info.actions.in_player.as_ref().unwrap())
                ))?;
            }
            if info.actions.enter_player.is_some() {
                formatter.writeln(&format!(
                    "enterPlayer: '{}',",
                    Formatter::safe_case(info.actions.enter_player.as_ref().unwrap())
                ))?;
            }
            if info.actions.exit_player.is_some() {
                formatter.writeln(&format!(
                    "exitPlayer: '{}',",
                    Formatter::safe_case(info.actions.exit_player.as_ref().unwrap())
                ))?;
            }
            formatter.outdent();
            formatter.writeln("},")?;

            formatter.outdent();
            formatter.writeln("},")?;
        }

        formatter.outdent();
        formatter.writeln("};")?;
        formatter.newline()?;

        formatter.writeln("export const translateSynonym = (word) => {")?;
        formatter.indent();

        formatter.writeln("switch (word) {")?;

        // key: synonym, value: object names
        let mut synonyms: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for info in self.as_codex() {
            for syn in info.synonyms.iter() {
                if synonyms.contains_key(syn) {
                    synonyms.get_mut(syn).unwrap().push(info.name.clone());
                } else {
                    synonyms.insert(syn.clone(), vec![info.name.clone()]);
                }
            }
        }

        // sort object names alphabetically
        for val in synonyms.values_mut() {
            val.sort();
        }

        // key: object names, value: synonyms
        let mut object_names: BTreeMap<Vec<String>, Vec<String>> = BTreeMap::new();
        for (syn, objs) in synonyms.iter() {
            if object_names.contains_key(objs) {
                object_names.get_mut(objs).unwrap().push(syn.clone());
            } else {
                object_names.insert(objs.clone(), vec![syn.clone()]);
            }
        }

        for (objs, syns) in object_names.iter() {
            for syn in syns.iter() {
                formatter.writeln(&format!("case '{}':", syn))?;
            }

            let object_list = objs
                .iter()
                .map(|o| format!("'{}'", Formatter::safe_case(o)))
                .collect::<Vec<String>>()
                .join(", ");

            formatter.indent();
            formatter.writeln(&format!("return [{}];", object_list))?;
            formatter.outdent();
        }

        formatter.writeln("default:")?;
        formatter.indent();
        formatter.writeln("return null;")?;
        formatter.outdent();

        formatter.writeln("}")?;

        formatter.outdent();
        formatter.writeln("}")?;

        Ok(())
    }
}

impl ToJs for DescType {
    fn to_js(&self) -> String {
        match self {
            DescType::Routine(s) => format!("{{ routine: '{}' }}", Formatter::safe_case(s)),
            DescType::Text(v, cr) => {
                let mut out = String::new();
                out.push_str(&format!("{{ text: \"{}", v));

                if *cr {
                    out.push_str(&format!("\\n"));
                } else {
                    out.push_str(&format!(""));
                }

                out.push_str(&format!("\" }}"));
                out
            }
        }
    }
}
