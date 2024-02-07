use crate::{files::ArticleMeta, pages::PageMeta};
use maud::{html, Markup, DOCTYPE};

/// layout template
pub fn base(meta: &PageMeta, content: &Markup, css: Option<Markup>, js: Option<Markup>) -> Markup {
    html! {
            (DOCTYPE)
            head {
                meta charset="utf-8";
                title { (meta.title) };
                link rel="stylesheet" href="/static/main.css";

                @if let Some(css) = css {
                    (css)
                }

                // meta
                meta name="title" content=(meta.title);
                meta name="author" content="lommix";
                meta name="description" content=(meta.description);
                meta name="keywords" content=(meta.keywords);
                meta name="viewport" content="width=device-width, initial-scale=1.0";

                @if let Some(image) = &meta.image {
                    meta property="og:image" content=(format!("https://lommix.de/{}", image));
                }

                meta name="og:title" content=(meta.title);
                meta name="og:description" content=(meta.description);
            }
            body {

                (header())
                main {(content)}
                (footer())

                script src="/static/js/wasm_frame.js" type="module"{}
                script src="/static/js/highlight.min.js" {}
                script src="/static/js/htmx.min.js"{}
                script src="/static/main.js" type="module" {}
                script {"hljs.highlightAll();"};

                @if let Some(js) = js {
                    (js)
                }
            }
    }
}

/// header template
pub fn header() -> Markup {
    html! {
        header class="header"{
            div class="image" {
                img class="banner" src="/static/images/new_banner.svg" alt="Banner" {};
            }
            div class="header-bar" {
                h2 class="logo" { a href="/" {"[ Lommix's Blog ]"} }
                nav class="navbar"{
                    ul {
                        li { a href="/" { "[ Home ]" } };
                        li { a href="/blog" { "[ Blog ]" } };
                        li { a href="/about" { "[ About ]" } };
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
        footer class="footer" {
            nav class="footer-navbar"{
                ul {
                    li {a href="https://github.com/Lommix" alt="Github" target="_blank" {"[ Github ]"}}
                    li {a href="https://www.youtube.com/channel/UCd1BUXaUHWnnNLWknIgxFHg" target="_blank" alt="Youtube" {"[ Youtube ]"}}
                    li {a href="https://github.com/Lommix/lommix_blog_sequel" target="_blank" alt="Source" {"[ Source ]"}}
                }
            }
        }
    }
}

/// article preview
pub fn article_preview(meta: &ArticleMeta) -> maud::Markup {
    html! {
        div class="preview"{
            a href=(format!("/article/{}", meta.alias)) {
                image src=(meta.cover);
            };
            div {
                a href=(format!("/article/{}", meta.alias)) {
                    h2 { (meta.title) }
                }
                p { (meta.teaser) };
            };
        }
    }
}
