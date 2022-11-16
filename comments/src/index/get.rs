use {
    crate::{Comment, CommentsByPostState, PostInfo},
    actix_web::{get, http::header::ContentType, web, HttpResponse, Responder},
    serde::Serialize,
};

#[derive(Serialize)]
struct GetPostCommentsOutput {
    comments: Vec<Comment>,
}

#[get("/posts/{post_id}/comments")]
pub async fn get_post_comments(
    post_info: web::Path<PostInfo>,
    comments_state: web::Data<CommentsByPostState>,
) -> impl Responder {
    let Some(comments) = comments_state.get_comments_by_post_id(&post_info.post_id) else {
			return HttpResponse::NoContent().finish();
		};

    let body = serde_json::to_string(&comments).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}
