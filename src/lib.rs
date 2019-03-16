use std::error::Error;
use std::path::Path;

mod db;
use db::Repository;
mod fingerprint;

struct MusicLibrary<T>
where
    T: Repository,
{
    repo: T,
}

impl<T> MusicLibrary<T>
where
    T: Repository,
{
    fn add(&self, filename: &str) -> Result<(), Box<Error>> {
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
}

#[cfg(test)]
mod tests {}
