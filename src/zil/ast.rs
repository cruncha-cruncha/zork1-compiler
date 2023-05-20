use crate::zil::node::TokenBunchType;

use super::{
    error::ZilErr,
    file_table::format_file_location,
    node::{ZilNode, ZilNodeType},
    token::{Token, TokenType},
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

fn print_tree_recursive(root: &ZilNode, depth: u64) {
    let spacer = String::from("  ");
    let mut out = String::new();
    for _ in 0..depth {
        out.push_str(&spacer);
    }

    out.push_str(&format!("{}: ", root.node_type));

    let mut dirty = true;
    for n in root.children.iter() {
        if n.node_type == ZilNodeType::Token {
            if !dirty && n.token.as_ref().unwrap().kind == TokenType::Space {
                continue;
            }

            let val = &n.token_val();
            if n.token.as_ref().unwrap().kind == TokenType::Text {
                out.push_str(&format!("\"{}\"", val));
            } else {
                out.push_str(val);
            }

            dirty = true;
        } else {
            if dirty {
                println!("{}", out);
                dirty = false;
            }

            print_tree_recursive(n, depth + 1);

            out = String::new();
            out.push_str(&spacer);
            for _ in 0..depth {
                out.push_str(&spacer);
            }
        }
    }

    if dirty {
        println!("{}", out);
    }
}

pub fn build_tree<'a>(tokens: &mut impl TokenGen) -> Result<Tree, ZilErr> {
    let mut root = ZilNode::new(ZilNodeType::Unknown);

    match build_tree_recursively(tokens, &mut root) {
        (_, Some(e)) => return Err(e),
        (_, None) => (),
    };

    match validate_tree(&root) {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    root = bunch_tokens(root);

    swallow_comments(&mut root);

    clean_top_level(&mut root);

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
                let mut child = ZilNode::new(ZilNodeType::Cluster);

                let (token_type, err) = build_tree_recursively(tokens, &mut child);
                if token_type != Some(TokenType::RightArrow) {
                    let msg = format!(
                        "Routine does not end with RightArrow\n{}",
                        format_file_location(tokens)
                    );
                    return (None, Some(ZilErr::origin(msg)));
                } else if !err.is_none() {
                    return (None, err);
                }

                /* the trait bound `Box<dyn TokenGen<Item = std::result::Result<Token, std::io::Error>>>: FileTableLocation` is not satisfied
                the trait `FileTableLocation` is not implemented for `Box<dyn TokenGen<Item = std::result::Result<Token, std::io::Error>>> */

                root.push_child(child);
            }
            TokenType::LeftParen => {
                let mut child = ZilNode::new(ZilNodeType::Group);

                let (token_type, err) = build_tree_recursively(tokens, &mut child);
                if token_type != Some(TokenType::RightParen) {
                    let msg = format!(
                        "Group does not end with RightParen\n{}",
                        format_file_location(tokens)
                    );
                    return (None, Some(ZilErr::origin(msg)));
                } else if !err.is_none() {
                    return (None, err);
                }

                root.push_child(child);
            }
            TokenType::RightArrow | TokenType::RightParen => {
                return (Some(t.kind), None);
            }
            TokenType::Symbol => {
                if t.value == ";" {
                    root.push_child(ZilNode::new(ZilNodeType::Comment));
                } else {
                    root.push_child(ZilNode::from_token(t));
                }
            }
            TokenType::Space | TokenType::Text | TokenType::Word => {
                root.push_child(ZilNode::from_token(t));
            }
        }
    }
}

fn validate_tree(root: &ZilNode) -> Result<(), ZilErr> {
    for n in root.children.iter() {
        if n.node_type == ZilNodeType::Token {
            if n.has_children() {
                let msg = format!("Token node has children: {}", n);
                return Err(ZilErr::origin(msg));
            }
            if !n.has_token() {
                let msg = format!("Token node has no token: {}", n);
                return Err(ZilErr::origin(msg));
            }
        } else if n.node_type == ZilNodeType::Unknown {
            let msg = format!("Unknown node: {}", n);
            return Err(ZilErr::origin(msg));
        } else {
            match validate_tree(n) {
                Ok(()) => (),
                Err(e) => return Err(e),
            }
        }
    }

    Ok(())
}

