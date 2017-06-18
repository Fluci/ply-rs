use std::io::{ Read, Result, Error, ErrorKind };
use std::str::FromStr;
use std::error;
use std::marker;

use byteorder::{ ReadBytesExt, ByteOrder };
use ply::{ PropertyAccess, ElementDef, PropertyType, Property, ScalarType };

use util::LocationTracker;
use super::Parser;

impl<E: PropertyAccess> Parser<E> {
    pub fn __read_binary_payload_for_element<T: Read, B: ByteOrder>(&self, reader: &mut T, location: &mut LocationTracker, element_def: &ElementDef) -> Result<Vec<E>> {
        let mut elems = Vec::<E>::new();
        for _ in 0..element_def.count {
            let element = try!(self.__read_binary_element::<T, B>(reader, element_def));
            elems.push(element);
            location.next_line();
        }
        Ok(elems)
    }
    pub fn __read_binary_element<T: Read, B: ByteOrder>(&self, reader: &mut T, element_def: &ElementDef) -> Result<E> {
        let mut raw_element = E::new();

        for (k, p) in &element_def.properties {
            let property = try!(self.__read_binary_property::<T, B>(reader, &p.data_type));
            raw_element.set_property(k.clone(), property);
        }
        Ok(raw_element)
    }
    fn __read_binary_property<T: Read, B: ByteOrder>(&self, reader: &mut T, data_type: &PropertyType) -> Result<Property> {
        let result = match *data_type {
            PropertyType::Scalar(ref scalar_type) => match *scalar_type {
                ScalarType::Char => Property::Char(try!(reader.read_i8())),
                ScalarType::UChar => Property::UChar(try!(reader.read_u8())),
                ScalarType::Short => Property::Short(try!(reader.read_i16::<B>())),
                ScalarType::UShort => Property::UShort(try!(reader.read_u16::<B>())),
                ScalarType::Int => Property::Int(try!(reader.read_i32::<B>())),
                ScalarType::UInt => Property::UInt(try!(reader.read_u32::<B>())),
                ScalarType::Float => Property::Float(try!(reader.read_f32::<B>())),
                ScalarType::Double => Property::Double(try!(reader.read_f64::<B>())),
            },
            PropertyType::List(ref index_type, ref property_type) => {
                let count : usize = match *index_type {
                    ScalarType::Char => try!(reader.read_i8()) as usize,
                    ScalarType::UChar => try!(reader.read_u8()) as usize,
                    ScalarType::Short => try!(reader.read_i16::<B>()) as usize,
                    ScalarType::UShort => try!(reader.read_u16::<B>()) as usize,
                    ScalarType::Int => try!(reader.read_i32::<B>()) as usize,
                    ScalarType::UInt => try!(reader.read_u32::<B>()) as usize,
                    ScalarType::Float => return Err(Error::new(ErrorKind::InvalidInput, "Index of list must be an integer type, float declared in ScalarType.")),
                    ScalarType::Double => return Err(Error::new(ErrorKind::InvalidInput, "Index of list must be an integer type, double declared in ScalarType.")),
                };
                match *property_type {
                    ScalarType::Char => Property::ListChar(try!(self.__read_binary_list(reader, &|r| r.read_i8(), count))),
                    ScalarType::UChar => Property::ListUChar(try!(self.__read_binary_list(reader, &|r| r.read_u8(), count))),
                    ScalarType::Short => Property::ListShort(try!(self.__read_binary_list(reader, &|r| r.read_i16::<B>(), count))),
                    ScalarType::UShort => Property::ListUShort(try!(self.__read_binary_list(reader, &|r| r.read_u16::<B>(), count))),
                    ScalarType::Int => Property::ListInt(try!(self.__read_binary_list(reader, &|r| r.read_i32::<B>(), count))),
                    ScalarType::UInt => Property::ListUInt(try!(self.__read_binary_list(reader, &|r| r.read_u32::<B>(), count))),
                    ScalarType::Float => Property::ListFloat(try!(self.__read_binary_list(reader, &|r| r.read_f32::<B>(), count))),
                    ScalarType::Double => Property::ListDouble(try!(self.__read_binary_list(reader, &|r| r.read_f64::<B>(), count))),
                }
            }
        };
        Ok(result)
    }
    fn __read_binary_list<T: Read, D: FromStr>(&self, reader: &mut T, read_from: &Fn(&mut T) -> Result<D>, count: usize) -> Result<Vec<D>>
        where <D as FromStr>::Err: error::Error + marker::Send + marker::Sync + 'static {
        let mut list = Vec::<D>::new();
        for i in 0..count {
            let value : D = match read_from(reader) {
                Err(e) => return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Couldn't find a list element at index {}.\n\tError: {:?}", i, e)
                )),
                Ok(x) => x
            };
            list.push(value);
        }
        Ok(list)
    }
}
