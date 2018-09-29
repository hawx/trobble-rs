extern crate r2d2;
extern crate rusqlite;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Scrobble {
    pub artist: String,
    pub album_artist: String,
    pub album: String,
    pub track: String,
    pub timestamp: i64
}

#[derive(Debug)]
pub struct NowPlaying {
    pub id: i64,
    pub artist: String,
    pub album_artist: String,
    pub album: String,
    pub track: String,
    pub timestamp: i64
}

pub fn connect() -> Result<Pool<SqliteConnectionManager>, r2d2::Error> {
    let manager = SqliteConnectionManager::memory();
    Pool::new(manager)
}

pub fn up(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute("CREATE TABLE IF NOT EXISTS scrobbles (
    Artist      TEXT,
    AlbumArtist TEXT,
    Album       TEXT,
    Track       TEXT,
    Timestamp   INTEGER PRIMARY KEY
  );
  CREATE TABLE IF NOT EXISTS nowplaying (
    Id          INTEGER PRIMARY KEY,
    Artist      TEXT,
    AlbumArtist TEXT,
    Album       TEXT,
    Track       TEXT,
    Timestamp   INTEGER
  );
  INSERT OR IGNORE INTO nowplaying VALUES(1, \"\", \"\", \"\", \"\", 0);
", &[])
}

pub fn add(conn: &Connection, scrobble: Scrobble) -> Result<usize, rusqlite::Error> {
    conn.execute("INSERT INTO scrobbles VALUES(?, ?, ?, ?, ?)", &[
        &scrobble.artist,
        &scrobble.album_artist,
        &scrobble.album,
        &scrobble.track,
        &scrobble.timestamp
    ])
}

pub fn recently_played(conn: &Connection) -> Result<Vec<Scrobble>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT Artist, Album, AlbumArtist, Track, Timestamp
  FROM scrobbles
  ORDER BY Timestamp DESC
  LIMIT 10")?;
    
    let rows = stmt.query_map(&[], |row| {
        Scrobble {
            artist: row.get(0),
            album_artist: row.get(1),
            album: row.get(2),
            track: row.get(3),
            timestamp: row.get(4)
        }
    })?;

    let mut scrobbles = Vec::new();
    for row in rows {
        match row {
            Ok(value) => scrobbles.push(value),
            Err(_) => {}
        }
    }

    Ok(scrobbles)
}
