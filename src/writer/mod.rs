use std::io::{ Write, Result, Error, ErrorKind };
use std::string::ToString;

use byteorder::{ BigEndian, LittleEndian };

use ply::*;

mod ascii;
mod binary;

pub enum NewLine {
    N,
    R,
    RN
}

use std::marker::PhantomData;
pub struct Writer<E: PropertyAccess> {
    /// Should be fairly efficient, se `as_bytes()` in https://doc.rust-lang.org/src/collections/string.rs.html#1001
    new_line: String,
    phantom: PhantomData<E>,
}

impl<E: PropertyAccess> Writer<E> {
    pub fn new() -> Self {
        Writer {
            new_line: "\r\n".to_string(),
            phantom: PhantomData,
        }
    }
    pub fn set_newline(&mut self, new_line: NewLine) {
        self.new_line = match new_line {
            NewLine::R => "\r".to_string(),
            NewLine::N => "\n".to_string(),
            NewLine::RN => "\r\n".to_string(),
        };
    }
    // TODO: think about masking and valid/invalid symbols
    // TODO: make consistency check
    pub fn write_ply<T: Write>(&mut self, out: &mut T, ply: &Ply<E>) -> Result<usize> {
        let mut written = 0;
        written += try!(self.write_header(out, &ply.header));
        written += try!(self.write_payload(out, &ply.payload, &ply.header));
        out.flush().unwrap();
        Ok(written)
    }
    pub fn write_line_magic_number<T: Write>(&self, out: &mut T) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write("ply".as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    pub fn write_line_format<T: Write>(&self, out: &mut T, encoding: &Encoding, version: &Version) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write("format ".as_bytes()));
        written += try!(self.write_encoding(out, encoding));
        written += try!(out.write(format!(" {}.{}", version.major, version.minor).as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    pub fn write_line_comment<T: Write>(&self, out: &mut T, comment: &Comment) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write(format!("comment {}", comment).as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    pub fn write_line_obj_info<T: Write>(&self, out: &mut T, obj_info: &ObjInfo) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write(format!("obj_info {}", obj_info).as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    pub fn write_line_element_definition<T: Write>(&self, out: &mut T, element: &ElementDef) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write(format!("element {} {}", element.name, element.count).as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
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
    pub fn write_element_definition<T: Write>(&self, out: &mut T, element: &ElementDef) -> Result<usize> {
        let mut written = 0;
        written += try!(self.write_line_element_definition(out, &element));
        for (_, p) in &element.properties {
            written += try!(self.write_line_property_definition(out, &p));
        }
        Ok(written)
    }
    pub fn write_line_end_header<T: Write>(&mut self, out: &mut T) -> Result<usize> {
        let mut written = 0;
        written += try!(out.write("end_header".as_bytes()));
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    pub fn write_header<T: Write>(&mut self, out: &mut T, header: &Header) -> Result<usize> {
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
    ///// Payload
    pub fn write_payload<T: Write>(&mut self, out: &mut T, payload: &Payload<E>, header: &Header) -> Result<usize> {
        let mut written = 0;
        let element_defs = &header.elements;
        for (k, element_list) in payload {
            let element_def = &element_defs[k];
            written += try!(self.write_payload_of_element(out, element_list, element_def, header));
        }
        Ok(written)
    }
    pub fn write_payload_of_element<T: Write>(&mut self, out: &mut T, element_list: &Vec<E>, element_def: &ElementDef, header: &Header) -> Result<usize> {
        let mut written = 0;
        match header.encoding {
            Encoding::Ascii => for element in element_list {
                written += try!(self.__write_ascii_element(out, element, &element_def));
            },
            Encoding::BinaryBigEndian => for element in element_list {
                written += try!(self.__write_binary_element::<T, BigEndian>(out, element, &element_def));
            },
            Encoding::BinaryLittleEndian => for element in element_list {
                written += try!(self.__write_binary_element::<T, LittleEndian>(out, element, &element_def));
            }
        }
        Ok(written)
    }
    pub fn write_ascii_element<T: Write>(&self, out: &mut T, element: &E, element_def: &ElementDef) -> Result<usize> {
        self.__write_ascii_element(out, element, element_def)
    }
    pub fn write_big_endian_element<T: Write> (&self, out: &mut T, element: &E, element_def: &ElementDef) -> Result<usize> {
        self.__write_binary_element::<T, BigEndian>(out, element, element_def)
    }
    pub fn write_little_endian_element<T: Write> (&self, out: &mut T, element: &E, element_def: &ElementDef) -> Result<usize> {
        self.__write_binary_element::<T, BigEndian>(out, element, element_def)
    }

    fn write_new_line<T: Write>(&self, out: &mut T) -> Result<usize> {
        out.write(self.new_line.as_bytes())
    }
}
