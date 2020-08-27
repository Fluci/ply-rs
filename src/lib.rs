/*#![warn(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]
*/
//! Library for reading/writing ascii and binary PLY files.
//!
//! This library has two goals:
//!
//! - Get you started easily.
//! - Give you enough flexibility to boost performance.
//!
//! Simplicity is provided by giving you high level operations and ready to use data structures:
//!
//! - Read an entire PLY file with `read_ply(reader)`, see the Parser module.
//! - Write an entire PLY with `ẁrite_ply(target, ply)`, se the Writer module.
//! - Don't care about data types: `DefaultElement` is nothing more than a [linked HashMap](https://github.com/contain-rs/linked-hash-map) where you access elements with String keys.
//!
//! Performance can be achieved by using the finer granular methods and your own structs:
//!
//! - `Writer` and `Parser` provide you with methods down to the line/element level for nice things like streaming architectures.
//! - `Ply`, `Writer`, and `Parser` use generics for the element-type. If HashMaps are too slow for you, define your own structs and implement the `PropertyAccess` trait. Data will then be written directly to your target format.

extern crate linked_hash_map;
extern crate byteorder;
extern crate peg;
pub mod parser;
pub mod ply;
pub mod writer;

mod util;
