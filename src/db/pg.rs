use crate::db::Repository;
use crate::MusicLibrary;
use postgres::{Connection, TlsMode};
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct PostgresConfig<'a> {
    pub dbname: &'a str,
    pub user: &'a str,
    pub password: &'a str,
}

#[derive(Debug)]
pub struct PostgresRepo {
    conn: Arc<Mutex<Connection>>,
}

impl PostgresRepo {
    pub fn open(cfg: &PostgresConfig) -> Result<MusicLibrary<PostgresRepo>, postgres::Error> {
        let addr = format!(
            "postgres://{}:{}@localhost/{}",
            cfg.user, cfg.password, cfg.dbname
        );
        let conn = Arc::new(Mutex::new(Connection::connect(addr, TlsMode::None)?));
        Ok(MusicLibrary {
            repo: PostgresRepo { conn },
        })
    }
}

impl Repository for PostgresRepo {
    fn index(&self, song: &str, hash_array: &Vec<usize>) -> Result<(), Box<Error>> {
        let conn = self.conn.lock().unwrap();
        let sid: i32 = conn
            .query(
                "INSERT INTO songs(song) VALUES($1) returning sid;",
                &[&song],
            )?
            .get(0)
            .get(0);

        for (time, hash) in hash_array.iter().enumerate() {
            conn.query(
                "INSERT INTO hashes(hash, time, sid) VALUES($1, $2, $3);",
                &[&(*hash as i64), &(time as i32), &sid],
            )?;
        }

        Ok(())
    }
    fn find(&self, filename: &str) -> Result<String, Box<Error>> {
        Ok("".to_string())
    }
    fn delete(&self, song: &str) -> Result<(), Box<Error>> {
        Ok(())
    }
}
