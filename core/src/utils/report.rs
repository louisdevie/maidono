use super::{Error, Result};

pub struct Report {
    errors: Vec<Error>,
}

impl Report {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn wrap<T>(self, value: T) -> Result<T> {
        match self.errors.len() {
            0 => Ok(value),
            1 => Err(self.errors.into_iter().next().unwrap()),
            _ => Err(Error::Multiple(self.errors)),
        }
    }
}
