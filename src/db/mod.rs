use std::error::Error;

pub trait Repository {
    fn index(&self, song: &str, hash_array: &Vec<usize>) -> Result<(), Box<Error>>;
    fn recognize(&self, filename: &str) -> Result<String, Box<Error>>;
    fn delete(&self, song: &str) -> Result<(), Box<Error>>;
}
