use std;
use std::io::{ Read, BufReader, BufRead, Result, Error, ErrorKind };
use std::slice::Iter;
use std::str::FromStr;
use grammar;
use ply::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Line {
    MagicNumber,
    Format((Encoding, Version)),
    Comment(Comment),
    ObjInfo(ObjInfo),
    Element(Element),
    Property(Property),
    EndHeader
}

macro_rules! read_line {
    ($e:expr) => (

    );
}
macro_rules! is_line {
    ($e:expr, $t:ty) => (
        match $e {
            Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
            Ok(l @ Line::MagicNumber) => (l),
            Ok(ob) => return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Invalid line encountered. Expected type: '$t', found: '{:?}'", ob)
            )),
        }
    );
}
macro_rules! try_io {
    ($e:expr) => (
        match $e {
            Ok(obj) => obj,
            Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
        }
    );
}
pub struct Parser {
    line_index : usize,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            line_index: 0,
        }
    }
    pub fn read<T: Read>(&mut self, source: &mut T) -> Result<Ply> {
        let mut source = BufReader::new(source);
        let header = try!(self.read_header(&mut source));
        let payload = try!(self.read_payload(&mut source, &header));
        Ok(Ply{
            header: header,
            payload: payload
        })
    }
    pub fn read_header<T: BufRead>(&mut self, reader: &mut T) -> Result<Header> {
        self.line_index = 1;
        let mut line_str = String::new();
        // read ply
        try!(reader.read_line(&mut line_str));
        is_line!(grammar::line(&line_str), Line::MagicNumber);

        let mut header_form_ver : Option<(Encoding, Version)> = None;
        let mut header_obj_infos = Vec::<ObjInfo>::new();
        let mut header_elements = ItemMap::<Element>::new();
        let mut header_comments = Vec::<Comment>::new();
        self.line_index += 1;
        'readlines: loop {
            line_str.clear();
            try!(reader.read_line(&mut line_str));
            let line = grammar::line(&line_str);
            match line {
                Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
                Ok(Line::MagicNumber) => return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Unexpected 'ply' found at line {}", self.line_index)
                )),
                Ok(Line::Format(ref t)) => (
                    if header_form_ver.is_none() {
                        header_form_ver = Some(t.clone());
                    } else {
                        let f = header_form_ver.unwrap();
                        if f != *t {
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                format!(
                                    "Line {}: Found contradicting format definition:\n\
                                    \tEncoding: {:?}, Version: {:?}\n\
                                    previous definition:\n\
                                    \tEncoding: {:?}, Version: {:?}",
                                    self.line_index, t.0, t.1, f.0, f.1)
                            ));
                        }
                    }
                ),
                Ok(Line::ObjInfo(ref o)) => (
                    header_obj_infos.push(o.clone())
                ),
                Ok(Line::Comment(ref c)) => (
                    header_comments.push(c.clone())
                ),
                Ok(Line::Element(ref e)) => {
                    header_elements.add(e.clone())
                },
                Ok(Line::Property(p)) => (
                    if header_elements.is_empty() {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Property {:?} found without preceding element.", p)
                        ));
                    } else {
                        let (_, mut e) = header_elements.pop_back().unwrap();
                        e.properties.add(p);
                        header_elements.add(e);
                    }
                ),
                Ok(Line::EndHeader) => { self.line_index += 1; break 'readlines; },
            };
            self.line_index += 1;
        }
        if header_form_ver.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No format line found."
            ));
        }
        let (encoding, version) = header_form_ver.unwrap();
        Ok(Header{
            encoding: encoding,
            version: version,
            obj_infos: header_obj_infos,
            comments: header_comments,
            elements: header_elements
        })
    }
    fn read_payload<T: BufRead>(&mut self, reader: &mut T, header: &Header) -> Result<ItemMap<Vec<ItemMap<DataItem>>>> {
        match header.encoding {
            Encoding::Ascii => (),
            _ => return Err(Error::new(ErrorKind::Other, "not implemented")),
        };
        let mut payload = ItemMap::<Vec<ItemMap<DataItem>>>::new();
        let mut line_str = String::new();
        for (k, e) in &header.elements {
            let mut elems = Vec::<ItemMap<DataItem>>::new();
            for _ in 0..e.count {
                line_str.clear();
                try!(reader.read_line(&mut line_str));

                let element = try!(self.read_element_line(&line_str, &e.properties));
                elems.push(element);
                self.line_index += 1;
            }
            payload.insert(k.clone(), elems);
        }
        Ok(payload)
    }
    pub fn read_element_line(&self, line: &str, props: &ItemMap<Property>) -> Result<ItemMap<DataItem>> {
        let elems = match grammar::data_line(line) {
            Ok(e) => e,
            Err(e) => return Err(Error::new(
                ErrorKind::InvalidInput,
                e
            ))
        };

        let mut elem_it : Iter<String> = elems.iter();
        let mut vals = ItemMap::<DataItem>::new();
        for (k, p) in props {
            let new_p : DataItem = try!(self.read_properties(&mut elem_it, &p.data_type));
            vals.insert(k.clone(), new_p);
        }
        Ok(vals)
    }
    fn parse<T: FromStr>(&self, s: &str) -> Result<T>
    where <T as FromStr>::Err: std::error::Error + std::marker::Send + std::marker::Sync + 'static {
        let v = s.parse();
        match v {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, format!("Line {}: Parse error. error: {:?}, value: '{}'", self.line_index, e, s))),
        }
    }
    fn read_properties(&self, elem_iter: &mut Iter<String>, data_type: &DataType) -> Result<DataItem> {
        let s : &String = match elem_iter.next() {
            None => return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Line {}: Expected element of type {:?}, but found nothing.", self.line_index, data_type)
            )),
            Some(x) => x
        };
        let result = match *data_type {
            DataType::Char => DataItem::Char(try!(self.parse(s))),
            DataType::UChar => DataItem::UChar(try_io!(s.parse())),
            DataType::Short => DataItem::Short(try_io!(s.parse())),
            DataType::UShort => DataItem::UShort(try_io!(s.parse())),
            DataType::Int => DataItem::Int(try_io!(s.parse())),
            DataType::UInt => DataItem::UInt(try_io!(s.parse())),
            DataType::Float => DataItem::Float(try_io!(s.parse())),
            DataType::Double => DataItem::Double(try_io!(s.parse())),
            DataType::List(ref item_type) => {
                let size : usize = try_io!(s.parse());
                let mut v = Vec::<DataItem>::new();
                for _ in 0..size {
                    let item = try!(self.read_properties(elem_iter, &item_type));
                    v.push(item);
                }
                DataItem::List(v)
            }
        };
        Ok(result)
    }
}



