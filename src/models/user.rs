
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub occupation: String,
    password: String
}

pub fn get_by_email_password<'a>(email: &'a str, plain_text_password: &'a str) -> Option<User> {
    get_all_users().into_iter().find(|u| u.email == email.to_string() && u.password == plain_text_password.to_string())
}

pub fn get_by_id(id: i32) -> Option<User> {
    get_all_users().into_iter().find(|u| u.id == id)
}

fn get_all_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            first_name: "Sally".to_string(),
            last_name: "Sharpe".to_string(),
            email: "manager@company.com".to_string(),
            occupation: "Manager".to_string(),
            password: "password123".to_string()
        },
        User {
            id: 2,
            first_name: "Galvin".to_string(),
            last_name: "McQuaid".to_string(),
            email: "employee@company.com".to_string(),
            occupation: "Programmer/Analyst".to_string(),
            password: "password123".to_string()
        }
    ]
}
