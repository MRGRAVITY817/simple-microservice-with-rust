use {
    actix_web::{post, web, App, HttpResponse, HttpServer, Responder},
    serde::{Deserialize, Serialize},
    std::sync::Mutex,
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum CommentStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Event {
    PostCreated {
        post_id: String,
        title: String,
    },
    CommentCreated {
        comment_id: String,
        content: String,
        post_id: String,
        status: CommentStatus,
    },
    CommentModerated {
        comment_id: String,
        content: String,
        post_id: String,
        status: CommentStatus,
    },
    CommentUpdated {
        comment_id: String,
        content: String,
        post_id: String,
        status: CommentStatus,
    },
}

#[post("/events")]
async fn broadcast_events(
    event: web::Json<Event>,
    event_stack: web::Data<EventStack>,
) -> impl Responder {
    let mut event_stack = event_stack.0.lock().unwrap();
    event_stack.push(event.clone());

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

    let to_moderation_service = client
        .post("http://localhost:4003/events")
        .json(&event)
        .send();

    let _ = tokio::join!(
        to_posts_service,
        to_comments_service,
        to_query_service,
        to_moderation_service
    );

    HttpResponse::Ok().finish()
}

struct EventStack(Mutex<Vec<Event>>);

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let event_stack = web::Data::new(EventStack(Mutex::new(vec![])));
    let app = HttpServer::new(move || {
        App::new()
            .app_data(event_stack.clone())
            .service(broadcast_events)
    })
    .bind(("127.0.0.1", 4005))?;
    println!("Event bus running on http://127.0.0.1:4005 ...");
    app.run().await
}
