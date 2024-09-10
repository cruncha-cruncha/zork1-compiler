use crate::{
    js::{
        formatter::Formatter,
        write_output::{CanWriteOutput, ToJs},
    },
    stats::top_level::objects::{DescType, ObjectStats},
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

            formatter.writeln("objects: [")?;
            formatter.indent();
            for obj in info.objects.iter() {
                formatter.writeln(&format!("'{}',", Formatter::safe_case(obj)))?;
            }
            formatter.outdent();
            formatter.writeln("],")?;

            if info.loc.is_some() {
                formatter.writeln(&format!(
                    "loc: '{}',",
                    Formatter::safe_case(info.loc.as_ref().unwrap())
                ))?;
            }

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
            if info.actions.prso.is_some() {
                formatter.writeln(&format!(
                    "prso: '{}',",
                    Formatter::safe_case(info.actions.prso.as_ref().unwrap())
                ))?;
            }
            if info.actions.prsi.is_some() {
                formatter.writeln(&format!(
                    "prsi: '{}',",
                    Formatter::safe_case(info.actions.prsi.as_ref().unwrap())
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

        formatter.writeln("export const lookupBySynonym = (word) => {")?;
        formatter.indent();

        formatter.writeln("switch (word) {")?;

        for info in self.as_codex() {
            if info.synonyms.is_empty() {
                continue;
            }

            for syn in info.synonyms.iter() {
                formatter.writeln(&format!("case '{}':", syn))?;
            }

            formatter.indent();
            formatter.writeln(&format!(
                "return objects['{}'];",
                Formatter::safe_case(&info.name)
            ))?;
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
            DescType::Text(v) => format!("{{ text: \"{}\" }}", v),
        }
    }
}
