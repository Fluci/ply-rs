//! Writes ascii or binary data from a `Ply` to a `Write` trait.

use std::marker::PhantomData;
use crate::ply::PropertyAccess;

/// Writes a `Ply` to a `Write` trait.
///
/// The simplest function to start with is `write_ply()`.
/// It performs all necessary checks and writes a complete PLY file.
/// Sometimes you might want to have better control over how much is written.
/// All other `write_` functions are for those cases.
/// The trade-off is, that then you get responsible to write consistent data.
/// See `Ply::make_consistent()`.
///
/// For further information on the PLY file format,
/// consult the [official reference](http://paulbourke.net/dataformats/ply/).
///
/// # Examples
///
/// Simplest case of writing an entire PLY file en bloc:
///
/// ```rust
/// # use ply_rs::ply::{Ply, DefaultElement};
/// # use ply_rs::writer::Writer;
/// // Get a Ply from somewhere
/// // let mut ply = ...;
/// # let mut ply = Ply::<DefaultElement>::new();
///
/// // Get a buffer with `Write` trait.
/// // For example a file: let buf = std::io::File(".../your.ply").unwrap();
/// # let mut buf = Vec::<u8>::new();
///
/// // Create a writer
/// let w = Writer::new();
///
/// // Write your data:
/// let written = w.write_ply(&mut buf, &mut ply).unwrap();
/// ```
pub struct Writer<E: PropertyAccess> {
    /// Should be fairly efficient, se `as_bytes()` in https://doc.rust-lang.org/src/collections/string.rs.html#1001
    new_line: String,
    phantom: PhantomData<E>,
}

/*
use std::marker::PhantomData;
use writer::Writer;
use ply::PropertyAccess;
// */

use std::io;
use std::io::{ Write, Result, ErrorKind };

use crate::ply::Ply;

// ////////////////////////////
// General
// /////////////
impl<E: PropertyAccess> Writer<E> {
    /// Create a new `Writer<E>` where `E` is the element type. To get started quickly use `DefaultElement`.
    pub fn new() -> Self {
        Writer {
            new_line: "\n".to_string(),
            phantom: PhantomData,
        }
    }
    /// Writes an entire PLY file modeled by `ply` to `out`, performs consistency chekc.
    ///
    /// `ply` must be mutable since a consistency check is performed.
    /// If problems can be corrected automatically, `ply` will be modified accordingly.
    ///
    /// Returns number of bytes written.
    pub fn write_ply<T: Write>(&self, out: &mut T, ply: &mut Ply<E>) -> Result<usize> {
        match ply.make_consistent() {
            Ok(()) => (),
            Err(e) => return Err(io::Error::new(ErrorKind::InvalidInput, format!("The given ply isn't consistent: {:?}", e))),
        };
        self.write_ply_unchecked(out, ply)
    }
    /// Writes an entire PLY file modeled by `ply` to `out`, performes no consistency check.
    ///
    /// Like `write_ply` but doesn't check the input for inconsistency.
    /// The user is responsible to provide a consistent `Ply`,
    /// if not, behaviour is undefined and might result
    /// in a corrupted output.
    pub fn write_ply_unchecked<T: Write>(&self, out: &mut T, ply: &Ply<E>) -> Result<usize> {
        let mut written = 0;
        written += self.write_header(out, &ply.header)?;
        written += self.write_payload(out, &ply.payload, &ply.header)?;
        out.flush().unwrap();
        Ok(written)
    }
    fn write_new_line<T: Write>(&self, out: &mut T) -> Result<usize> {
        out.write(self.new_line.as_bytes())
    }
}

/*
use writer::Writer;
use std::io;
use std::io::{ Write, ErrorKind, Result };
use super::general;
use ply::PropertyAccess;
// */
use crate::ply::{ Header, Encoding, Version, Comment, ObjInfo, ElementDef, PropertyDef, PropertyType, ScalarType };

