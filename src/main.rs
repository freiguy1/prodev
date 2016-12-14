#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate hyper;
extern crate cookie;

mod api;
mod auth;
mod endpoints;
mod models;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    set_up_endpoints(&mut server);
    set_up_api(&mut server);

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
