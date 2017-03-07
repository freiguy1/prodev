use nickel::{Request, Response, MiddlewareResult};
use nickel::status::StatusCode;

pub fn handler<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    // When you need user in the future, user this stuff
// use ::auth::UserCookie;
    // let user = req.get_user();
    // if user.is_none() {
        // return res.redirect("/login");
    // }
    // let user = user.unwrap();

    let year = req.param("year").and_then(|y| y.parse::<i32>().ok());
    if year.is_none() {
        res.set(StatusCode::BadRequest);
        return res.send("Invalid year");
    }
    let year = year.unwrap();

    let activities: Vec<::models::activity::Activity> = Vec::new();// = ::models::activity::get_activities_by_user(user.id);
    let data = WorksheetModel {
        year: year,
        has_activities: activities.len() > 0,
        activities: activities.iter().map(|a| ActivityModel {
            points: a.points,
            description: a.description.clone()
        }).collect()
    };

    res.render("templates/worksheet", &data)
}

#[derive(RustcEncodable)]
struct WorksheetModel {
    year: i32,
    has_activities: bool,
    activities: Vec<ActivityModel>
}

#[derive(RustcEncodable)]
struct ActivityModel {
    points: i32,
    description: String
}
