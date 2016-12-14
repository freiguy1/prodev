use nickel::{Request, Response, MiddlewareResult, JsonBody};
use nickel::status::StatusCode;

pub fn handler<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let add_activity_request = try_with!(
        res,
        req.json_as::<AddActivityRequest>().map_err(|e| (StatusCode::BadRequest, e)));

    let formatted = format!("Request description: {:?}\nRequest Points: {:?}", add_activity_request.description, add_activity_request.points);
    res.send(formatted)
}

#[derive(RustcDecodable)]
struct AddActivityRequest {
    description: String,
    points: i32
}
