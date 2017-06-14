use std;
use std::io::{ Read, BufReader, BufRead, Result, Error, ErrorKind };
use std::fmt::Debug;
use std::slice::Iter;
use std::str::FromStr;
use std::result;
use grammar;
use ply::*;
use util::LocationTracker;

#[derive(Debug, PartialEq, Clone)]
pub enum Line {
    MagicNumber,
    Format((Encoding, Version)),
    Comment(Comment),
    ObjInfo(ObjInfo),
    Element(Element),
    Property(Property),
    EndHeader
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

fn parse_rethrow<T, E: Debug>(location: &LocationTracker, line_str: &str, e: E, message: &str) -> Result<T> {
    Err(Error::new(
        ErrorKind::InvalidInput,
        format!("Line {}: {}\n\tString: '{}'\n\tError: {:?}", location.line_index, message, line_str, e)
    ))
}
fn parse_error<T>(location: &LocationTracker, line_str: &str, message: &str) -> Result<T> {
    Err(Error::new(
        ErrorKind::InvalidInput,
        format!("Line {}: {}\n\tString: '{}'", location.line_index, message, line_str)
    ))
}


pub struct Parser {}
impl Parser {
    pub fn new() -> Self {
        Parser {}
    }
    pub fn read_ply<T: Read>(&self, source: &mut T) -> Result<Ply> {
        let mut source = BufReader::new(source);
        let mut location = LocationTracker::new();
        let mut ply = try!(self.__read_header(&mut source, &mut location));
        try!(self.__read_payload(&mut source, &mut location, &mut ply));
        Ok(ply)
    }
    pub fn read_header<T: BufRead>(&self, reader: &mut T) -> Result<Ply> {
        let mut line = LocationTracker::new();
        self.__read_header(reader, &mut line)
    }
    pub fn read_header_line(&self, line: &str) -> Result<Line> {
        match self.__read_header_line(line) {
            Ok(l) => Ok(l),
            Err(e) => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Couldn't parse line.\n\tString: {}\n\tError: {:?}", line, e)
            )),
        }
    }
    pub fn read_payload<T: BufRead>(&self, reader: &mut T, ply: &mut Ply) -> Result<()> {
        let mut location = LocationTracker::new();
        self.__read_payload(reader, &mut location, ply)
    }
    pub fn read_element_line(&self, line: &str, props: &ItemMap<Property>) -> Result<ItemMap<DataItem>> {
        self.__read_element_line(line, props)
    }
}

