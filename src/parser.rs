#[cfg(test)]
mod tests {
    use grammar as g;
    use ply::*;
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
        assert_ok!(g::line(" ply "), Line::MagicNumber);
        assert_ok!(g::line(" format ascii 1.0 "), Line::Format(Format::Ascii(Version{major: 1, minor: 0})));
        assert_ok!(g::line(" comment a very nice comment "));
        assert_ok!(g::line(" element vertex 8 "));
        assert_ok!(g::line(" property float x "));
        assert_ok!(g::line(" element face 6 "));
        assert_ok!(g::line(" property list uchar int vertex_index "));
        assert_ok!(g::line(" end_header "));
    }
}
