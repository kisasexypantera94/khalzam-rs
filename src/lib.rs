//! `khalzam` is an audio recognition library
//! that makes it easy to index and recognize audio files.
//! It focuses on speed, efficiency and simplicity.
//!
//! Its algrorithm is based on [this article].
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
    /// Create a new instance of `MusicLibrary`
    pub fn new(repo: T) -> MusicLibrary<T> {
        MusicLibrary {
            repo,
            fp_handle: FingerprintHandle::new(),
        }
    }

    /// Add song.
    pub fn add(&self, filename: &str) -> Result<(), Box<Error>> {
        check_extension(filename)?;

        let song = get_songname(filename)?;
        let hash_array = self.fp_handle.calc_fingerprint(filename)?;
        self.repo.index(&song, &hash_array)
    }

    /// Recognize song. It returns the songname of the closest match in repository.
    pub fn recognize(&self, filename: &str) -> Result<String, Box<Error>> {
        check_extension(filename)?;

        let hash_array = self.fp_handle.calc_fingerprint(filename)?;
        match self.repo.find(&hash_array)? {
            Some(res) => Ok(res),
            None => Ok("No matchings".to_string()),
        }
    }

    /// Delete song.
    pub fn delete(&self, songname: &str) -> Result<String, Box<Error>> {
        match self.repo.delete(songname)? {
            x if x > 0 => Ok("Successfully deleted".to_string()),
            _ => Ok("Song not found".to_string()),
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

/// Get file basename without extension
fn get_songname(filename: &str) -> Result<String, Box<Error>> {
    match Path::new(filename).file_stem() {
        Some(stem) => match stem.to_str() {
            Some(stem_str) => Ok(stem_str.to_string()),
            None => Err(Box::from(format!("can't convert {:?} to str", stem))),
        },
        None => Err(Box::from("filename is empty")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_extension() {
        assert!(check_extension("good.mp3").is_ok());
        assert!(check_extension("bad.pdf").is_err());
    }

    #[test]
    fn test_get_songname() {
        assert_eq!(get_songname("some_name.mp3").unwrap(), "some_name");
    }
}
