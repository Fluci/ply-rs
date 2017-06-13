use std::io;
use std::string::ToString;
use ply::*;

pub struct Writer {}

impl Writer {
    pub fn new() -> Self {
        Writer {}
    }
    // TODO: think about masking and valid/invalid symbols
    pub fn write_ply<T: io::Write>(&mut self, out: &mut T, ply: &Ply) -> io::Result<usize> {
        let mut written: usize = 0;
        written += try!(self.write_header(out, &ply.header));
        written += try!(self.write_payload(out, &ply.payload));
        out.flush().unwrap();
        Ok(written)
    }
    pub fn write_header<T: io::Write>(&mut self, out: &mut T, header: &Header) -> io::Result<usize> {
        let mut written: usize = 0;
        written += try!(out.write("ply\n".as_bytes()));
        written += try!(out.write("format ".as_bytes()));
        written += try!(self.write_encoding(out, &header.encoding));
        written += try!(out.write(format!(" {}.{}\n", header.version.major, header.version.minor).as_bytes()));
        for c in &header.comments {
            written += try!(out.write(format!("comment {}\n", c).as_bytes()));
        }
        for oi in &header.obj_infos {
            written += try!(out.write(format!("obj_info {}\n", oi).as_bytes()));
        }
        for (_, e) in &header.elements {
            written += try!(out.write(format!("element {} {}\n", e.name, e.count).as_bytes()));
            for (_, p) in &e.properties {
                written += try!(out.write("property ".as_bytes()));
                written += try!(self.write_property_type(out, &p.data_type));
                written += try!(out.write(" ".as_bytes()));
                written += try!(out.write(p.name.as_bytes()));
                written += try!(self.write_line_break(out));
            }
        }
        written += try!(out.write("end_header\n".as_bytes()));
        Ok(written)
    }
    fn write_encoding<T: io::Write>(&self, out: &mut T, encoding: &Encoding) -> io::Result<usize> {
        let s = match *encoding {
            Encoding::Ascii => "ascii",
            Encoding::BinaryBigEndian => "binary_big_endian",
            Encoding::BinaryLittleEndian => "binary_little_endian",
        };
        out.write(s.as_bytes())
    }
    fn write_property_type<T: io::Write>(&self, out: &mut T, data_type: &DataType) -> io::Result<usize> {
        match *data_type {
            DataType::Char => out.write("char".as_bytes()),
            DataType::UChar => out.write("uchar".as_bytes()),
            DataType::Short => out.write("short".as_bytes()),
            DataType::UShort => out.write("ushort".as_bytes()),
            DataType::Int => out.write("int".as_bytes()),
            DataType::UInt => out.write("uint".as_bytes()),
            DataType::Float => out.write("float".as_bytes()),
            DataType::Double => out.write("double".as_bytes()),
            DataType::List(ref t) => {
                let mut written = try!(out.write("list uchar ".as_bytes()));
                written += try!(self.write_property_type(out, t));
                Ok(written)
            }
        }
    }
    pub fn write_payload<T: io::Write>(&mut self, out: &mut T, payload: &Payload) -> io::Result<usize> {
        let mut written = 0;
        for (_, elems) in payload {
            for e in elems {
                written += try!(self.write_payload_element(out, e));
            }
        }
        Ok(written)
    }
    pub fn write_payload_element<T: io::Write>(&mut self, out: &mut T, element: &PayloadElement) -> io::Result<usize> {
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
        written += try!(self.write_line_break(out));
        Ok(written)
    }

    pub fn write_payload_property<T: io::Write>(&self, out: &mut T, data_item: &DataItem) -> io::Result<usize> {
         let result = match *data_item {
            DataItem::Char(ref v) => self.write_simple_value(v, out),
            DataItem::UChar(ref v) => self.write_simple_value(v, out),
            DataItem::Short(ref v) => self.write_simple_value(v, out),
            DataItem::UShort(ref v) => self.write_simple_value(v, out),
            DataItem::Int(ref v) => self.write_simple_value(v, out),
            DataItem::UInt(ref v) => self.write_simple_value(v, out),
            DataItem::Float(ref v) => self.write_simple_value(v, out),
            DataItem::Double(ref v) => self.write_simple_value(v, out),
            DataItem::List(ref v) => {
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

    fn write_line_break<T: io::Write>(&self, out: &mut T) -> io::Result<usize> {
        out.write("\r\n".as_bytes())
    }
    fn write_simple_value<T: io::Write, V: ToString>(&self, value: &V, out: &mut T) -> io::Result<usize> {
        out.write(value.to_string().as_bytes())
    }
}
