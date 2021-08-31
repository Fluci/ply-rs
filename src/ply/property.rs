
/// Scalar type used to encode properties in the payload.
///
/// For the translation to rust types, see individual documentation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScalarType {
    /// Signed 8 bit integer, rust: `i8`.
    Char,
    /// Unsigned 8 bit integer, rust: `u8`.
    UChar,
    /// Signed 16 bit integer, rust: `i16`.
    Short,
    /// Unsigned 16 bit integer, rust: `u16`.
    UShort,
    /// Signed 32 bit integer, rust: `i32`.
    Int,
    /// Unsigned 32 bit integer, rust: `u32`.
    UInt,
    /// 32 bit floating point number, rust: `f32`.
    Float,
    /// 64 bit floating point number, rust: `f64`.
    Double,
}

/// Data type used to encode properties in the payload.
///
/// There are two possible types: scalars and lists.
/// Lists are a sequence of scalars with a leading integer value defining how many elements the list contains.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PropertyType {
    /// Simple, "one-number" type.
    Scalar(ScalarType),
    /// Defines a sequence of scalars with the same type.
    ///
    /// First value is the index type which should be an integer variant,
    /// Encoded in ascii, you always get the same number in the file (for example `32` or `17`).
    /// Hence, a good choice is mainly important for internal representation and binary encoding. T
    /// he possible trade-off should be obvious:
    /// List length/flexibility against storage size. Though this obviously depends on your specific use case.
    ///
    /// Second value is the type of the list elemetns.
    List(ScalarType, ScalarType)
}

/// Wrapper used to implement a dynamic type system as required by the PLY file format.
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

/// Provides setters and getters for the Parser and the Writer.
///
/// This trait allows you to create your own data structure for the case that the
/// default HashMap isn't efficient enough for you.
///
/// All setters and getters have default implementations that do nothing or at most return `None`.
///
/// Feel free only to implement what your application actually uses:
/// If you know, that you only expect unsigned shorts, don't bother about implementing signed shorts or floats, it won't be called.
///
/// The getters are named in congruence with `PropertyType` and `ScalarType`.
pub trait PropertyAccess {
    fn new() -> Self;
    fn set_property(&mut self, _property_name: &String, _property: Property) {
        // By default, do nothing
        // Sombody might only want to write, no point in bothering him/her with setter implementations.
    }
    fn get_char(&self, _property_name: &String) -> Option<i8> {
        None
    }
    fn get_uchar(&self, _property_name: &String) -> Option<u8> {
        None
    }
    fn get_short(&self, _property_name: &String) -> Option<i16> {
        None
    }
    fn get_ushort(&self, _property_name: &String) -> Option<u16> {
        None
    }
    fn get_int(&self, _property_name: &String) -> Option<i32> {
        None
    }
    fn get_uint(&self, _property_name: &String) -> Option<u32> {
        None
    }
    fn get_float(&self, _property_name: &String) -> Option<f32> {
        None
    }
    fn get_double(&self, _property_name: &String) -> Option<f64> {
        None
    }
    fn get_list_char(&self, _property_name: &String) -> Option<&[i8]> {
        None
    }
    fn get_list_uchar(&self, _property_name: &String) -> Option<&[u8]> {
        None
    }
    fn get_list_short(&self, _property_name: &String) -> Option<&[i16]> {
        None
    }
    fn get_list_ushort(&self, _property_name: &String) -> Option<&[u16]> {
        None
    }
    fn get_list_int(&self, _property_name: &String) -> Option<&[i32]> {
        None
    }
    fn get_list_uint(&self, _property_name: &String) -> Option<&[u32]> {
        None
    }
    fn get_list_float(&self, _property_name: &String) -> Option<&[f32]> {
        None
    }
    fn get_list_double(&self, _property_name: &String) -> Option<&[f64]> {
        None
    }
}
