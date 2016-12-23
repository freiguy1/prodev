use ::auth::UserCookie;
use nickel::{Request, Response, MiddlewareResult};
use nickel::extensions::{Redirect};

pub fn handler<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let user = req.get_user();
    if user.is_none() {
        return res.redirect("/login");
    }
    let user = user.unwrap();

    let activities: Vec<::models::activity::Activity> = Vec::new();// = ::models::activity::get_activities_by_user(user.id);
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
        occupation: user.occupation.clone(),
        years: years
    };

    res.render("templates/home", &data)
}

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
