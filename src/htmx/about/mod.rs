use super::HtmxComponent;
use crate::AppState;
use axum::{
    response::IntoResponse,
    routing::{get, MethodRouter},
};
use maud::html;

pub struct AboutContent;

impl HtmxComponent<AppState> for AboutContent {
    fn path() -> &'static str {
        "/about"
    }

    fn css() -> &'static str {
        include_str!("style.css")
    }

    fn handle() -> MethodRouter<AppState> {
        get(|| async move {
            html!{
                h1{"About"}
                hr {}
                div class="about-content" {
                    img src="/static/images/mcu.jpg"{}
                    p{"My name is Lorenz, a web/game developer from Germany with a passion for exploring and learning new things. I am driven by curiosity and a desire to continuously learn and explore new ideas, which has led me on a journey from hardware development to embedded systems and eventually to web development and gaming. My interests lie in the field of system design, experimentation, and improving upon existing technologies through constant exploration and seeking out new ways of doing things."}
                }
                p{"Dram Crystal Display Driver, AVR-Programming adapter"}
            }.into_response()
        })
    }
}
