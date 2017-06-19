use std::fmt::{ Display, Formatter };
use std::fmt;
use super::PropertyType;
use super::KeyMap;
use super::PropertyAccess;

/// Models all necessary information to interact with a PLY file.
///
/// The generic parameter `E` is the element type used to store the payload data.
#[derive(Debug, Clone, PartialEq)]
pub struct Ply<E: PropertyAccess> {
    /// All header information found in a PLY file.
    pub header: Header,
    /// The payloud found after the `end_header` line in a PLY file.
    ///
    /// One line in an ascii PLY file corresponds to a single element.
    /// The payload groups elments with the same type together in a vector.
    ///
    /// # Examples
    ///
    /// Assume you have a `Ply` object called `ply` and want to access the third `point` element:
    ///
    /// ```rust,no_run
    /// # use ply_rs::ply::{Ply, DefaultElement};
    /// # let ply = Ply::<DefaultElement>::new();
    /// // get ply from somewhere ...
    /// let ref a_point = ply.payload["point"][2];
    /// let ref a_point_x = ply.payload["point"][2]["x"];
    /// ```
    pub payload: Payload<E>,
}
impl<E: PropertyAccess> Ply<E> {
    /// Creates a new `Ply<E>`
    pub fn new() -> Self {
        Ply::<E> {
            header: Header::new(),
            payload: Payload::new(),
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

/// The part after `end_header`.
pub type Payload<E> = KeyMap<Vec<E>>;
