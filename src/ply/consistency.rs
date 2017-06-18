use std::fmt::{ Display, Formatter };
use std::fmt;
use std::error;
use super::Ply;

#[derive(Debug)]
pub struct ConsistencyError {
    description: String,
}
impl ConsistencyError {
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


impl<E> Ply<E>{
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
