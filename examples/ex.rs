use khalzam::db::pg::PostgresRepo;
use khalzam::MusicLibrary;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use std::fs;
use std::io::Write;
use std::sync::Arc;

fn main() {
    let pgrepo = PostgresRepo::open("postgres://kisasexypantera94:@localhost/khalzam").unwrap();
    let m_lib = Arc::new(MusicLibrary::new(pgrepo));

    let resources = fs::read_dir("../assets/resources").unwrap();
    let paths: Vec<_> = resources.collect();
    paths.par_iter().for_each(|path| {
        if let Ok(path) = path {
            let name = String::from(path.path().file_name().unwrap().to_str().unwrap());
            let path = String::from(path.path().to_str().unwrap());
            let stdout = std::io::stdout();
            match m_lib.add(&path) {
                Ok(()) => writeln!(&mut stdout.lock(), "Added {}", name),
                Err(e) => writeln!(&mut stdout.lock(), "Can't add {}: {}", name, e),
            }
            .unwrap();
        }
    });

    let samples = fs::read_dir("../assets/samples").unwrap();
    for path in samples {
        if let Ok(path) = path {
            let name = String::from(path.path().file_name().unwrap().to_str().unwrap());
            let path = String::from(path.path().to_str().unwrap());
            println!("Recognizing `{}` ...", name);
            println!("Best match: {}", m_lib.recognize(&path).unwrap());
        }
    }
}
