use {
    crate::{Comment, CommentsByPostState, PostInfo},
    actix_web::{post, web, Responder},
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
    let new_comment = Comment {
        id: Uuid::new_v4().to_string(),
        content: input.content.clone(),
    };

    comments_state.create_post_comment(&post_info.post_id, new_comment)
}
