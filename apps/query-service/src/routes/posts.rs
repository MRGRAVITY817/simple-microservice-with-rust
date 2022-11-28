use {
    crate::PostState,
    actix_web::{get, http::header::ContentType, web, HttpResponse, Responder},
};

#[get("/posts")]
pub async fn get_posts(post_state: web::Data<PostState>) -> impl Responder {
    let posts = post_state.posts.lock().unwrap();
    let body = serde_json::to_string(&posts.clone()).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}
