use rusqlite::Result as SqliteResult;

pub mod user;
pub mod activity;

trait Accessor {
    type Dao;
    type UpdateDao;
    type CreateDao;

    fn create(&self, create_dao: &Self::CreateDao) -> SqliteResult<i32>;
    fn update(&self, update_dao: &Self::UpdateDao) -> SqliteResult<()>;
    fn get_by_id(&self, id: i32) -> SqliteResult<Option<Self::Dao>>;
    fn delete(&self, id: i32) -> SqliteResult<()>;
}
