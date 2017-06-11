extern crate linked_hash_map;

mod test_grammar {
    include!(concat!(env!("OUT_DIR"), "/ply_grammar.rs"));
}

use self::test_grammar as grammar;
pub mod parser;
pub mod ply;
