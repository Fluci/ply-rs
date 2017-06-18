use std;
use std::io::{ BufRead, Result, Error, ErrorKind };
use std::slice::Iter;
use std::str::FromStr;

use grammar;
use ply::{ PropertyAccess, ElementDef, Property, PropertyType, ScalarType };
use util::LocationTracker;
use super::Parser;
use super::parse_ascii_rethrow;

impl<E: PropertyAccess> Parser<E> {
    pub fn __read_ascii_payload_for_element<T: BufRead>(&self, reader: &mut T, location: &mut LocationTracker, element_def: &ElementDef) -> Result<Vec<E>> {
        let mut elems = Vec::<E>::new();
        let mut line_str = String::new();
        for _ in 0..element_def.count {
            line_str.clear();
            try!(reader.read_line(&mut line_str));

            let element = match self.__read_ascii_element(&line_str, element_def) {
                Ok(e) => e,
                Err(e) => return parse_ascii_rethrow(location, &line_str, e, "Couln't read element line.")
            };
            elems.push(element);
            location.next_line();
        }
        Ok(elems)
    }
    pub fn __read_ascii_element(&self, line: &str, element_def: &ElementDef) -> Result<E> {
        let elems = match grammar::data_line(line) {
            Ok(e) => e,
            Err(ref e) => return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Couldn't parse element line.\n\tString: '{}'\n\tError: {}", line, e)
                )),
        };

        let mut elem_it : Iter<String> = elems.iter();
        let mut vals = E::new();
        for (k, p) in &element_def.properties {
            let new_p : Property = try!(self.__read_ascii_property(&mut elem_it, &p.data_type));
            vals.set_property(k.clone(), new_p);
        }
        Ok(vals)
    }
    fn __read_ascii_property(&self, elem_iter: &mut Iter<String>, data_type: &PropertyType) -> Result<Property> {
        let s : &String = match elem_iter.next() {
            None => return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Expected element of type '{:?}', but found nothing.", data_type)
            )),
            Some(x) => x
        };

        let result = match *data_type {
            PropertyType::Scalar(ref scalar_type) => match *scalar_type {
                ScalarType::Char => Property::Char(try!(self.parse(s))),
                ScalarType::UChar => Property::UChar(try!(self.parse(s))),
                ScalarType::Short => Property::Short(try!(self.parse(s))),
                ScalarType::UShort => Property::UShort(try!(self.parse(s))),
                ScalarType::Int => Property::Int(try!(self.parse(s))),
                ScalarType::UInt => Property::UInt(try!(self.parse(s))),
                ScalarType::Float => Property::Float(try!(self.parse(s))),
                ScalarType::Double => Property::Double(try!(self.parse(s))),
            },
            PropertyType::List(_, ref scalar_type) => {
                let count : usize = try!(self.parse(s));
                match *scalar_type {
                    ScalarType::Char => Property::ListChar(try!(self.__read_ascii_list(elem_iter, count))),
                    ScalarType::UChar => Property::ListUChar(try!(self.__read_ascii_list(elem_iter, count))),
                    ScalarType::Short => Property::ListShort(try!(self.__read_ascii_list(elem_iter, count))),
                    ScalarType::UShort => Property::ListUShort(try!(self.__read_ascii_list(elem_iter, count))),
                    ScalarType::Int => Property::ListInt(try!(self.__read_ascii_list(elem_iter, count))),
                    ScalarType::UInt => Property::ListUInt(try!(self.__read_ascii_list(elem_iter, count))),
                    ScalarType::Float => Property::ListFloat(try!(self.__read_ascii_list(elem_iter, count))),
                    ScalarType::Double => Property::ListDouble(try!(self.__read_ascii_list(elem_iter, count))),
                }
            }
        };
        Ok(result)
    }

    fn parse<D: FromStr>(&self, s: &str) -> Result<D>
    where <D as FromStr>::Err: std::error::Error + std::marker::Send + std::marker::Sync + 'static {
        let v = s.parse();
        match v {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::new(ErrorKind::InvalidInput,
                format!("Parse error.\n\tValue: '{}'\n\tError: {:?}, ", s, e))),
        }
    }
    fn __read_ascii_list<D: FromStr>(&self, elem_iter: &mut Iter<String>, count: usize) -> Result<Vec<D>>
        where <D as FromStr>::Err: std::error::Error + std::marker::Send + std::marker::Sync + 'static {
        let mut list = Vec::<D>::new();
        for i in 0..count {
            let s : &String = match elem_iter.next() {
                None => return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Couldn't find a list element at index {}.", i)
                )),
                Some(x) => x
            };
            let value : D = try!(self.parse(s));
            list.push(value);
        }
        Ok(list)
    }
}
