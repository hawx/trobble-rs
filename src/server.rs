extern crate r2d2;

use actix::{prelude::*, SystemRunner};
use actix_web::{http::Method, middleware::Logger, App, Responder, HttpRequest, server::HttpServer};
use data_actor::DatabaseExecutor;
use num_cpus;
use data;

pub struct Server {
    runner: SystemRunner,
}

pub struct State {
    pub database: Addr<DatabaseExecutor>,
}

fn greet(req: &HttpRequest<State>) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

impl Server {
    pub fn new() -> Result<Self, r2d2::Error> {
        let runner = actix::System::new("trobble-rs");
        
        let pool = data::connect()?;
        
        let db_addr = SyncArbiter::start(num_cpus::get(), move || DatabaseExecutor(pool.clone()));

        let server = HttpServer::new(move || {
            App::with_state(State { database: db_addr.clone() })
                .middleware(Logger::default())
                .resource("/", |r| r.method(Method::GET).f(greet)) })
            .bind("127.0.0.1:8080").unwrap()
            .start();

        Ok(Server { runner })
    }

    pub fn start(self) -> i32 {
        self.runner.run()
    }
}
