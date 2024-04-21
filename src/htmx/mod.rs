use axum::{
    http::header,
    routing::{get, MethodRouter},
    Router,
};

use crate::AppState;

mod about;
mod article_detail;
mod article_list;
mod blog;
mod contact;
mod feedback;
mod home;

pub(crate) fn htmx_router() -> Router<AppState> {
    HtmxRouter::new()
        .add(article_detail::ArticleDetail)
        .add(article_list::ArticleList)
        .add(home::HomeContent)
        .add(about::AboutContent)
        .add(blog::BlogContent)
        .add(contact::ContactContent)
        .add(feedback::Feedback)
        .into()
}

pub struct HtmxRouter<S>
where
    S: Clone + Sync + Send + 'static,
{
    router: Router<S>,
    css: String,
    js: String,
}

impl<S> HtmxRouter<S>
where
    S: Clone + Sync + Send + 'static,
{
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            css: String::new(),
            js: String::new(),
        }
    }

    pub fn add<T: HtmxComponent<S>>(mut self, comp: T) -> Self {
        self.css.push_str(T::css());
        self.js.push_str(T::js());
        self.router = self.router.route(T::path(), T::handle());
        self
    }
}

impl<S> Into<Router<S>> for HtmxRouter<S>
where
    S: Clone + Sync + Send + 'static,
{
    fn into(self) -> Router<S> {
        self.router
            .route(
                "/script.js",
                get(|| async {
                    axum::http::Response::builder()
                        .header(header::CONTENT_TYPE, "application/javascript")
                        .body(self.js)
                        .unwrap()
                }),
            )
            .route(
                "/style.css",
                get(|| async {
                    axum::http::Response::builder()
                        .header(header::CONTENT_TYPE, "text/css")
                        .body(self.css)
                        .unwrap()
                }),
            )
    }
}

pub trait HtmxComponent<S: Clone + Sync + Send + 'static> {
    fn path() -> &'static str;
    fn handle() -> MethodRouter<S>;
    fn css() -> &'static str {
        ""
    }
    fn js() -> &'static str {
        ""
    }
}
