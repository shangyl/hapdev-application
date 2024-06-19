use crate_config::prelude::ConfigError;
use hapdev_app::prelude::config::FusionError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Application(FusionError),
    SqlError(sqlx::Error),
    Custom(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::SqlError(err)
    }
}

impl From<FusionError> for AppError {
    fn from(err: FusionError) -> Self {
        AppError::Application(err)
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::Custom(err)
    }
}

impl<'a> From<&'a str> for AppError {
    fn from(err: &'a str) -> Self {
        AppError::Custom(err.to_string())
    }
}

impl From<ConfigError> for AppError {
    fn from(err: ConfigError) -> Self {
        AppError::Custom(err.to_string())
    }
}
