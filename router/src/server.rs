use axum::routing::get;
use state::mystate::AppState;

use handler::server::health_check;
pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/api/v1/server/health_check", get(health_check))
}