#[cfg(test)]
mod tests {
    use grammar as g;
    use super::*;
    macro_rules! assert_ok {
        ($e:expr) => (
            match $e {
                Ok(obj) => (obj),
                Err(e) => panic!("{}", e),
            }
        );
        ($e:expr , $o:expr) => (
            let obj = assert_ok!($e);
            assert_eq!(obj, $o);
        );
    }
    macro_rules! assert_err {
        ($e:expr) => (
            let result = $e;
            assert!(result.is_err());
        );
    }
    #[test]
    fn parser_header_ok(){
        let mut p = Parser::new();
        let txt = "ply\nformat ascii 1.0\nend_header\n";
        let mut bytes = txt.as_bytes();
        assert_ok!(p.read_header(&mut bytes));

        let txt = "ply\n\
        format ascii 1.0\n\
        element vertex 8\n\
        property float x\n\
        property float y\n\
        element face 6\n\
        property list uchar int vertex_index\n\
        end_header\n";
        let mut bytes = txt.as_bytes();
        assert_ok!(p.read_header(&mut bytes));
    }
    #[test]
    fn parser_demo_ok(){
        let mut p = Parser::new();
        let txt = "ply\nformat ascii 1.0\nend_header\n";
        let mut bytes = txt.as_bytes();
        assert_ok!(p.read_header(&mut bytes));

        let txt = "ply\n\
        format ascii 1.0\n\
        element vertex 1\n\
        property float x\n\
        end_header\n
        6.28318530718"; // no newline at end!
        let mut bytes = txt.as_bytes();
        assert_ok!(p.read_header(&mut bytes));
    }
    #[test]
    fn read_property_ok() {
        let p = Parser::new();
        let txt = "0 1 2 3";
        let mut prop = ItemMap::<Property>::new();
        prop.add(Property{name: "a".to_string(), data_type: DataType::Char});
        prop.add(Property{name: "b".to_string(), data_type: DataType::UChar});
        prop.add(Property{name: "c".to_string(), data_type: DataType::Short});
        prop.add(Property{name: "d".to_string(), data_type: DataType::UShort});

        let items = p.read_element_line(&txt, &prop);
        assert!(items.is_ok(), format!("error: {:?}", items));
    }
    #[test]
    fn magic_number_ok() {
        assert_ok!(g::magic_number("ply"));
    }
    #[test]
    fn magic_number_err() {
        assert_err!(g::magic_number("py"));
        assert_err!(g::magic_number("plyhi"));
        assert_err!(g::magic_number("hiply"));
    }
    #[test]
    fn format_ok() {
        assert_ok!(
            g::format("format ascii 1.0"),
            (Encoding::Ascii, Version{major: 1, minor: 0})
        );
        assert_ok!(
            g::format("format binary_big_endian 2.1"),
            (Encoding::BinaryBigEndian, Version{major: 2, minor: 1})
        );
        assert_ok!(
            g::format("format binary_little_endian 1.0"),
            (Encoding::BinaryLittleEndian, Version{major: 1, minor: 0})
        );
    }
    #[test]
    fn format_err() {
        assert_err!(g::format("format asciii 1.0"));
        assert_err!(g::format("format ascii -1.0"));
    }
    #[test]
    fn comment_ok() {
        assert_ok!(g::comment("comment hi"));
        assert_ok!(
            g::comment("comment   hi, I'm a comment!"),
            "hi, I'm a comment!"
        );
        assert_ok!(g::comment("comment "));
        assert_ok!(g::comment("comment"));
    }
    #[test]
    fn comment_err() {
        assert_err!(g::comment("commentt"));
        assert_err!(g::comment("comment hi\na comment"));
        assert_err!(g::comment("comment hi\r\na comment"));
    }
    #[test]
    fn obj_info_ok() {
        assert_ok!(g::obj_info("obj_info Hi, I can help."));
    }
    #[test]
    fn element_ok() {
        assert_ok!(
            g::element("element vertex 8"),
            Element::new("vertex".to_string(), 8)
        );
    }
    #[test]
    fn element_err() {
        assert_err!(g::comment("element 8 vertex"));
    }
    #[test]
    fn property_ok() {
        assert_ok!(
            g::property("property char c"),
            Property {
                name: "c".to_string(),
                data_type: DataType::Char,
            }
        );
    }
    #[test]
    fn property_list_ok() {
        assert_ok!(
            g::property("property list uchar int c"),
            Property {
                name: "c".to_string(),
                data_type: DataType::List(Box::new(DataType::Int)),
            }
        );
    }
    #[test]
    fn line_ok() {
        assert_ok!(g::line("ply "), Line::MagicNumber);
        assert_ok!(g::line("format ascii 1.0 "), Line::Format((Encoding::Ascii, Version{major: 1, minor: 0})));
        assert_ok!(g::line("comment a very nice comment "));
        assert_ok!(g::line("element vertex 8 "));
        assert_ok!(g::line("property float x "));
        assert_ok!(g::line("element face 6 "));
        assert_ok!(g::line("property list uchar int vertex_index "));
        assert_ok!(g::line("end_header "));
    }
    #[test]
    fn line_breaks() {
        assert_ok!(g::line("ply \n"), Line::MagicNumber); // Unix, Mac OS X
        assert_ok!(g::line("ply \r"), Line::MagicNumber); // Mac pre OS X
        assert_ok!(g::line("ply \r\n"), Line::MagicNumber); // Windows
    }
}
