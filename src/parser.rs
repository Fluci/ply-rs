use std::io::{ Read, BufReader, BufRead, Result, Error, ErrorKind };

use grammar;
use ply::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Line {
	MagicNumber,
	Format(Format),
	Comment(Comment),
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
pub struct Parser {
}

impl Parser {
    pub fn read_header<T: Read>(&self, source: T) -> Result<Header> {
        let mut reader = BufReader::new(source);
        let mut line_str = String::new();
        // read ply
        try!(reader.read_line(&mut line_str));
        is_line!(grammar::line(&line_str), Line::MagicNumber);

        let mut header_format : Option<Format> = None;
        let mut header_elements = Vec::<Element>::new();
        let mut header_comments = Vec::<Comment>::new();
        let mut line_index = 1;
        'readlines: loop {
            line_str.clear();
            try!(reader.read_line(&mut line_str));
            let line = grammar::line(&line_str);
            match line {
                Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
                Ok(Line::MagicNumber) => return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Unexpected 'ply' found at line {}", line_index)
                )),
                Ok(Line::Format(ref f)) => (
                    if header_format.is_none() {
                        header_format = Some(f.clone());
                    } else if header_format.unwrap() != *f {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Found contradicting format definition at line {}", line_index)
                        ));
                    }
                ),
                Ok(Line::Comment(ref c)) => (
                    header_comments.push(c.clone())
                ),
                Ok(Line::Element(ref e)) => {
                    header_elements.push(e.clone())
                },
                Ok(Line::Property(p)) => (
                    if header_elements.is_empty() {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Property {:?} found without preceding element.", p)
                        ));
                    } else {
                        header_elements.last_mut().unwrap().properties.push(p);
                    }
                ),
                Ok(Line::EndHeader) => { break 'readlines; },
            };
            line_index += 1;
        }
        if header_format.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No format line found."
            ));
        }
        Ok(Header{
            format: header_format.unwrap().clone(),
            comments: header_comments,
            elements: header_elements
        })
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
        let p = Parser{};
        let txt = "ply\nformat ascii 1.0\nend_header\n";
        let bytes = txt.as_bytes();
        assert_ok!(p.read_header(bytes));

        let txt = "ply\n\
        format ascii 1.0\n\
        element vertex 8\n\
        property float x\n\
        property float y\n\
        element face 6\n\
        property list uchar int vertex_index\n\
        end_header\n";
        let bytes = txt.as_bytes();
        assert_ok!(p.read_header(bytes));
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
            Format::Ascii(Version{major: 1, minor: 0})
        );
        assert_ok!(
            g::format("format binary_big_endian 2.1"),
            Format::BinaryBigEndian(Version{major: 2, minor: 1})
        );
        assert_ok!(
            g::format("format binary_little_endian 1.0"),
            Format::BinaryLittleEndian(Version{major: 1, minor: 0})
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
            Comment{message: "hi, I'm a comment!".to_string()}
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
        assert_ok!(g::line("format ascii 1.0 "), Line::Format(Format::Ascii(Version{major: 1, minor: 0})));
        assert_ok!(g::line("comment a very nice comment "));
        assert_ok!(g::line("element vertex 8 "));
        assert_ok!(g::line("property float x "));
        assert_ok!(g::line("element face 6 "));
        assert_ok!(g::line("property list uchar int vertex_index "));
        assert_ok!(g::line("end_header "));
        assert_ok!(g::line("ply \n"), Line::MagicNumber); // Unix, Mac OS X
        assert_ok!(g::line("ply \r"), Line::MagicNumber); // Mac pre OS X
        assert_ok!(g::line("ply \r\n"), Line::MagicNumber); // Windows
    }
}
