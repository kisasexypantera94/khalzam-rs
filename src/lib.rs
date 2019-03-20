pub mod db;
mod fingerprint;

use db::Repository;
use std::error::Error;
use std::path::Path;

pub struct MusicLibrary<T>
where
    T: Repository,
{
    repo: T,
}

impl<T> MusicLibrary<T>
where
    T: Repository,
{
    /// Add song
    pub fn add(&self, filename: &str) -> Result<(), Box<Error>> {
        let hash_array = fingerprint::calc_fingerprint(filename)?;

        let song = match Path::new(filename).file_stem() {
            Some(stem) => match stem.to_str() {
                Some(stem_str) => stem_str,
                None => return Err(Box::from(format!("can't convert {:?} to str", stem))),
            },
            None => return Err(Box::from("filename is empty")),
        };

        self.repo.index(song, &hash_array)
    }

    /// Recognize song
    pub fn recognize(&self, filename: &str) -> Result<String, Box<Error>> {
        let hash_array = fingerprint::calc_fingerprint(filename)?;

        match self.repo.find(&hash_array) {
            Ok(opt) => match opt {
                Some(res) => Ok(res),
                None => Ok(String::from("No matchings")),
            },
            Err(e) => Err(Box::from(e)),
        }
    }
}

#[cfg(test)]
mod tests {}
