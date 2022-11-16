use {
    crate::{Event, Post, PostState},
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
    let post_id = Uuid::new_v4().to_string();

    let new_post = Post {
        id: post_id.clone(),
        title: input.title.clone(),
    };

    let mut posts = post_state.posts.lock().unwrap();
    posts.push(new_post);

    let client = reqwest::Client::new();
    match client
        .post("http://127.0.0.1:4005")
        .json(&Event::PostCreated {
            post_id,
            title: input.title.clone(),
        })
        .send()
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Created new post!"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
