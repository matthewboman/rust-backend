#![allow(unused_imports)]
use async_trait::async_trait; // https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
use axum::{
    extract::{FromRequest, RequestParts},
    Extension,
};
use biscuit::{
    jwa::SignatureAlgorithm,
    jws::Header,
    Empty,
    JWT,
};
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};

use crate::{
    errors::AuthError::{self, InvalidAuthHeaderError},
    jwks::{get_secret_from_key_set, JWKS},
};

const BEARER: &str = "Bearer ";

/// The token's Subject claim, which corresponds with the username
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Subject(
    pub Option<String>
);

// TODO: learn lifetimes w/ implementations

#[cfg(not(feature = "integration"))]
#[async_trait]
impl<B> FromRequest<B> for Subject
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> std::result::Result<Self, Self::Rejection> {
        let Extension(jwks): Extension<&'static JWKS> = Extension::from_request(req)
            .await
            .expect("The JWKS laer is missing");

        // TODO: learn match statements
        match jwt_from_header(req.headers()) {
            Ok(Some(jwt)) => {
                // First, extract without verifying the header to locate the key-id (kid)
                let token = JWT::<Empty, Empty>::new_encoded(jwt);

                let header: Header<Empty> = token
                    .unverified_header()
                    .map_err(AuthError::JWTTokenError)?;

                // https://docs.rs/biscuit/latest/biscuit/jws/struct.RegisteredHeader.html
                // The Key ID. This is currently not implemented (correctly). Serialized to kid. Defined in RFC7515#4.1.3.
                let key_id = header.registered.key_id.ok_or(AuthError::JWKSError)?;

                debug!("Fetching signing key for '{:?}'", key_id);

                // Now that we have the key, construct our RSA public key secret
                let secret = get_secret_from_key_set(jwks, &key_id)
                    .map_err(|_err| AuthError::JWKSError)?;

                // Fully extract and verify the token
                let token  = token
                    .into_decoded(&secret, SignatureAlgorithm::RS256)
                    .map_err(AuthError::JWTTokenError)?;

                let payload = token.payload().map_err(AuthError::JWTTokenError)?;
                let subject = payload.registered.subject.clone();

                debug!("Successfully verified token with subject: {:?}", subject);

                Ok(Subject(subject))
            }
            Ok(None) => Ok(Subject(None)),
            Err(e)   => Err(e)
        }
    }
}

// If an authorization header is provided, verify it's the expect format and return a String.
pub fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<Option<&str>, AuthError> {
    let header = if let Some(value) = headers.get(AUTHORIZATION) {
        value
    } else {
        // No Authorization header found
        return Ok(None);
    };

    let auth_header = if let Ok(value) = std::str::from_utf8(header.as_bytes()) {
        value
    } else {
        // Authorization header couldn't be decoded
        return Ok(None);
    };

    if !auth_header.starts_with(BEARER) {
        return Err(InvalidAuthHeaderError);
    }

    Ok(Some(auth_header.trim_start_matches(BEARER)))
}