// ////////////////////////
/// # Header
// ////////////////////////
impl<E: PropertyAccess> Writer<E> {
    /// Writes the magic number "ply" and a new line.
    ///
    /// Each PLY file must start with "ply\n".
    pub fn write_line_magic_number<T: Write>(&self, out: &mut T) -> Result<usize> {
        let mut written = 0;
        written += out.write("ply".as_bytes())?;
        written += self.write_new_line(out)?;
        Ok(written)
    }
    /// Writes "format <encoding> <version>".
    ///
    /// Each PLY file must define its format.
    pub fn write_line_format<T: Write>(&self, out: &mut T, encoding: &Encoding, version: &Version) -> Result<usize> {
        let mut written = 0;
        written += out.write("format ".as_bytes())?;
        written += self.write_encoding(out, encoding)?;
        written += out.write(format!(" {}.{}", version.major, version.minor).as_bytes())?;
        written += self.write_new_line(out)?;
        Ok(written)
    }
    /// Writes a comment line.
    ///
    /// A comment must not contain a line break and only consist of ascii characters.
    pub fn write_line_comment<T: Write>(&self, out: &mut T, comment: &Comment) -> Result<usize> {
        let mut written = 0;
        written += out.write(format!("comment {}", comment).as_bytes())?;
        written += self.write_new_line(out)?;
        Ok(written)
    }
    /// Writes an object information line.
    ///
    /// An object informatio line must not contain a line break an only consist of ascii characters.
    pub fn write_line_obj_info<T: Write>(&self, out: &mut T, obj_info: &ObjInfo) -> Result<usize> {
        let mut written = 0;
        written += out.write(format!("obj_info {}", obj_info).as_bytes())?;
        written += self.write_new_line(out)?;
        Ok(written)
    }
    /// Writes an element line from the header: "element <name> <count>"
    ///
    /// This line is part of the header. It defines the format of an element.
    /// It is directly followed by its property definitions.
    ///
    /// Make sure the header is consistent with the payload.
    pub fn write_line_element_definition<T: Write>(&self, out: &mut T, element: &ElementDef) -> Result<usize> {
        let mut written = 0;
        written += out.write(format!("element {} {}", element.name, element.count).as_bytes())?;
        written += self.write_new_line(out)?;
        Ok(written)
    }
    /// Writes a property line form the header: "property [list <index_type> <scalar_type> | <scalar_type> ]"
    ///
    /// Make sure the property definition is consistent with the payload.
    pub fn write_line_property_definition<T: Write>(&self, out: &mut T, property: &PropertyDef) -> Result<usize> {
        let mut written = 0;
        written += out.write("property ".as_bytes())?;
        written += self.write_property_type(out, &property.data_type)?;
        written += out.write(" ".as_bytes())?;
        written += out.write(property.name.as_bytes())?;
        written += self.write_new_line(out)?;
        Ok(written)
    }
    /// Writes the element line and all the property definitions
    ///
    /// Convenience method to call `write_line_element_definition` and `write_line_property_definition` in the correct way.
    ///
    /// Make sure the element definition is consistent with the payload.
    pub fn write_element_definition<T: Write>(&self, out: &mut T, element: &ElementDef) -> Result<usize> {
        let mut written = 0;
        written += self.write_line_element_definition(out, &element)?;
        for (_, p) in &element.properties {
            written += self.write_line_property_definition(out, &p)?;
        }
        Ok(written)
    }
    /// Writes `end_header\n`. This terminates the header. Each following byte belongs to the payload.
    pub fn write_line_end_header<T: Write>(&self, out: &mut T) -> Result<usize> {
        let mut written = 0;
        written += out.write("end_header".as_bytes())?;
        written += self.write_new_line(out)?;
        Ok(written)
    }
    /// Convenience method to write all header elements.
    ///
    /// It starts with writing the magic number "ply\n" and ends with "end_header".
    ///
    /// Make sure the header is consistent with the payload.
    pub fn write_header<T: Write>(&self, out: &mut T, header: &Header) -> Result<usize> {
        let mut written = 0;
        written += self.write_line_magic_number(out)?;
        written += self.write_line_format(out, &header.encoding, &header.version)?;
        for c in &header.comments {
            written += self.write_line_comment(out, c)?;
        }
        for oi in &header.obj_infos {
            written += self.write_line_obj_info(out, oi)?;
        }
        for (_, e) in &header.elements {
            written += self.write_element_definition(out, &e)?;
        }
        written += self.write_line_end_header(out)?;
        Ok(written)
    }
    fn write_encoding<T: Write>(&self, out: &mut T, encoding: &Encoding) -> Result<usize> {
        let s = match *encoding {
            Encoding::Ascii => "ascii",
            Encoding::BinaryBigEndian => "binary_big_endian",
            Encoding::BinaryLittleEndian => "binary_little_endian",
        };
        out.write(s.as_bytes())
    }
    fn write_property_type<T: Write>(&self, out: &mut T, data_type: &PropertyType) -> Result<usize> {
        match *data_type {
            PropertyType::Scalar(ref scalar_type) => self.write_scalar_type(out, &scalar_type),
            PropertyType::List(ref index_type, ref content_type) => {
                let mut written = out.write("list ".as_bytes())?;
                match *index_type {
                    ScalarType::Float => return Err(io::Error::new(ErrorKind::InvalidInput, "List index can not be of type float.")),
                    ScalarType::Double => return Err(io::Error::new(ErrorKind::InvalidInput, "List index can not be of type double.")),
                    _ => (),
                };
                written += self.write_scalar_type(out, &index_type)?;
                written += out.write(" ".as_bytes())?;
                written += self.write_scalar_type(out, &content_type)?;
                Ok(written)
            }
        }
    }
    fn write_scalar_type<T: Write>(&self, out: &mut T, scalar_type: &ScalarType) -> Result<usize> {
        match *scalar_type {
            ScalarType::Char => out.write("char".as_bytes()),
            ScalarType::UChar => out.write("uchar".as_bytes()),
            ScalarType::Short => out.write("short".as_bytes()),
            ScalarType::UShort => out.write("ushort".as_bytes()),
            ScalarType::Int => out.write("int".as_bytes()),
            ScalarType::UInt => out.write("uint".as_bytes()),
            ScalarType::Float => out.write("float".as_bytes()),
            ScalarType::Double => out.write("double".as_bytes()),
        }
    }
}
/*
use writer::Writer;
use std::io::{ Write, Result };
use ply::{ Header, PropertyAccess, Encoding, ElementDef };
// */
use crate::ply::Payload;

