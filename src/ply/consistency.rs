//! Allows a `Ply` object to be checked for consistency.

use std::fmt::{ Display, Formatter };
use std::fmt;
use std::error;
use super::Ply;
use super::PropertyAccess;

/// Contains a description, why a given `Ply` object isn't consistent and could not be made consistent.
#[derive(Debug)]
pub struct ConsistencyError {
    /// Describes in natural language, why a consistency check failed.
    description: String,
}
impl ConsistencyError {
    /// Create a new error object with a given description of the problem.
    pub fn new(description: &str) -> Self {
        ConsistencyError {
            description: description.to_string(),
        }
    }
}

impl Display for ConsistencyError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&format!("ConsistencyError: {}", self.description))
    }
}

impl error::Error for ConsistencyError {
    fn description(&self) -> &str {
        &self.description
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

fn has_white_space(s: &str) -> bool {
    return s.contains(" ") || s.contains("\t");
}

fn has_line_break(s: &str) -> bool {
    return s.contains("\n") || s.contains("\r");
}

impl<E: PropertyAccess> Ply<E>{
    /// Takes a mutable `Ply` object, performs common operations to make it consistent,
    ///
    /// When written, a consistent `Ply` object generates a valid PLY file.
    /// This method also checks for invariants that can't be fixed automatically.
    /// If something can not be fixed automatically, it returns a `ConsistencyError` describing the problem.
    ///
    /// # Remarks
    ///
    /// This method should always be called before writing to a file with `Writer`.
    /// Only exception is `write_ply()`, which, for convenience, performs the check itself.
    /// See `write_ply_unchecked()` for a variant that expects the client to assure consistency.
    ///
    /// No checks on encoding are performed.
    /// For maximal compatability, only ascii characters should be used but this is not checked.
    /// Every relevant string is checked to not contain line breaks.
    /// Identifiers are also checked to not contain white spaces.
    pub fn make_consistent(&mut self) -> Result<(), ConsistencyError>{
        for (ek, _) in &self.header.elements {
            if !self.payload.contains_key(ek) {
                self.payload.insert(ek.clone(), Vec::new());
            }
        }
        for (pk, pe) in &self.payload {
            if pk.is_empty() {
                return Err(ConsistencyError::new("Element cannot have empty name."));
            }
            let ed = self.header.elements.get_mut(pk);
            if ed.is_none() {
                return Err(ConsistencyError::new(&format!("No decleration for element `{}` found.", pk)));
            }
            ed.unwrap().count = pe.len();
        }
        for ref oi in &self.header.obj_infos {
            if has_line_break(oi) {
                return Err(ConsistencyError::new(&format!("Objection information `{}` should not contain any line breaks.", oi)));
            }
        }
        for ref c in &self.header.comments {
            if has_line_break(&c) {
               return Err(ConsistencyError::new(&format!("Comment `{}` should not contain any line breaks.", c)));
            }
        }
        for (_, ref e) in &self.header.elements {
            if has_line_break(&e.name) {
                return Err(ConsistencyError::new(&format!("Name of element `{}` should not contain any line breaks.", e.name)));
            }
            if has_white_space(&e.name) {
                return Err(ConsistencyError::new(&format!("Name of element `{}` should not contain any white spaces.", e.name)));
            }
            for (_, ref p) in &e.properties {
                if has_line_break(&p.name) {
                    return Err(ConsistencyError::new(&format!("Name of property `{}` of element `{}` should not contain any line breaks.", p.name, e.name)));
                }
                if has_white_space(&p.name) {
                    return Err(ConsistencyError::new(&format!("Name of property `{}` of element `{}` should not contain any spaces.", p.name, e.name)));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    type P = Ply<DefaultElement>;
    #[test]
    fn consistent_new_line_fail_comment() {
        let mut p = P::new();
        p.header.comments.push("a beautiful\r\nnew line!".to_string());
        let r = p.make_consistent();
        assert!(r.is_err());
    }
    #[test]
    fn consistent_new_line_fail_obj_infos() {
        let mut p = P::new();
        p.header.obj_infos.push("some\r\nnew line!".to_string());
        let r = p.make_consistent();
        assert!(r.is_err());
    }
    #[test]
    fn consistent_new_line_fail_element() {
        let mut p = P::new();
        p.header.elements.add(ElementDef::new("new\nline".to_string()));
        let r = p.make_consistent();
        assert!(r.is_err());
    }
    #[test]
    fn consistent_new_line_fail_property () {
        let mut p = P::new();
        let mut e = ElementDef::new("ok".to_string());
        e.properties.add(PropertyDef::new("prop\nwith new line".to_string(), PropertyType::Scalar(ScalarType::Char)));
        p.header.elements.add(e);
        let r = p.make_consistent();
        assert!(r.is_err());
    }
    #[test]
    fn consistent_white_space_fail_element() {
        let mut p = P::new();
        p.header.elements.add(ElementDef::new("white space".to_string()));
        let r = p.make_consistent();
        assert!(r.is_err());
    }
    #[test]
    fn consistent_white_space_fail_property(){
        let mut p = P::new();
        let mut e = ElementDef::new("ok".to_string());
        e.properties.add(PropertyDef::new("prop\twhite space".to_string(), PropertyType::Scalar(ScalarType::Char)));
        p.header.elements.add(e);
        let r = p.make_consistent();
        assert!(r.is_err());
    }
}
