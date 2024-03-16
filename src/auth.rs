use crate::{error::Error, Result, WebResult};
use core::fmt;

use chrono::prelude::*;
use jsonwebtoken::{decode, encode, EncodingKey, Header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection
};


const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret"; //TODO change to a long, securely stored string that is changed regularly
// TODO: consider having a JWT_SECRET per user to have the option to invalidate single users

#[derive(Clone, PartialEq)]
pub enum Role{
    User, 
    Admin
}
impl Role{
pub fn from_str(role: &str) -> Role{
    match role{
        "Admin" => Role::Admin,
        _ => Role::User
    }
}
}
impl fmt::Display for Role{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}


// data we save and expect inside our JWT
#[derive(Deserialize, Debug, Serialize)]
struct Claims{
    sub: String, //subject aka "Who"
    role: String,
    exp: usize, //expiration
}

pub fn with_auth(role: Role) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    // can be added to an endpoint with e.g. `.and(with_auth(Role::Admin))` which would mean 
    // the respective handler can only be accessed with Role Admin

    // clones the headers to avoid ownership issues and original request headers remain
    // accessible.
    headers_cloned()
    // map here returns a tuple of cloned role and request headers
    // `move` statement is used to move the ownership role into the closure
        .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
        .and_then(authorize)
}

pub fn create_jwt(uid: &str, role: &Role) -> Result<String>{
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(120)) //TODO: increase expiration for live system
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims{
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
    .map_err(|_| Error::JWTTokenCreationError)
}

async fn authorize((role, headers): (Role, HeaderMap<HeaderValue>)) -> WebResult<String>{
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(Error::JWTTokenError))?;
            
            // TODO: in case more roles are needed -> checkout casbin crate
            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(reject::custom(Error::NoPermissionError));
            }
            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(reject::custom(e))

    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String>{
    // checks if the authorization header is there, is valid, contains the 
    // Bearer prefix and extracts the JWT.
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::NoAuthHeaderError)
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()){
        Ok(v) => v,
        Err(_) => return Err(Error::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())

}