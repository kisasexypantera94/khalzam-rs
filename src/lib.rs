//! # Khalzam
//!
//! `khalzam` is an audio recognition library
//! that makes it easy to index and recognize audio files.
//! It focuses on speed, efficiency and simplicity.
//!
//! Its algrorithm is based on [this article]
//!
//! [this article]: https://royvanrijn.com/blog/2010/06/creating-shazam-in-java/
pub mod db;
mod fingerprint;

use db::Repository;
use fingerprint::FingerprintHandle;
use std::error::Error;
use std::path::Path;

/// `MusicLibrary` is the central structure of the algorithm.
/// It is the link for fingerprinting and repository interaction.
pub struct MusicLibrary<T>
where
    T: Repository,
{
    repo: T,
    fp_handle: FingerprintHandle,
}

impl<T> MusicLibrary<T>
where
    T: Repository,
{
    /// Add song.
    pub fn add(&self, filename: &str) -> Result<(), Box<Error>> {
        check_extension(filename)?;

        let song = match Path::new(filename).file_stem() {
            Some(stem) => match stem.to_str() {
                Some(stem_str) => stem_str,
                None => return Err(Box::from(format!("can't convert {:?} to str", stem))),
            },
            None => return Err(Box::from("filename is empty")),
        };
        let hash_array = self.fp_handle.calc_fingerprint(filename)?;
        self.repo.index(song, &hash_array)
    }

    /// Recognize song. It returns the songname of the closest match in repository.
    pub fn recognize(&self, filename: &str) -> Result<String, Box<Error>> {
        check_extension(filename)?;

        let hash_array = self.fp_handle.calc_fingerprint(filename)?;
        match self.repo.find(&hash_array) {
            Ok(opt) => match opt {
                Some(res) => Ok(res),
                None => Ok("No matchings".to_string()),
            },
            Err(e) => Err(e),
        }
    }

    /// Delete song.
    pub fn delete(&self, songname: &str) -> Result<String, Box<Error>> {
        match self.repo.delete(songname) {
            Ok(x) if x > 0 => Ok("Successfully deleted".to_string()),
            Ok(_) => Ok("Song not found".to_string()),
            Err(e) => Err(Box::from(e)),
        }
    }
}

/// Сheck whether it is possible to process a file.
fn check_extension(filename: &str) -> Result<(), Box<Error>> {
    let path = Path::new(filename);
    let ext = match path.extension() {
        Some(e_osstr) => match e_osstr.to_str() {
            Some(e) => e,
            None => return Err(Box::from("Invalid extension")),
        },
        None => return Err(Box::from("Invalid extension")),
    };
    if ext != "mp3" {
        return Err(Box::from("Invalid extension"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
