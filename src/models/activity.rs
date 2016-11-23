pub struct Activity {
    pub user_id: i32,
    pub year: i32,
    pub points: i32,
    pub description: String
}

// pub fn get_activities_by_user_year(user_id: i32, year: i32) -> Vec<Activity> {
    // get_activities().into_iter().filter(|a| a.user_id == user_id && a.year == year).collect()
// }

pub fn get_activities_by_user(user_id: i32) -> Vec<Activity> {
    get_activities().into_iter().filter(|a| a.user_id == user_id).collect()
}

fn get_activities() -> Vec<Activity> {
    vec![
        Activity {
            user_id: 1,
            year: 2015,
            points: 4,
            description: "Read a book".to_string()
        },
        Activity {
            user_id: 1,
            year: 2015,
            points: 1,
            description: "Study Group".to_string()
        },
        Activity {
            user_id: 1,
            year: 2016,
            points: 2,
            description: "Read a blog".to_string()
        },
        Activity {
            user_id: 1,
            year: 2016,
            points: 5,
            description: "Mentor".to_string()
        },
        Activity {
            user_id: 2,
            year: 2015,
            points: 4,
            description: "Read a book".to_string()
        },
        Activity {
            user_id: 2,
            year: 2015,
            points: 1,
            description: "Study Group".to_string()
        },
        Activity {
            user_id: 2,
            year: 2016,
            points: 2,
            description: "Read a blog".to_string()
        },
        Activity {
            user_id: 2,
            year: 2016,
            points: 5,
            description: "Mentor".to_string()
        },
    ]
}
