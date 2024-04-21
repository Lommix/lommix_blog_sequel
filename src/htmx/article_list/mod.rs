use crate::AppState;

use super::HtmxComponent;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, MethodRouter},
};
use maud::html;

pub struct ArticleList;
impl HtmxComponent<AppState> for ArticleList {
    fn path() -> &'static str {
        "/articles/:limit/:offset"
    }

    fn css() -> &'static str {
        include_str!("style.css")
    }

    fn handle() -> MethodRouter<AppState> {
        get(
            |Path((limit, offset)): Path<(usize, usize)>, State(state): State<AppState>| async move {
                let articles = state
                    .articles
                    .iter()
                    .skip(offset)
                    .enumerate()
                    .map_while(|(i, article)| {
                        if i < limit {
                            return Some(&article.meta);
                        }
                        None
                    })
                    .collect::<Vec<_>>();

                if articles.len() == 0 {
                    return "".into_response();
                }

                let html = html! {
                    @for meta in articles{
                        div class="article-preview" {
                            a
                                track=(meta.alias)
                                href=(format!("/article/{}", meta.alias))
                                hx-push-url=(format!("/article/{}", meta.alias))
                                hx-get=(format!("/htmx/article/{}", meta.alias))
                                hx-target="#main"
                                {
                                    image src=(format!("/{}",meta.cover));
                                };
                            div {
                                a
                                    track=(meta.alias)
                                    href=(format!("/article/{}", meta.alias))
                                    hx-push-url=(format!("/article/{}", meta.alias))
                                    hx-get=(format!("/htmx/article/{}", meta.alias))
                                    hx-target="#main"
                                    {
                                    h2 { (meta.title) }
                                }
                                p { (meta.teaser) };
                            };
                        }
                    }

                    div hx-trigger="revealed" hx-get=(format!("/htmx/articles/3/{}", offset + 3)) hx-swap="outerHTML" {
                        div class="loading-spinner"{}
                    }
                };

                html.into_response()
            },
        )
    }
}
