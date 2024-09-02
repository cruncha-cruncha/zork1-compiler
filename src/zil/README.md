Takes a bunch of zil files, parses them into a tree.

Tree nodes look like:

```
pub struct ZilNode {
    pub node_type: ZilNodeType,
    pub token: Option<Token>, // in general only leaf nodes will have a token
    pub children: Vec<ZilNode>,
}
```

Where ZilNodeType is:

```
pub enum ZilNodeType {
    Unknown,          // only the root node is allowed to have this type
    Cluster,          // < ... >
    Group,            // ( ... )
    Token(TokenType), // leaf node, must have a token
}

pub enum TokenType {
    Text,   // " ... "
    Number, // an integer
    Word,   // anything else
}
```

Technically Cluster and Group can also be leaf nodes if they're empty. Token (as in `Option<Token>`) is:

```
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub file_key: FileKey,
    pub line_number: u64,
    pub char_number: u64,
}

pub enum TokenType {
    LeftArrow,  // <
    RightArrow, // >
    LeftParen,  // (
    RightParen, // )
    Text,       // " ... "
    Space,      // any whitespace outside of text
    Word,       // anything else
}
```

All comments are removed from the tree. All whitespace is removed from the tree. If adjacent children are both TokenType(Word), we can infer that there used to be whitespace between them.
