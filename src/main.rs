extern crate actix;
extern crate actix_web;
extern crate failure;
extern crate num_cpus;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

mod data;
mod data_actor;
mod server;

use failure::Error;
use server::Server;
use std::process::exit;

fn main() -> Result<(), Error> {
    // router.get("/", index, "index");
    // router.get("/feed", feed, "feed");
    // router.get("/played", played, "played");
    // router.get("/artist/:artist", artist, "artist");
    // router.get("/artist/:artist/:album", album, "album");
    // router.get("/artist/:artist/:album/:track", track, "track");

	  // route.HandleFunc("/styles.css", func(w http.ResponseWriter, r *http.Request) {
		//     w.Header().Add("Content-Type", "text/css")
		//         fmt.Fprint(w, views.Styles)
	  // })

	  // auth := handlers.NewAuth(*username, *apiKey, *secret)
    // route.Handle("/scrobble/*any", handlers.Scrobble(auth, db))

    let server = Server::new()?;
    exit(server.start())
}
