use std::error::Error;

pub mod pg;

pub trait Repository {
    fn index(&self, song: &str, hash_array: &Vec<usize>) -> Result<(), Box<Error>>;
    fn find(&self, hash_array: &Vec<usize>) -> Result<Option<String>, Box<Error>>;
    fn delete(&self, song: &str) -> Result<(), Box<Error>>;
}
