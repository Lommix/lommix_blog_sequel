use deadpool::unmanaged::Pool;

const DB_PATH: &'static str = "lommix.db";

pub(crate) async fn open_or_create_db() -> anyhow::Result<Pool<rusqlite::Connection>> {
    if tokio::fs::metadata(DB_PATH).await.is_err() {
        let connection = rusqlite::Connection::open(DB_PATH)?;
        connection.execute(
            r#"
            CREATE TABLE IF NOT EXISTS clicks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                page TEXT NOT NULL,
                count INTEGER DEFAULT 0,
                date INTEGER DEFAULT 0,
                UNIQUE(page, date)
            );
            "#,
            [],
        )?;
    }

    let pool = Pool::from(vec![
        rusqlite::Connection::open(DB_PATH)?,
        rusqlite::Connection::open(DB_PATH)?,
        rusqlite::Connection::open(DB_PATH)?,
        rusqlite::Connection::open(DB_PATH)?,
    ]);

    Ok(pool)
}

pub async fn inc(pool: &Pool<rusqlite::Connection>, page: &str) -> anyhow::Result<()> {
    let Ok(con) = pool.get().await else {
        anyhow::bail!("server error");
    };

    let mut stmt = con.prepare(
        r#"
INSERT INTO clicks (page, count, date)
VALUES (?1, 1, ?2)
ON CONFLICT(page, date) DO UPDATE SET count = count + 1;
"#,
    )?;

    let now = time::OffsetDateTime::now_utc()
        .date()
        .with_time(time::Time::MIDNIGHT)
        .assume_offset(time::UtcOffset::UTC)
        .unix_timestamp();

    stmt.execute(rusqlite::params![page, now])?;
    Ok(())
}

#[derive(Debug)]
pub struct Stats {
    _id: i64,
    page: String,
    clicks: i64,
    date: i64,
}

fn pad(input: &str, min_length: usize) -> String {
    let mut output = String::from(input);
    while output.len() < min_length {
        output.push(' ');
    }
    output
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date_string = time::OffsetDateTime::from_unix_timestamp(self.date)
            .ok()
            .map(|date| {
                time::format_description::parse("[year]-[month]-[day]")
                    .ok()
                    .and_then(|formatter| Some(date.format(&formatter).ok()).flatten())
            })
            .flatten()
            .unwrap_or("date format failed".to_string());

        write!(
            f,
            "{}:{} count:[{}]",
            pad(self.page.as_str(), 35),
            pad(date_string.as_str(), 12),
            self.clicks
        )
    }
}

pub async fn stats(pool: &Pool<rusqlite::Connection>) -> anyhow::Result<Vec<Stats>> {
    let Ok(con) = pool.get().await else {
        anyhow::bail!("failed to get con from pool");
    };

    let mut stmt = con.prepare(
        r#"
       SELECT *
       FROM clicks
       ORDER BY date DESC
       LIMIT 60;
       "#,
    )?;

    let stats = stmt
        .query_map(rusqlite::params![], |row| {
            Ok(Stats {
                _id: row.get(0)?,
                page: row.get(1)?,
                clicks: row.get(2)?,
                date: row.get(3)?,
            })
        })?
        .flatten()
        .collect::<Vec<_>>();

    Ok(stats)
}
