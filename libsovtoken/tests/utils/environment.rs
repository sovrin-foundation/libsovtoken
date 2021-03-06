extern crate dirs;

use std::env;
use std::path::PathBuf;

pub struct EnvironmentUtils {}

impl EnvironmentUtils {
    pub fn indy_home_path() -> PathBuf {
        // TODO: FIXME: Provide better handling for the unknown home path case!!!
        let mut path = dirs::home_dir().unwrap_or(PathBuf::from("/home/indy"));
        path.push(if cfg!(target_os = "ios") { "Documents/.indy_client" } else { ".indy_client" });
        path
    }

    pub fn tmp_path() -> PathBuf {
        let mut path = env::temp_dir();
        path.push("indy_client");
        path
    }

    pub fn tmp_file_path(file_name: &str) -> PathBuf {
        let mut path = EnvironmentUtils::tmp_path();
        path.push(file_name);
        path
    }

    pub fn test_pool_ip() -> String {
        env::var("TEST_POOL_IP").unwrap_or("127.0.0.1".to_string())
    }
}