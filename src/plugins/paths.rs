use std::env;
use std::path::PathBuf;

pub enum Browser {
    Chrome,
    Edge,
    Firefox,
}

pub fn history_path(browser: &Browser) -> Option<PathBuf> {
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

            std::fs::read_dir(base).ok()?
                .filter_map(|e| e.ok())
                .find(|e| e.path().join("places.sqlite").exists())
                .map(|e| e.path().join("places.sqlite"))
        }
    }
}
