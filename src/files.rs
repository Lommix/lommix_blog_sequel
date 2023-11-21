use std::path::PathBuf;

use axum::{body::StreamBody, http::Uri, response::IntoResponse};
use serde::Deserialize;

use crate::pages::PageMeta;

#[derive(Deserialize, Clone, Debug)]
pub struct ArticleMeta {
    pub title: String,
    pub alias: String,
    pub published: Option<String>,
    pub teaser: String,
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
    files: Vec<(PathBuf, ArticleMeta)>,
}

fn read_blog_path(path: PathBuf) -> anyhow::Result<Vec<(PathBuf, ArticleMeta)>> {
    let mut out = Vec::new();
    let dir = std::fs::read_dir(path)?;
    let mut markdown = None;
    let mut meta = None;
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
                match extension {
                    "md" => {
                        markdown = Some(path);
                    }
                    "ron" => {
                        meta = Some(path);
                    }
                    _ => {}
                }
            }
        });

    if let (Some(markdown), Some(meta)) = (markdown, meta) {
        let m = ron::from_str::<ArticleMeta>(&std::fs::read_to_string(meta)?)?;
        out.push((markdown, m));
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
    type Item = (PathBuf, ArticleMeta);
    fn next(&mut self) -> Option<Self::Item> {
        self.files.pop()
    }
}

pub async fn static_files(uri: Uri) -> axum::response::Response {
    let path = PathBuf::from(uri.path());

    let file = match tokio::fs::File::open(&path.strip_prefix("/").unwrap()).await {
        Ok(f) => f,
        Err(_) => {
            return (axum::http::StatusCode::BAD_REQUEST, "file does not exists").into_response()
        }
    };

    let content_type = match mime_guess::from_path(path).first_raw() {
        Some(m) => m,
        None => {
            return (axum::http::StatusCode::BAD_REQUEST, "type does not exists").into_response()
        }
    };

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = StreamBody::new(stream);
    let headers = [(axum::http::header::CONTENT_TYPE, content_type)];

    (headers, body).into_response()
}
