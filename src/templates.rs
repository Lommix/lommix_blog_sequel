use crate::{files::ArticleMeta, pages::PageMeta};
use maud::{html, Markup};

/// layout template
pub fn base(meta: &PageMeta, content: &Markup) -> Markup {
    html! {
            head {
                meta charset="utf-8";
                title { (meta.title) };
                link rel="stylesheet" href="/assets/main.css";
                // meta
                meta name="title" content="(meta.title)";
                meta name="description" content="(meta.description)";
            }
            body {

                (header())
                main class="container" {(content)}
                (footer())

                script src="/assets/js/highlight.min.js" {}
                script src="/assets/js/htmx.min.js"{}
                script src="/assets/js/response-targets.js" {}
                script src="/assets/js/pako.min.js" {}
                script src="/assets/main.js" type="module" {}

                script {"hljs.highlightAll();"};
            }
    }
}

/// header template
pub fn header() -> Markup {
    html! {
        header class="container"{
            div class="image" {
                img class="banner" src="/assets/images/new_banner.svg" alt="Banner" {};
            }
            div class="header-bar" {
                h1 class="logo" { a href="/" {"[ Lommix's Blog ]"} }
                nav class="navbar"{
                    ul {
                        li { a href="/" { "Home" } }
                        li { a href="/about" { "About" } }
                    }
                }
            }
            p {"Gamedev, web wizardry & educational content"}
        }
    }
}

/// footer template
pub fn footer() -> Markup {
    html! {
        footer {
            nav class="footer-navbar"{
                ul {
                    li {a href="https://github.com" {"Github"}}
                    li {a href="https://youtube.com" {"Youtube"}}
                }
            }
        }
    }
}

/// article preview
pub fn article_preview(meta: &ArticleMeta) -> maud::Markup {
    html! {
        div class="grid preview"{
            a href=(format!("/article/{}", meta.alias)) {
                image src=(meta.cover);
            };
            div class="container"{
                a href=(format!("/article/{}", meta.alias)) {
                    h1 { (meta.title) }
                }
                p { (meta.teaser) };
            };
        }
    }
}
