use crate::{
    stats::validate_recursive::{CanValidate, HasZilName, Validator},
    zil::{
        file_table::format_file_location,
        node::{ZilNode, ZilNodeType},
    },
};

// <REPEAT () <COND (<L? <SET N <- .N 1>> 1> <RETURN>) (T <TELL "    Fweep!" CR>)>>
// <REPEAT () <COND (<NOT .F> <RETURN>) (<G? <WEIGHT .F> 4> <SETG EMPTY-HANDED <>> <RETURN>)>
// <REPEAT () <COND (<NOT .L1> <RETURN>)> <SET N <NEXT? .L1>> <MOVE .L1 .RM2> <SET L1 .N>>

pub struct Repeat {}

impl HasZilName for Repeat {
    fn zil_name(&self) -> &'static str {
        "REPEAT"
    }
}

impl CanValidate for Repeat {
    fn validate(&self, n: &ZilNode, v: &Validator) -> Result<(), String> {
        if n.children.len() < 3 {
            return Err(format!(
                "REPEAT node does not have at least three children\n{}",
                format_file_location(&n)
            ));
        }

        match n.children[1].node_type {
            ZilNodeType::Group => {
                if n.children[1].children.len() > 0 {
                    return Err(format!(
                        "Second child of REPEAT node is not an empty group\n{}",
                        format_file_location(&n)
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "Second child of REPEAT node is not a group\n{}",
                    format_file_location(&n)
                ));
            }
        }

        for child in n.children.iter().skip(2) {
            match child.node_type {
                ZilNodeType::Cluster => v.validate_cluster(child)?,
                _ => {
                    return Err(format!(
                        "Child of REPEAT node is not a cluster\n{}",
                        format_file_location(&child)
                    ));
                }
            }
        }

        Ok(())
    }
}
