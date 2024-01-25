use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

// pub fn error_chain_fmt(
//     e: &impl std::error::Error,
//     f: &mut std::fmt::Formatter<'_>,
// ) -> std::fmt::Result {
//     writeln!(f, "{}\n", e)?;
//     let mut current = e.source();
//     while let Some(cause) = current {
//         writeln!(f, "Caused by:\n\t{}", cause)?;
//         current = cause.source();
//     }
//     Ok(())
// }

// Make our own error that wraps `color_eyre::Report`.
#[derive(Debug)]
pub struct AppError(color_eyre::Report);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("\n{:?}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, color_eyre::Report>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<color_eyre::Report>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
