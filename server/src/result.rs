use crate_config::prelude::ConfigError;
use hapdev_app::prelude::config::FusionError;

pub type AppServerResult<T> = Result<T, AppServerError>;

#[derive(Debug)]
pub enum AppServerError {
    Application(FusionError),
    SqlError(sqlx::Error),
    Custom(String),
}

impl From<sqlx::Error> for AppServerError {
    fn from(err: sqlx::Error) -> Self {
        AppServerError::SqlError(err)
    }
}

impl From<FusionError> for AppServerError {
    fn from(err: FusionError) -> Self {
        AppServerError::Application(err)
    }
}

impl From<String> for AppServerError {
    fn from(err: String) -> Self {
        AppServerError::Custom(err)
    }
}

impl<'a> From<&'a str> for AppServerError {
    fn from(err: &'a str) -> Self {
        AppServerError::Custom(err.to_string())
    }
}

impl From<ConfigError> for AppServerError {
    fn from(err: ConfigError) -> Self {
        AppServerError::Custom(err.to_string())
    }
}
