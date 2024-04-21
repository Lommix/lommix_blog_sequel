use std::{error::Error, time::Duration};

use super::HtmxComponent;
use crate::{db, AppState};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::{get, MethodRouter},
    Form,
};
use lettre::{
    message::header, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};
use maud::{html, Markup};

pub struct ContactContent;
impl HtmxComponent<AppState> for ContactContent {
    fn path() -> &'static str {
        "/contact"
    }

    fn css() -> &'static str {
        include_str!("style.css")
    }

    fn handle() -> MethodRouter<AppState> {
        get(on_get).post(on_post)
    }
}

async fn on_get(State(state): State<AppState>) -> Response {
    _ = db::inc(&state.db_pool, "about").await;
    html! {
        div class="contact"{
            h1 {"Contact"}
            hr {}
            p {"Feel welcome to reach out to me with any inquiries or questions you may have. I also offer consulting for backend, gamedev and Rust in general."}

            form class="contact-form" hx-post="/htmx/contact" hx-indicator=".send-button"{
                input type="email" name="email" placeholder="E-Mail" required {}
                input type="text" name="subject" placeholder="Subject" required {}
                textarea name="message" rows="4" cols="50" required {}
                div class="datasecurity" {
                    input type="checkbox" name="datasecurity" required {}
                    p {"Your email will be send to my inbox and you are ok with that."}
                }
                input class="captcha" type="text" name="captcha" value="" {}
                input type="hidden" name="csrf" value="" {}
                button type="submit" class="send-button" {
                    span {"Send"}
                    img height="35" width="35" id="button-spinner" src="/static/images/spinner.svg"{}
                }
            }
        }
    }
    .into_response()
}

async fn on_post(
    State(state): State<AppState>,
    Form(data): Form<ContactData>,
) -> Result<Response, ContactError> {
    // honeypot
    if data.captcha.len() > 0 {
        return Err(ContactError::InvalidCaptcha);
    }

    // honeypot
    if data.csrf.len() > 0 {
        return Err(ContactError::InvalidCsrf);
    }

    if data.message.len() == 0 {
        return Err(ContactError::NoMessage);
    }

    // send a mail
    let email = Message::builder()
        .from(state.mailer.mail_from.clone())
        .to(state.mailer.mail_to.clone())
        .subject("Contact Request from website")
        .header(header::ContentType::TEXT_HTML)
        .reply_to(data.email.parse().map_err(|_| ContactError::InvalidEmail)?)
        .body(mail_template(&data).into_string())
        .map_err(|e| ContactError::Fuck(e.into()))?;

    let mailer = SmtpTransport::relay(&state.mailer.smtp_host)
        .map_err(|e| ContactError::Fuck(e.into()))?
        .credentials(Credentials::new(
            state.mailer.smtp_user.clone(),
            state.mailer.smtp_pass.clone(),
        ))
        .build();

    match mailer.send(&email) {
        Ok(_) => (),
        Err(e) => tracing::error!("Could not send email: {:?}", e),
    }

    Ok(html! {
        p{"Thank your for your contact request"}
    }
    .into_response())
}

fn mail_template(value: &ContactData) -> Markup {
    html! {
        div class="mail"{
            h1 { "Conatct Request" }
            p { "Email: " (value.email) }
            p { "Subject: " (value.subject) }
            hr;
            p { (value.message) }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ContactData {
    pub email: String,
    pub subject: String,
    pub message: String,
    pub csrf: String,
    pub captcha: String,
}

pub enum ContactError {
    InvalidCaptcha,
    InvalidCsrf,
    NoMessage,
    InvalidEmail,
    Fuck(Box<dyn Error>),
}

impl IntoResponse for ContactError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ContactError::InvalidEmail => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid email").into_response()
            }
            ContactError::InvalidCaptcha => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid captcha").into_response()
            }
            ContactError::InvalidCsrf => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid csrf").into_response()
            }
            ContactError::NoMessage => {
                (axum::http::StatusCode::BAD_REQUEST, "No message").into_response()
            }
            ContactError::Fuck(e) => {
                tracing::error!(e);
                (axum::http::StatusCode::BAD_REQUEST, "Interal Server Error").into_response()
            }
        }
    }
}
