use {
    actix_web::{post, web, HttpResponse, Responder},
    common::Event,
};

#[post("/events")]
pub async fn post_event(event: web::Json<Event>) -> impl Responder {
    match *event {
        Event::PostCreated { .. } => println!("Post created!"),
        Event::CommentCreated { .. } => println!("Comment created!"),
        _ => {}
    }
    HttpResponse::Ok().finish()
}
