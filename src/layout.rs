use maud::{html, Markup};

pub fn base(title: &str, content: &Markup) -> Markup {
    html! {
        html {
            head {
                meta charset="utf-8";
                title { (title) }
                // Add your CSS, JavaScript, or other head elements here
                link rel="stylesheet" href="/assets/main.css";
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
            h1 { "Website Header" }
        }
    }
}
pub fn footer() -> Markup {
    html! {
        footer {
            p { "Copyright © 2023" }
            // Add your footer content here
        }
    }
}
