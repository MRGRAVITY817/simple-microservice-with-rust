pub mod index;

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
