use {
    actix_web::{
        body::BoxBody, get, http::header::ContentType, post, web, App, HttpResponse, HttpServer,
        Responder,
    },
    serde::{Deserialize, Serialize},
    std::sync::Mutex,
    uuid::Uuid,
};

#[derive(Serialize, Deserialize, Clone)]
struct Post {
    id: String,
    title: String,
}

#[derive(Deserialize)]
struct CreatePostInput {
    title: String,
}

struct PostState {
    posts: Mutex<Vec<Post>>,
}

#[derive(Serialize)]
struct GetPostsOutput {
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

#[get("/")]
async fn get_posts(post_state: web::Data<PostState>) -> GetPostsOutput {
    let posts = post_state.posts.lock().unwrap();
    GetPostsOutput {
        posts: posts.to_vec(),
    }
}

#[post("/")]
async fn create_post(
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let posts = web::Data::new(PostState {
        posts: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(posts.clone())
            .service(get_posts)
            .service(create_post)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
