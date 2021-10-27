#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod actors;
mod models;
mod response;
mod schema;
mod utils;

use actix::SyncArbiter;
use actix_web::{middleware, route, web, App, HttpResponse, HttpServer, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use dotenv::dotenv;
use models::Post;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use utils::{get_pool, run_migrations};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

// API :
#[route("/posts", method = "POST")]
async fn create_post(post: web::Json<Post>, state: web::Data<models::AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let post = post.into_inner();

    let msg = actors::CreatePost {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    };

    let msg_request = db.send(msg);

    match msg_request.await {
        Ok(Ok(p)) => HttpResponse::Ok().json(p),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[route("/posts", method = "POST")]
async fn update_post(post: web::Json<Post>, state: web::Data<models::AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let post = post.into_inner();

    let msg = actors::UpdatePost {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
    };

    let msg_request = db.send(msg);

    match msg_request.await {
        Ok(Ok(p)) => HttpResponse::Ok().json(p),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[route("/posts", method = "DELETE")]
async fn delete_post(post: web::Json<Post>, state: web::Data<models::AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let post = post.into_inner();

    let msg = actors::DeletePost { id: post.id };

    let msg_request = db.send(msg);

    match msg_request.await {
        Ok(Ok(p)) => HttpResponse::Ok().json(p),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[route("/posts", method = "GET")]
async fn get_posts(state: web::Data<models::AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    let msg = actors::GetPosts;

    let msg_request = db.send(msg);

    match msg_request.await {
        Ok(Ok(p)) => HttpResponse::Ok().json(p),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let config: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    print!("{}", database_url);
    let api_port: u16 = std::env::var("API_PORT")
        .expect("API_PORT")
        .parse()
        .expect("API_PORT env var should parsse to u16");
    // set up database connection pool
    run_migrations(&database_url);
    let pool = get_pool(&database_url);
    let db_addr = SyncArbiter::start(5, move || actors::DatabaseActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(create_post)
            .service(update_post)
            .service(delete_post)
            .service(get_posts)
            .data(models::AppState {
                db: db_addr.clone(),
                config: config.clone(),
            })
    })
    .bind(("0.0.0.0", api_port))?
    .run()
    .await
}
