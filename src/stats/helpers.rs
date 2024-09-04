use crate::zil::node::{TokenType, ZilNode, ZilNodeType};

pub fn get_nth_child_as_word(n: usize, node: &ZilNode) -> Option<String> {
    if node.children.len() <= n {
        return None;
    }

    get_token_as_word(&node.children[n])
}

pub fn get_token_as_word(node: &ZilNode) -> Option<String> {
    if node.node_type != ZilNodeType::Token(TokenType::Word) || node.token.is_none() {
        return None;
    }

    Some(node.token.as_ref().unwrap().value.clone())
}

pub fn get_token_as_number(node: &ZilNode) -> Option<i32> {
    if node.node_type != ZilNodeType::Token(TokenType::Number) || node.token.is_none() {
        return None;
    }

    match node.token.as_ref().unwrap().value.clone().parse::<i32>() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

pub fn contain_same_elements<T>(a: &[T], b: &[T]) -> bool
where
    T: PartialEq,
{
    // if items need to be in the same order:
    // a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b)

    // this is fine for small vectors, but it's better to use a HashSet for larger ones
    a.len() == b.len() && a.iter().all(|item| b.contains(item))
}
