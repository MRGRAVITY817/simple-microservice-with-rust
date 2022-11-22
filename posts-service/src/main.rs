use {
    actix_cors::Cors,
    actix_web::{web, App, HttpServer},
    posts_service::{
        routes::{create_post, get_posts, post_event},
        PostState,
    },
    std::sync::Mutex,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let posts = web::Data::new(PostState {
        posts: Mutex::new(vec![]),
    });

    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(posts.clone())
            .service(get_posts)
            .service(create_post)
            .service(post_event)
    })
    .bind(("127.0.0.1", 4000))?;

    println!("Running app on http://127.0.0.1:4000 ...");
    app.run().await
}
