#[macro_use] extern crate nickel;
extern crate nickel_sqlite;
extern crate rustc_serialize;
extern crate hyper;
extern crate cookie;
extern crate rusqlite;
extern crate r2d2;
extern crate r2d2_sqlite;

mod api;
mod auth;
mod endpoints;
mod models;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler, Mountable};
use nickel_sqlite::SqliteMiddleware;
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn main() {
    let mut server = Nickel::new();

    server.utilize(initialize_database());

    set_up_endpoints(&mut server);
    set_up_api(&mut server);

    server.mount("/assets/", StaticFilesHandler::new("assets"));

    server.listen("0.0.0.0:6767").ok().expect("Unable to host site");
}

fn set_up_endpoints(server: &mut Nickel) {
    server.get("/", middleware! { |_req, res|
        let data: HashMap<&'static str, &'static str> = HashMap::new();
        return res.render("templates/landing", &data)
    });

    server.get("/login", endpoints::login::handler);
    server.post("/login", endpoints::login::post_handler);
    server.get("/home", endpoints::home::handler);
    server.get("/worksheets/:year", endpoints::worksheet::handler);
}

fn set_up_api(server: &mut Nickel) {
    server.post("/api/worksheets/:year/activities", api::add_activity::handler);
}

fn initialize_database() -> SqliteMiddleware {
    let db_url = "file.db";
    let db_path = Path::new(db_url);
    if db_path.exists() {
        println!("Database exists, using it");
        let mw = SqliteMiddleware::new(&db_url).expect("Unable to connect to database");
        mw
    } else {
        let mut db_init_script = String::new();
        let mut f = File::open("init.sql").expect("Cannot open database initialization script");
        f.read_to_string(&mut db_init_script).expect("Cannot read from database initialization script");
        let mw = SqliteMiddleware::new(&db_url).expect("Unable to connect to database");
        let db = mw.pool.clone().get().unwrap();
        println!("Running database initialization script");
        match db.execute_batch(&db_init_script) {
            Ok(_) => println!("Database initialization script successful"),
            Err(e) => println!("Error running database initialization script:\n{}", e)
        };
        mw
    }
}
