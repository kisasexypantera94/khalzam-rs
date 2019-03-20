use futures;
use futures::future::Future;
use khalzam::db::pg::PostgresRepo;
use std::fs;
use std::sync::Arc;
use tokio_threadpool::ThreadPool;

fn main() {
    let pgrepo =
        Arc::new(PostgresRepo::open("postgres://kisasexypantera94:@localhost/khalzam").unwrap());

    let rt = ThreadPool::new();
    let resources = fs::read_dir("../resources").unwrap();
    for path in resources {
        let repo = pgrepo.clone();
        rt.spawn(futures::lazy(move || {
            repo.add(path.unwrap().path().to_str().unwrap()).unwrap();
            Ok(())
        }));
    }

    let samples = fs::read_dir("../samples").unwrap();
    for path in samples {
        let name = path.as_ref().unwrap();
        print!("{:?} ", name);

        println!("{:?}", pgrepo.recognize(name.path().to_str().unwrap()));
    }

    rt.shutdown_on_idle().wait().unwrap();
}
