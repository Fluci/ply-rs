use std::fmt::{ Display, Formatter };
use std::fmt;
use linked_hash_map::LinkedHashMap;

pub trait Addable<V: Key> {
    fn add(&mut self, new_value: V);
}

pub trait Access<V> {
    fn last(&self) -> Option<&V>;
}

pub trait Key {
    fn get_key(&self) -> String;
}

pub type KeyMap<V> = LinkedHashMap<String, V>;

impl<V: Key> Addable<V> for KeyMap<V> {
    fn add(&mut self, value: V) {
        self.insert(value.get_key(), value);
    }
}

impl<V> Access<V> for KeyMap<V> {
    fn last(&self) -> Option<&V> {
        match self.iter().last() {
            None => None,
            Some((_, v)) => Some(v),
        }
    }
}

/////// Header Types
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Header {
    pub encoding: Encoding,
    pub version: Version,
    pub obj_infos: Vec<ObjInfo>,
    pub elements: KeyMap<ElementDef>,
    pub comments: Vec<Comment>,
}

impl Header {
    pub fn new() -> Self {
        Header {
            encoding: Encoding::Ascii,
            version: Version{major: 1, minor: 0},
            obj_infos: Vec::new(),
            elements: KeyMap::new(),
            comments: Vec::new(),
        }
    }
}

pub type ObjInfo = String;
pub type Comment = String;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Version {
    pub major: u16,
    pub minor: u8,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&format!("{}.{}", self.major, self.minor))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Encoding {
    Ascii,
    BinaryBigEndian,
    BinaryLittleEndian,
}

impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(
            match *self {
                Encoding::Ascii => "ascii",
                Encoding::BinaryBigEndian => "binary_big_endian",
                Encoding::BinaryLittleEndian => "binary_little_endian",
            }
        )
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ElementDef {
    pub name: String,
    pub count: usize,
    pub properties: KeyMap<PropertyDef>,
}
impl ElementDef {
    pub fn new(name: String, count: usize) -> Self {
        ElementDef {
            name: name,
            count: count,
            properties: KeyMap::new(),
        }
    }
}

impl Key for ElementDef {
    fn get_key(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PropertyDef {
    pub name: String,
    pub data_type: PropertyType,
}

impl PropertyDef {
    pub fn new(name: String, data_type: PropertyType) -> Self {
        PropertyDef {
            name: name,
            data_type: data_type,
        }
    }
}

impl Key for PropertyDef {
    fn get_key(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PropertyType {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
    /// Index type (important for binary), Element type
    List(Box<PropertyType>, Box<PropertyType>)
}

/// one line in the payload section is an element
pub type DefaultElement = KeyMap<Property>;
/// The part after `end_header`.
pub type Payload<E> = KeyMap<Vec<E>>;


#[derive(Debug, PartialEq, Clone)]
pub enum Property {
    Char(i8),
    UChar(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    Float(f32),
    Double(f64),
    ListChar(Vec<i8>),
    ListUChar(Vec<u8>),
    ListShort(Vec<i16>),
    ListUShort(Vec<u16>),
    ListInt(Vec<i32>),
    ListUInt(Vec<u32>),
    ListFloat(Vec<f32>),
    ListDouble(Vec<f64>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ply<E> {
    pub header: Header,
    pub payload: KeyMap<Vec<E>>
}
impl<E> Ply<E> {
    pub fn new() -> Self {
        Ply::<E> {
            header: Header::new(),
            payload: Payload::new(),
        }
    }

    pub fn make_consistent(&mut self) -> Result<(), ConsistencyError>{
        for (ek, _) in &self.header.elements {
            if !self.payload.contains_key(ek) {
                self.payload.insert(ek.clone(), Vec::new());
            }
        }
        for (pk, pe) in &self.payload {
            if pk.is_empty() {
                return Err(ConsistencyError::new("Element cannot have empty name."));
            }
            let ed = self.header.elements.get_mut(pk);
            if ed.is_none() {
                return Err(ConsistencyError::new(&format!("No decleration for element `{}` found.", pk)));
            }
            ed.unwrap().count = pe.len();
        }
        Ok(())
    }
}

///// helper
/////
use std::error;

#[derive(Debug)]
pub struct ConsistencyError {
    description: String,
}
impl ConsistencyError {
    pub fn new(description: &str) -> Self {
        ConsistencyError {
            description: description.to_string(),
        }
    }
}

impl Display for ConsistencyError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&format!("ConsistencyError: {}", self.description))
    }
}

impl error::Error for ConsistencyError {
    fn description(&self) -> &str {
        &self.description
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
