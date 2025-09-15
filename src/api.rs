use crate::config::Config;
use crate::database::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Database,
}

impl AppState {
    pub fn new(config: Config, db: Database) -> Arc<Self> {
        Arc::new(Self { config, db })
    }
}
