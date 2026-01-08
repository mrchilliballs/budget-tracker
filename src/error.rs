use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use color_eyre::eyre::Error;

pub type Result<T> = core::result::Result<T, AppError>;

#[derive(Debug)]
pub struct AppError(Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