fn clean_top_level(root: &mut ZilNode) {
    let mut indices: Vec<usize> = Vec::new();
    for (i, n) in root.children.iter().enumerate() {
        match n.node_type {
            ZilNodeType::TokenBunch(_) => indices.push(i),
            _ => (),
        }
    }

    for i in indices.iter().rev() {
        root.children.remove(*i);
    }
}

fn swallow_comments(root: &mut ZilNode) {
    let mut comment_indices: Vec<usize> = Vec::new();

    for (i, n) in root.children.iter().enumerate() {
        if n.node_type == ZilNodeType::Comment {
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

pub fn bunch_tokens(mut root: ZilNode) -> ZilNode {
    let mut token_buf: Vec<Token> = Vec::new();
    let mut new_children: Vec<ZilNode> = Vec::new();

    fn bunch_token_buf(token_buf: Vec<Token>, new_children: &mut Vec<ZilNode>) {
        if token_buf.len() == 0 {
            return;
        }

        // validate token_buf, determine type

        let mut has_text = false;
        let mut is_number = true;

        match token_buf[0].kind {
            TokenType::Text => {
                has_text = true;
                is_number = false;
            }
            TokenType::Symbol => {
                if &token_buf[0].value != "-" || token_buf.len() <= 1 {
                    is_number = false;
                }
            }
            TokenType::Word => {
                for c in token_buf[0].value.chars() {
                    if !c.is_digit(10) {
                        is_number = false;
                        break;
                    }
                }
            }
            _ => panic!("Bad token type in ast::bunch_tokens"),
        }

        for t in token_buf.iter().skip(1) {
            if t.kind == TokenType::Text {
                has_text = true;
                is_number = false;
                break;
            }

            if is_number {
                for c in t.value.chars() {
                    if !c.is_digit(10) {
                        is_number = false;
                        break;
                    }
                }
            }
        }

        let mut bunch_type = TokenBunchType::Word;
        if has_text {
            if token_buf.len() > 1 {
                panic!("Text token with other tokens in ast::bunch_tokens");
            }
            bunch_type = TokenBunchType::Text;
        } else if is_number {
            bunch_type = TokenBunchType::Number;
        }

        let mut bunch = ZilNode::new(ZilNodeType::TokenBunch(bunch_type));
        for t in token_buf {
            bunch.push_child(ZilNode::from_token(t));
        }

        new_children.push(bunch);
    }

    for n in root.children {
        match n.node_type {
            ZilNodeType::Unknown => panic!("Unknown node type in stats::lookups::get_name_tokens"),
            ZilNodeType::TokenBunch(_) => {
                panic!("Already bunched in stats::lookups::get_name_tokens")
            }
            ZilNodeType::Comment | ZilNodeType::Group | ZilNodeType::Cluster => {
                bunch_token_buf(token_buf, &mut new_children);
                new_children.push(n);
                token_buf = Vec::new();
            }
            ZilNodeType::Token => {
                let token = n.token.unwrap();
                match token.kind {
                    TokenType::Space => {
                        bunch_token_buf(token_buf, &mut new_children);
                        token_buf = Vec::new();
                    }
                    TokenType::Text | TokenType::Word | TokenType::Symbol => token_buf.push(token),
                    _ => panic!(
                        "Bad token type in stats::lookups::get_name_tokens, {}",
                        token.kind
                    ),
                }
            }
        }
    }

    bunch_token_buf(token_buf, &mut new_children);

    root.children = Vec::new();
    for n in new_children {
        match n.node_type {
            ZilNodeType::TokenBunch(_) => root.push_child(n),
            _ => root.push_child(bunch_tokens(n)),
        }
    }

    root
}
