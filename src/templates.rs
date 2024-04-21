use maud::{html, Markup, DOCTYPE};

/// layout template
pub fn base(meta: &Markup, content: &Markup) -> Markup {
    html! {
            (DOCTYPE)
            head {
                meta charset="utf-8";
                link rel="stylesheet" href="/static/main.css";
                link rel="stylesheet" href="/htmx/style.css" {}

                meta name="author" content="lommix";
                meta name="viewport" content="width=device-width, initial-scale=1.0";

                ((meta))
                // @if let Some(image) = &meta.image {
                //     meta property="og:image" content=(format!("https://lommix.com/{}", image));
                // }
                // meta name="og:title" content=(meta.title);
                // meta name="og:description" content=(meta.description);
            }
            body {

                (header())
                main id="main" {(content)}
                (footer())

                script src="/static/js/wasm_frame.js" type="module"{}
                script src="/static/js/highlight.min.js" {}
                script src="/static/js/htmx.min.js"{}
                script src="/htmx/script.js" type="module" {}
                script src="/static/main.js" type="module" {}

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
                h2 class="logo" { "Lommix's Blog" }
                nav hx-target="#main" class="nav"{
                    a class="nav-link" hx-get="/htmx/home" hx-push-url="/" { "Home" }
                    a class="nav-link" hx-get="/htmx/blog" hx-push-url="/blog" { "Blog" }
                    a class="nav-link" hx-get="/htmx/contact" hx-push-url="/contact" { "Contact" }
                    a class="nav-link" hx-get="/htmx/about" hx-push-url="/about" { "About" }
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


            a class="branding" href="https://github.com/Lommix/lommix_blog_sequel" target="_blank" {
                div class="htmx"{
                    span {"<"}
                    b {"/"}
                    span {">htm"}
                    b {"x"}
                    span style="color:white" {" & "}
                }
                img height="30" src="/static/images/ferris.svg"{}
                p{"Link to Repo"}
            }

            nav class="footer-navbar"{
                ul {
                    li {a class="footer-link" href="https://twitter.com/Lommix1" alt="Twitter" target="_blank" {
                        img height="30" src="/static/images/twitter.svg" {}
                    }}
                    li {a class="footer-link" href="https://github.com/Lommix" alt="Github" target="_blank" {
                        img height="30" src="/static/images/github.svg" {}
                    }}
                    li { a class="footer-link" href="https://www.youtube.com/channel/UCd1BUXaUHWnnNLWknIgxFHg" target="_blank" alt="Youtube" {
                        img height="30" src="/static/images/youtube.svg" {}
                    }}
                }
            }
            div {

            }
        }
    }
}
