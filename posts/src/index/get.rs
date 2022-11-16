use {
    actix_web::{body::BoxBody, get, http::header::ContentType, web, HttpResponse, Responder},
    serde::Serialize,
};

use crate::{Post, PostState};

#[derive(Serialize)]
pub struct GetPostsOutput {
    posts: Vec<Post>,
}

impl Responder for GetPostsOutput {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/posts")]
pub async fn get_posts(post_state: web::Data<PostState>) -> GetPostsOutput {
    let posts = post_state.posts.lock().unwrap();
    GetPostsOutput {
        posts: posts.to_vec(),
    }
}
