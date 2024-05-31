use state::mystate::AppState;
use axum::Router;

use crate::server;


pub fn create_router_app(state: AppState) -> Router {
    let router = Router::new();
    let router = server::add_routers(router);

    router.with_state(state)
}
