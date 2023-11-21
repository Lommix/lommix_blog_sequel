use maud::{html, Markup};

use crate::pages::PageMeta;

pub fn base(meta: &PageMeta, content: &Markup) -> Markup {
    html! {
        html {
            head {
                meta charset="utf-8";
                title { (meta.title) };
                link rel="stylesheet" href="/assets/main.css";
                // meta
                meta name="title" content="(meta.description)";
                meta name="description" content="This is my website";
            }
            body {
                (header())
                main {
                    (content)
                }
                (footer())
                script src="/assets/main.js" type="module" {}
            }
        }
    }
}

pub fn header() -> Markup {
    html! {
        header {
            img src="/assets/images/new_banner.svg" alt="Banner" class="banner";
            div {
                span alt="" { "[ Lommix's Blog ]" }
                nav {
                    a href="/" { "Home" }
                    a href="/about" { "About" }
                }
            }
        }
    }
}
pub fn footer() -> Markup {
    html! {
        footer {
            a href="https://github.com" {"Github"}
            a href="https://youtube.com" {"Youtube"}
            p { "Copyright © 2023" }
        }
    }
}
