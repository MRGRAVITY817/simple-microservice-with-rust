pub mod index;

use {
    actix_web::HttpResponse,
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, sync::Mutex},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    id: String,
    content: String,
}

pub struct CommentsByPostState(Mutex<HashMap<String, Vec<Comment>>>);

impl CommentsByPostState {
    pub fn new() -> Self {
        CommentsByPostState(Mutex::new(HashMap::new()))
    }

    pub fn get_comments_by_post_id(&self, post_id: &str) -> Option<Vec<Comment>> {
        let comments_map = self.0.lock().unwrap().clone();

        comments_map.get(post_id).cloned()
    }

    pub fn create_post_comment(&self, post_id: &str, comment: Comment) -> HttpResponse {
        let Ok(mut comments_map) = self.0.lock() else {
            return HttpResponse::InternalServerError().body("Cannot create comment.");
        };

        match comments_map.get_mut(post_id) {
            Some(comments) => {
                comments.push(comment);
            }
            None => {
                comments_map.insert(post_id.to_string(), vec![comment]);
            }
        }
        HttpResponse::Ok().body("Created Comment!")
    }
}

#[derive(Deserialize)]
pub struct PostInfo {
    post_id: String,
}
