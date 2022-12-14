pub mod routes;

use {
    common::Comment,
    serde::Deserialize,
    std::{collections::HashMap, sync::Mutex},
    thiserror::Error as ThisError,
};

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(ThisError, Debug)]
pub enum ServiceError {
    #[error("Cannot access comments state")]
    CannotAccessCommentsState,
}

pub struct CommentsByPostState(Mutex<HashMap<String, Vec<Comment>>>);

impl CommentsByPostState {
    pub fn new() -> Self {
        CommentsByPostState(Mutex::new(HashMap::new()))
    }

    pub fn get_comments(&self, post_id: &str) -> Option<Vec<Comment>> {
        let comments_map = self.0.lock().unwrap().clone();

        comments_map.get(post_id).cloned()
    }

    pub fn create_post_comment(&self, post_id: &str, comment: Comment) -> ServiceResult<()> {
        let mut comments_map = self
            .0
            .lock()
            .map_err(|_| ServiceError::CannotAccessCommentsState.into())?;

        match comments_map.get_mut(post_id) {
            Some(comments) => {
                comments.push(comment);
            }
            None => {
                comments_map.insert(post_id.to_string(), vec![comment]);
            }
        }
        Ok(())
    }

    pub fn update_comment(&self, post_id: &str, comment: Comment) -> ServiceResult<()> {
        let mut comments_map = self
            .0
            .lock()
            .map_err(|_| ServiceError::CannotAccessCommentsState.into())?;

        if let Some(comments) = comments_map.get(post_id) {
            let comments: Vec<Comment> = comments
                .into_iter()
                .map(|c| {
                    if c.id == comment.id {
                        comment.clone()
                    } else {
                        c.to_owned()
                    }
                })
                .collect();

            comments_map.insert(post_id.to_string(), comments);
        }

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct PostInfo {
    post_id: String,
}
