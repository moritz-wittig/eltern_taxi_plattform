use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct User{
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
    // TODO: potentially to be extended
}



#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest{
    pub email: String,
    pub pw: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequestWrapper{
    pub user: LoginRequest
}



#[derive(Serialize)]
pub struct LoginResponse{
    // This (JWT) token is returned to the client, so the user in the future, 
    // can use it to further (then authenticated) requests by attaching it to the
    // header.
    pub token: String
}