use sqlx::sqlite::SqlitePool;
use std::{env, fs, path::PathBuf};

use super::model::*;
use sqlx::Row;
use crate::plugins::browser_history::paths::Browser;



fn history_path(browser: &Browser) -> Option<PathBuf> {
    let user = env::var("USERNAME").ok()?;

    match browser {
        Browser::Chrome => Some(format!(
            r"C:\Users\{}\AppData\Local\Google\Chrome\User Data\Default\History",
            user
        ).into()),

        Browser::Edge => Some(format!(
            r"C:\Users\{}\AppData\Local\Microsoft\Edge\User Data\Default\History",
            user
        ).into()),

        Browser::Firefox => {
            let base = format!(
                r"C:\Users\{}\AppData\Roaming\Mozilla\Firefox\Profiles",
                user
            );

            fs::read_dir(base).ok()?
                .filter_map(|e| e.ok())
                .find(|e| e.path().join("places.sqlite").exists())
                .map(|e| e.path().join("places.sqlite"))
        }
    }
}

fn copy_to_temp(src: &PathBuf) -> Result<PathBuf, String> {
    let mut dst = env::temp_dir();
    dst.push("browser_history.sqlite");
    fs::copy(src, &dst).map_err(|e| e.to_string())?;
    Ok(dst)
}

pub async fn collect(browser: Browser) -> Result<BrowserHistorySnapshot, String> {
    let src = history_path(&browser).ok_or("History file not found")?;
    let temp = copy_to_temp(&src)?;

    let pool = SqlitePool::connect(
        &format!("sqlite://{}", temp.display())
    ).await.map_err(|e| e.to_string())?;

    let entries: Vec<BrowserHistoryEntry> = match browser {
        Browser::Chrome | Browser::Edge => {
            let rows = sqlx::query(
    r#"
    SELECT urls.url, urls.title, visits.visit_time
    FROM urls
    JOIN visits ON urls.id = visits.url
    ORDER BY visits.visit_time DESC
    LIMIT 500
    "#
)
.fetch_all(&pool)
.await
.map_err(|e| e.to_string())?;

rows.into_iter().map(|row| {
    BrowserHistoryEntry {
        url: row.get::<String, _>("url"),
        title: row.get::<Option<String>, _>("title"),
        visit_time: row.get::<i64, _>("visit_time"),
        browser: format!("{:?}", browser),
    }
}).collect()}

        Browser::Firefox => {
            use sqlx::Row;

let rows = sqlx::query(
    r#"
    SELECT urls.url, urls.title, visits.visit_time
    FROM urls
    JOIN visits ON urls.id = visits.url
    ORDER BY visits.visit_time DESC
    LIMIT 500
    "#
)
.fetch_all(&pool)
.await
.map_err(|e| e.to_string())?;

rows.into_iter().map(|row| {
    BrowserHistoryEntry {
        url: row.get::<String, _>("url"),
        title: row.get::<Option<String>, _>("title"),
        visit_time: row.get::<i64, _>("visit_time"),
        browser: format!("{:?}", browser),
    }
}).collect()

        }
    };

    Ok(BrowserHistorySnapshot {
        browser: format!("{:?}", browser),
        entries,
    })
}


	

