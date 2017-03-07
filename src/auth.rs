use ::models::user::User;
use hyper::header;
use nickel::Request;

// This trait is implemented for nickel's Request to get user information from cookie
pub trait UserCookie {
    fn get_user(&self) -> Option<User>;
}

impl<'mw, 'server, D> UserCookie for Request<'mw, 'server, D> {
    fn get_user(&self) -> Option<User> {
        self.origin.headers.get::<header::Cookie>()
            .and_then(|cookies| cookies.iter().find(|c| c.value == "id".to_string()))
            .and_then(|c| c.value.parse::<i32>().ok())
            .and_then(|id| ::models::user::get_by_id(id))
    }
}
