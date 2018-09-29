extern crate rusqlite;

use actix::prelude::*;
use data::{Scrobble, add, recently_played};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub struct DatabaseExecutor(pub Pool<SqliteConnectionManager>);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

pub struct AddMessage {
    scrobble: Scrobble,
}

impl Message for AddMessage {
    type Result = Result<(), rusqlite::Error>;
}

impl Handler<AddMessage> for DatabaseExecutor {
    type Result = Result<(), rusqlite::Error>;

    fn handle(&mut self, msg: AddMessage, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().unwrap();
        add(&conn, msg.scrobble).map(|_| ())
    }
}

pub struct RecentlyPlayedMessage();

impl Message for RecentlyPlayedMessage {
    type Result = Result<Vec<Scrobble>, rusqlite::Error>;
}

impl Handler<RecentlyPlayedMessage> for DatabaseExecutor {
    type Result = Result<Vec<Scrobble>, rusqlite::Error>;

    fn handle(&mut self, msg: RecentlyPlayedMessage, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().unwrap();
        recently_played(&conn)
    }
}
