pub mod routes;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    id: String,
    title: String,
    comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    id: String,
    content: String,
}

pub struct PostState {
    pub posts: Mutex<HashMap<String, Post>>,
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
