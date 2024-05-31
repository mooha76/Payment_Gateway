use std::sync::Arc;

use tokio::sync::Notify;

use client::{

    http::HttpClient,
    ClientBuilder,
};
use configuration::initialize::AppConfig;
use errors::errors::AppResult;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub http: HttpClient,
}

impl AppState {
    pub async fn new(config: AppConfig) -> AppResult<Self> {

        let http = HttpClient::build_from_config(&config)?;
        Ok(Self {
            config: Arc::new(config),
            http,
        })
    }
}
