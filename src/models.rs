use crate::actors::DatabaseActor;
use crate::schema::posts;
use actix::Addr;
use actix_web::web;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait ApiEndPoints {
    fn register_endpoints(config: &mut web::ServiceConfig) -> &mut web::ServiceConfig;
}

pub struct AppState {
    pub db: Addr<DatabaseActor>,
    // pub logger: Addr<LogActor>,
    pub config: Arc<RwLock<HashMap<String, String>>>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize, Identifiable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
