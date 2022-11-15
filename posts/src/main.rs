use {
    actix_web::{web, App, HttpServer},
    posts::{
        index::{create_post, get_posts},
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
        App::new()
            .app_data(posts.clone())
            .service(get_posts)
            .service(create_post)
    })
    .bind(("127.0.0.1", 4000))?;

    println!("Running app on http://127.0.0.1:4000");
    app.run().await
}
