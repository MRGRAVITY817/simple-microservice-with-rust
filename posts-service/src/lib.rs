pub mod routes;

use {
    serde::{Deserialize, Serialize},
    std::sync::Mutex,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    id: String,
    title: String,
}

pub struct PostState {
    pub posts: Mutex<Vec<Post>>,
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    PostCreated {
        post_id: String,
        title: String,
    },
    CommentCreated {
        comment_id: String,
        content: String,
        post_id: String,
    },
}
