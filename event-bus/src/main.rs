use {
    actix_web::{post, web, App, HttpResponse, HttpServer, Responder},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
enum Event {
    PostCreated {
        post_id: String,
        title: String,
    },
    CommentCreated {
        comment_id: String,
        content: String,
        post_id: String,
    },
}

#[post("/events")]
async fn broadcast_events(event: web::Json<Event>) -> impl Responder {
    let client = reqwest::Client::new();

    let to_posts_service = client
        .post("http://localhost:4000/events")
        .json(&event)
        .send();

    let to_comments_service = client
        .post("http://localhost:4001/events")
        .json(&event)
        .send();

    let to_query_service = client
        .post("http://localhost:4002/events")
        .json(&event)
        .send();

    let _ = tokio::join!(to_posts_service, to_comments_service, to_query_service);

    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = HttpServer::new(|| App::new().service(broadcast_events)).bind(("127.0.0.1", 4005))?;
    println!("Event bus running on http://127.0.0.1:4005 ...");
    app.run().await
}
