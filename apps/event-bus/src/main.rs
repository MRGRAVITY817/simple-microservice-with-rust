use {
    actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder},
    common::Event,
    serde::Serialize,
    std::sync::Mutex,
};

#[post("/events")]
async fn broadcast_events(
    event: web::Json<Event>,
    event_stack: web::Data<EventStack>,
) -> impl Responder {
    let mut event_stack = event_stack.0.lock().unwrap();
    event_stack.push(event.clone());

    println!("Stack upgraded: {event_stack:?}");

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

#[get("/events")]
async fn get_events(event_stack: web::Data<EventStack>) -> impl Responder {
    let events = event_stack.0.lock().unwrap();
    let body = serde_json::to_string(&events.to_vec()).unwrap();

    println!("Sending events ... {body}");

    HttpResponse::Ok().body(body)
}

#[derive(Serialize)]
struct EventStack(Mutex<Vec<Event>>);

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let event_stack = web::Data::new(EventStack(Mutex::new(vec![])));
    let app = HttpServer::new(move || {
        App::new()
            .app_data(event_stack.clone())
            .service(get_events)
            .service(broadcast_events)
    })
    .bind(("127.0.0.1", 4005))?;
    println!("Event bus running on http://127.0.0.1:4005 ...");
    app.run().await
}
