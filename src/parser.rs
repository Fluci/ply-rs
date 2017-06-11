#[cfg(test)]
mod tests {
    use grammar as g;
    use ply::*;
    #[test]
    fn magic_number_ok() {
        assert!(g::magic_number("ply").is_ok());
    }
    #[test]
    fn magic_number_err() {
        assert!(g::magic_number("py").is_err());
        assert!(g::magic_number("plyhi").is_err());
        assert!(g::magic_number("hiply").is_err());
    }
    #[test]
    fn format_ok() {
        assert_eq!(
            g::format("ascii 1.0").unwrap(),
            Format::Ascii(Version{major: 1, minor: 0})
        );
        assert_eq!(
            g::format("binary_big_endian 2.1").unwrap(),
            Format::BinaryBigEndian(Version{major: 2, minor: 1})
        );
        assert_eq!(
            g::format("binary_little_endian 1.0").unwrap(),
            Format::BinaryLittleEndian(Version{major: 1, minor: 0})
        );
    }
    #[test]
    fn format_err() {
        assert!(g::format("asciii 1.0").is_err());
        assert!(g::format("ascii -1.0").is_err());
    }
    #[test]
    fn comment_ok() {
        assert!(g::comment("comment hi").is_ok());
        assert_eq!(
            g::comment("comment   hi, I'm a comment!").unwrap(),
            Comment{message: "hi, I'm a comment!".to_string()}
        );
        assert!(g::comment("comment ").is_ok());
        assert!(g::comment("comment").is_ok());
    }
    #[test]
    fn comment_err() {
        assert!(g::comment("commentt").is_err());
        assert!(g::comment("comment hi\na comment").is_err());
        assert!(g::comment("comment hi\r\na comment").is_err());
    }
    #[test]
    fn element_ok() {
        assert_eq!(
            g::element("element vertex 8").unwrap(),
            Element::new("vertex".to_string(), 8)
        );
    }
    #[test]
    fn element_err() {
        assert!(g::comment("element 8 vertex").is_err());
    }
    #[test]
    fn property_ok() {
        assert_eq!(
            g::property("property char c").unwrap(),
            Property {
                name: "c".to_string(),
                data_type: DataType::Char,
            }
        );
    }
    #[test]
    fn property_list_ok() {
        assert_eq!(
            g::property("property list uchar int c").unwrap(),
            Property {
                name: "c".to_string(),
                data_type: DataType::List(Box::new(DataType::Int)),
            }
        );
    }
}
