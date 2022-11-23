pub mod routes;

use {
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, sync::Mutex},
};

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
    status: CommentStatus,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum CommentStatus {
    Pending,
    Approved,
    Rejected,
}

pub struct PostState {
    pub posts: Mutex<HashMap<String, Post>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    PostCreated {
        post_id: String,
        title: String,
    },
    CommentCreated {
        comment_id: String,
        content: String,
        post_id: String,
        status: CommentStatus,
    },
    CommentModerated {
        comment_id: String,
        content: String,
        post_id: String,
        status: CommentStatus,
    },
    CommentUpdated {
        comment_id: String,
        content: String,
        post_id: String,
        status: CommentStatus,
    },
}
