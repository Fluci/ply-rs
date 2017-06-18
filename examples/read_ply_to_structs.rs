extern crate ply_rs;
use ply_rs::ply;
use ply_rs::parser;


/// We know, what data we want to read, hence we can be more efficient by loading the data into structs.
#[derive(Debug)] // not necessary for parsing, only for println at end of example.
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}


#[derive(Debug)]
struct Face {
    vertex_index: Vec<i32>,
}


// The structs need to implement the PropertyAccess trait, otherwise the parser doesn't know how to write to them.
// Most functions have default, hence you only need to implement, what you expect to need.

impl ply::PropertyAccess for Vertex {
    fn new() -> Self {
        Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    fn set_property(&mut self, key: String, property: ply::Property) {
        match (key.as_ref(), property) {
            ("x", ply::Property::Float(v)) => self.x = v,
            ("y", ply::Property::Float(v)) => self.y = v,
            ("z", ply::Property::Float(v)) => self.z = v,
            (k, _) => panic!("Vertex: Unexpected key/value combination: key: {}", k),
        }
    }
}

// same thing for Face
impl ply::PropertyAccess for Face {
    fn new() -> Self {
        Face {
            vertex_index: Vec::new(),
        }
    }
    fn set_property(&mut self, key: String, property: ply::Property) {
        match (key.as_ref(), property) {
            ("vertex_index", ply::Property::ListInt(vec)) => self.vertex_index = vec,
            (k, _) => panic!("Face: Unexpected key/value combination: key: {}", k),
        }
    }
}

/// Demonstrates simplest use case for reading from a file.
fn main() {
    // set up a reader, in this a file.
    let path = "example_plys/greg_turk_example1_ok_ascii.ply";
    let f = std::fs::File::open(path).unwrap();
    // The header of a ply file consists of ascii lines, BufRead provides useful methods for that.
    let mut f = std::io::BufReader::new(f);

    // Create a parser for each struct. Parsers are cheap objects.
    let vertex_parser = parser::Parser::<Vertex>::new();
    let face_parser = parser::Parser::<Face>::new();

    // lets first consume the header
    // We also could use `face_parser`, The configuration is a parser's only state.
    // The reading position only depends on `f`.
    let header = vertex_parser.read_header(&mut f).unwrap();

    // Depending on the header, read the data into our structs..
    let mut vertex_list = Vec::new();
    let mut face_list = Vec::new();
    for (_ignore_key, element) in &header.elements {
        // we could also just parse them in sequence, but the file format might change
        match element.name.as_ref() {
            "vertex" => {vertex_list = vertex_parser.read_payload_for_element(&mut f, &element, &header).unwrap();},
            "face" => {face_list = face_parser.read_payload_for_element(&mut f, &element, &header).unwrap();},
            _ => panic!("Enexpeced element!"),
        }
    }

    // proof that data has been read
    println!("header: {:#?}", header);
    println!("vertex list: {:#?}", vertex_list);
    println!("face list: {:#?}", face_list);
}
