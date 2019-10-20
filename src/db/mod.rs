//! `db` module takes care of interaction with repository/database.
pub mod pg;

use std::error::Error;

/// `Repository` is an abstraction of database containing fingerprints.
pub trait Repository {
    /// Map hashes from hash_array to song.
    fn index(&self, song: &str, hash_array: &[usize]) -> Result<(), Box<dyn Error>>;
    /// Find the most similar song by hashes.
    fn find(&self, hash_array: &[usize]) -> Result<Option<String>, Box<dyn Error>>; // It may be better to replace String result with generic.
    /// Delete song from database.
    fn delete(&self, song: &str) -> Result<u64, Box<dyn Error>>;
}
