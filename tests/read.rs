extern crate ply_rs;
use ply_rs::*;

fn read_file_ok(path: &str) -> ply::Ply {
    let mut f = std::fs::File::open(path).unwrap();
    let mut p = parser::Parser::new();
    let ply = p.read(&mut f);
    assert!(ply.is_ok(), format!("failed: {}", ply.err().unwrap()));
    ply.unwrap()
}

fn read_file_err(path: &str) {
    let mut f = std::fs::File::open(path).unwrap();
    let mut p = parser::Parser::new();
    let ply = p.read(&mut f);
    assert!(ply.is_err(), format!("ply should have failed: {:?}", ply.unwrap()));
}

#[test]
fn read_empty() {
    let ply = read_file_ok("tests/plys/empty_ok_ascii.ply");
    assert_eq!(ply.header.elements["face"].count, 0);
    assert_eq!(ply.payload.len(), 2);
    assert!(ply.payload["vertex"].is_empty());
    assert!(ply.payload["face"].is_empty());
}
#[test]
fn read_header_min() {
    let ply = read_file_ok("tests/plys/header_min_ok_ascii.ply");
    assert_eq!(ply.header.elements.len(), 0);
    assert_eq!(ply.header.obj_infos.len(), 0);
    assert_eq!(ply.header.comments.len(), 0);
    assert_eq!(ply.payload.len(), 0);
}
#[test]
fn read_house() {
    let ply = read_file_ok("tests/plys/house_ok_ascii.ply");
    println!("Created ply: {:?}", ply);
    assert_eq!(ply.header.elements["face"].count, 3);
    assert_eq!(ply.payload.len(), 2);
    assert_eq!(ply.payload["vertex"].len(), 5);
    assert_eq!(ply.payload["face"].len(), 3);
}
#[test]
fn read_greg_turk_1() {
    let ply = read_file_ok("tests/plys/greg_turk_example1_ok_ascii.ply");
    println!("Created ply: {:?}", ply);
}
#[test]
fn read_greg_turk_2() {
    let ply = read_file_ok("tests/plys/greg_turk_example2_ok_ascii.ply");
    println!("Created ply: {:?}", ply);
}
#[test]
fn read_single_elements_err_ascii() {
    read_file_err("tests/plys/single_elements_err_ascii.ply");
}
#[test]
fn read_single_elements_ok_ascii() {
    let ply = read_file_ok("tests/plys/single_elements_ok_ascii.ply");
    println!("Created ply: {:?}", ply);
}
