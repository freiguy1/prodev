use ::models::user::User;
use hyper::header;
use nickel::{Request, Response, MiddlewareResult}; //, FormBody};
use nickel::extensions::{Redirect};

#[derive(RustcEncodable)]
struct HomeModel {
    first_name: String,
    last_name: String,
    occupation: String,
    years: Vec<YearModel>
}

#[derive(RustcEncodable)]
struct YearModel {
    year: i32,
    activities: Vec<ActivityModel>
}

#[derive(RustcEncodable)]
struct ActivityModel {
    points: i32,
    description: String
}

pub fn handler<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let user = req.get_user();
    if user.is_none() {
        return res.redirect("/login");
    }
    let user = user.unwrap();

    let activities = ::models::activity::get_activities_by_user(user.id);
    let mut years: Vec<YearModel> = Vec::new();
    for a in activities.iter() {
        let activity = ActivityModel {
            points: a.points,
            description: a.description.clone()
        };
        if let Some(year) = years.iter_mut().find(|y| y.year == a.year) {
            year.activities.push(activity);
            continue;
        }
        let year = YearModel {
            year: a.year,
            activities: vec![activity]
        };
        years.push(year);
    }

    let data = HomeModel {
        first_name: user.first_name.clone(),
        last_name: user.last_name.clone(),
        occupation: user.occupation.to_lowercase(),
        years: years
    };
    // let mut data = HashMap::new();
    // data.insert("first_name", user.first_name);
    // data.insert("last_name", user.last_name);
    // data.insert("occupation", user.occupation.to_lowercase());
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
