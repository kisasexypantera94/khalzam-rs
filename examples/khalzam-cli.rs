use futures;
use futures::future::Future;
use khalzam::db::pg::PostgresRepo;
use shrust::{Shell, ShellIO};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio_threadpool::ThreadPool;

fn main() {
    let pgrepo =
        Arc::new(PostgresRepo::open("postgres://kisasexypantera94:@localhost/khalzam").unwrap());

    let mut shell = Shell::new(pgrepo);
    shell.new_command("add", "Add song to database", 1, move |io, repo, args| {
        let path = Path::new(args[0]);
        let name = path.file_name().unwrap().to_str().unwrap();
        let path = String::from(path.to_str().unwrap());
        match repo.add(&path) {
            Ok(()) => {
                let _ = writeln!(io, "Added {}", name,);
            }
            Err(e) => {
                writeln!(io, "Can't add {}: {}", name, e)?;
            }
        };
        Ok(())
    });
    shell.new_command("recognize", "Recognize song", 1, move |io, repo, args| {
        let path = Path::new(args[0]);
        let path = String::from(path.to_str().unwrap());
        writeln!(io, "Recognizing `{}` ...", path)?;
        match repo.recognize(&path) {
            Ok(res) => writeln!(io, "Best match: {}", res)?,
            Err(e) => writeln!(io, "Error: {}", e)?,
        };
        Ok(())
    });
    shell.new_command(
        "delete",
        "Delete song from database",
        1,
        move |io, repo, args| {
            let path = Path::new(args[0]);
            let path = String::from(path.to_str().unwrap());
            writeln!(io, "{}", repo.delete(&path).unwrap())?;
            match repo.delete(&path) {
                Ok(res) => writeln!(io, "{}", res)?,
                Err(e) => writeln!(io, "Error: {}", e)?,
            };
            Ok(())
        },
    );
    shell.new_command("add_dir", "Add directory", 1, move |io, repo, args| {
        let rt = ThreadPool::new();
        let resources = fs::read_dir(args[0]).unwrap();
        for path in resources {
            if let Ok(path) = path {
                let repo = repo.clone();
                let mut io = io.clone();
                rt.spawn(futures::lazy(move || {
                    let name = String::from(path.path().file_name().unwrap().to_str().unwrap());
                    let path = String::from(path.path().to_str().unwrap());
                    match repo.add(&path) {
                        Ok(()) => {
                            let _ = writeln!(io, "Added {}", name,);
                        }
                        Err(e) => {
                            writeln!(io, "Can't add {}: {}", name, e);
                        }
                    };
                    Ok(())
                }));
            }
        }
        rt.shutdown().wait().unwrap();
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}
