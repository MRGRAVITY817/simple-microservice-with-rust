mod events;
mod posts_comments;

pub use {
    events::post_event,
    posts_comments::{create_new_post_comment, get_post_comments},
};