// ////////////////////////
/// # Payload
// ////////////////////////
impl<E: PropertyAccess> Writer<E> {
    /// Writes the payload of a `ply` (`ply.playload`).
    ///
    /// Make sure the Header is consistent with the payload.
    pub fn write_payload<T: Write>(&self, out: &mut T, payload: &Payload<E>, header: &Header) -> Result<usize> {
        let mut written = 0;
        let element_defs = &header.elements;
        for (k, element_list) in payload {
            let element_def = &element_defs[k];
            written += self.write_payload_of_element(out, element_list, element_def, header)?;
        }
        Ok(written)
    }
    /// Write all elments as stored in the `element_list`.
    ///
    /// Make sure the header and the element definition is consistent with the payload.
    pub fn write_payload_of_element<T: Write>(&self, out: &mut T, element_list: &Vec<E>, element_def: &ElementDef, header: &Header) -> Result<usize> {
        let mut written = 0;
        match header.encoding {
            Encoding::Ascii => for element in element_list {
                written += self.write_ascii_element(out, element, &element_def)?;
            },
            Encoding::BinaryBigEndian => for element in element_list {
                written += self.write_big_endian_element(out, element, &element_def)?;
            },
            Encoding::BinaryLittleEndian => for element in element_list {
                written += self.write_little_endian_element(out, element, &element_def)?;
            }
        }
        Ok(written)
    }
}
/*
use std::io::{ Write, Result, ErrorKind };
use ply::{ PropertyAccess, ElementDef, PropertyDef, PropertyType, ScalarType };
use super::Writer;
// */
use std::fmt::Display;

macro_rules! get_prop(
    ($e:expr) => (match $e {None => return Err(io::Error::new(ErrorKind::InvalidInput, "No property available for given key.")), Some(x) => x})
);

/// # Ascii
impl<E: PropertyAccess> Writer<E> {

    /// Write a single ascii formatted element.
    pub fn write_ascii_element<T: Write>(&self, out: &mut T, element: &E, element_def: &ElementDef) -> Result<usize> {
        let mut written = 0;
        let mut p_iter = element_def.properties.iter();
        let (_k, prop_type) = p_iter.next().unwrap();
        written += self.write_ascii_property(out, element, &prop_type)?;
        loop {
            written += out.write(" ".as_bytes())?;
            let n = p_iter.next();
            if n == None {
                break;
            }
            let (_name, prop_type) = n.unwrap();
            written += self.write_ascii_property(out, element, prop_type)?;
        }
        written += self.write_new_line(out)?;
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
        written += out.write(&list.len().to_string().as_bytes())?;
        let b = " ".as_bytes();
        for v in list {
            written += out.write(b)?;
            written += out.write(v.to_string().as_bytes())?;
        }
        Ok(written)
    }
}
/*
use ply::{ PropertyAccess, ElementDef, PropertyType, ScalarType };
use super::Writer;

use std::io;
use std::io::{ Write, Result, ErrorKind };
// */
use byteorder::{ BigEndian, LittleEndian, WriteBytesExt, ByteOrder };

/*
macro_rules! get_prop(
    ($e:expr) => (match $e {None => return Err(io::Error::new(ErrorKind::InvalidInput, "No property available for given key.")), Some(x) => x})
);
// */

