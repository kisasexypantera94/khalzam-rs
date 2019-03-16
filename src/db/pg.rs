use postgres::{Connection, TlsMode};

pub struct PostgresConfig<'a> {
    pub dbname: &'a str,
    pub user: &'a str,
    pub password: &'a str,
}

#[derive(Debug)]
pub struct PostgresRepo {
    conn: Connection,
}

impl PostgresRepo {
    pub fn open(cfg: &Config) -> Result<MusicLibrary, postgres::Error> {
        let addr = format!(
            "postgres://{}:{}@localhost/{}",
            cfg.user, cfg.password, cfg.dbname
        );
        let conn = Connection::connect(addr, TlsMode::None)?;
        Ok(MusicLibrary { conn })
    }
}