use std::env;
use std::path::PathBuf;

pub struct Config {
    namespace: PathBuf,
}

impl Into<PathBuf> for Config {
    fn into(self) -> PathBuf {
        self.namespace
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            namespace: env::home_dir().unwrap().join(".local/share/irnix"),
        }
    }
}

pub fn unwrap_namespace(namespace: Option<PathBuf>) -> PathBuf {
    namespace.unwrap_or(
        env::var("IRNIX_NAMESPACE")
            .ok()
            .map(PathBuf::from)
            .unwrap_or(Config::default().into()),
    )
}
