use std::error::Error;

use super::HtmxComponent;
use crate::AppState;
use axum::{
    response::{IntoResponse, Response},
    routing::{post, MethodRouter},
    Form,
};
use maud::html;
use serde::Deserialize;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

#[derive(Debug, Deserialize)]
pub struct FeedbackData {
    pub message: String,
    pub csrf: String,
    pub captcha: String,
}

pub struct Feedback;
impl HtmxComponent<AppState> for Feedback {
    fn path() -> &'static str {
        "/feedback"
    }

    fn css() -> &'static str {
        include_str!("style.css")
    }

    fn handle() -> MethodRouter<AppState> {
        post(on_post).get(on_get)
    }
}

async fn on_get() -> Response {
    html!(
        form class="feedback" hx-post="/htmx/feedback" {
            textarea name="message" rows="4" cols="50" {}
            input class="captcha" type="text" name="captcha" value="" {}
            input type="hidden" name="csrf" value="" {}
            input type="submit" value="Submit" {}
        }
    )
    .into_response()
}

async fn on_post(Form(data): Form<FeedbackData>) -> Result<Response, FeedbackError> {
    // honeypot
    if data.captcha.len() > 0 {
        return Err(FeedbackError::InvalidCaptcha);
    }

    // honeypot
    if data.csrf.len() > 0 {
        return Err(FeedbackError::InvalidCsrf);
    }

    if data.message.len() == 0 {
        return Err(FeedbackError::NoMessage);
    }

    let feedback_path = std::env::var("FEEDBACK_FILE").unwrap_or("feedback/feedback.txt".into());
    let message = format!(
        "\n-----------------------------------\n{}",
        sanatize(data.message.as_str())
    );

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(feedback_path)
        .await
        .map_err(|e| FeedbackError::Fuck(e.into()))?;

    file.write_all(message.as_bytes())
        .await
        .map_err(|e| FeedbackError::Fuck(e.into()))?;

    Ok("Thank you for your feedback".into_response())
}

const ALLOWED_PUNCTUATION: [char; 10] = ['.', '(', ')', ',', '-', '+', '!', '?', ':', '@'];

fn sanatize(data: &str) -> String {
    let mut out = String::new();

    data.chars().for_each(|c| {
        if c.is_alphanumeric() || c.is_whitespace() || ALLOWED_PUNCTUATION.contains(&c) {
            out.push(c);
        } else {
            out.push('*');
        }
    });
    out
}

pub enum FeedbackError {
    InvalidCaptcha,
    InvalidCsrf,
    NoMessage,
    Fuck(Box<dyn Error>),
}

impl IntoResponse for FeedbackError {
    fn into_response(self) -> axum::response::Response {
        match self {
            FeedbackError::InvalidCaptcha => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid captcha").into_response()
            }
            FeedbackError::InvalidCsrf => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid csrf").into_response()
            }
            FeedbackError::NoMessage => {
                (axum::http::StatusCode::BAD_REQUEST, "No message").into_response()
            }
            FeedbackError::Fuck(e) => {
                tracing::error!(e);
                (axum::http::StatusCode::BAD_REQUEST, "Interal Server Error").into_response()
            }
        }
    }
}