/// # Binary
impl<E: PropertyAccess> Writer<E> {
    // private payload
    /// Write a single binary formatted element in big endian.
    pub fn write_big_endian_element<T: Write> (&self, out: &mut T, element: &E, element_def: &ElementDef) -> Result<usize> {
        self.__write_binary_element::<T, BigEndian>(out, element, element_def)
    }
    /// Write a single binary formatted element in little endian.
    pub fn write_little_endian_element<T: Write> (&self, out: &mut T, element: &E, element_def: &ElementDef) -> Result<usize> {
        self.__write_binary_element::<T, LittleEndian>(out, element, element_def)
    }
    fn __write_binary_element<T: Write, B: ByteOrder>(&self, out: &mut T, element: &E, element_def: &ElementDef) -> Result<usize> {
        let mut written = 0;
        for (k, property_def) in &element_def.properties {
            match property_def.data_type {
                PropertyType::Scalar(ref scalar_type) => {
                    written += match *scalar_type {
                        ScalarType::Char => {out.write_i8(get_prop!(element.get_char(k)))?; 1},
                        ScalarType::UChar => {out.write_u8(get_prop!(element.get_uchar(k)))?; 1},
                        ScalarType::Short => {out.write_i16::<B>(get_prop!(element.get_short(k)))?; 2},
                        ScalarType::UShort => {out.write_u16::<B>(get_prop!(element.get_ushort(k)))?; 2},
                        ScalarType::Int => {out.write_i32::<B>(get_prop!(element.get_int(k)))?; 4},
                        ScalarType::UInt => {out.write_u32::<B>(get_prop!(element.get_uint(k)))?; 4},
                        ScalarType::Float => {out.write_f32::<B>(get_prop!(element.get_float(k)))?; 4},
                        ScalarType::Double => {out.write_f64::<B>(get_prop!(element.get_double(k)))?; 8},
                    };
                },
                PropertyType::List(ref index_type, ref scalar_type) => {
                    let vec_len = element_def.count;
                    written += match *index_type {
                        ScalarType::Char => {out.write_i8(vec_len as i8)?; 1},
                        ScalarType::UChar => {out.write_u8(vec_len as u8)?; 1},
                        ScalarType::Short => {out.write_i16::<B>(vec_len as i16)?; 2},
                        ScalarType::UShort => {out.write_u16::<B>(vec_len as u16)?; 2},
                        ScalarType::Int => {out.write_i32::<B>(vec_len as i32)?; 4},
                        ScalarType::UInt => {out.write_u32::<B>(vec_len as u32)?; 4},
                        ScalarType::Float => return Err(io::Error::new(ErrorKind::InvalidInput, "Index of list must be an integer type, float declared in PropertyType.")),
                        ScalarType::Double => return Err(io::Error::new(ErrorKind::InvalidInput, "Index of list must be an integer type, double declared in PropertyType.")),
                    };

                    written += match *scalar_type {
                        ScalarType::Char => self.write_binary_list::<T, i8, B>(get_prop!(element.get_list_char(k)), out, &|o, x| {o.write_i8(*x)?; Ok(1)} )?,
                        ScalarType::UChar => self.write_binary_list::<T, u8, B>(get_prop!(element.get_list_uchar(k)), out, &|o, x| {o.write_u8(*x)?; Ok(1)} )?,
                        ScalarType::Short => self.write_binary_list::<T, i16, B>(get_prop!(element.get_list_short(k)), out, &|o, x| {o.write_i16::<B>(*x)?; Ok(2)} )?,
                        ScalarType::UShort => self.write_binary_list::<T, u16, B>(get_prop!(element.get_list_ushort(k)), out, &|o, x| {o.write_u16::<B>(*x)?; Ok(2)} )?,
                        ScalarType::Int => self.write_binary_list::<T, i32, B>(get_prop!(element.get_list_int(k)), out, &|o, x| {o.write_i32::<B>(*x)?; Ok(4)} )?,
                        ScalarType::UInt => self.write_binary_list::<T, u32, B>(get_prop!(element.get_list_uint(k)), out, &|o, x| {o.write_u32::<B>(*x)?; Ok(4)} )?,
                        ScalarType::Float => self.write_binary_list::<T, f32, B>(get_prop!(element.get_list_float(k)), out, &|o, x| {o.write_f32::<B>(*x)?; Ok(4)} )?,
                        ScalarType::Double => self.write_binary_list::<T, f64, B>(get_prop!(element.get_list_double(k)), out, &|o, x| {o.write_f64::<B>(*x)?; Ok(8)} )?,
                    }
                }
            }
        };
        Ok(written)
    }
    fn write_binary_list<T: Write, D, B: ByteOrder>(&self, list: &[D], out: &mut T, out_val: &dyn Fn(&mut T, &D) -> Result<usize>) -> Result<usize> {
        let mut written = 0;
        for v in list {
            written += out_val(out, v)?;
        }
        Ok(written)
    }
}
