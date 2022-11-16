use {
    crate::{Post, PostState},
    actix_web::{post, web, HttpResponse, Responder},
    serde::Deserialize,
    uuid::Uuid,
};

#[derive(Deserialize)]
pub struct CreatePostInput {
    title: String,
}

#[post("/posts")]
pub async fn create_post(
    input: web::Json<CreatePostInput>,
    post_state: web::Data<PostState>,
) -> impl Responder {
    let new_post = Post {
        id: Uuid::new_v4().to_string(),
        title: input.title.clone(),
    };

    let mut posts = post_state.posts.lock().unwrap();
    posts.push(new_post);

    HttpResponse::Ok().body("Created new post!")
}
