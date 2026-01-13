use sqlx::sqlite::SqlitePool;
use crate::plugins::browser_history::{
    model::{HistoryDump, HistoryEntry},
    paths::{Browser, history_path},
    utils::copy_to_temp,
};

pub async fn collect(browser: Browser) -> Result<HistoryDump, String> {
    let source = history_path(&browser)
        .ok_or("Nie znaleziono pliku historii")?;

    let temp = copy_to_temp(&source, "history_copy.sqlite")?;
    let db_url = format!("sqlite://{}", temp.display());

    let pool = SqlitePool::connect(&db_url).await
        .map_err(|e| e.to_string())?;

    let entries = match browser {
        Browser::Chrome | Browser::Edge => {
            sqlx::query!(
                r#"
                SELECT urls.url, urls.title, visits.visit_time
                FROM urls
                JOIN visits ON urls.id = visits.url
                ORDER BY visits.visit_time DESC
                LIMIT 100
                "#
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|r| HistoryEntry {
                url: r.url,
                title: r.title,
                visit_time: r.visit_time,
                browser: format!("{:?}", browser),
            })
            .collect()
        }

        Browser::Firefox => {
            sqlx::query!(
                r#"
                SELECT moz_places.url, moz_places.title, moz_historyvisits.visit_date
                FROM moz_places
                JOIN moz_historyvisits
                  ON moz_places.id = moz_historyvisits.place_id
                ORDER BY moz_historyvisits.visit_date DESC
                LIMIT 100
                "#
            )
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|r| HistoryEntry {
                url: r.url,
                title: r.title,
                visit_time: r.visit_date,
                browser: "Firefox".into(),
            })
            .collect()
        }
    };

    Ok(HistoryDump {
        browser: format!("{:?}", browser),
        entries,
    })
}
