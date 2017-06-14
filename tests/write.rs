extern crate ply_rs;
use ply_rs::*;
use ply_rs::ply::*;
use std::io::{ Read, BufReader };

fn read_buff<T: Read>(mut buf: &mut T) -> ply::Ply {
    let p = parser::Parser::new();
    let ply = p.read_ply(&mut buf);
    assert!(ply.is_ok(), format!("failed: {}", ply.err().unwrap()));
    ply.unwrap()
}

fn write_buff(ply: &Ply) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    let mut w = writer::Writer::new();
    w.write_ply(&mut buf, ply).unwrap();
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
    new_ply
}

fn create_min() -> Ply {
    Ply::new()
}

fn create_basic_header() -> Ply {
    let mut ply = Ply::new();
    let p = Property::new("x".to_string(), DataType::Int);
    let mut e = Element::new("point".to_string(), 0);
    e.properties.add(p);
    let c = "Hi, I'm your friendly comment.".to_string();
    let oi = "And I'm your object information.".to_string();
    ply.elements.add(e);
    ply.comments.push(c);
    ply.obj_infos.push(oi);
    ply
}

fn create_single_elements() -> Ply {
    let mut ply = Ply::new();

    let mut e = Element::new("point".to_string(), 2);
    let p = Property::new("x".to_string(), DataType::Int);
    e.properties.add(p);
    let p = Property::new("y".to_string(), DataType::UInt);
    e.properties.add(p);

    let mut pe = PayloadElement::new();
    pe.insert("x".to_string(), DataItem::Int(-7));
    pe.insert("y".to_string(), DataItem::UInt(5));
    e.payload.push(pe);
    let mut pe = PayloadElement::new();
    pe.insert("x".to_string(), DataItem::Int(2));
    pe.insert("y".to_string(), DataItem::UInt(4));
    e.payload.push(pe);

    let c = "Hi, I'm your friendly comment.".to_string();
    let oi = "And I'm your object information.".to_string();
    ply.elements.add(e);
    ply.comments.push(c);
    ply.obj_infos.push(oi);

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
