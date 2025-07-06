use actix_web::{cookie, ResponseError};

#[derive(Debug, thiserror::Error, Clone)]
pub struct CookieParsingError(#[from] cookie::ParseError);

impl ResponseError for CookieParsingError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }
}

impl std::fmt::Display for CookieParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
