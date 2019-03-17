use std::error::Error;

pub mod pg;

pub trait Repository {
    fn index(&self, song: &str, hash_array: &Vec<usize>) -> Result<(), Box<Error>>;
    fn find(&self, filename: &str) -> Result<String, Box<Error>>;
    fn delete(&self, song: &str) -> Result<(), Box<Error>>;
}
