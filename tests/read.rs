extern crate ply_rs;
use ply_rs::*;

#[test]
fn read_empty() {
    let mut f = std::fs::File::open("tests/plys/empty_ascii.ply").unwrap();
    let mut p = parser::Parser::new();
    let ply = p.read(&mut f).unwrap();
    assert_eq!(ply.header.elements["face"].count, 0);
    assert_eq!(ply.payload.len(), 2);
    assert!(ply.payload["vertex"].is_empty());
    assert!(ply.payload["face"].is_empty());
}
#[test]
fn read_house() {
    let mut f = std::fs::File::open("tests/plys/house_ascii.ply").unwrap();
    let mut p = parser::Parser::new();
    let ply = p.read(&mut f);
    assert!(ply.is_ok(), format!("failed: {}", ply.err().unwrap()));
    let ply = ply.unwrap();
    println!("Created ply: {:?}", ply);
    assert_eq!(ply.header.elements["face"].count, 3);
    assert_eq!(ply.payload.len(), 2);
    assert_eq!(ply.payload["vertex"].len(), 5);
    assert_eq!(ply.payload["face"].len(), 3);
}
