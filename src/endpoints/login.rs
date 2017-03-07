use hyper::header::SetCookie;
use nickel::{Request, Response, MiddlewareResult, FormBody};
use nickel::extensions::{Redirect};
use std::collections::HashMap;

pub fn handler<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let data: HashMap<&'static str, &'static str> = HashMap::new();
    res.render("templates/login", &data)
}

pub fn post_handler<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let form_data = try_with!(res, req.form_body());

    let email = form_data.get("email");
    let password = form_data.get("password");
    if email.is_none() && password.is_none() {
        return res.send("Crappy input");
    }
    let email = email.unwrap();
    let password = password.unwrap();

    let user = ::models::user::get_by_email_password(email, password);

    if user.is_none() {
        let mut data = HashMap::new();
        data.insert("message", "Incorrect username password combination");
        return res.render("templates/login", &data);
    }
    let user = user.unwrap();

    let cookie = ::hyper::header::CookiePair::new("id".to_string(), user.id.to_string());
    res.set(SetCookie(vec![cookie]));
    res.redirect("/home")
}
