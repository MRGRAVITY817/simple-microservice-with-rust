use {
    actix_web::{post, web, App, HttpResponse, HttpServer, Responder},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Debug)]
enum Event {
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

#[derive(Serialize, Deserialize, Debug)]
enum CommentStatus {
    Pending,
    Approved,
    Rejected,
}

#[post("/events")]
async fn moderate_comment(event: web::Json<Event>) -> impl Responder {
    println!("Received event: {event:?}");
    match *event {
        Event::CommentCreated {
            ref comment_id,
            ref content,
            ref post_id,
            ..
        } => {
            let status = if content.contains("orange") {
                CommentStatus::Rejected
            } else {
                CommentStatus::Approved
            };

            println!("Moderated Comment!");

            let client = reqwest::Client::new();
            match client
                .post("http://127.0.0.1:4005/events")
                .json(&Event::CommentModerated {
                    comment_id: comment_id.to_owned(),
                    content: content.to_owned(),
                    post_id: post_id.to_owned(),
                    status,
                })
                .send()
                .await
            {
                Ok(_) => HttpResponse::Ok().body("Moderated comment."),
                Err(_) => HttpResponse::InternalServerError().body("Moderated comment."),
            }
        }
        _ => HttpResponse::Ok().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = HttpServer::new(|| App::new().service(moderate_comment)).bind(("127.0.0.1", 4003))?;

    println!("Moderation service running on http://127.0.0.1:4003 ...");

    app.run().await
}
