use {
    crate::{Comment, CommentsByPostState, Event},
    actix_web::{post, web, HttpResponse, Responder},
};

#[post("/events")]
pub async fn post_event(
    event: web::Json<Event>,
    comment_state: web::Data<CommentsByPostState>,
) -> impl Responder {
    match *event {
        Event::PostCreated { .. } => println!("Post created!"),
        Event::CommentCreated { .. } => println!("Comment created!"),
        Event::CommentUpdated { .. } => println!("Comment updated!"),
        Event::CommentModerated {
            ref comment_id,
            ref content,
            ref post_id,
            ref status,
        } => {
            if let Ok(_) = comment_state.update_comment(
                post_id,
                Comment {
                    id: comment_id.to_owned(),
                    content: content.to_owned(),
                    status: status.to_owned(),
                },
            ) {
                let client = reqwest::Client::new();
                if let Ok(_) = client
                    .post("http://127.0.0.1:4005/events")
                    .json(&Event::CommentUpdated {
                        comment_id: comment_id.to_owned(),
                        content: content.to_owned(),
                        post_id: post_id.to_owned(),
                        status: status.to_owned(),
                    })
                    .send()
                    .await
                {
                    return HttpResponse::Ok().finish();
                }
            }
            return HttpResponse::InternalServerError()
                .body("Cannot update comment with new status.");
        }
    }
    HttpResponse::Ok().finish()
}
