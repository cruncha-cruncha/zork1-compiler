use std::collections::BTreeMap;

use std::convert::TryFrom;

use crate::zil::{
    file_table::format_file_location,
    node::{TokenType, ZilNode, ZilNodeType},
};

pub type ValidationResult<T> = Result<T, Vec<String>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DescType {
    Routine(String),
    Text(String, bool), // bool is whether to append a newline, aka CR
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub enum VarVal {
//     Number(i32),
//     Text(String),
//     Word(String),
// }

pub struct Helpers {}

pub fn parse_token_as_word(node: &ZilNode) -> Option<String> {
    if node.node_type != ZilNodeType::Token(TokenType::Word) || node.token.is_none() {
        return None;
    }

    Some(node.token.as_ref().unwrap().value.clone())
}

pub fn get_token_as_word(node: &ZilNode) -> Result<String, String> {
    let word = parse_token_as_word(node);
    if word.is_none() {
        return Err(format!(
            "Expected word, found {}\n{}",
            node.node_type,
            format_file_location(&node)
        ));
    }

    Ok(word.unwrap())
}

pub fn parse_token_as_text(node: &ZilNode) -> Option<String> {
    if node.node_type != ZilNodeType::Token(TokenType::Text) || node.token.is_none() {
        return None;
    }

    Some(node.token.as_ref().unwrap().value.clone())
}

pub fn get_token_as_text(node: &ZilNode) -> Result<String, String> {
    let text = parse_token_as_text(node);
    if text.is_none() {
        return Err(format!(
            "Expected text, found {}\n{}",
            node.node_type,
            format_file_location(&node)
        ));
    }

    Ok(text.unwrap())
}

pub fn parse_token_as_number(node: &ZilNode) -> Option<i32> {
    if node.node_type != ZilNodeType::Token(TokenType::Number) || node.token.is_none() {
        return None;
    }

    match node.token.as_ref().unwrap().value.clone().parse::<i32>() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

pub fn get_token_as_number(node: &ZilNode) -> Result<i32, String> {
    let number = parse_token_as_number(node);
    if number.is_none() {
        return Err(format!(
            "Expected number, found {}\n{}",
            node.node_type,
            format_file_location(&node)
        ));
    }

    Ok(number.unwrap())
}

pub fn contains_same_elements<T>(a: &[T], b: &[T]) -> bool
where
    T: PartialEq,
{
    // if items need to be in the same order:
    // a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b)

    // this is fine for small vectors, but it's better to use a HashSet for larger ones
    a.len() == b.len() && a.iter().all(|item| b.contains(item))
}

pub fn num_children(node: &ZilNode, len: usize) -> Result<(), String> {
    if node.children.len() != len {
        return Err(format!(
            "Expected {} children, found {}\n{}",
            len,
            node.children.len(),
            format_file_location(&node)
        ));
    }

    Ok(())
}

pub fn num_children_between(node: &ZilNode, start: usize, end: usize) -> Result<(), String> {
    if node.children.len() < start || node.children.len() > end {
        return Err(format!(
            "Expected between {} and {} children, found {}\n{}",
            start,
            end,
            node.children.len(),
            format_file_location(&node)
        ));
    }

    Ok(())
}

pub fn num_children_more_than(node: &ZilNode, len: usize) -> Result<(), String> {
    if node.children.len() <= len {
        return Err(format!(
            "Expected more than {} children, found {}\n{}",
            len,
            node.children.len(),
            format_file_location(&node)
        ));
    }

    Ok(())
}

pub fn num_children_less_than(node: &ZilNode, len: usize) -> Result<(), String> {
    if node.children.len() >= len {
        return Err(format!(
            "Expected less than {} children, found {}\n{}",
            len,
            node.children.len(),
            format_file_location(&node)
        ));
    }

    Ok(())
}

pub fn i32_to_usize(i: i32) -> Result<usize, String> {
    match usize::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Expected usize, found i32: {}", i)),
    }
}

impl Helpers {
    pub fn crunch_desc(node: &ZilNode) -> ValidationResult<DescType> {
        match num_children_more_than(node, 1) {
            Err(e) => return Err(vec![e]),
            _ => (),
        }

        let mut errors: Vec<String> = Vec::new();
        let mut text = String::new();

        let second_child = &node.children[1];

        match second_child.node_type {
            ZilNodeType::Token(TokenType::Text) => {
                text = match get_token_as_text(second_child) {
                    Ok(v) => v,
                    Err(e) => return Err(vec![e]),
                }
            }
            ZilNodeType::Cluster => {
                match num_children_less_than(node, 3) {
                    Err(e) => return Err(vec![e]),
                    _ => (),
                }

                match num_children(second_child, 1) {
                    Err(e) => return Err(vec![e]),
                    _ => (),
                }

                let routine = match get_token_as_word(&second_child.children[0]) {
                    Ok(v) => v,
                    Err(e) => return Err(vec![e]),
                };
                return Ok(DescType::Routine(routine));
            }
            _ => errors.push(format!(
                "Desc node has invalid second child type: {}\n{}",
                second_child.node_type,
                format_file_location(&second_child)
            )),
        }

        let mut cr = false;
        if node.children.len() == 3 {
            let third_child = &node.children[2];
            let word = match get_token_as_word(&third_child) {
                Ok(v) => v,
                Err(e) => return Err(vec![e]),
            };
            if word != "CR" {
                errors.push(format!(
                    "Desc node has invalid third child type: {}\n{}",
                    word,
                    format_file_location(&third_child)
                ));
            } else {
                cr = true;
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(DescType::Text(text, cr))
    }

    pub fn crunch_vars(node: &ZilNode) -> ValidationResult<BTreeMap<String, i32>> {
        match num_children_more_than(node, 2) {
            Err(e) => return Err(vec![e]),
            _ => (),
        }

        if node.children.len() % 2 == 0 {
            return Err(vec![format!(
                "Vars node has {} children, expected an odd number\n{}",
                node.children.len(),
                format_file_location(&node)
            )]);
        }

        let mut out: BTreeMap<String, i32> = BTreeMap::new();
        let mut errors: Vec<String> = Vec::new();

        for i in 0..(node.children.len() - 1) / 2 {
            let name_child = &node.children[i * 2 + 1];
            let name = match get_token_as_word(&name_child) {
                Ok(v) => Some(v),
                Err(e) => {
                    errors.push(e);
                    None
                }
            };

            let val_child = &node.children[i * 2 + 2];
            let val = match val_child.node_type {
                ZilNodeType::Token(TokenType::Number) => parse_token_as_number(val_child),
                _ => {
                    errors.push(format!(
                        "Expected number or text, found {}\n{}",
                        val_child.node_type,
                        format_file_location(&val_child)
                    ));
                    None
                }
            };

            if name.is_some() && val.is_some() {
                out.insert(name.unwrap(), val.unwrap());
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(out)
    }

    pub fn crunch_action(node: &ZilNode) -> ValidationResult<String> {
        match num_children(node, 2) {
            Err(e) => return Err(vec![e]),
            _ => (),
        }

        if node.children[1].node_type != ZilNodeType::Cluster {
            return Err(vec![format!(
                "Expected cluster, found {}\n{}",
                node.children[0].node_type,
                format_file_location(&node.children[0])
            )]);
        }

        match num_children(&node.children[1], 1) {
            Err(e) => return Err(vec![e]),
            _ => (),
        }

        let word = match get_token_as_word(&node.children[1].children[0]) {
            Ok(v) => v,
            Err(e) => return Err(vec![e]),
        };

        Ok(word)
    }
}
