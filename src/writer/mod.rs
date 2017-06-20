//! Writes ascii or binary data from a `Ply` to a `Write` trait.

use std::io::{ Write, Result, Error, ErrorKind };
use std::string::ToString;

use ply::*;

use std::marker::PhantomData;

mod ascii;
mod binary;

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

impl<E: PropertyAccess> Writer<E> {
    /// Create a new `Writer<E>` where `E` is the element type. To get started quickly use `DefaultElement`.
    pub fn new() -> Self {
        Writer {
            new_line: "\n".to_string(),
            phantom: PhantomData,
        }
    }
    // TODO: think about masking and valid/invalid symbols
    /// Writes an entire PLY file modeled by `ply` to `out`, performs consistency chekc.
    ///
    /// `ply` must be mutable since a consistency check is performed.
    /// If problems can be corrected automatically, `ply` will be modified accordingly.
    ///
    /// Returns number of bytes written.
    pub fn write_ply<T: Write>(&self, out: &mut T, ply: &mut Ply<E>) -> Result<usize> {
        match ply.make_consistent() {
            Ok(()) => (),
            Err(e) => return Err(Error::new(ErrorKind::InvalidInput, format!("The given ply isn't consistent: {:?}", e))),
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
        written += try!(self.write_header(out, &ply.header));
        written += try!(self.write_payload(out, &ply.payload, &ply.header));
        out.flush().unwrap();
        Ok(written)
    }
    /// Writes the magic number "ply" and a new line.
    ///
    /// Each PLY file must start with "ply\n".
    pub fn write_line_magic_number<T: Write>(&self, out: &mut T) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write("ply".as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    /// Writes "format <encoding> <version>".
    ///
    /// Each PLY file must define its format.
    pub fn write_line_format<T: Write>(&self, out: &mut T, encoding: &Encoding, version: &Version) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write("format ".as_bytes()));
        written += try!(self.write_encoding(out, encoding));
        written += try!(out.write(format!(" {}.{}", version.major, version.minor).as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    /// Writes a comment line.
    ///
    /// A comment must not contain a line break and only consist of ascii characters.
    pub fn write_line_comment<T: Write>(&self, out: &mut T, comment: &Comment) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write(format!("comment {}", comment).as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    /// Writes an object information line.
    ///
    /// An object informatio line must not contain a line break an only consist of ascii characters.
    pub fn write_line_obj_info<T: Write>(&self, out: &mut T, obj_info: &ObjInfo) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write(format!("obj_info {}", obj_info).as_bytes()));
        written += try!(self.write_new_line(out));
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
        written += try!(out.write(format!("element {} {}", element.name, element.count).as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    /// Writes a property line form the header: "property [list <index_type> <scalar_type> | <scalar_type> ]"
    ///
    /// Make sure the property definition is consistent with the payload.
    pub fn write_line_property_definition<T: Write>(&self, out: &mut T, property: &PropertyDef) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write("property ".as_bytes()));
        written += try!(self.write_property_type(out, &property.data_type));
        written += try!(out.write(" ".as_bytes()));
        written += try!(out.write(property.name.as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    /// Writes the element line and all the property definitions
    ///
    /// Convenience method to call `write_line_element_definition` and `write_line_property_definition` in the correct way.
    ///
    /// Make sure the element definition is consistent with the payload.
    pub fn write_element_definition<T: Write>(&self, out: &mut T, element: &ElementDef) -> Result<usize> {
        let mut written = 0;
        written += try!(self.write_line_element_definition(out, &element));
        for (_, p) in &element.properties {
            written += try!(self.write_line_property_definition(out, &p));
        }
        Ok(written)
    }
    /// Writes `end_header\n`. This terminates the header. Each following byte belongs to the payload.
    pub fn write_line_end_header<T: Write>(&self, out: &mut T) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write("end_header".as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    /// Convenience method to write all header elements.
    ///
    /// It starts with writing the magic number "ply\n" and ends with "end_header".
    ///
    /// Make sure the header is consistent with the payload.
    pub fn write_header<T: Write>(&self, out: &mut T, header: &Header) -> Result<usize> {
        let mut written = 0;
        written += try!(self.write_line_magic_number(out));
        written += try!(self.write_line_format(out, &header.encoding, &header.version));
        for c in &header.comments {
            written += try!(self.write_line_comment(out, c));
        }
        for oi in &header.obj_infos {
            written += try!(self.write_line_obj_info(out, oi));
        }
        for (_, e) in &header.elements {
            written += try!(self.write_element_definition(out, &e));
        }
        written += try!(self.write_line_end_header(out));
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
                let mut written = try!(out.write("list ".as_bytes()));
                match *index_type {
                    ScalarType::Float => return Err(Error::new(ErrorKind::InvalidInput, "List index can not be of type float.")),
                    ScalarType::Double => return Err(Error::new(ErrorKind::InvalidInput, "List index can not be of type double.")),
                    _ => (),
                };
                written += try!(self.write_scalar_type(out, &index_type));
                written += try!(out.write(" ".as_bytes()));
                written += try!(self.write_scalar_type(out, &content_type));
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

    // ///// Payload

    /// Writes the payload of a `ply` (`ply.playload`).
    ///
    /// Make sure the Header is consistent with the payload.
    pub fn write_payload<T: Write>(&self, out: &mut T, payload: &Payload<E>, header: &Header) -> Result<usize> {
        let mut written = 0;
        let element_defs = &header.elements;
        for (k, element_list) in payload {
            let element_def = &element_defs[k];
            written += try!(self.write_payload_of_element(out, element_list, element_def, header));
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
                written += try!(self.write_ascii_element(out, element, &element_def));
            },
            Encoding::BinaryBigEndian => for element in element_list {
                written += try!(self.write_big_endian_element(out, element, &element_def));
            },
            Encoding::BinaryLittleEndian => for element in element_list {
                written += try!(self.write_little_endian_element(out, element, &element_def));
            }
        }
        Ok(written)
    }
    fn write_new_line<T: Write>(&self, out: &mut T) -> Result<usize> {
        out.write(self.new_line.as_bytes())
    }
}
