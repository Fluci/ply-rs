use crate::ply::{ PropertyDef, PropertyType, ScalarType, Encoding, Version, Comment, ObjInfo,ElementDef };
#[derive(Debug, PartialEq, Clone)]
pub enum Line {
    MagicNumber,
    Format((Encoding, Version)),
    Comment(Comment),
    ObjInfo(ObjInfo),
    Element(ElementDef),
    Property(PropertyDef),
    EndHeader
}

peg::parser!{pub grammar grammar() for str {

/// Grammar for PLY header

pub rule number() -> String
	= n:$(['0'..='9']+) { n.to_string() }

rule space() = [' '|'\t']+

rule uint() -> u64
	= n:$(['0'..='9']+) { n.parse().unwrap() }

rule ident() -> String
	= s:$(['a'..='z'|'A'..='Z'|'_']['a'..='z'|'A'..='Z'|'0'..='9'|'_'|'-']*) { s.to_string() }

rule text() -> String
	= s:$((!['\n'|'\r'][_])+) { s.to_string() }

rule line_break()
	= "\r\n" / ['\n'|'\r']

rule scalar() -> ScalarType
	= "char"    { ScalarType::Char }
	/ "int8"    { ScalarType::Char }
	/ "uchar"   { ScalarType::UChar }
	/ "uint8"   { ScalarType::UChar }
	/ "short"   { ScalarType::Short }
	/ "int16"   { ScalarType::Short }
	/ "uint16"  { ScalarType::UShort }
	/ "ushort"  { ScalarType::UShort }
	/ "int32"   { ScalarType::Int }
	/ "int"     { ScalarType::Int }
	/ "uint32"  { ScalarType::UInt }
	/ "uint"    { ScalarType::UInt }
	/ "float32" { ScalarType::Float }
	/ "float64" { ScalarType::Double }
	/ "float"   { ScalarType::Float }
	/ "double"  { ScalarType::Double }

rule data_type() -> PropertyType
	= s:scalar()   { PropertyType::Scalar(s) }
	/ "list" space() it:scalar() space() t:scalar() {
		PropertyType::List(it, t)
	}

pub rule magic_number()
	= "ply"

pub rule format() -> (Encoding, Version)
	= "format" space() "ascii" space() v:version() { (Encoding::Ascii, v) }
	/ "format" space() "binary_big_endian" space() v:version() { (Encoding::BinaryBigEndian, v) }
	/ "format" space() "binary_little_endian" space() v:version() { (Encoding::BinaryLittleEndian, v) }

rule version() -> Version
	= maj:uint() "." min:uint() {
		Version {
			major: maj as u16,
			minor: min as u8,
		}
	}

pub rule comment() -> Comment
	= "comment" space() c:text() {
		c.to_string()
	}
	/ "comment" space()? {
		String::new()
	}

pub rule obj_info() -> ObjInfo
	= "obj_info" space() c:text() {
	    c.to_string()
	}
	/ "obj_info" space()? {
	    String::new()
	}

pub rule element() -> ElementDef
	= "element" space() id:$(ident()) space() n:uint() {
		let mut e = ElementDef::new(id.to_string());
		e.count = n as usize;
		e
	}

pub rule property() -> PropertyDef
	= "property" space() data_type:data_type() space() id:ident() {
		PropertyDef::new(id, data_type)
	}

pub rule end_header()
	= "end_header"

pub rule line() -> Line
	= l:trimmed_line() space()? line_break()? { l }

rule trimmed_line() -> Line
	= magic_number() { Line::MagicNumber }
	/ end_header() { Line::EndHeader }
	/ v:format() { Line::Format(v) }
	/ v:obj_info() { Line::ObjInfo(v) }
	/ v:comment() { Line::Comment(v) }
	/ v:element() { Line::Element(v) }
	/ v:property() { Line::Property(v) }

rule any_number() -> String
	= s:$(['-'|'+']? ['0'..='9']+("."['0'..='9']+)?("e"['-'|'+']?['0'..='9']+)?) { s.to_string() }

rule trimmed_data_line() -> Vec<String>
	= any_number() ** space()

pub rule data_line() -> Vec<String>
	= space()? l:trimmed_data_line() space()? line_break()? {l}

}}