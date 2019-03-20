use std::error::Error;

pub mod pg;

pub trait Repository {
    fn index(&self, song: &str, hash_array: &[usize]) -> Result<(), Box<Error>>;
    fn find(&self, hash_array: &[usize]) -> Result<Option<String>, Box<Error>>;
    fn delete(&self, song: &str) -> Result<(), Box<Error>>;
}
