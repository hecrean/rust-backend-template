use crate::models::Post;
use crate::schema::posts;
use actix::{Actor, Handler, Message, SyncContext};
use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub struct DatabaseActor(pub Pool<ConnectionManager<PgConnection>>);

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct CreatePost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct UpdatePost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct DeletePost {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Post>>")]
pub struct GetPosts;

impl Actor for DatabaseActor {
    type Context = SyncContext<Self>;
}

impl Handler<CreatePost> for DatabaseActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: CreatePost, _: &mut Self::Context) -> Self::Result {
        let pooled_connection = self.0.get().expect("Unable to get a connectio");

        let new_post = Post {
            id: msg.id,
            title: msg.title,
            body: msg.body,
            published: msg.published,
        };

        diesel::insert_into(posts::dsl::posts)
            .values(new_post)
            .get_result::<Post>(&pooled_connection)
    }
}

impl Handler<UpdatePost> for DatabaseActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: UpdatePost, _: &mut Self::Context) -> Self::Result {
        let pooled_connection = self.0.get().expect("Unable to get a connectio");

        diesel::update(posts::dsl::posts.filter(posts::dsl::id.eq(msg.id)))
            .set((
                posts::dsl::title.eq(msg.title),
                posts::dsl::body.eq(msg.body),
                posts::dsl::published.eq(msg.published),
            ))
            .get_result::<Post>(&pooled_connection)
    }
}

impl Handler<DeletePost> for DatabaseActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: DeletePost, _: &mut Self::Context) -> Self::Result {
        let pooled_connection = self.0.get().expect("Unable to get a connectio");

        diesel::delete(posts::dsl::posts.filter(posts::dsl::id.eq(msg.id)))
            .get_result::<Post>(&pooled_connection)
    }
}

impl Handler<GetPosts> for DatabaseActor {
    type Result = QueryResult<Vec<Post>>;

    fn handle(&mut self, msg: GetPosts, _: &mut Self::Context) -> Self::Result {
        let pooled_connection = self.0.get().expect("Unable to get a connectio");

        posts::dsl::posts.get_results::<Post>(&pooled_connection)
    }
}
