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

    fn create(&self, create_dao: &Self::CreateDao) -> SqliteResult<i32> {
        let query = "INSERT INTO activity (points, worksheet_id, description, note) VALUES (?, ?, ?, ?);";
        try!(self.conn.execute(query, &[&create_dao.points, &create_dao.worksheet_id, &create_dao.description, &create_dao.note]));
        Ok(self.conn.last_insert_rowid() as i32)
    }

    fn update(&self, update_dao: &Self::UpdateDao) -> SqliteResult<()> {
        let query = "UPDATE activity SET points = ?, worksheet_id = ?, description = ?, note = ? WHERE id = ?;";
        let query_data: &[&::rusqlite::types::ToSql] = &[&update_dao.points,
                          &update_dao.worksheet_id,
                          &update_dao.description,
                          &update_dao.note,
                          &update_dao.id];
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

// Flagged for removal!
pub struct Activity {
    pub user_id: i32,
    pub points: i32,
    pub year: i32,
    pub description: String
}

#[cfg(test)]
mod tests {
    use r2d2_sqlite::SqliteConnectionManager;
    use r2d2::{Pool, PooledConnection, Config};
    use super::*;
    use models::Accessor;

    fn get_db_conn() -> PooledConnection<SqliteConnectionManager> {
        let config = Config::builder().pool_size(1).build();
        let manager = SqliteConnectionManager::new("file.db");
        let pool = Pool::new(config, manager).expect("Could not create db pool");
        pool.get().expect("Could not get connection from pool")
    }

    #[test]
    fn create_activity() {
        let conn = get_db_conn();
        let accessor = ActivityAccessor::new(&conn);
        // Create activity
        let create_dao = CreateActivityDao {
            worksheet_id: 1,
            points: 1,
            description: "Test Activity".to_string(),
            note: None
        };
        let result = accessor.create(&create_dao);
        assert!(result.is_ok());

        // Get created activity from db
        let result = accessor.get_by_id(result.unwrap());
        assert!(result.is_ok());
        let created_activity_opt = result.unwrap();
        assert!(created_activity_opt.is_some());
        let created_activity = created_activity_opt.unwrap();
        assert_eq!(create_dao.worksheet_id, created_activity.worksheet_id);
        assert_eq!(create_dao.points, created_activity.points);
        assert_eq!(create_dao.description, created_activity.description);
        assert_eq!(create_dao.note, created_activity.note);

        // Clean up
        let result = accessor.delete(created_activity.id);
        assert!(result.is_ok());
    }

    #[test]
    fn update_activity() {
        let conn = get_db_conn();
        let accessor = ActivityAccessor::new(&conn);
        // Create activity
        let create_dao = CreateActivityDao {
            worksheet_id: 1,
            points: 1,
            description: "Test Activity".to_string(),
            note: None
        };
        let result = accessor.create(&create_dao);
        assert!(result.is_ok());

        // Update activity
        let update_dao = ActivityDao {
            id: result.unwrap(),
            worksheet_id: 4,
            points: 2,
            description: "Test Activity - edited".to_string(),
            note: Some("Updating note!".to_string())
        };
        let result = accessor.update(&update_dao);
        assert!(result.is_ok());

        // Get updated activity from db
        let result = accessor.get_by_id(update_dao.id);
        assert!(result.is_ok());
        let updated_activity_opt = result.unwrap();
        assert!(updated_activity_opt.is_some());
        let updated_activity = updated_activity_opt.unwrap();
        assert_eq!(update_dao.worksheet_id, updated_activity.worksheet_id);
        assert_eq!(update_dao.points, updated_activity.points);
        assert_eq!(update_dao.description, updated_activity.description);
        assert_eq!(update_dao.note, updated_activity.note);

        // Clean up
        let result = accessor.delete(updated_activity.id);
        assert!(result.is_ok());
    }

    #[test]
    fn delete_activity() {
        let conn = get_db_conn();
        let accessor = ActivityAccessor::new(&conn);
        // Create activity
        let create_dao = CreateActivityDao {
            worksheet_id: 1,
            points: 1,
            description: "Test Activity".to_string(),
            note: None
        };
        let result = accessor.create(&create_dao);
        assert!(result.is_ok());

        // Delete activity
        let activity_id = result.unwrap();
        let result = accessor.delete(activity_id);
        assert!(result.is_ok());

        // Get nothing when searcing by deleted id
        let result = accessor.get_by_id(activity_id);
        assert!(result.is_ok());
        let activity_opt = result.unwrap();
        assert!(activity_opt.is_none());
    }
}
