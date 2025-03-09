mod auth;
mod reqwest;

pub use self::{
    auth::*,
};
use actix_web::{
    HttpRequest,
    web,
};
use crate::schema;
use serde::{Deserialize, Serialize};
use crate::diesel::{
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
    PgConnection,
};
use crate::models::User;
use std::fs::File;
use std::io::Write;


#[derive(Deserialize)]
pub struct FileForm {
    pub name: String,
    pub size:  i32,
}

pub fn save_file(data: String) -> String {
    let file_data: FileForm = serde_json::from_str(&data).unwrap();
    let path = "/beaches_front/media/".to_owned() + &file_data.name;
    let mut f = File::create(&path).expect("Unable to create file");
    f.write_all_at(data.as_bytes()).expect("Unable to write data");
    println!("f: {:?}", f);
    return path.replace("/beaches_front", "");
} 

#[derive(Deserialize, Serialize)]
pub struct NewUserForm {
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub password:   String,
}

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn get_secret<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("secret")?.to_str().ok();
}

pub fn is_signed_in(req: &HttpRequest) -> bool {
  get_secret(&req).is_some()
}

pub fn get_current_user(req: &HttpRequest) -> User {

    let secret = get_secret(&req).unwrap();
    let uuid = hex::decode(secret).expect("failed decode");
    let _connection = establish_connection();
    return schema::users::table
        .filter(schema::users::uuid.eq(uuid))
        .first::<User>(&_connection)
        .expect("Error.");
} 

pub fn get_limit (
    limit: Option<i64>,
    default_limit: i64
) -> i64 {
    let _limit: i64;
    if limit.is_some() {
        let l_unwrap = limit.unwrap();
        if l_unwrap > 100 {
            _limit = default_limit;
        }
        else {
            _limit = l_unwrap;
        }
    }
    else {
        _limit = default_limit;
    }
    _limit
}

pub fn get_page(req: &HttpRequest) -> i32 {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub page: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let page: i32;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
    }
    else {
        page = 1;
    }
    page
}

pub fn get_id(req: &HttpRequest) -> i32 {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub id: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let id: i32;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.id.is_some() {
            id = params.id.unwrap();
        }
        else {
            id = 0;
        }
    }
    else {
        id = 0;
    }
    id
}


pub fn get_user(id: String) -> User { 
    let _connection = establish_connection();
    return schema::users::table
        .filter(schema::users::id.eq(id))
        .first::<User>(&_connection)
        .expect("Error.");
}