Takes ownership of a tree and breaks it down. Every file (and corresponding struct) in ./top_level implements Populator, and may implement Codex.

```
pub trait Populator {
    fn add_node(&mut self, node: ZilNode);
    fn populate(&mut self) -> Result<(), String>;
    fn validate(&self, cross_ref: &CrossRef) -> Result<(), String>;
}

pub trait Codex {
    fn lookup(&self, word: &str) -> Option<&ZilNode>;
}
```

In practice, all structs that implement Populator have a 'basis': read-only tree Nodes from which all other information is derived. There are nine constructs in zil that can only appear at the top level (excluding comments):

- GLOBAL
- CONSTANT
- DIRECTIONS
- ROOM
- OBJECT
- ROUTINE
- BUZZ
- SYNONYM
- SYNTAX

In code they look like `<GLOBAL ... >` or `<CONSTANT ...>` etc. Each type of construct generally has it's own syntax. A CrossRef struct (in the cross_ref file) orchestrates all top level structs:

1. iterate through the first layer of the tree (root children), add each node to the appropriate struct
2. call populate() on each struct. This checks basic syntax validity and populates lookup tables
3. call validate() on each struct. This verifies that cross-struct dependencies exist (an object's room exists, a synonym's base syntax exists, etc.). This does not pre-empt every run-time error.

Specific debug information (file and line number) may be lost during this process. There should be enough context left to write error messages using just variable/field/key names.

The weaver.rs file defines a simple thread controller for multi-threading.

## Any Level

Files in the ./any_level folder are specifically for validating clusters. After the top-level constructs have been parsed and validated, it's time to dive deeper into the routines (aka clusters). This process is orchestrated by the Validator (in validate_recursive).
