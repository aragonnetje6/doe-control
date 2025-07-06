use std::future::{ready, Ready};

use actix_web::{cookie, FromRequest};

use super::cookie_parsing_error::CookieParsingError;

pub struct CookieJar(cookie::CookieJar);

impl CookieJar {
    #[must_use]
    pub fn signed(&self, key: &cookie::Key) -> cookie::SignedJar<&cookie::CookieJar> {
        self.0.signed(key)
    }

    #[must_use]
    pub fn signed_mut(&mut self, key: &cookie::Key) -> cookie::SignedJar<&mut cookie::CookieJar> {
        self.0.signed_mut(key)
    }

    #[must_use]
    pub fn private(&self, key: &cookie::Key) -> cookie::PrivateJar<&cookie::CookieJar> {
        self.0.private(key)
    }

    #[must_use]
    pub fn private_mut(&mut self, key: &cookie::Key) -> cookie::PrivateJar<&mut cookie::CookieJar> {
        self.0.private_mut(key)
    }
}

impl std::ops::Deref for CookieJar {
    type Target = cookie::CookieJar;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for CookieJar {
    type Error = CookieParsingError;

    type Future = Ready<Result<Self, CookieParsingError>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let mut jar = cookie::CookieJar::new();
        let cookies = match req.cookies() {
            Ok(cookies) => cookies,
            Err(err) => return ready(Err(CookieParsingError::from(err))),
        };
        for cookie in cookies.iter() {
            jar.add_original(cookie.clone());
        }
        ready(Ok(Self(jar)))
    }
}
