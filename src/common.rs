use maud::html;

use crate::files::ArticleMeta;

pub fn article_preview(meta: &ArticleMeta) -> maud::Markup {
    html! {
        div class="preview-box"{
            image src=(meta.cover)
            a href=(format!("/article/{}", meta.alias)) {
                h1 { (meta.title) }
            }
            p { (meta.teaser) }
        }
    }
}
