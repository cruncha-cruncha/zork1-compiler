use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <COND (<FSET? ,KITCHEN-WINDOW ,OPENBIT> <TELL "open.">) (T <TELL "slightly ajar.">)>
// <COND (<FSET? .OBJ ,OPENBIT> <TELL .STRCLS> <FCLEAR .OBJ ,OPENBIT> T) (T <TELL <PICK-ONE ,DUMMY>>)>
// <COND (<VERB? FIND READ> <TELL "It only SAYS \"Granite Wall\"." CR>) (T <TELL "The wall isn't granite." CR>)>
// <COND (.W <SET PI? T>)>

// if statement in js can be represented by an array of { condition, body } objects, which is exactly what we have here!

pub struct Cond {}

impl HasZilName for Cond {
    fn zil_name(&self) -> &'static str {
        "COND"
    }
}

impl CanValidate for Cond {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() < 2 {
            return Err(format!(
                "COND node has less than two children\n{}",
                format_file_location(&n)
            ));
        }

        for child in n.children.iter().skip(1) {
            if child.node_type != ZilNodeType::Group {
                return Err(format!(
                    "Child of COND node is not a group\n{}",
                    format_file_location(&child)
                ));
            }

            if child.children.len() == 0 {
                return Err(format!(
                    "Group node of COND has no children\n{}",
                    format_file_location(&child)
                ));
            }

            for c in child.children.iter() {
                match c.node_type {
                    ZilNodeType::Token(TokenType::Word) => (),
                    ZilNodeType::Cluster => v.validate_cluster(&c)?,
                    _ => {
                        return Err(format!(
                            "Child of COND node is not a word or cluster\n{}",
                            format_file_location(&c)
                        ));
                    }
                }
            }
        }

        return Ok(());
    }
}
