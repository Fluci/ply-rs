use std::fmt::{ Display, Formatter, Error };
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

pub type ItemMap<V> = LinkedHashMap<String, V>;

impl<V: Key> Addable<V> for ItemMap<V> {
    fn add(&mut self, value: V) {
        self.insert(value.get_key(), value);
    }
}

impl<V> Access<V> for ItemMap<V> {
    fn last(&self) -> Option<&V> {
        match self.iter().last() {
            None => None,
            Some((_, v)) => Some(v),
        }
    }
}

pub type ObjInfo = String;
pub type Comment = String;
/// one line in the payload section is an element
pub type PayloadElement = ItemMap<DataItem>;
/// The part after `end_header`.
pub type Payload = Vec<PayloadElement>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Version {
    pub major: u16,
    pub minor: u8,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
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
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(
            match *self {
                Encoding::Ascii => "ascii",
                Encoding::BinaryBigEndian => "binary_big_endian",
                Encoding::BinaryLittleEndian => "binary_little_endian",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    pub name: String,
    pub count: u64,
    pub properties: ItemMap<Property>,
    pub payload: Payload,
}

impl Element {
    pub fn new(name: String, count: u64) -> Self {
        Element {
            name: name,
            count: count,
            properties: ItemMap::new(),
            payload: Payload::new(),
        }
    }
}

impl Key for Element {
    fn get_key(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Property {
    pub name: String,
    pub data_type: DataType,
}

impl Property {
    pub fn new(name: String, data_type: DataType) -> Self {
        Property {
            name: name,
            data_type: data_type,
        }
    }
}

impl Key for Property {
    fn get_key(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataType {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
    List(Box<DataType>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataItem {
    Char(i8),
    UChar(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    Float(f32),
    Double(f64),
    List(Vec<DataItem>)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ply {
    pub encoding: Encoding,
    pub version: Version,
    pub obj_infos: Vec<ObjInfo>,
    pub elements: ItemMap<Element>,
    pub comments: Vec<Comment>,
}

impl Ply {
    pub fn new() -> Self {
        Ply {
            encoding: Encoding::Ascii,
            version: Version{major: 1, minor: 0},
            obj_infos: Vec::new(),
            elements: ItemMap::new(),
            comments: Vec::new()
        }
    }
}
