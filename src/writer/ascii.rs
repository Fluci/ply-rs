use std::io::{ Write, Result };
use std::fmt::Display;

use ply::{ PropertyAccess, ElementDef, PropertyDef, PropertyType, ScalarType };
use super::Writer;

macro_rules! get_prop(
    // TODO: errror
    ($e:expr) => (match $e {None => return Ok(17), Some(x) => x})
);

impl<E: PropertyAccess> Writer<E> {
    pub fn __write_ascii_element<T: Write>(&self, out: &mut T, element: &E, element_def: &ElementDef) -> Result<usize> {
        let mut written = 0;
        let mut p_iter = element_def.properties.iter();
        let (_k, prop_type) = p_iter.next().unwrap();
        written += try!(self.write_ascii_property(out, element, &prop_type));
        loop {
            written += try!(out.write(" ".as_bytes()));
            let n = p_iter.next();
            if n == None {
                break;
            }
            let (_name, prop_type) = n.unwrap();
            written += try!(self.write_ascii_property(out, element, prop_type));
        }
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    fn write_ascii_property<T: Write>(&self, out: &mut T, element: &E, prop_type: &PropertyDef) -> Result<usize> {
        let k = &prop_type.name;
        let result = match prop_type.data_type {
            PropertyType::Scalar(ref scalar_type) => match *scalar_type {
                ScalarType::Char => self.write_ascii_scalar(out, get_prop!(element.get_char(k))),
                ScalarType::UChar => self.write_ascii_scalar(out, get_prop!(element.get_uchar(k))),
                ScalarType::Short => self.write_ascii_scalar(out, get_prop!(element.get_short(k))),
                ScalarType::UShort => self.write_ascii_scalar(out, get_prop!(element.get_ushort(k))),
                ScalarType::Int => self.write_ascii_scalar(out, get_prop!(element.get_int(k))),
                ScalarType::UInt => self.write_ascii_scalar(out, get_prop!(element.get_uint(k))),
                ScalarType::Float => self.write_ascii_scalar(out, get_prop!(element.get_float(k))),
                ScalarType::Double => self.write_ascii_scalar(out, get_prop!(element.get_double(k))),
            },
            PropertyType::List(_, ref scalar_type) => match *scalar_type {
                ScalarType::Char => self.write_ascii_list(get_prop!(element.get_list_char(k)), out),
                ScalarType::UChar => self.write_ascii_list(get_prop!(element.get_list_uchar(k)), out),
                ScalarType::Short => self.write_ascii_list(get_prop!(element.get_list_short(k)), out),
                ScalarType::UShort => self.write_ascii_list(get_prop!(element.get_list_ushort(k)), out),
                ScalarType::Int => self.write_ascii_list(get_prop!(element.get_list_int(k)), out),
                ScalarType::UInt => self.write_ascii_list(get_prop!(element.get_list_uint(k)), out),
                ScalarType::Float => self.write_ascii_list(get_prop!(element.get_list_float(k)), out),
                ScalarType::Double => self.write_ascii_list(get_prop!(element.get_list_double(k)), out),
            }
        };
        result
    }
    fn write_ascii_scalar<T: Write, V: ToString>(&self, out: &mut T, value: V) -> Result<usize> {
        out.write(value.to_string().as_bytes())
    }
    fn write_ascii_list<T: Write, D: Clone + Display>(&self, list: &[D], out: &mut T) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write(&list.len().to_string().as_bytes()));
        let b = " ".as_bytes();;
        for v in list {
            written += try!(out.write(b));
            written += try!(out.write(v.to_string().as_bytes()));
        }
        Ok(written)
    }
}
