mod auth;

pub use self::{
    auth::*,
};
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    dev::ConnectionInfo,
};
use crate::schema;
use serde::{Deserialize, Serialize};
use actix_session::Session;
use crate::errors::AuthError;
use diesel::{PgConnection, Connection};


#[derive(Serialize)]
pub struct ErrorParams {
    pub error: String,
}
#[derive(Serialize)]
pub struct InfoParams {
    pub info: String,
}

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}