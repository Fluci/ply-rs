use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Version {
	pub major: u16,
	pub minor: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Format {
	Ascii(Version),
	BinaryBigEndian(Version),
	BinaryLittleEndian(Version),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comment {
	pub message: String,
}

impl Comment {
	pub fn new(message: String) -> Self {
		Comment {
			message: message
		}
	}
	pub fn empty() -> Self {
		Comment {
			message: String::new()
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Element {
	pub name: String,
	pub count: u64,
	pub properties: Vec<Property>
}

impl Element {
	pub fn new(name: String, count: u64) -> Self {
		Element {
			name: name,
			count: count,
			properties: Vec::new()
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Property {
	pub name: String,
	pub data_type: DataType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataType {
	Char,
	UChar,
	Short,
	UShort,
	Int,
	UInt,
	Float,
	Double,
	List(Box<DataType>)
}

pub struct Header {
	pub format: Format,
	pub elements: Vec<Element>,
	pub comments: Vec<Comment>,
}
