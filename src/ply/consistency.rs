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
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}


impl<E: PropertyAccess> Ply<E>{
    /// Takes a mutable `Ply` object, performs commen operations to make it consistent,
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
        Ok(())
    }
}
