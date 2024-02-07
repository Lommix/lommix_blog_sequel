use std::error::Error;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

#[derive(Debug, Deserialize)]
pub struct FeedbackData {
    pub message: String,
    pub csrf: String,
    pub captcha: String,
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

#[rustfmt::skip]
pub async fn feedback_form  (
    Json(data): Json<FeedbackData>,
) -> Result<Response, FeedbackError>{

    if data.captcha.len() > 0 {
        return Err(FeedbackError::InvalidCaptcha);
    }

    if data.csrf.len() > 0 {
        return Err(FeedbackError::InvalidCsrf);
    }

    if data.message.len() == 0 {
        return Err(FeedbackError::NoMessage);
    }


    let feedback_path = std::env::var("FEEDBACK_FILE").unwrap_or("feedback/feedback.txt".into());
    let message = format!("\n-----------------------------------\n{}", sanatize(&data.message));

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(feedback_path)
        .await.map_err(|e| FeedbackError::Fuck(e.into()))?;

    file.write_all(message.as_bytes()).await.map_err(|e| FeedbackError::Fuck(e.into()))?;

    Ok((axum::http::StatusCode::OK, "Thanks for your feedback").into_response())

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
