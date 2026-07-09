use crate::configs::config::Config;
use sqlx::SqlitePool;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Config,
}
