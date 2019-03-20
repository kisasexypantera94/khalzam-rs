use crate::db::Repository;
use crate::MusicLibrary;
use postgres::{Connection, TlsMode};
use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

type PgBigInt = i64;
type PgInteger = i32;

struct Candidate {
    song_id: i32,
    match_num: usize,
}

struct Table {
    /// highest number of matches among timedelta_best
    absolute_best: usize,
    /// highest number of matches for every timedelta
    timedelta_best: HashMap<i32, usize>,
}

struct Hash {
    hid: PgInteger,
    hash: PgBigInt,
    time: PgInteger,
    sid: PgInteger,
}

#[derive(Debug)]
pub struct PostgresRepo {
    conn: Arc<Mutex<Connection>>,
}

impl PostgresRepo {
    pub fn open(pg_url: &str) -> Result<MusicLibrary<PostgresRepo>, postgres::Error> {
        let conn = Arc::new(Mutex::new(Connection::connect(pg_url, TlsMode::None)?));
        Ok(MusicLibrary {
            repo: PostgresRepo { conn },
        })
    }
}

impl Repository for PostgresRepo {
    fn index(&self, song: &str, hash_array: &Vec<usize>) -> Result<(), Box<Error>> {
        let conn = self.conn.lock().unwrap();

        let sid: PgInteger = conn
            .query(
                "INSERT INTO songs(song) VALUES($1) returning sid;",
                &[&song],
            )?
            .get(0)
            .get("sid");

        for (time, hash) in hash_array.iter().enumerate() {
            conn.query(
                "INSERT INTO hashes(hash, time, sid) VALUES($1, $2, $3);",
                &[&(*hash as PgBigInt), &(time as PgInteger), &sid],
            )?;
        }

        Ok(())
    }

    fn find(&self, hash_array: &Vec<usize>) -> Result<Option<String>, Box<Error>> {
        let mut cnt = HashMap::<i32, Table>::new();

        let conn = self.conn.lock().unwrap();
        for (t, &h) in hash_array.iter().enumerate() {
            for row in &conn.query("SELECT * FROM hashes WHERE hash=$1;", &[&(h as PgBigInt)])? {
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

                if cnt
                    .get(&hash_row.sid)
                    .unwrap()
                    .timedelta_best
                    .get(&(hash_row.time - t as i32))
                    .unwrap()
                    > &cnt.get(&hash_row.sid).unwrap().absolute_best
                {
                    cnt.get_mut(&hash_row.sid).unwrap().absolute_best = *cnt
                        .get(&hash_row.sid)
                        .unwrap()
                        .timedelta_best
                        .get(&(hash_row.time - t as i32))
                        .unwrap();
                }
            }
        }

        let mut matchings = Vec::<Candidate>::new();
        for (song, table) in cnt {
            matchings.push(Candidate {
                song_id: song,
                match_num: table.absolute_best,
            });
        }
        if matchings.len() == 0 {
            return Ok(None);
        }
        matchings.sort_by_key(|a| Reverse(a.match_num));

        let song_name: String = conn
            .query(
                "SELECT song FROM songs WHERE sid=$1;",
                &[&matchings[0].song_id],
            )?
            .get(0)
            .get("song");
        Ok(Some(song_name))
    }

    fn delete(&self, song: &str) -> Result<(), Box<Error>> {
        Ok(())
    }
}
