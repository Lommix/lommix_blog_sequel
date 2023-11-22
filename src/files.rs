use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{body::StreamBody, http::Uri, response::IntoResponse};
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use serde::Deserialize;

use crate::pages::PageMeta;
use crate::AppState;

const ALLOWED_EXTENSIONS: [&str; 12] = [
    "jpg", "jpeg", "svg", "gz", "png", "gif", "webm", "wasm", "js", "css", "html", "ico",
];

#[derive(Debug)]
pub struct Articles(Vec<Article>);
impl Articles {
    pub fn find_by_alias(&self, alias: &str) -> Option<&Article> {
        self.0.iter().find(|a| a.meta.alias == alias)
    }

    pub fn iter(&self) -> std::slice::Iter<Article> {
        self.0.iter()
    }

    pub fn from_dir(path: PathBuf) -> anyhow::Result<Self> {
        let mut articles = BlogFsIter::new(path)?.collect::<Vec<_>>();
        articles.sort_by(|a, b| b.meta.published_at.cmp(&a.meta.published_at));
        Ok(Self(articles))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Article {
    pub meta: ArticleMeta,
    pub source: PathBuf,
    pub dir: PathBuf,
    pub compiled: Option<String>,
    pub files: Vec<PathBuf>,
}

impl Article {
    pub fn valid(&self) -> bool {
        self.source.exists() && !self.meta.title.is_empty() && !self.meta.alias.is_empty()
    }

    pub async fn compile(&mut self) -> anyhow::Result<()> {
        self.compiled = Some(read_markdown(self.source.clone()).await?);
        Ok(())
    }
}

pub async fn read_markdown(path: PathBuf) -> anyhow::Result<String> {
    let raw = tokio::fs::read_to_string(&path).await?;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&raw, options);
    let mut out = String::new();
    pulldown_cmark::html::push_html(&mut out, parser);
    Ok(out)
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct ArticleMeta {
    pub title: String,
    pub alias: String,
    pub cover: String,
    pub tags: Option<String>,
    pub published: String,
    pub teaser: String,

    #[serde(skip)]
    pub published_at: chrono::NaiveDate,
}

impl Into<PageMeta> for ArticleMeta {
    fn into(self) -> PageMeta {
        PageMeta {
            title: self.title,
            description: self.teaser,
            keywords: "".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlogFsIter {
    files: Vec<Article>,
}

fn read_blog_path(path: PathBuf) -> anyhow::Result<Vec<Article>> {
    let mut out = Vec::new();
    let dir = std::fs::read_dir(path)?;

    let mut article = Article::default();
    dir.into_iter()
        .flatten()
        .for_each(|file| match file.path().is_dir() {
            true => {
                let _ = read_blog_path(file.path()).and_then(|d| {
                    out.extend(d);
                    Ok(())
                });
            }
            false => {
                // filter for .md file
                let path = file.path();
                let extension = match path.extension() {
                    Some(ext) => ext.to_str().unwrap_or(""),
                    None => return,
                };

                match file.file_name().to_str().unwrap_or("") {
                    "meta.ron" => {
                        article.meta =
                            ron::from_str::<ArticleMeta>(&std::fs::read_to_string(path).unwrap())
                                .expect("Invalid meta.ron");
                        article.meta.published_at =
                            chrono::NaiveDate::parse_from_str(&article.meta.published, "%d.%m.%Y")
                                .expect("Invalid date format");
                    }
                    "content.md" => {
                        article.source = path;
                    }
                    _ => {
                        if ALLOWED_EXTENSIONS.contains(&extension) {
                            article.files.push(path);
                        }
                    }
                }
            }
        });

    if article.valid() {
        out.push(article);
    }

    Ok(out)
}

impl BlogFsIter {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let files = read_blog_path(path)?;
        Ok(Self { files })
    }
}

impl std::iter::Iterator for BlogFsIter {
    type Item = Article;
    fn next(&mut self) -> Option<Self::Item> {
        self.files.pop()
    }
}

pub async fn static_files(uri: Uri) -> axum::response::Response {
    let path = PathBuf::from(uri.path());

    let file = match tokio::fs::File::open(&path.strip_prefix("/").unwrap()).await {
        Ok(f) => f,
        Err(_) => return (StatusCode::NOT_FOUND, "file does not exists").into_response(),
    };

    let content_type = match mime_guess::from_path(path).first_raw() {
        Some(m) => m,
        None => return (StatusCode::BAD_REQUEST, "type does not exists").into_response(),
    };

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = StreamBody::new(stream);
    let headers = [(axum::http::header::CONTENT_TYPE, content_type)];

    (headers, body).into_response()
}

pub async fn media(
    Path((alias, file)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> axum::response::Response {
    match state.articles.find_by_alias(&alias) {
        Some(article) => {
            for path in &article.files {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name == file {
                    let file = match tokio::fs::File::open(path).await {
                        Ok(f) => f,
                        Err(_) => {
                            return (StatusCode::NOT_FOUND, "file does not exists").into_response()
                        }
                    };

                    let content_type = match mime_guess::from_path(path).first_raw() {
                        Some(m) => m,
                        None => {
                            return (StatusCode::BAD_REQUEST, "type does not exists")
                                .into_response()
                        }
                    };

                    let stream = tokio_util::io::ReaderStream::new(file);
                    let body = StreamBody::new(stream);
                    let headers = [(axum::http::header::CONTENT_TYPE, content_type)];

                    return (headers, body).into_response();
                }
            }
        }
        None => (),
    }
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}
