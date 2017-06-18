use ply::{ PropertyAccess, ElementDef, PropertyType, ScalarType };
use super::Writer;

use std::io::{ Write, Result, Error, ErrorKind };

use byteorder::{ WriteBytesExt, ByteOrder };


macro_rules! get_prop(
    // TODO: errror
    ($e:expr) => (match $e {None => return Ok(17), Some(x) => x})
);

impl<E: PropertyAccess> Writer<E> {
    // private payload
    pub fn __write_binary_element<T: Write, B: ByteOrder>(&self, out: &mut T, element: &E, element_def: &ElementDef) -> Result<usize> {
        let mut written = 0;
        for (k, property_def) in &element_def.properties {
            match property_def.data_type {
                PropertyType::Scalar(ref scalar_type) => {
                    written += match *scalar_type {
                        ScalarType::Char => {try!(out.write_i8(get_prop!(element.get_char(k)))); 1},
                        ScalarType::UChar => {try!(out.write_u8(get_prop!(element.get_uchar(k)))); 1},
                        ScalarType::Short => {try!(out.write_i16::<B>(get_prop!(element.get_short(k)))); 2},
                        ScalarType::UShort => {try!(out.write_u16::<B>(get_prop!(element.get_ushort(k)))); 2},
                        ScalarType::Int => {try!(out.write_i32::<B>(get_prop!(element.get_int(k)))); 4},
                        ScalarType::UInt => {try!(out.write_u32::<B>(get_prop!(element.get_uint(k)))); 4},
                        ScalarType::Float => {try!(out.write_f32::<B>(get_prop!(element.get_float(k)))); 4},
                        ScalarType::Double => {try!(out.write_f64::<B>(get_prop!(element.get_double(k)))); 8},
                    };
                },
                PropertyType::List(ref index_type, ref scalar_type) => {
                    let vec_len = element_def.count;
                    written += match *index_type {
                        ScalarType::Char => {try!(out.write_i8(vec_len as i8)); 1},
                        ScalarType::UChar => {try!(out.write_u8(vec_len as u8)); 1},
                        ScalarType::Short => {try!(out.write_i16::<B>(vec_len as i16)); 2},
                        ScalarType::UShort => {try!(out.write_u16::<B>(vec_len as u16)); 2},
                        ScalarType::Int => {try!(out.write_i32::<B>(vec_len as i32)); 4},
                        ScalarType::UInt => {try!(out.write_u32::<B>(vec_len as u32)); 4},
                        ScalarType::Float => return Err(Error::new(ErrorKind::InvalidInput, "Index of list must be an integer type, float declared in PropertyType.")),
                        ScalarType::Double => return Err(Error::new(ErrorKind::InvalidInput, "Index of list must be an integer type, double declared in PropertyType.")),
                    };

                    written += match *scalar_type {
                        ScalarType::Char => try!(self.write_binary_list::<T, i8, B>(get_prop!(element.get_list_char(k)), out, &|o, x| {try!(o.write_i8(*x)); Ok(1)} )),
                        ScalarType::UChar => try!(self.write_binary_list::<T, u8, B>(get_prop!(element.get_list_uchar(k)), out, &|o, x| {try!(o.write_u8(*x)); Ok(1)} )),
                        ScalarType::Short => try!(self.write_binary_list::<T, i16, B>(get_prop!(element.get_list_short(k)), out, &|o, x| {try!(o.write_i16::<B>(*x)); Ok(2)} )),
                        ScalarType::UShort => try!(self.write_binary_list::<T, u16, B>(get_prop!(element.get_list_ushort(k)), out, &|o, x| {try!(o.write_u16::<B>(*x)); Ok(2)} )),
                        ScalarType::Int => try!(self.write_binary_list::<T, i32, B>(get_prop!(element.get_list_int(k)), out, &|o, x| {try!(o.write_i32::<B>(*x)); Ok(4)} )),
                        ScalarType::UInt => try!(self.write_binary_list::<T, u32, B>(get_prop!(element.get_list_uint(k)), out, &|o, x| {try!(o.write_u32::<B>(*x)); Ok(4)} )),
                        ScalarType::Float => try!(self.write_binary_list::<T, f32, B>(get_prop!(element.get_list_float(k)), out, &|o, x| {try!(o.write_f32::<B>(*x)); Ok(4)} )),
                        ScalarType::Double => try!(self.write_binary_list::<T, f64, B>(get_prop!(element.get_list_double(k)), out, &|o, x| {try!(o.write_f64::<B>(*x)); Ok(8)} )),
                    }
                }
            }
        };
        Ok(written)
    }
    fn write_binary_list<T: Write, D, B: ByteOrder>(&self, list: &[D], out: &mut T, out_val: &Fn(&mut T, &D) -> Result<usize>) -> Result<usize> {
        let mut written = 0;
        for v in list {
            written += try!(out_val(out, v));
        }
        Ok(written)
    }
}
