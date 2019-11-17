use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum TokenType {
    Escape,
    Quote,
    Single,
    LeftArrow,
    RightArrow,
    LeftParen,
    RightParen,
    Space,
    Tab,
    Comment,
    Period,
    Comma,
    Question,
    Percent,
    Word,
    Nl // newline
}

pub fn get_token_type_description(t: &TokenType) -> String {
    match t {
        TokenType::Escape => "ESCAPE".to_string(),
        TokenType::Quote => "QUOTE".to_string(),
        TokenType::Single => "SINGLE".to_string(),
        TokenType::LeftArrow => "LEFT_ARROW".to_string(),
        TokenType::RightArrow => "RIGHT_ARROW".to_string(),
        TokenType::LeftParen => "LEFT_PAREN".to_string(),
        TokenType::RightParen => "RIGHT_PAREN".to_string(),
        TokenType::Space => "SPACE".to_string(),
        TokenType::Tab => "TAB".to_string(),
        TokenType::Comment => "COMMENT".to_string(),
        TokenType::Period => "PERIOD".to_string(),
        TokenType::Comma => "COMMA".to_string(),
        TokenType::Question => "QUESTION".to_string(),
        TokenType::Percent => "PERCENT".to_string(),
        TokenType::Word => "WORD".to_string(),
        TokenType::Nl => "NL".to_string()
    }
}

pub fn get_token_type_map () -> HashMap<char, TokenType> {
    let mut token_map = HashMap::new();

    token_map.insert('\\', TokenType::Escape);
    token_map.insert('"', TokenType::Quote);
    token_map.insert('\'', TokenType::Single);
    token_map.insert('<', TokenType::LeftArrow);
    token_map.insert('>', TokenType::RightArrow);
    token_map.insert('(', TokenType::LeftParen);
    token_map.insert(')', TokenType::RightParen);
    token_map.insert(' ', TokenType::Space);
    token_map.insert('\t', TokenType::Tab);
    token_map.insert(';', TokenType::Comment);
    token_map.insert('.', TokenType::Period);
    token_map.insert(',', TokenType::Comma);
    token_map.insert('?', TokenType::Question);
    token_map.insert('%', TokenType::Percent);

    token_map
}

#[derive(Copy, Clone)]
pub enum NodeType {
    AccessModifier,
    PartWord,
    FullWord,
    CommentBuilder,
    FullComment,
    QuoteBuilder,
    FullQuote,
    PointyBuilder,
    PointyFunc,
    SmoothBuilder,
    SmoothFunc,
    FunkyBunch
}

pub fn get_node_type_description(t: &NodeType) -> String {
    match t {
        NodeType::AccessModifier => "access_modifier".to_string(),
        NodeType::PartWord => "part_word".to_string(),
        NodeType::FullWord => "full_word".to_string(),
        NodeType::CommentBuilder => "comment_builder".to_string(),
        NodeType::FullComment => "full_comment".to_string(),
        NodeType::QuoteBuilder => "quote_builder".to_string(),
        NodeType::FullQuote => "full_quote".to_string(),
        NodeType::PointyBuilder => "pointy_builder".to_string(),
        NodeType::PointyFunc => "pointy_func".to_string(),
        NodeType::SmoothBuilder => "smooth_builder".to_string(),
        NodeType::SmoothFunc => "smooth_func".to_string(),
        NodeType::FunkyBunch => "funky_bunch".to_string()
    }
}

pub enum TokenOrNode {
    Node(Node),
    Token(Token)
}

pub enum TokenOrNodeType {
    Node(NodeType),
    Token(TokenType)
}

trait Describe {
    fn describe(&self, offset: String);
}

pub struct NodeWrapper {
    pub data: TokenOrNode
}

impl NodeWrapper {
    #[allow(dead_code)]
    pub fn is_token (&self) -> bool {
        match self.data {
            TokenOrNode::Node(_) => false,
            TokenOrNode::Token(_) => true
        }
    }

    #[allow(dead_code)]
    pub fn is_node (&self) -> bool {
        match self.data {
            TokenOrNode::Node(_) => true,
            TokenOrNode::Token(_) => false
        }
    }

    pub fn get_type(&self) -> TokenOrNodeType {
        match self.data {
            TokenOrNode::Node(ref n) => TokenOrNodeType::Node(n.name),
            TokenOrNode::Token(ref t) => TokenOrNodeType::Token(t.name)
        }
    }

    pub fn add_child(&mut self, node_wrapper: NodeWrapper) {
        match self.data {
            TokenOrNode::Node(ref mut n) => {
                n.add_node_wrapper(node_wrapper);
            },
            TokenOrNode::Token(_) => panic!()
        };
    }

    pub fn prepend_child(&mut self, node_wrapper: NodeWrapper) {
        match self.data {
            TokenOrNode::Node(ref mut n) => {
                n.prepend_node_wrapper(node_wrapper);
            },
            TokenOrNode::Token(_) => panic!()
        };
    }
}

impl Describe for NodeWrapper {
    fn describe(&self, offset: String) {
        match self.data {
            TokenOrNode::Node(ref n) => n.describe(offset),
            TokenOrNode::Token(ref t) => t.describe(offset)
        }
    }
}

pub struct Node {
    pub name: NodeType,
    pub children: Vec<NodeWrapper>
    // cannot have a parent ref, as that would be circular
    // could implement using unsafe pointers?
    // or use some sort of id lookup table thingy?
}

impl Node {
    pub fn add_node_wrapper (&mut self, node_wrapper: NodeWrapper) {
        self.children.push(node_wrapper)
    }

    pub fn prepend_node_wrapper (&mut self, node_wrapper: NodeWrapper) {
        self.children.insert(0, node_wrapper)
    }
}

impl Describe for Node {
    fn describe(&self, mut offset: String) {
        println!("{}{}", offset, get_node_type_description(&self.name));
        offset.push_str("  ");  
        for nw in &self.children {
            nw.describe(offset.clone());
        }
    }
}

pub struct Token {
    pub name: TokenType,
    pub value: String
}

impl Describe for Token {
    fn describe(&self, offset: String) {
        println!("{}{}, {}", offset, get_token_type_description(&self.name), &self.value);
    }
}