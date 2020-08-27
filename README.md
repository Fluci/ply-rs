# Ply-rs

[Documentation](https://fluci.github.io/travis_docs/ply-rs/ply_rs/index.html) |
[![Build Status](https://travis-ci.org/Fluci/ply-rs.svg?branch=master)](https://travis-ci.org/Fluci/ply-rs)
[![crates.io](https://img.shields.io/crates/v/ply-rs.svg)](https://crates.io/crates/ply-rs)

Ply-rs is a small library built to read and write the PLY file format (also Polygon File Format, Standford Triangle Format). The library supports all three subformats: ascii, big endian, and little endian.

It focuses on two main points:

- An easy and fast start.
- High performance if you're willing to do some things yourself.

## Getting started

### Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
ply-rs = "0.1.3"
```

Add to your root:

```rust
extern crate ply_rs;

fn main() {}
```

### Read a ply file

This is the easiest way to read a ply file:

```rust,no_run
extern crate ply_rs;
use ply_rs as ply;

/// Demonstrates simplest use case for reading from a file.
fn main() {
    // set up a reader, in this case a file.
    let path = "example_plys/greg_turk_example1_ok_ascii.ply";
    let mut f = std::fs::File::open(path).unwrap();

    // create a parser
    let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();

    // use the parser: read the entire file
    let ply = p.read_ply(&mut f);

    // make sure it did work
    assert!(ply.is_ok());
    let ply = ply.unwrap();

    // proof that data has been read
    println!("Ply header: {:#?}", ply.header);
    println!("Ply data: {:?}", ply.payload);
}

```

### Write ply file

The simplest case of writing a ply file:

```rust
extern crate ply_rs;
use ply_rs::ply::{ Ply, DefaultElement };
use ply_rs::writer::{ Writer };

/// Demonstrates simplest use case for reading from a file.
fn main() {
    // set up a target, could also be a file
    let mut buf = Vec::<u8>::new();

    // crete a ply objet
    let mut ply = Ply::<DefaultElement>::new();

    // set up a writer
    let w = Writer::new();
    let written = w.write_ply(&mut buf, &mut ply).unwrap();
    println!("{} bytes written", written);
    println!("buffer size: {}", buf.len());

    // proof that data has been read

    // We can use `from_utf8` since PLY files only contain ascii characters
    let output = String::from_utf8(buf).unwrap();
    println!("Written data:\n{}", output);
}
```

For more complicated examples, please see the [examples](examples/).

This implementation is mainly based on [these specifications](http://paulbourke.net/dataformats/ply/) with additions from [here](https://people.sc.fsu.edu/%7Ejburkardt/data/ply/ply.txt).
