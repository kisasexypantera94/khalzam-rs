//! `pg` module implements `Repository` with PostgreSQL database and takes care
//! of direct interaction with database.
use crate::db::Repository;

use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;

use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;

type PgBigInt = i64;
type PgInteger = i32;

struct Candidate {
    song_id: i32,
    match_num: usize,
}

/// `Table` is used as a counter structure to find the most similar songs in database.
struct Table {
    /// highest number of matches among timedelta_best
    absolute_best: usize,
    /// highest number of matches for every timedelta
    timedelta_best: HashMap<i32, usize>,
}

#[allow(dead_code)]
struct Hash {
    hid: PgInteger,
    hash: PgBigInt,
    time: PgInteger,
    sid: PgInteger,
}

/// `PostgresRepo` is an implementation of `Repository` interface.
pub struct PostgresRepo {
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
}

impl PostgresRepo {
    /// Connect to postgres database
    pub fn open(config: &str) -> Result<PostgresRepo, postgres::Error> {
        let manager = PostgresConnectionManager::new(config.parse()?, NoTls);
        let pool = r2d2::Pool::new(manager).unwrap();
        Ok(PostgresRepo { pool })
    }
}

impl Repository for PostgresRepo {
    fn index(&self, song: &str, hash_array: &[usize]) -> Result<(), Box<Error>> {
        let mut conn = self.pool.clone().get().unwrap();

        let sid: PgInteger = conn
            .query(
                "INSERT INTO songs(song) VALUES($1) returning sid;",
                &[&song],
            )?
            .get(0)
            .unwrap()
            .get("sid");

        let stmt = conn.prepare("INSERT INTO hashes(hash, time, sid) VALUES($1, $2, $3);")?;
        for (time, hash) in hash_array.iter().enumerate() {
            conn.query(&stmt, &[&(*hash as PgBigInt), &(time as PgInteger), &sid])?;
        }

        Ok(())
    }

    fn find(&self, hash_array: &[usize]) -> Result<Option<String>, Box<Error>> {
        let mut conn = self.pool.clone().get().unwrap();

        let mut cnt = HashMap::<i32, Table>::new();
        let stmt = conn.prepare("SELECT * FROM hashes WHERE hash=$1;")?;

        for (t, &h) in hash_array.iter().enumerate() {
            let result = conn.query(&stmt, &[&(h as PgBigInt)])?;
            for row in &result {
                let hash_row = Hash {
                    hid: row.get("hid"),
                    hash: row.get("hash"),
                    time: row.get("time"),
                    sid: row.get("sid"),
                };

                *cnt.entry(hash_row.sid)
                    .or_insert(Table {
                        absolute_best: 0,
                        timedelta_best: HashMap::new(),
                    })
                    .timedelta_best
                    .entry(hash_row.time - t as i32)
                    .or_insert(0) += 1;

                if cnt[&(hash_row.sid)].timedelta_best[&(hash_row.time - t as i32)]
                    > cnt[&hash_row.sid].absolute_best
                {
                    cnt.get_mut(&hash_row.sid).unwrap().absolute_best =
                        cnt[&hash_row.sid].timedelta_best[&(hash_row.time - t as i32)]
                }
            }
        }

        if cnt.is_empty() {
            return Ok(None);
        }

        let mut matchings = Vec::<Candidate>::new();
        for (song, table) in cnt {
            matchings.push(Candidate {
                song_id: song,
                match_num: table.absolute_best,
            });
        }

        matchings.sort_by_key(|a| Reverse(a.match_num));

        let song_name: String = conn
            .query(
                "SELECT song FROM songs WHERE sid=$1;",
                &[&matchings[0].song_id],
            )?
            .get(0)
            .unwrap()
            .get("song");
        let similarity = (100.0 * matchings[0].match_num as f64 / hash_array.len() as f64) as isize;
        Ok(Some(format!("{} ({}% matched)", song_name, similarity)))
    }

    fn delete(&self, song: &str) -> Result<u64, Box<Error>> {
        let mut conn = self.pool.clone().get().unwrap();
        match conn.execute("DELETE FROM songs WHERE song=$1;", &[&song]) {
            Ok(affected) => Ok(affected),
            Err(e) => Err(Box::from(e)),
        }
    }
}
