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
                meta name="title" content=(meta.title);
                meta name="author" content="lommix";
                meta name="description" content=(meta.description);
                meta name="keywords" content=(meta.keywords);

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

                script src="/assets/js/highlight.min.js" {}
                script src="/assets/js/htmx.min.js"{}
                script src="/assets/js/pako.min.js" {}
                script src="/assets/main.js" type="module" {}

                script {"hljs.highlightAll();"};
            }
    }
}

/// header template
pub fn header() -> Markup {
    html! {
        header class="header"{
            div class="image" {
                img class="banner" src="/assets/images/new_banner.svg" alt="Banner" {};
            }
            div class="header-bar" {
                h2 class="logo" { a href="/" {"[ Lommix's Blog ]"} }
                nav class="navbar"{
                    ul {
                        li { a href="/" { "[ Home ]" } }
                        li { a href="/about" { "[ About ]" } }
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
