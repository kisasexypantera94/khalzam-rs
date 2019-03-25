use futures;
use futures::future::Future;
use khalzam::db::pg::PostgresRepo;
use std::fs;
use std::io::Write;
use std::sync::Arc;
use tokio_threadpool::ThreadPool;

fn main() {
    let pgrepo =
        Arc::new(PostgresRepo::open("postgres://kisasexypantera94:@localhost/khalzam").unwrap());

    let rt = ThreadPool::new();
    let resources = fs::read_dir("../assets/resources").unwrap();
    for path in resources {
        if let Ok(path) = path {
            let repo = pgrepo.clone();
            rt.spawn(futures::lazy(move || {
                let name = String::from(path.path().file_name().unwrap().to_str().unwrap());
                let path = String::from(path.path().to_str().unwrap());
                match repo.add(&path) {
                    Ok(()) => {
                        let stdout = std::io::stdout();
                        let _ = writeln!(&mut stdout.lock(), "Added {}", name,);
                    }
                    Err(e) => println!("Can't add {}: {}", name, e),
                }
                Ok(())
            }));
        }
    }

    rt.shutdown().wait().unwrap();

    let samples = fs::read_dir("../assets/samples").unwrap();
    for path in samples {
        if let Ok(path) = path {
            let name = String::from(path.path().file_name().unwrap().to_str().unwrap());
            let path = String::from(path.path().to_str().unwrap());
            println!("Recognizing `{}` ...", name);
            println!("Best match: {}", pgrepo.recognize(&path).unwrap());
        }
    }
}
