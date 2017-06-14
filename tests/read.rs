extern crate ply_rs;
use ply_rs::*;

type Ply = ply::Ply<ply::DefaultElementType>;

fn read_file(path: &str) -> Ply {
    let mut f = std::fs::File::open(path).unwrap();
    let p = parser::Parser::new();
    let ply = p.read_ply(&mut f);
    assert!(ply.is_ok(), format!("failed: {}", ply.err().unwrap()));
    ply.unwrap()
}

#[test]
fn read_empty() {
    let ply = read_file("example_plys/empty_ok_ascii.ply");
    assert_eq!(ply.elements["face"].header.count, 0);
    assert!(ply.elements["vertex"].payload.is_empty());
    assert!(ply.elements["face"].payload.is_empty());
}
#[test]
fn read_house() {
    let ply = read_file("example_plys/house_ok_ascii.ply");
    println!("Created ply: {:?}", ply);
    assert_eq!(ply.elements["face"].header.count, 3);
    assert_eq!(ply.elements["vertex"].payload.len(), 5);
    assert_eq!(ply.elements["face"].payload.len(), 3);
}
#[test]
fn read_greg_turk_1() {
    let ply = read_file("example_plys/greg_turk_example1_ok_ascii.ply");
    println!("Created ply: {:?}", ply);
}
#[test]
fn read_greg_turk_2() {
    let ply = read_file("example_plys/greg_turk_example2_ok_ascii.ply");
    println!("Created ply: {:?}", ply);
}
