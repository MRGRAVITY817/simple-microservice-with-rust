use {
    crate::{CommentsByPostState, PostInfo},
    actix_web::{post, web, HttpResponse, Responder},
    common::{Comment, CommentStatus, Event},
    serde::Deserialize,
    uuid::Uuid,
};

#[derive(Deserialize, Clone)]
pub struct CreateCommentInput {
    pub content: String,
}

#[post("/posts/{post_id}/comments")]
pub async fn create_new_post_comment(
    post_info: web::Path<PostInfo>,
    input: web::Json<CreateCommentInput>,
    comments_state: web::Data<CommentsByPostState>,
) -> impl Responder {
    let comment_id = Uuid::new_v4().to_string();
    let content = input.content.clone();
    let status = CommentStatus::Pending;
    let new_comment = Comment {
        id: comment_id.clone(),
        content: content.clone(),
        status,
    };

    match comments_state.create_post_comment(&post_info.post_id, new_comment) {
        Ok(_) => {
            let client = reqwest::Client::new();
            match client
                .post("http://127.0.0.1:4005/events")
                .json(&Event::CommentCreated {
                    comment_id,
                    content,
                    post_id: post_info.post_id.clone(),
                    status,
                })
                .send()
                .await
            {
                Ok(_) => HttpResponse::Ok().body("Created comment!"),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
