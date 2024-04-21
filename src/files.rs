use std::collections::HashMap;
use std::path::PathBuf;

use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use serde::Deserialize;

use crate::pages::PageMeta;

const ALLOWED_EXTENSIONS: [&str; 12] = [
    "jpg", "jpeg", "svg", "gz", "png", "gif", "webm", "wasm", "js", "css", "html", "ico",
];

#[derive(Debug)]
pub struct ArticleStore(Vec<Article>);
impl ArticleStore {
    pub fn find_by_alias(&self, alias: &str) -> Option<&Article> {
        self.0.iter().find(|a| a.meta.alias == alias)
    }

    pub fn iter(&self) -> std::slice::Iter<Article> {
        self.0.iter()
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub async fn from_dir(path: PathBuf) -> anyhow::Result<Self> {
        let mut articles = BlogFsIter::new(path)?.collect::<Vec<_>>();
        articles.sort_by(|a, b| b.meta.published_at.cmp(&a.meta.published_at));

        for article in &mut articles {
            article.compile().await?;
        }

        Ok(Self(articles))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Article {
    pub meta: ArticleMeta,
    pub source: PathBuf,
    pub dir: PathBuf,
    pub compiled: Option<String>,
    pub files: HashMap<String, PathBuf>,
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
            keywords: self.tags.unwrap_or("".into()),
            image: Some(format!("https:/lommix.com/{}", self.cover)),
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
                            let base_name = path.file_name().unwrap().to_str().unwrap().to_string();
                            article.files.insert(base_name, path);
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
