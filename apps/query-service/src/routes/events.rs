use {
    crate::{Post, PostState},
    actix_web::{post, web, HttpResponse, Responder},
    common::{Event, Comment},
    std::collections::HashMap,
};

#[post("/events")]
pub async fn post_event(
    event: web::Json<Event>,
    post_state: web::Data<PostState>,
) -> impl Responder {
    let mut posts = post_state.posts.lock().unwrap();

    process_event(&mut posts, &event);

    HttpResponse::Ok().finish()
}

pub fn process_event(posts: &mut HashMap<String, Post>, event: &Event) {
    match *event {
        Event::PostCreated {
            ref post_id,
            ref title,
        } => {
            posts.insert(
                post_id.to_owned(),
                Post {
                    id: post_id.to_owned(),
                    title: title.to_owned(),
                    comments: vec![],
                },
            );
        }
        Event::CommentCreated {
            ref comment_id,
            ref content,
            ref post_id,
            ref status,
        } => {
            let Some(post) = posts.get_mut(post_id) else {
                return 
            };
            post.comments.push(Comment {
                id: comment_id.to_owned(),
                content: content.to_owned(),
                status: status.to_owned(),
            });
        }
        Event::CommentUpdated {
            ref comment_id,
            ref content,
            ref post_id,
            ref status,
        } => {
            let Some(post) = posts.get_mut(post_id) else {
                return 
            };
            post.comments.iter_mut().for_each(|comment| {
                if comment.id == *comment_id {
                    comment.content = content.to_owned();
                    comment.status = status.to_owned();
                }
            })
        }
        _ => {}
    }
}
