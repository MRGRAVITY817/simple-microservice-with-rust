use {
    actix_cors::Cors,
    actix_web::{web, App, HttpServer},
    common::Event,
    query_service::{
        routes::{get_posts, post_event, process_event},
        Post, PostState,
    },
    std::{collections::HashMap, sync::Mutex},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let post_state = sync_state().await;

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

async fn sync_state() -> web::Data<PostState> {
    if let Ok(response) = reqwest::get("http://127.0.0.1:4005/events").await {
        if let Ok(events) = response.json::<Vec<Event>>().await {
            let mut posts: HashMap<String, Post> = HashMap::new();
            events
                .iter()
                .for_each(|event| process_event(&mut posts, event));
            let post_state = web::Data::new(PostState {
                posts: Mutex::new(posts),
            });

            return post_state;
        }
    }

    web::Data::new(PostState {
        posts: Mutex::new(HashMap::new()),
    })
}
