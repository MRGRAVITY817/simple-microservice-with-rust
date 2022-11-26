use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum CommentStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
