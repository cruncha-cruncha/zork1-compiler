use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::LinkedList;

#[derive(Copy, Clone)]
enum TokenType {
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

fn get_token_type_description(t: &TokenType) -> String {
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

fn get_token_type_map () -> std::collections::HashMap<char, TokenType> {
    let mut token_map = std::collections::HashMap::new();

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
enum NodeType {
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

fn get_node_type_description(t: &NodeType) -> String {
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

enum TokenOrNode {
    Node(Node),
    Token(Token)
}

enum TokenOrNodeType {
    Node(NodeType),
    Token(TokenType)
}

trait Describe {
    fn describe(&self, offset: String);
}

struct NodeWrapper {
    data: TokenOrNode
}

impl NodeWrapper {
    #[allow(dead_code)]
    fn is_token (&self) -> bool {
        match self.data {
            TokenOrNode::Node(_) => false,
            TokenOrNode::Token(_) => true
        }
    }

    #[allow(dead_code)]
    fn is_node (&self) -> bool {
        match self.data {
            TokenOrNode::Node(_) => true,
            TokenOrNode::Token(_) => false
        }
    }

    fn get_type(&self) -> TokenOrNodeType {
        match self.data {
            TokenOrNode::Node(ref n) => TokenOrNodeType::Node(n.name),
            TokenOrNode::Token(ref t) => TokenOrNodeType::Token(t.name)
        }
    }

    fn add_child(&mut self, node_wrapper: NodeWrapper) {
        match self.data {
            TokenOrNode::Node(ref mut n) => {
                n.add_node_wrapper(node_wrapper);
            },
            TokenOrNode::Token(_) => panic!()
        };
    }

    fn prepend_child(&mut self, node_wrapper: NodeWrapper) {
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

struct Node {
    name: NodeType,
    children: Vec<NodeWrapper>
    // cannot have a parent ref, as that would be circular
    // could implement using unsafe pointers?
    // or use some sort of id lookup table thingy?
}

impl Node {
    fn add_node_wrapper (&mut self, node_wrapper: NodeWrapper) {
        self.children.push(node_wrapper)
    }

    fn prepend_node_wrapper (&mut self, node_wrapper: NodeWrapper) {
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

struct Token {
    name: TokenType,
    value: String
}

impl Describe for Token {
    fn describe(&self, offset: String) {
        println!("{}{}, {}", offset, get_token_type_description(&self.name), &self.value);
    }
}

fn tokenize (reader: BufReader<File>, token_names: &std::collections::HashMap<char, TokenType>) -> LinkedList<Token> {
    let mut out: LinkedList<Token> = LinkedList::new();
    let mut tmp_string = String::new();
    for line in reader.lines() {
        for c in line.unwrap().trim().chars() {
            match token_names.get(&c) {
                Some(n) => {
                    if tmp_string.len() > 0 {
                        out.push_back(Token {name: TokenType::Word, value: tmp_string.clone()});
                        tmp_string.clear();
                    }
                    out.push_back(Token {name: *n, value: c.to_string()});
                },
                None => {
                    tmp_string.push(c);
                }
            }
        }
        if tmp_string.len() > 0 {
            out.push_back(Token {name: TokenType::Word, value: tmp_string.clone()});
            tmp_string.clear();
        }
        out.push_back(Token {name: TokenType::Nl, value: "\n".to_string()});
    }

    out
}

fn parse (tokens: &mut LinkedList<Token>) -> NodeWrapper {
    let mut stack: Vec<NodeWrapper> = Vec::new();

    // stack = Vec<Box<NodeWrapper>>
    // NodeWrapper {data: TokenOrNode}
    // TokenOrNode = [Node(Node), Token(Token)]
    // Node {name: NodeType, children: Vec<NodeWrapper>}
    // Token {name: TokenType, value: String}

    fn parse_tokens (tokens: &mut LinkedList<Token>, stack: &mut Vec<NodeWrapper>) {
        let mut i: usize = 0;
        while tokens.len() > 0 {
            let t = match tokens.pop_front() {
                Some(t) => t,
                None => panic!()
            };
            stack.push(NodeWrapper{data: TokenOrNode::Token(t)});
            i = parse_stack(stack, i);
        }

        fn parse_stack (stack: &mut Vec<NodeWrapper>, mut i: usize) -> usize {
            let mut matched_rule = false;
            while stack.len() > 1 {
                if i >= stack.len() {
                    i = 0;
                    matched_rule = false;
                } else if matched_rule {
                    i = if i < 3 { 0 } else { i - 3 };
                }
                while i < stack.len() {
                    let t0 = stack[i].get_type(); 
                    let t1 = if i + 1 < stack.len() { Some(stack[i+1].get_type()) } else { None };

                    // match rules
                    match (t0, t1) {
                        // ESCAPE + [any basic token] -> WORD
                        (TokenOrNodeType::Token(TokenType::Escape), Some(TokenOrNodeType::Token(_))) => {
                            match stack[i+1].data {
                                TokenOrNode::Node(_) => panic!(),
                                TokenOrNode::Token(ref mut t) => {
                                    t.name = TokenType::Word;
                                }
                            }
                            stack.remove(i);
                            matched_rule = true;
                            break;
                        },
                        // TAB -> SPACE
                        (TokenOrNodeType::Token(TokenType::Tab), _) => {
                            match stack[i].data {
                                TokenOrNode::Node(_) => panic!(),
                                TokenOrNode::Token(ref mut t) => {
                                    t.name = TokenType::Space;
                                }
                            }
                            matched_rule = true;
                            break;
                        },
                        // SPACE + SPACE -> SPACE
                        (TokenOrNodeType::Token(TokenType::Space), Some(TokenOrNodeType::Token(TokenType::Space))) => {
                            stack.remove(i);
                            matched_rule = true;
                            break;
                        },
                        // SPACE + NL -> NL
                        (TokenOrNodeType::Token(TokenType::Space), Some(TokenOrNodeType::Token(TokenType::Nl))) => {
                            stack.remove(i);
                            matched_rule = true;
                            break;
                        },
                        // NL + SPACE -> NL
                        (TokenOrNodeType::Token(TokenType::Nl), Some(TokenOrNodeType::Token(TokenType::Space))) => {
                            stack.remove(i+1);
                            matched_rule = true;
                            break;
                        },
                        // NL + NL -> NL
                        (TokenOrNodeType::Token(TokenType::Nl), Some(TokenOrNodeType::Token(TokenType::Nl))) => {
                            stack.remove(i);
                            matched_rule = true;
                            break;
                        },
                        // [SINGLE, COMMA, PERIOD, PERCENT] -> access_modifier
                        (TokenOrNodeType::Token(TokenType::Single), _) |
                        (TokenOrNodeType::Token(TokenType::Comma), _) |
                        (TokenOrNodeType::Token(TokenType::Period), _) |
                        (TokenOrNodeType::Token(TokenType::Percent), _) => {
                            let tmp = std::mem::replace(&mut stack[i],
                                    NodeWrapper{
                                        data: TokenOrNode::Node(
                                            Node {
                                                name: NodeType::AccessModifier,
                                                children: Vec::new()
                                })});

                            let tmp = match tmp.data {
                                TokenOrNode::Node(_) => panic!(),
                                TokenOrNode::Token(t) => t
                            };

                            stack[i].add_child(
                                NodeWrapper { data: TokenOrNode::Token(tmp)}
                            );

                            matched_rule = true;
                            break;
                        },
                        // access_modifier + [SINGLE, COMMA, PERIOD, PERCENT, SPACE] -> access_modifier
                        (TokenOrNodeType::Node(NodeType::AccessModifier), Some(TokenOrNodeType::Token(TokenType::Single))) |
                        (TokenOrNodeType::Node(NodeType::AccessModifier), Some(TokenOrNodeType::Token(TokenType::Comma))) |
                        (TokenOrNodeType::Node(NodeType::AccessModifier), Some(TokenOrNodeType::Token(TokenType::Period))) |
                        (TokenOrNodeType::Node(NodeType::AccessModifier), Some(TokenOrNodeType::Token(TokenType::Percent))) |
                        (TokenOrNodeType::Node(NodeType::AccessModifier), Some(TokenOrNodeType::Token(TokenType::Space))) => {
                            let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // WORD -> part_word
                        (TokenOrNodeType::Token(TokenType::Word), _) => {
                            let tmp = std::mem::replace(&mut stack[i],
                                    NodeWrapper{
                                        data: TokenOrNode::Node(
                                            Node {
                                                name: NodeType::PartWord,
                                                children: Vec::new()
                                })});

                            let tmp = match tmp.data {
                                TokenOrNode::Node(_) => panic!(),
                                TokenOrNode::Token(t) => t
                            };

                            stack[i].add_child(
                                NodeWrapper { data: TokenOrNode::Token(tmp)}
                            );

                            matched_rule = true;
                            break;
                        },
                        // access_modifier + WORD -> part_word
                        (TokenOrNodeType::Node(NodeType::AccessModifier), Some(TokenOrNodeType::Token(TokenType::Word))) => {
                            let tmp1 = std::mem::replace(&mut stack[i],
                                    NodeWrapper{
                                        data: TokenOrNode::Node(
                                            Node {
                                                name: NodeType::PartWord,
                                                children: Vec::new()
                                })});
                            let tmp2 = stack.remove(i+1);

                            stack[i].add_child(tmp1);
                            stack[i].add_child(tmp2);

                            matched_rule = true;
                            break;
                        }
                        // access_modifier + [pointy_func, smooth_func] -> [pointy_func, smooth_func]
                        (TokenOrNodeType::Node(NodeType::AccessModifier), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Node(NodeType::AccessModifier), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) => {
                            let tmp = stack.remove(i);
                            stack[i].prepend_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // part_word + [QUESTION, WORD] -> part_word
                        (TokenOrNodeType::Node(NodeType::PartWord), Some(TokenOrNodeType::Token(TokenType::Question))) |
                        (TokenOrNodeType::Node(NodeType::PartWord), Some(TokenOrNodeType::Token(TokenType::Word))) => {
                            let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // part_word + ESCAPE -> (do nothing)
                        (TokenOrNodeType::Node(NodeType::PartWord), Some(TokenOrNodeType::Token(TokenType::Escape))) => {},
                        // part_word + None -> (do nothing)
                        (TokenOrNodeType::Node(NodeType::PartWord), None) => {},
                        // part_word -> full_word
                        (TokenOrNodeType::Node(NodeType::PartWord), _) => {
                            match stack[i].data {
                                TokenOrNode::Node(ref mut n) => {
                                    n.name = NodeType::FullWord;
                                },
                                TokenOrNode::Token(_) => panic!()
                            }
                            matched_rule = true;
                            break;
                        },
                        // COMMENT -> comment_builder
                        (TokenOrNodeType::Token(TokenType::Comment), _) => {
                            let tmp = std::mem::replace(&mut stack[i],
                                    NodeWrapper{
                                        data: TokenOrNode::Node(
                                            Node {
                                                name: NodeType::CommentBuilder,
                                                children: Vec::new()
                                })});

                            let tmp = match tmp.data {
                                TokenOrNode::Node(_) => panic!(),
                                TokenOrNode::Token(t) => t
                            };

                            stack[i].add_child(
                                NodeWrapper { data: TokenOrNode::Token(tmp)}
                            );

                            matched_rule = true;
                            break;
                        },
                        // QUOTE -> quote_builder
                        (TokenOrNodeType::Token(TokenType::Quote), _) => {
                            let tmp = std::mem::replace(&mut stack[i],
                                    NodeWrapper{
                                        data: TokenOrNode::Node(
                                            Node {
                                                name: NodeType::QuoteBuilder,
                                                children: Vec::new()
                                })});

                            let tmp = match tmp.data {
                                TokenOrNode::Node(_) => panic!(),
                                TokenOrNode::Token(t) => t
                            };

                            stack[i].add_child(
                                NodeWrapper { data: TokenOrNode::Token(tmp)}
                            );

                            matched_rule = true;
                            break;
                        },
                        // LEFT_ARROW -> pointy_builder
                        (TokenOrNodeType::Token(TokenType::LeftArrow), _) => {
                            let tmp = std::mem::replace(&mut stack[i],
                                    NodeWrapper{
                                        data: TokenOrNode::Node(
                                            Node {
                                                name: NodeType::PointyBuilder,
                                                children: Vec::new()
                                })});

                            let tmp = match tmp.data {
                                TokenOrNode::Node(_) => panic!(),
                                TokenOrNode::Token(t) => t
                            };

                            stack[i].add_child(
                                NodeWrapper { data: TokenOrNode::Token(tmp)}
                            );

                            matched_rule = true;
                            break;
                        },
                        // LEFT_PAREN -> smooth_builder
                        (TokenOrNodeType::Token(TokenType::LeftParen), _) => {
                            let tmp = std::mem::replace(&mut stack[i],
                                    NodeWrapper{
                                        data: TokenOrNode::Node(
                                            Node {
                                                name: NodeType::SmoothBuilder,
                                                children: Vec::new()
                                })});

                            let tmp = match tmp.data {
                                TokenOrNode::Node(_) => panic!(),
                                TokenOrNode::Token(t) => t
                            };

                            stack[i].add_child(
                                NodeWrapper { data: TokenOrNode::Token(tmp)}
                            );

                            matched_rule = true;
                            break;
                        },
                        // comment_builder + SPACE -> comment_builder
                        (TokenOrNodeType::Node(NodeType::CommentBuilder), Some(TokenOrNodeType::Token(TokenType::Space))) => {
                            let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // comment_builder + [full_word, full_comment, full_quote, pointy_func, smooth_func, NL] -> full_comment
                        (TokenOrNodeType::Node(NodeType::CommentBuilder), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Node(NodeType::CommentBuilder), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Node(NodeType::CommentBuilder), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Node(NodeType::CommentBuilder), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Node(NodeType::CommentBuilder), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) |
                        (TokenOrNodeType::Node(NodeType::CommentBuilder), Some(TokenOrNodeType::Token(TokenType::Nl))) => {
                            match stack[i].data {
                                TokenOrNode::Node(ref mut n) => {
                                    n.name = NodeType::FullComment;
                                },
                                TokenOrNode::Token(_) => panic!()
                            }
                            let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        /*
                        // comment_builder + None -> full_comment
                        (TokenOrNodeType::Node(NodeType::CommentBuilder), None) => {
                            match stack[i].data {
                                TokenOrNode::Node(ref mut n) => {
                                    n.name = NodeType::FullComment;
                                },
                                TokenOrNode::Token(_) => panic!()
                            }
                            matched_rule = true;
                            break;
                        }
                        */
                        // quote_builder + QUOTE -> full_quote
                        (TokenOrNodeType::Node(NodeType::QuoteBuilder), Some(TokenOrNodeType::Token(TokenType::Quote))) => {
                            match stack[i].data {
                                TokenOrNode::Node(ref mut n) => {
                                    n.name = NodeType::FullQuote;
                                },
                                TokenOrNode::Token(_) => panic!()
                            }
                            let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // quote_builder + ESCAPE -> (do nothing)
                        (TokenOrNodeType::Node(NodeType::QuoteBuilder), Some(TokenOrNodeType::Token(TokenType::Escape))) => {},
                        // quote_builder + [anything] -> quote_builder
                        (TokenOrNodeType::Node(NodeType::QuoteBuilder), Some(_)) => {
                            let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // pointy_builder + RIGHT_ARROW -> pointy_func
                        (TokenOrNodeType::Node(NodeType::PointyBuilder), Some(TokenOrNodeType::Token(TokenType::RightArrow))) => {
                            match stack[i].data {
                                TokenOrNode::Node(ref mut n) => {
                                    n.name = NodeType::PointyFunc;
                                },
                                TokenOrNode::Token(_) => panic!()
                            }
                            let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // pointy_builder + [SPACE, NL, full_word, full_comment, full_quote, pointy_func, smooth_func] -> pointy_builder
                        (TokenOrNodeType::Node(NodeType::PointyBuilder), Some(TokenOrNodeType::Token(TokenType::Space))) |
                        (TokenOrNodeType::Node(NodeType::PointyBuilder), Some(TokenOrNodeType::Token(TokenType::Nl))) |
                        (TokenOrNodeType::Node(NodeType::PointyBuilder), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Node(NodeType::PointyBuilder), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Node(NodeType::PointyBuilder), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Node(NodeType::PointyBuilder), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Node(NodeType::PointyBuilder), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) => {
                           let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // smooth_builder + RIGHT_PAREN -> smooth_func
                        (TokenOrNodeType::Node(NodeType::SmoothBuilder), Some(TokenOrNodeType::Token(TokenType::RightParen))) => {
                            match stack[i].data {
                                TokenOrNode::Node(ref mut n) => {
                                    n.name = NodeType::SmoothFunc;
                                },
                                TokenOrNode::Token(_) => panic!()
                            }
                            let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // smooth_builder + [SPACE, NL, full_word, full_comment, full_quote, pointy_func, smooth_func] -> smooth_builder
                        (TokenOrNodeType::Node(NodeType::SmoothBuilder), Some(TokenOrNodeType::Token(TokenType::Space))) |
                        (TokenOrNodeType::Node(NodeType::SmoothBuilder), Some(TokenOrNodeType::Token(TokenType::Nl))) |
                        (TokenOrNodeType::Node(NodeType::SmoothBuilder), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Node(NodeType::SmoothBuilder), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Node(NodeType::SmoothBuilder), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Node(NodeType::SmoothBuilder), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Node(NodeType::SmoothBuilder), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) => {
                           let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // funky_bunch + [SPACE, NL, full_word, full_comment, full_quote, pointy_func, smooth_func] -> funky_bunch
                        (TokenOrNodeType::Node(NodeType::FunkyBunch), Some(TokenOrNodeType::Token(TokenType::Space))) |
                        (TokenOrNodeType::Node(NodeType::FunkyBunch), Some(TokenOrNodeType::Token(TokenType::Nl))) |
                        (TokenOrNodeType::Node(NodeType::FunkyBunch), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Node(NodeType::FunkyBunch), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Node(NodeType::FunkyBunch), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Node(NodeType::FunkyBunch), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Node(NodeType::FunkyBunch), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) => {
                           let tmp = stack.remove(i+1);
                            stack[i].add_child(tmp);
                            matched_rule = true;
                            break;
                        },
                        // # [SPACE, NL, full_word, full_comment, full_quote, pointy_func, smooth_func] + [SPACE, NL, full_word, full_comment, full_quote, pointy_func, smooth_func] -> funky_bunch
                        (TokenOrNodeType::Token(TokenType::Space), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Token(TokenType::Space), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Token(TokenType::Space), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Token(TokenType::Space), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Token(TokenType::Space), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) |

                        (TokenOrNodeType::Token(TokenType::Nl), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Token(TokenType::Nl), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Token(TokenType::Nl), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Token(TokenType::Nl), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Token(TokenType::Nl), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) |

                        (TokenOrNodeType::Node(NodeType::FullWord), Some(TokenOrNodeType::Token(TokenType::Space))) |
                        (TokenOrNodeType::Node(NodeType::FullWord), Some(TokenOrNodeType::Token(TokenType::Nl))) |
                        (TokenOrNodeType::Node(NodeType::FullWord), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Node(NodeType::FullWord), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Node(NodeType::FullWord), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Node(NodeType::FullWord), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Node(NodeType::FullWord), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) |

                        (TokenOrNodeType::Node(NodeType::FullComment), Some(TokenOrNodeType::Token(TokenType::Space))) |
                        (TokenOrNodeType::Node(NodeType::FullComment), Some(TokenOrNodeType::Token(TokenType::Nl))) |
                        (TokenOrNodeType::Node(NodeType::FullComment), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Node(NodeType::FullComment), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Node(NodeType::FullComment), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Node(NodeType::FullComment), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Node(NodeType::FullComment), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) |

                        (TokenOrNodeType::Node(NodeType::FullQuote), Some(TokenOrNodeType::Token(TokenType::Space))) |
                        (TokenOrNodeType::Node(NodeType::FullQuote), Some(TokenOrNodeType::Token(TokenType::Nl))) |
                        (TokenOrNodeType::Node(NodeType::FullQuote), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Node(NodeType::FullQuote), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Node(NodeType::FullQuote), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Node(NodeType::FullQuote), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Node(NodeType::FullQuote), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) |

                        (TokenOrNodeType::Node(NodeType::PointyFunc), Some(TokenOrNodeType::Token(TokenType::Space))) |
                        (TokenOrNodeType::Node(NodeType::PointyFunc), Some(TokenOrNodeType::Token(TokenType::Nl))) |
                        (TokenOrNodeType::Node(NodeType::PointyFunc), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Node(NodeType::PointyFunc), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Node(NodeType::PointyFunc), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Node(NodeType::PointyFunc), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Node(NodeType::PointyFunc), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) |

                        (TokenOrNodeType::Node(NodeType::SmoothFunc), Some(TokenOrNodeType::Token(TokenType::Space))) |
                        (TokenOrNodeType::Node(NodeType::SmoothFunc), Some(TokenOrNodeType::Token(TokenType::Nl))) |
                        (TokenOrNodeType::Node(NodeType::SmoothFunc), Some(TokenOrNodeType::Node(NodeType::FullWord))) |
                        (TokenOrNodeType::Node(NodeType::SmoothFunc), Some(TokenOrNodeType::Node(NodeType::FullComment))) |
                        (TokenOrNodeType::Node(NodeType::SmoothFunc), Some(TokenOrNodeType::Node(NodeType::FullQuote))) |
                        (TokenOrNodeType::Node(NodeType::SmoothFunc), Some(TokenOrNodeType::Node(NodeType::PointyFunc))) |
                        (TokenOrNodeType::Node(NodeType::SmoothFunc), Some(TokenOrNodeType::Node(NodeType::SmoothFunc))) => {
                            let tmp1 = std::mem::replace(&mut stack[i],
                                    NodeWrapper{
                                        data: TokenOrNode::Node(
                                            Node {
                                                name: NodeType::FunkyBunch,
                                                children: Vec::new()
                                })});
                            let tmp2 = stack.remove(i+1);

                            stack[i].add_child(tmp1);
                            stack[i].add_child(tmp2);

                            matched_rule = true;
                            break;
                        }
                        (_, _) => {}
                    }
                    i += 1;
                }
                if !matched_rule {
                    i = if i < 3 { 0 } else { i - 3 };
                    return i;
                }
            }
            0
        }
    }

    parse_tokens(tokens, &mut stack);

    println!("{}", stack.len());

    /*
    for s in &stack {
        s.describe(String::new());
    }
    */

    if stack.len() > 1 {
        panic!();
    }

    match stack.pop() {
        Some(nw) => nw,
        None => panic!()
    }
}

fn main() {
    let token_map = get_token_type_map();
    
    let input_path = Path::new(".").join("edited-zork").join("test.txt");
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(_) => panic!("could not open file")
    };

    let reader = BufReader::new(file);

    let mut token_list = tokenize(reader, &token_map);

    /*
    for t in &token_list {
        println!("{}", t.value);  
    }
    */

    let root = parse(&mut token_list);

    //root.describe(String::new());

    // https://doc.rust-lang.org/1.30.0/book/second-edition/ch16-00-concurrency.html
}

