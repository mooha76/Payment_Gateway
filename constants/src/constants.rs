
use once_cell::sync::Lazy;
use std::{path::PathBuf, time::Duration};

use configuration::initialize::AppConfig;
use configuration::env::get_env_source;
use utils::*;


pub const ENV_PREFIX: &str = "APP";
pub const CODE_LEN: usize = 5;
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(120);



pub static CONFIG: Lazy<AppConfig> =
    Lazy::new(|| AppConfig::read(get_env_source(ENV_PREFIX)).unwrap());




