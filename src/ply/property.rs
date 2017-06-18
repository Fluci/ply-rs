
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScalarType {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PropertyType {
    Scalar(ScalarType),
    /// Index type (important for binary), Element type
    List(ScalarType, ScalarType)
}

pub trait PropertyAccess {
    // TODO: default function?
    fn new() -> Self;
    fn set_property(&mut self, _key: String, _property: Property) {
        // By default, do nothing
        // Sombody might only want to write, no point in bothering him/her with setter implementations.
    }
    fn get_char(&self, _key: &String) -> Option<i8> {
        None
    }
    fn get_uchar(&self, _key: &String) -> Option<u8> {
        None
    }
    fn get_short(&self, _key: &String) -> Option<i16> {
        None
    }
    fn get_ushort(&self, _key: &String) -> Option<u16> {
        None
    }
    fn get_int(&self, _key: &String) -> Option<i32> {
        None
    }
    fn get_uint(&self, _key: &String) -> Option<u32> {
        None
    }
    fn get_float(&self, _key: &String) -> Option<f32> {
        None
    }
    fn get_double(&self, _key: &String) -> Option<f64> {
        None
    }
    fn get_list_char(&self, _key: &String) -> Option<&[i8]> {
        None
    }
    fn get_list_uchar(&self, _key: &String) -> Option<&[u8]> {
        None
    }
    fn get_list_short(&self, _key: &String) -> Option<&[i16]> {
        None
    }
    fn get_list_ushort(&self, _key: &String) -> Option<&[u16]> {
        None
    }
    fn get_list_int(&self, _key: &String) -> Option<&[i32]> {
        None
    }
    fn get_list_uint(&self, _key: &String) -> Option<&[u32]> {
        None
    }
    fn get_list_float(&self, _key: &String) -> Option<&[f32]> {
        None
    }
    fn get_list_double(&self, _key: &String) -> Option<&[f64]> {
        None
    }
}
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
    ListDouble(Vec<f64>),
}
