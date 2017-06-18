use super::KeyMap;
use super::Property;
use super::PropertyAccess;

/// one line in the payload section is an element
pub type DefaultElement = KeyMap<Property>;
macro_rules! get(
    ($e:expr) => (match $e {None => return None, Some(x) => x})
);
impl PropertyAccess for DefaultElement {
    fn new() -> Self {
        DefaultElement::new()
    }
    fn set_property(&mut self, key: String, property: Property) {
        self.insert(key, property);
    }
    fn get_char(&self, key: &String) -> Option<i8> {
        match *get!(self.get(key)) {
            Property::Char(x) => Some(x),
            _ => None,
        }
    }
    fn get_uchar(&self, key: &String) -> Option<u8> {
        match *get!(self.get(key)) {
            Property::UChar(x) => Some(x),
            _ => None,
        }
    }
    fn get_short(&self, key: &String) -> Option<i16> {
        match *get!(self.get(key)) {
            Property::Short(x) => Some(x),
            _ => None,
        }
    }
    fn get_ushort(&self, key: &String) -> Option<u16> {
        match *get!(self.get(key)) {
            Property::UShort(x) => Some(x),
            _ => None,
        }
    }
    fn get_int(&self, key: &String) -> Option<i32> {
        match *get!(self.get(key)) {
            Property::Int(x) => Some(x),
            _ => None,
        }
    }
    fn get_uint(&self, key: &String) -> Option<u32> {
        match *get!(self.get(key)) {
            Property::UInt(x) => Some(x),
            _ => None,
        }
    }
    fn get_float(&self, key: &String) -> Option<f32> {
        match *get!(self.get(key)) {
            Property::Float(x) => Some(x),
            _ => None,
        }
    }
    fn get_double(&self, key: &String) -> Option<f64> {
        match *get!(self.get(key)) {
            Property::Double(x) => Some(x),
            _ => None,
        }
    }
    fn get_list_char(&self, key: &String) -> Option<&[i8]> {
        match *get!(self.get(key)) {
            Property::ListChar(ref x) => Some(x),
            _ => None,
        }
    }
    fn get_list_uchar(&self, key: &String) -> Option<&[u8]> {
        match *get!(self.get(key)) {
            Property::ListUChar(ref x) => Some(x),
            _ => None,
        }
    }
    fn get_list_short(&self, key: &String) -> Option<&[i16]> {
        match *get!(self.get(key)) {
            Property::ListShort(ref x) => Some(x),
            _ => None,
        }
    }
    fn get_list_ushort(&self, key: &String) -> Option<&[u16]> {
        match *get!(self.get(key)) {
            Property::ListUShort(ref x) => Some(x),
            _ => None,
        }
    }
    fn get_list_int(&self, key: &String) -> Option<&[i32]> {
        match *get!(self.get(key)) {
            Property::ListInt(ref x) => Some(x),
            _ => None,
        }
    }
    fn get_list_uint(&self, key: &String) -> Option<&[u32]> {
        match *get!(self.get(key)) {
            Property::ListUInt(ref x) => Some(x),
            _ => None,
        }
    }
    fn get_list_float(&self, key: &String) -> Option<&[f32]> {
        match *get!(self.get(key)) {
            Property::ListFloat(ref x) => Some(x),
            _ => None,
        }
    }
    fn get_list_double(&self, key: &String) -> Option<&[f64]> {
        match *get!(self.get(key)) {
            Property::ListDouble(ref x) => Some(x),
            _ => None,
        }
    }
}
