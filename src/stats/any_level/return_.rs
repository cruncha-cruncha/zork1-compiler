use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{TokenType, ZilNode, ZilNodeType},
    },
};

// <RETURN>
// <RETURN ,KITCHEN>
// <RETURN <GOTO ,ROUND-ROOM>>

pub struct Return {}

impl HasZilName for Return {
    fn zil_name(&self) -> &'static str {
        "RETURN"
    }
}

impl CanValidate for Return {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() > 2 {
            return Err(format!(
                "RETURN node has more than two children\n{}",
                format_file_location(&n)
            ));
        }

        if n.children.len() == 2 {
            match n.children[1].node_type {
                ZilNodeType::Token(TokenType::Word) => (),
                ZilNodeType::Cluster => v.validate_cluster(&n.children[1])?,
                _ => {
                    return Err(format!(
                        "Second child of RETURN node is not a word or cluster\n{}",
                        format_file_location(&n)
                    ));
                }
            }
        }
        Ok(())
    }
}
