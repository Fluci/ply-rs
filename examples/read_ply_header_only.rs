extern crate ply_rs;
use ply_rs as ply;

/// Sometimes only the meta data is interesting to us.
/// Reading the entire ply file would be a waste of ressources.
fn main() {
    // set up a reader, in this a file.
    let path = "example_plys/greg_turk_example1_ok_ascii.ply";
    let f = std::fs::File::open(path).unwrap();

    // Reading a header, requires a reader that provides a way to read single line
    // in read_ply, this conversion happens internally.
    let mut reader = std::io::BufReader::new(f);

    // create a parser
    let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();

    // use the parser: read the entire file
    let ply = p.read_header(&mut reader);

    // make sure it did work
    assert!(ply.is_ok());

    // proof that data has been read
    println!("Read ply data: {:#?}", ply.unwrap());
}
