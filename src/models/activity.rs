use ::models::Accessor;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result as SqliteResult;

pub struct ActivityDao {
    pub id: i32,
    pub worksheet_id: i32,
    pub points: i32,
    pub description: String,
    pub note: Option<String>
}

pub struct CreateActivityDao {
    pub worksheet_id: i32,
    pub points: i32,
    pub description: String,
    pub note: Option<String>
}

pub struct ActivityAccessor<'mw> {
    conn: &'mw PooledConnection<SqliteConnectionManager>
}

impl<'mw> ActivityAccessor<'mw> {
    fn new(conn: &'mw PooledConnection<SqliteConnectionManager>) -> ActivityAccessor {
        ActivityAccessor {
            conn: conn
        }
    }
}

impl<'mw> Accessor for ActivityAccessor<'mw> {
    type Dao = ActivityDao;
    type UpdateDao = ActivityDao;
    type CreateDao = CreateActivityDao;

    fn create(&self, createDao: &Self::CreateDao) -> SqliteResult<i32> {
        let query = "INSERT INTO activity (points, worksheet_id, description, note) VALUES (?, ?, ?, ?);";
        try!(self.conn.execute(query, &[&createDao.points, &createDao.worksheet_id, &createDao.description, &createDao.note]));
        Ok(self.conn.last_insert_rowid() as i32)
        /*
    id INTEGER PRIMARY KEY,
    points INTEGER NOT NULL,
    worksheet_id INTEGER NOT NULL,
    description VARCHAR(128) NOT NULL,
    note TEXT NULL
        */
    }

    fn update(&self, updateDao: &Self::UpdateDao) -> SqliteResult<()> {
        let query = "UPDATE activity (points, worksheet_id, description, note) VALUES (?, ?, ?, ?) WHERE id = ?;";
        let query_data: &[&::rusqlite::types::ToSql] = &[&updateDao.points,
                          &updateDao.worksheet_id,
                          &updateDao.description,
                          &updateDao.note,
                          &updateDao.id];
        try!(self.conn.execute(query, query_data));
        Ok(())
    }
    fn get_by_id(&self, id: i32) -> SqliteResult<Option<Self::Dao>> {
        let mut stmt  = try!(self.conn.prepare("SELECT id, points, worksheet_id, description, note FROM activity WHERE id = ?"));
        let mut activity_iter = try!(stmt.query_map(&[&id], |row| {
            ActivityDao {
                id: row.get(0),
                points: row.get(1),
                worksheet_id: row.get(2),
                description: row.get(3),
                note: row.get(4)
            }
        }));

        match activity_iter.next() {
            Some(Ok(a)) => Ok(Some(a)),
            Some(Err(e)) => Err(e),
            None => Ok(None)
        }
    }
    fn delete(&self, id: i32) -> SqliteResult<()> {
        let query = "DELETE FROM activity WHERE id = ?;";
        try!(self.conn.execute(query, &[&id]));
        Ok(())
    }
}



pub struct Activity {
    pub user_id: i32,
    pub points: i32,
    pub year: i32,
    pub description: String,
}


pub fn get_activities_by_user_year(user_id: i32, year: i32) -> Vec<Activity> {
    get_activities().into_iter().filter(|a| a.user_id == user_id && a.year == year).collect()
}

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
        }
    ]
}
