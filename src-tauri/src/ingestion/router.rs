use axum::Router;
use crate::plugins::{browser_history, terminal};

pub fn router() -> Router {
    Router::new()
        .merge(browser_history::routes())
        .merge(terminal::routes())
}
