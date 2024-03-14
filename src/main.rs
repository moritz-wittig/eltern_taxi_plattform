use auth::{with_auth, Role};
use error::Error::*;
use std::sync::Arc;
use std::collections::HashMap;
use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use warp::{reject, reply, Filter, Rejection, Reply};


mod auth;
mod error;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult = std::result::Result<T, Rejection>;
type Users = Arc<HashMap<String, User>>;

#[derive(Clone)]
pub struct User{
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest{
    pub email: String,
    pub pw: String
}

#[derive(Serialize)]
pub struct LoginResponse{
    pub token: String
}

#[tokio::main]
async fn main(){
    let users = Arc::new(init_users());

    let login_route = warp::path!("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler);

    let admin_route = warp::path!("admin")
        .and(with_auth(Role::User))
        .and_then(admin_handler);

    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .recover(error:handle_rejection)

    warp::serve(routes).run(([127, 0, 0, 0, 1], 8000)).await;
}

// Warp Filter injects the users parameter 
// Clone command indicates that this returned filter is clonable
fn with_users(users: User) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone{
    // returns a Warp filter that extracts a tuple (Users,) from the Request

    warp::any().map(move || users.clone())
}

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply>{
    match users
        .iter()
        .find(| _uid, user | user.email == body.email && user.pw == body.pw)
    {
        some((uid, user)) => {
            let token = auth::create_jwt(&uid, &Role::from_str(&user.role))
            .map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LoginResponse {token}))

        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}

fn init_users() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User {
            uid: String::from("1"),
            email: String::from("user@userland.com"),
            pw: String::from("1234"),
            role: String::from("User")
        },
    );
    map.insert(
        String::from("2"),
        User {
            uid: String::from("2"),
            email: String::from("admin@admin.com"),
            pw: String::from("4321"),
            role: String::from("Admin")
        },
    );
    map
}