use crate::{stats::helpers::get_token_as_word, zil::node::TokenType as NodeTokenType};

use super::{
    error::ZilErr,
    file_table::format_file_location,
    node::{ZilNode, ZilNodeType},
    token::{word_is_integer, TokenType},
    token_gen::TokenGen,
};

pub struct Tree {
    root: ZilNode,
}

impl Tree {
    pub fn get_root(&self) -> &ZilNode {
        &self.root
    }
}

#[allow(dead_code)]
pub fn print(root: &ZilNode) {
    print_tree_recursive(root, 0);
}

fn print_tree_recursive(root: &ZilNode, offset: u64) {
    let spacer = String::from("  ");
    let mut spaces = String::new();
    for _ in 0..offset {
        spaces.push_str(&spacer);
    }

    for n in root.children.iter() {
        match n.node_type {
            ZilNodeType::Token(_) => {
                println!("{}{}: {}", spaces, n.node_type, n.token_val());
            }
            ZilNodeType::Cluster | ZilNodeType::Group => {
                println!("{}{}", spaces, n.node_type);
                print_tree_recursive(n, offset + 1);
            }
            _ => (),
        }
    }
}

pub fn build_tree<'a>(tokens: &mut impl TokenGen) -> Result<Tree, ZilErr> {
    let mut root = ZilNode::new_no_token(ZilNodeType::Unknown);

    match build_tree_recursively(tokens, &mut root) {
        (_, Some(e)) => return Err(e),
        (_, None) => (),
    };

    match validate_tree(&root) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    swallow_comments(&mut root);

    Ok(Tree { root: root })
}

fn build_tree_recursively<'a>(
    tokens: &'a mut impl TokenGen,
    root: &mut ZilNode,
) -> (Option<TokenType>, Option<ZilErr>) {
    loop {
        let t = match tokens.next() {
            Some(Ok(v)) => v,
            Some(Err(e)) => {
                let msg = format!("Unable to get next token: {}", e);
                return (None, Some(ZilErr::origin(msg)));
            }
            None => return (None, None),
        };

        match t.kind {
            TokenType::LeftArrow => {
                let mut child = ZilNode::new(ZilNodeType::Cluster, t);

                let (token_type, err) = build_tree_recursively(tokens, &mut child);
                if token_type != Some(TokenType::RightArrow) {
                    let msg = format!(
                        "Routine does not end with RightArrow\n{}",
                        format_file_location(tokens)
                    );
                    return (None, Some(ZilErr::origin(msg)));
                } else if err.is_some() {
                    return (None, err);
                }

                root.push_child(child);
            }
            TokenType::LeftParen => {
                let mut child = ZilNode::new(ZilNodeType::Group, t);

                let (token_type, err) = build_tree_recursively(tokens, &mut child);
                if token_type != Some(TokenType::RightParen) {
                    let msg = format!(
                        "Group does not end with RightParen\n{}",
                        format_file_location(tokens)
                    );
                    return (None, Some(ZilErr::origin(msg)));
                } else if err.is_some() {
                    return (None, err);
                }

                root.push_child(child);
            }
            TokenType::RightArrow | TokenType::RightParen => {
                return (Some(t.kind), None);
            }
            TokenType::Text => {
                root.push_child(ZilNode::new(ZilNodeType::Token(NodeTokenType::Text), t));
            }
            TokenType::Word => {
                // determine if word is an integer
                if word_is_integer(&t.value) {
                    root.push_child(ZilNode::new(ZilNodeType::Token(NodeTokenType::Number), t));
                } else {
                    root.push_child(ZilNode::new(ZilNodeType::Token(NodeTokenType::Word), t));
                }
            }
            TokenType::Space => {
                // discard
            }
        }
    }
}

fn validate_tree(root: &ZilNode) -> Result<(), ZilErr> {
    for n in root.children.iter() {
        match n.node_type {
            ZilNodeType::Token(_) => {
                if n.has_children() {
                    let msg = format!("Token node has children: {}", n);
                    return Err(ZilErr::origin(msg));
                }

                if !n.has_token() {
                    let msg = format!("Token node has no token: {}", n);
                    return Err(ZilErr::origin(msg));
                }
            }
            ZilNodeType::Cluster | ZilNodeType::Group => match validate_tree(n) {
                Ok(()) => (),
                Err(e) => return Err(e),
            },
            ZilNodeType::Unknown => {
                let msg = format!("Unknown node: {}", n);
                return Err(ZilErr::origin(msg));
            }
        }
    }

    Ok(())
}

fn swallow_comments(root: &mut ZilNode) {
    let mut comment_indices: Vec<usize> = Vec::new();

    let marker = String::from(";");

    for (i, n) in root.children.iter().enumerate() {
        if n.node_type == ZilNodeType::Token(NodeTokenType::Word)
            && get_token_as_word(n).unwrap_or_default() == marker
        {
            comment_indices.push(i);
        }
    }

    for i in comment_indices.into_iter().rev() {
        if root.children.len() >= i + 2 {
            root.children.remove(i + 1);
        }
        root.children.remove(i);
    }

    for n in root.children.iter_mut() {
        swallow_comments(n);
    }
}
