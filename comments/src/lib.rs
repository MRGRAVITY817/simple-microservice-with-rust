use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

pub mod comments;

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    id: String,
    content: String,
}

pub struct CommentsByPostState(Mutex<HashMap<String, Vec<Comment>>>);

impl CommentsByPostState {
    pub fn get_comments_by_post_id(&self, post_id: &str) -> Option<Vec<Comment>> {
        let comments = self.0.lock().unwrap().get(post_id);

        comments.map(ToOwned::to_owned)
    }
}
