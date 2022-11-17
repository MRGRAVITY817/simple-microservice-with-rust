use {
    actix_cors::Cors,
    actix_web::{web, App, HttpServer},
    query_service::{
        routes::{get_posts, post_event},
        PostState,
    },
    std::{collections::HashMap, sync::Mutex},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let post_state = web::Data::new(PostState {
        posts: Mutex::new(HashMap::new()),
    });

    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(post_state.clone())
            .service(post_event)
            .service(get_posts)
    })
    .bind(("127.0.0.1", 4002))?;

    println!("Running app on http://127.0.0.1:4002 ...");
    app.run().await
}
