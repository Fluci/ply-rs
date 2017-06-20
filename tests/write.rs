extern crate ply_rs;
use ply_rs::*;
use ply_rs::ply::*;
use std::io::{ Read, BufReader };

type Ply = ply::Ply<ply::DefaultElement>;

fn read_buff<T: Read>(mut buf: &mut T) -> Ply {
    let p = parser::Parser::new();
    let ply = p.read_ply(&mut buf);
    assert!(ply.is_ok(), format!("failed: {}", ply.err().unwrap()));
    ply.unwrap()
}

fn write_buff(ply: &Ply) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    let w = writer::Writer::new();
    w.write_ply_unchecked(&mut buf, ply).unwrap();
    buf
}

fn read_write_ply(ply: &Ply) -> Ply {
    println!("writing ply:\n{:?}", ply);
    let ve : Vec<u8> = write_buff(&ply);
    let txt = String::from_utf8(ve.clone()).unwrap();
    println!("written ply:\n{}", txt);
    let mut buff = BufReader::new(&(*ve));
    let new_ply = read_buff(&mut buff);
    println!("read ply:\n{:?}", new_ply);
    assert_eq!(ply.header, new_ply.header);
    assert_eq!(ply.payload, new_ply.payload);
    new_ply
}

fn create_min() -> Ply {
    let mut ply = Ply::new();
    assert!(ply.make_consistent().is_ok());
    ply
}

fn create_basic_header() -> Ply {
    let mut ply = Ply::new();
    let p = PropertyDef::new("x".to_string(), PropertyType::Scalar(ScalarType::Int));
    let mut e = ElementDef::new("point".to_string());
    e.properties.add(p);
    let c = "Hi, I'm your friendly comment.".to_string();
    let oi = "And I'm your object information.".to_string();
    ply.header.elements.add(e);
    ply.header.comments.push(c);
    ply.header.obj_infos.push(oi);
    assert!(ply.make_consistent().is_ok());
    ply
}

fn create_single_elements() -> Ply {
    let mut ply = Ply::new();

    let mut e = ElementDef::new("point".to_string());
    let p = PropertyDef::new("x".to_string(), PropertyType::Scalar(ScalarType::Int));
    e.properties.add(p);
    let p = PropertyDef::new("y".to_string(), PropertyType::Scalar(ScalarType::UInt));
    e.properties.add(p);

    let mut list = Vec::new();
    let mut pe = KeyMap::new();
    pe.insert("x".to_string(), Property::Int(-7));
    pe.insert("y".to_string(), Property::UInt(5));
    list.push(pe);
    let mut pe = KeyMap::new();
    pe.insert("x".to_string(), Property::Int(2));
    pe.insert("y".to_string(), Property::UInt(4));
    list.push(pe);
    ply.payload.insert("point".to_string(), list);

    let c = "Hi, I'm your friendly comment.".to_string();
    let oi = "And I'm your object information.".to_string();
    ply.header.elements.add(e);
    ply.header.comments.push(c);
    ply.header.obj_infos.push(oi);
    assert!(ply.make_consistent().is_ok());
    ply
}
fn create_list_elements() -> Ply {
    let mut ply = Ply::new();

    let mut e = ElementDef::new("aList".to_string());
    let p = PropertyDef::new("x".to_string(), PropertyType::List(ScalarType::Int, ScalarType::Int));
    e.properties.add(p);

    let mut list = Vec::new();
    let mut pe = KeyMap::new();
    pe.insert("x".to_string(), Property::ListInt(vec![-7, 17, 38]));
    list.push(pe);
    let mut pe = KeyMap::new();
    pe.insert("x".to_string(), Property::ListInt(vec![13, -19, 8, 33]));
    list.push(pe);
    ply.payload.insert("aList".to_string(), list);

    let c = "Hi, I'm your friendly comment.".to_string();
    let oi = "And I'm your object information.".to_string();
    ply.header.elements.add(e);
    ply.header.comments.push(c);
    ply.header.obj_infos.push(oi);
    assert!(ply.make_consistent().is_ok());
    ply
}

#[test]
fn write_header_min() {
    let ply = create_min();
    let new_ply = read_write_ply(&ply);
    assert_eq!(ply, new_ply);
}
#[test]
fn write_basic_header() {
    let ply = create_basic_header();
    let new_ply = read_write_ply(&ply);
    assert_eq!(ply, new_ply);
}
#[test]
fn write_single_elements() {
    let ply = create_single_elements();
    let new_ply = read_write_ply(&ply);
    assert_eq!(ply, new_ply);
}
#[test]
fn write_list_elements() {
    let ply = create_list_elements();
    let new_ply = read_write_ply(&ply);
    assert_eq!(ply, new_ply);
}
