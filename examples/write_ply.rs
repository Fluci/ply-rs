extern crate ply_rs;
use ply_rs::ply::{ Ply, DefaultElement, Encoding, ElementDef, PropertyDef, PropertyType, ScalarType, Property, Addable };
use ply_rs::writer::{ Writer };

/// Demonstrates simplest use case for reading from a file.
fn main() {
    // set up a target, could also be a file
    let mut buf = Vec::<u8>::new();

    // crete a ply objet
    let mut ply = {
        let mut ply = Ply::<DefaultElement>::new();
        ply.header.encoding = Encoding::Ascii;
        ply.header.comments.push("A beautiful comment!".to_string());

        // Define the elements we want to write. In our case we write a 2D Point.
        // When writing, the `count` will be set automatically to the correct value by calling `make_consistent`
        let mut point_element = ElementDef::new("point".to_string());
        let p = PropertyDef::new("x".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);
        let p = PropertyDef::new("y".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);
        ply.header.elements.add(point_element);

        // Add data
        let mut points = Vec::new();

        // Add first point
        let mut point = DefaultElement::new();
        point.insert("x".to_string(), Property::Float(17.3));
        point.insert("y".to_string(), Property::Float(-23.4));
        points.push(point);

        // Add second point
        let mut point = DefaultElement::new();
        point.insert("x".to_string(), Property::Float(6.28));
        point.insert("y".to_string(), Property::Float(-1.42));
        points.push(point);

        ply.payload.insert("point".to_string(), points);

        // only `write_ply` calls this by itself, for all other methods the client is
        // responsible to make the data structure consistent.
        // We do it here for demonstration purpose.
        ply.make_consistent().unwrap();
        ply
    };

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
