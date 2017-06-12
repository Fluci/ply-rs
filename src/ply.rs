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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Version {
    pub major: u16,
    pub minor: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Encoding {
    Ascii,
    BinaryBigEndian,
    BinaryLittleEndian,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Element {
    pub name: String,
    pub count: u64,
    pub properties: ItemMap<Property>
}

impl Element {
    pub fn new(name: String, count: u64) -> Self {
        Element {
            name: name,
            count: count,
            properties: ItemMap::new()
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
pub struct Header {
    pub encoding: Encoding,
    pub version: Version,
    pub obj_infos: Vec<ObjInfo>,
    pub elements: ItemMap<Element>,
    pub comments: Vec<Comment>,
}

impl Header {
    pub fn new(encoding: Encoding) -> Self {
        Header {
            encoding: encoding,
            version: Version{major: 1, minor: 0},
            obj_infos: Vec::new(),
            elements: ItemMap::new(),
            comments: Vec::new()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ply {
    pub header: Header,
    pub payload: ItemMap<Vec<ItemMap<DataItem>>>,
}
