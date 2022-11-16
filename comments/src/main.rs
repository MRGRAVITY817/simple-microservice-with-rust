use {
    actix_web::{web, App, HttpServer},
    comments::{
        index::{create_new_post_comment, get_post_comments},
        CommentsByPostState,
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let comments_by_post = web::Data::new(CommentsByPostState::new());

    let app = HttpServer::new(move || {
        App::new()
            .app_data(comments_by_post.clone())
            .service(get_post_comments)
            .service(create_new_post_comment)
    })
    .bind(("127.0.0.1", 4001))?;

    println!("Running app on http://127.0.0.1:4001");
    app.run().await
}
