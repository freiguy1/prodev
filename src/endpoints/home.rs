use ::models::user::User;
use hyper::header;
use nickel::{Request, Response, MiddlewareResult}; //, FormBody};
use nickel::extensions::{Redirect};
use std::collections::HashMap;

pub fn handler<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let user = req.get_user();
    if user.is_none() {
        return res.redirect("/login");
    }
    let user = user.unwrap();

    let mut data = HashMap::new();
    data.insert("first_name", user.first_name);
    data.insert("last_name", user.last_name);
    data.insert("occupation", user.occupation);
    res.render("templates/home", &data)
}

pub trait UserCookie {
    fn get_user(&self) -> Option<User>;
}

impl<'mw, 'server, D> UserCookie for Request<'mw, 'server, D> {
    fn get_user(&self) -> Option<User> {
        self.origin.headers.get::<header::Cookie>()
            .and_then(|cookies| cookies.iter().find(|c| c.name == "id".to_string()))
            .and_then(|c| c.value.parse::<i32>().ok())
            .and_then(|id| ::models::user::get_by_id(id))
    }
}
