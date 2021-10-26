use crate::actors::DatabaseActor;
use crate::schema::posts;
use actix::Addr;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub db: Addr<DatabaseActor>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize, Identifiable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
