use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::collections::LinkedList;

use crate::tokens_and_nodes::*;
use crate::tokenizer::*;

pub fn parse (tokens: &mut LinkedList<Token>) -> NodeWrapper {
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
                None => return
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
                                    t.value = format!("{}{}", "\\", t.value);
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

    //println!("{}", stack.len());

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

pub fn clean_tree(mut nw: NodeWrapper) -> Option<NodeWrapper> {
    if nw.is_token() {
        match &nw.borrow_token().name {
            TokenType::Space | TokenType::Nl | TokenType::Quote |
            TokenType::LeftArrow | TokenType::RightArrow |
            TokenType::LeftParen | TokenType::RightParen => {
                return None;
            },
            _ => return Some(nw) 
        }
    }

    if nw.is_node() {
        match nw.data {
            TokenOrNode::Node(n) => {
                match &n.name {
                    NodeType::FullComment => { 
                        nw.data = TokenOrNode::Node(n); 
                    },
                    _ => {
                        let mut recycle = Vec::new();
                        for child_nw in n.children {
                            match clean_tree(child_nw) {
                                Some(r) => recycle.push(r),
                                None => {}
                            }
                        }
                        nw.data = TokenOrNode::Node(Node { name: n.name, children: recycle });
                    }
                }
                
            },
            TokenOrNode::Token(_) => {}
        };
    }

    Some(nw)
}

pub fn read_file_to_tree(input_path: &Path) -> Option<NodeWrapper> {
    let file = match File::open(input_path) {
        Ok(file) => file,
        Err(_) => return None
    };
    let reader = BufReader::new(file);
    let token_map = TokenType::get_map();
    let mut token_list = tokenize(reader, &token_map);
    clean_tree(parse(&mut token_list))
}