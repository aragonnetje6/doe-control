use std::future::{ready, Ready};

use actix_web::{cookie, FromRequest, ResponseError};

#[derive(Debug, thiserror::Error)]
struct CookieParsingError(#[from] cookie::ParseError);

impl ResponseError for CookieParsingError {}

impl std::fmt::Display for CookieParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

struct CookieJar(cookie::CookieJar);

impl FromRequest for CookieJar {
    type Error = CookieParsingError;

    type Future = Ready<Result<Self, CookieParsingError>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let mut jar = cookie::CookieJar::new();
        let cookies = match req.cookies() {
            Ok(cookies) => cookies,
            Err(err) => return ready(Err(CookieParsingError(err))),
        };
        for cookie in cookies.iter() {
            jar.add_original(cookie.clone());
        }
        ready(Ok(Self(jar)))
    }
}
