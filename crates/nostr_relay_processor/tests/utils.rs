use global_utils::logger::{LoggerGuard, init_logger};
use std::sync::LazyLock;

pub static TEST_LOGGER: LazyLock<LoggerGuard> = LazyLock::new(init_logger);
pub const DEFAULT_RELAY_LIST: [&str; 1] = ["wss://relay.damus.io"];
pub const DEFAULT_CLIENT_TIMEOUT: u64 = 10;
