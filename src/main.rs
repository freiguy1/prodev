#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate hyper;
extern crate cookie;

mod endpoints;
mod models;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    set_up_endpoints(&mut server);

    server.listen("127.0.0.1:6767").ok().expect("Unable to host site");
}

fn set_up_endpoints(server: &mut Nickel) {
    server.get("/", middleware! { |_req, res|
        let data: HashMap<&'static str, &'static str> = HashMap::new();
        return res.render("templates/landing", &data)
    });

    server.get("/login", endpoints::login::handler);
    server.post("/login", endpoints::login::post_handler);
    server.get("/home", endpoints::home::handler);
}
