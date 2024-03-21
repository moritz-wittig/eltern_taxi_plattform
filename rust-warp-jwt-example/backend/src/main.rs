use auth::{with_auth, Role};
use common::{User, LoginRequest, LoginResponse};
use error::Error::*;
use std::sync::Arc;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::RwLock;

use warp::{reject, reply, Filter, Rejection, Reply};


mod auth;
mod error;


type Result<T> = std::result::Result<T, error::Error>; // internal for propagating errors
type WebResult<T> = std::result::Result<T, Rejection>; // external for sending errors to caller
type Users = Arc<RwLock<HashMap<String, User>>>;


#[tokio::main]
async fn main(){
    // RwLock (ReadWriteLock) for concurrent access to shared data
    // Arc: an atomic, reference counted smart pointer
    // --> with these both, we can share users across different threads
    let users = Arc::new(RwLock::new(init_users()));

    let login_route = warp::path!("login")
        .and(warp::post()) // only post requests
        .and(with_users(users.clone())) //injects users data into handler 
        .and(warp::body::json()) //parsing request body from json to rust
        .and_then(login_handler); // which handler to be invoked when all previous conditions are met

    let user_route = warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler);

    let admin_route = warp::path!("admin")
        .and(with_auth(Role::User))
        .and_then(admin_handler);

    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

// Warp Filter injects the users parameter 
// Clone command indicates that this returned filter is clonable
fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    // returns a Warp filter that extracts a tuple (Users,) from the Request.
    // This filter is used to pass the users map to endpoints.
    warp::any().map(move || users.clone())
}

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    
    match users.read() { //read() gives a read-lock on the map
        Ok(read_handle) => {
            match read_handle
                .iter()
                .find(|(_uid, user)| user.email == body.email && user.pw == body.pw)
            {
                Some((uid, user)) => {
                    let token = auth::create_jwt(&uid, &Role::from_str(&user.role))
                        .map_err(|e| reject::custom(e))?;
                    Ok(reply::json(&LoginResponse { token }))
                }
                None => Err(reject::custom(WrongCredentialsError)),
            }
        }
        Err(_) => Err(reject()),
    }
}


pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}

pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello Admin {}", uid))
}


fn init_users() -> HashMap<String, User> {
    // HashMap for easy search user based on uid
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