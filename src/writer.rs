use std::io::{ Write, Result };
use std::string::ToString;
use ply::*;

pub enum NewLine {
    N,
    R,
    RN
}

pub trait ToElement<P> {
    fn to_element(&self, element_def: &ElementDef) -> Result<DefaultElement>;
}

impl ToElement<DefaultElement> for DefaultElement {
    // simple identity
    fn to_element(&self, _props_def: &ElementDef) -> Result<DefaultElement> {
        Ok(self.clone())
    }
}


use std::marker::PhantomData;
pub struct Writer<P: ToElement<P>> {
    /// Should be fairly efficient, se `as_bytes()` in https://doc.rust-lang.org/src/collections/string.rs.html#1001
    new_line: String,
    phantom: PhantomData<P>,
}

impl<P: ToElement<P>> Writer<P> {
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
    pub fn write_ply<T: Write>(&mut self, out: &mut T, ply: &Ply<P>) -> Result<usize> {
        let mut written = 0;
        written += try!(self.write_header(out, &ply.header));
        written += try!(self.write_payload(out, &ply.payload, &ply.header.elements));
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
            PropertyType::Char => out.write("char".as_bytes()),
            PropertyType::UChar => out.write("uchar".as_bytes()),
            PropertyType::Short => out.write("short".as_bytes()),
            PropertyType::UShort => out.write("ushort".as_bytes()),
            PropertyType::Int => out.write("int".as_bytes()),
            PropertyType::UInt => out.write("uint".as_bytes()),
            PropertyType::Float => out.write("float".as_bytes()),
            PropertyType::Double => out.write("double".as_bytes()),
            PropertyType::List(ref t) => {
                let mut written = try!(out.write("list uchar ".as_bytes()));
                written += try!(self.write_property_type(out, t));
                Ok(written)
            }
        }
    }
    ///// Payload
    pub fn write_payload<T: Write>(&mut self, out: &mut T, payload: &Payload<P>, element_defs: &KeyMap<ElementDef>) -> Result<usize> {
        let mut written = 0;
        for (k, element_list) in payload {
            let element_def = &element_defs[k];
            for e in element_list {
                let raw_element = try!(e.to_element(element_def));
                written += try!(self.write_line_payload_element(out, &raw_element));
            }
        }
        Ok(written)
    }
    pub fn write_line_payload_element<T: Write>(&mut self, out: &mut T, element: &DefaultElement) -> Result<usize> {
        let mut written = 0;
        let mut p_iter = element.iter();
        let (_name, prop_val) = p_iter.next().unwrap();
        written += try!(self.write_payload_property(out, prop_val));
        loop {
            written += try!(out.write(" ".as_bytes()));
            let n = p_iter.next();
            if n == None {
                break;
            }
            let (_name, prop_val) = n.unwrap();
            written += try!(self.write_payload_property(out, prop_val));
        }
        written += try!(self.write_new_line(out));
        Ok(written)
    }
    fn write_payload_property<T: Write>(&self, out: &mut T, data_element: &Property) -> Result<usize> {
         let result = match *data_element {
            Property::Char(ref v) => self.write_simple_value(v, out),
            Property::UChar(ref v) => self.write_simple_value(v, out),
            Property::Short(ref v) => self.write_simple_value(v, out),
            Property::UShort(ref v) => self.write_simple_value(v, out),
            Property::Int(ref v) => self.write_simple_value(v, out),
            Property::UInt(ref v) => self.write_simple_value(v, out),
            Property::Float(ref v) => self.write_simple_value(v, out),
            Property::Double(ref v) => self.write_simple_value(v, out),
            Property::List(ref v) => {
                let mut written = 0;
                written += try!(out.write(&v.len().to_string().as_bytes()));
                for e in v {
                    written += try!(out.write(" ".as_bytes()));
                    written += try!(self.write_payload_property(out, &e));
                }
                Ok(written)
            },
        };
        result
    }

    fn write_new_line<T: Write>(&self, out: &mut T) -> Result<usize> {
        out.write(self.new_line.as_bytes())
    }
    fn write_simple_value<T: Write, V: ToString>(&self, value: &V, out: &mut T) -> Result<usize> {
        out.write(value.to_string().as_bytes())
    }
}
