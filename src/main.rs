extern crate actix;
#[macro_use]
extern crate diesel;
// #[macro_use]
extern crate diesel_migrations;

mod actors;
mod models;
mod response;
mod schema;

use actix_web::{
    middleware, post,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

use dotenv::dotenv;
use models::Post;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[post("/create")]
async fn create_post(post: Json<Post>, state: Data<models::AppState>) -> impl Responder {
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
        Ok(Ok(pointcloud)) => HttpResponse::Ok().json(pointcloud),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    print!("{}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(create_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
