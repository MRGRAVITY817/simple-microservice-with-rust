use {
    actix_cors::Cors,
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
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(comments_by_post.clone())
            .service(get_post_comments)
            .service(create_new_post_comment)
    })
    .bind(("127.0.0.1", 4001))?;

    println!("Running app on http://127.0.0.1:4001");
    app.run().await
}