impl Parser {
    fn __read_header_line(&self, line_str: &str) -> result::Result<Line, grammar::ParseError> {
        grammar::line(line_str)
    }
    fn __read_header<T: BufRead>(&self, reader: &mut T, location: &mut LocationTracker) -> Result<Ply> {
        location.next_line();
        let mut line_str = String::new();
        try!(reader.read_line(&mut line_str));
        match self.__read_header_line(&line_str) {
            Ok(Line::MagicNumber) => (),
            Ok(l) => return parse_error(location, &line_str, &format!("Expected magic number 'ply', but saw '{:?}'.", l)),
            Err(e) => return parse_rethrow(location, &line_str, e, "Expected magic number 'ply'.")
        }
        is_line!(grammar::line(&line_str), Line::MagicNumber);

        let mut header_form_ver : Option<(Encoding, Version)> = None;
        let mut header_obj_infos = Vec::<ObjInfo>::new();
        let mut header_elements = ItemMap::<Element>::new();
        let mut header_comments = Vec::<Comment>::new();
        location.next_line();
        'readlines: loop {
            line_str.clear();
            try!(reader.read_line(&mut line_str));
            let line = self.__read_header_line(&line_str);

            match line {
                Err(e) => return parse_rethrow(location, &line_str, e, "Couldn't parse line."),
                Ok(Line::MagicNumber) => return parse_error(location, &line_str, "Unexpected 'ply' found."),
                Ok(Line::Format(ref t)) => (
                    if header_form_ver.is_none() {
                        header_form_ver = Some(t.clone());
                    } else {
                        let f = header_form_ver.unwrap();
                        if f != *t {
                            return parse_error(
                                location,
                                &line_str,
                                &format!(
                                    "Found contradicting format definition:\n\
                                    \tEncoding: {:?}, Version: {:?}\n\
                                    previous definition:\n\
                                    \tEncoding: {:?}, Version: {:?}",
                                    t.0, t.1, f.0, f.1)
                            )
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
                        return parse_error(
                            location,
                            &line_str,
                            &format!("Property '{:?}' found without preceding element.", p)
                        );
                    } else {
                        let (_, mut e) = header_elements.pop_back().unwrap();
                        e.properties.add(p);
                        header_elements.add(e);
                    }
                ),
                Ok(Line::EndHeader) => { location.next_line(); break 'readlines; },
            };
            location.next_line();
        }
        if header_form_ver.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No format line found."
            ));
        }
        let (encoding, version) = header_form_ver.unwrap();
        Ok(Ply{
            encoding: encoding,
            version: version,
            obj_infos: header_obj_infos,
            comments: header_comments,
            elements: header_elements
        })
    }
    fn __read_payload<T: BufRead>(&self, reader: &mut T, location: &mut LocationTracker, header: &mut Ply) -> Result<()> {
        match header.encoding {
            Encoding::Ascii => (),
            e => return Err(Error::new(ErrorKind::Other, format!("Encoding '{}' not implemented.", e))),
        };
        for (_, ref mut e) in &mut header.elements {
            let elems = try!(self.__read_payload_n(reader, location, &e));
            e.payload = elems;
        }
        Ok(())
    }
    fn __read_payload_n<T: BufRead>(&self, reader: &mut T, location: &mut LocationTracker, e: &Element) -> Result<Vec<ItemMap<DataItem>>> {
        let mut elems = Vec::<ItemMap<DataItem>>::new();
        let mut line_str = String::new();
        for _ in 0..e.count {
            line_str.clear();
            try!(reader.read_line(&mut line_str));

            let element = match self.__read_element_line(&line_str, &e.properties) {
                Ok(e) => e,
                Err(e) => return parse_rethrow(location, &line_str, e, "Couln't read element line.")
            };
            elems.push(element);
            location.next_line();
        }
        Ok(elems)
    }
    fn __read_element_line(&self, line: &str, props: &ItemMap<Property>) -> Result<ItemMap<DataItem>> {
        let elems = match grammar::data_line(line) {
            Ok(e) => e,
            Err(ref e) => return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Couldn't parse element line.\n\tString: '{}'\n\tError: {}", line, e)
                )),
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
            Err(e) => Err(Error::new(ErrorKind::InvalidInput,
                format!("Parse error.\n\tValue: '{}'\n\tError: {:?}, ", s, e))),
        }
    }
    fn read_properties(&self, elem_iter: &mut Iter<String>, data_type: &DataType) -> Result<DataItem> {
        let s : &String = match elem_iter.next() {
            None => return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Expected element of type '{:?}', but found nothing.", data_type)
            )),
            Some(x) => x
        };
        let result = match *data_type {
            DataType::Char => DataItem::Char(try!(self.parse(s))),
            DataType::UChar => DataItem::UChar(try!(self.parse(s))),
            DataType::Short => DataItem::Short(try!(self.parse(s))),
            DataType::UShort => DataItem::UShort(try!(self.parse(s))),
            DataType::Int => DataItem::Int(try!(self.parse(s))),
            DataType::UInt => DataItem::UInt(try!(self.parse(s))),
            DataType::Float => DataItem::Float(try!(self.parse(s))),
            DataType::Double => DataItem::Double(try!(self.parse(s))),
            DataType::List(ref item_type) => {
                let size : usize = try!(self.parse(s));
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
        let p = Parser::new();
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
        let txt = "ply\nformat ascii 1.0\nend_header\n";
        let mut bytes = txt.as_bytes();
        let p = Parser::new();
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
    fn parser_single_elements_ok(){
        let txt = "ply\r\n\
        format ascii 1.0\r\n\
        comment Hi, I'm your friendly comment.\r\n\
        obj_info And I'm your object information.\r\n\
        element point 2\r\n\
        property int x\r\n\
        property int y\r\n\
        end_header\r\n\
        -7 5\r\n\
        2 4\r\n";
        let mut bytes = txt.as_bytes();
        let p = Parser::new();
        assert_ok!(p.read_ply(&mut bytes));
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
    fn line_breaks_ok() {
        assert_ok!(g::line("ply \n"), Line::MagicNumber); // Unix, Mac OS X
        assert_ok!(g::line("ply \r"), Line::MagicNumber); // Mac pre OS X
        assert_ok!(g::line("ply \r\n"), Line::MagicNumber); // Windows
    }
    #[test]
    fn data_line_ok() {
        assert_ok!(g::data_line("-7 +5.21 \r\n"));
    }
}
