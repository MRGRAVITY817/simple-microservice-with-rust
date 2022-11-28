pub mod routes;

use {
    common::Comment,
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, sync::Mutex},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    id: String,
    title: String,
    comments: Vec<Comment>,
}

pub struct PostState {
    pub posts: Mutex<HashMap<String, Post>>,
}
