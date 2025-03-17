use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::Json,
};
use crate::api_error::ApiError;
use serde::{Deserialize, Serialize};
use crate::utils::{
    verify,
};
use crate::models::{
    User, 
    SessionUser, 
};
use crate::errors::AuthError;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.route("/signup/", web::post().to(process_signup));
    config.route("/login/", web::post().to(login));
    config.route("/get_user_data/", web::post().to(get_user_data));
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IdUser {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub email:    String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUserJson {
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub password:   String,
    //pub token:      String,
}

#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct AuthResp {
    pub id:         String,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub perm:       i16,
    pub image:      Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct AuthResp2 {
    pub id:         String,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub perm:       i16,
    pub image:      Option<String>,
    pub uuid:       String,
}

fn find_user(email: String, password: String) -> Result<User, AuthError> {
    let user_some = User::get_user_with_email(&email); 
    if user_some.is_ok() { 
        let _user = user_some.expect("Error.");
        if let Ok(matching) = verify(&_user.password, &password) {
            if matching {
                return Ok(_user);
            }
        }
    }
    Err(AuthError::NotFound(String::from("User not found")))
}
fn find_user_with_id(id: String) -> Result<User, AuthError> {
    let user_some = User::get_user_with_id(&id); 
    if user_some.is_ok() { 
        let _user = user_some.expect("Error.");
        return Ok(_user);
    }
    Err(AuthError::NotFound(String::from("User not found")))
}

pub async fn login(req: HttpRequest, data: Json<LoginUser2>) -> Json<AuthResp2> {
    let result = find_user(data.email.clone(), data.password.clone());
    match result {
        Ok(_new_user) => {
            return Json(AuthResp2 { 
                id:         _new_user.id.clone(),
                first_name: _new_user.first_name.clone(),
                last_name:  _new_user.last_name.clone(),
                email:      _new_user.email.clone(),
                perm:       _new_user.perm,
                image:      _new_user.image.clone(),
                uuid:       _new_user.get_uuid(),
            });   
        },
        Err(err) => {
            return Json(AuthResp2 {
                id:         "".to_string(),
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                uuid:       "".to_string(),
            });      
        },
    }
}

pub async fn get_user_data(req: HttpRequest, data: Json<IdUser>) -> Json<AuthResp2> {
    let result = find_user_with_id(data.id.clone());
    match result {
        Ok(_new_user) => {
            return Json(AuthResp2 { 
                id:         _new_user.id.clone(),
                first_name: _new_user.first_name.clone(),
                last_name:  _new_user.last_name.clone(),
                email:      _new_user.email.clone(),
                perm:       _new_user.perm,
                image:      _new_user.image.clone(),
                uuid:       _new_user.get_uuid(),
            });   
        },
        Err(err) => {
            return Json(AuthResp2 {
                id:         "".to_string(),
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                image:      None,
                uuid:       "".to_string(),
            });      
        },
    }
}

pub async fn process_signup(data: Json<NewUserJson>) -> Json<AuthResp2> {
        let _new_user = User::create(data);

        let _session_user = SessionUser {
            id:    _new_user.id.clone(),
            email: _new_user.email.clone(),
        };

        println!("Yes!");
        return Json(AuthResp2 {
            id:         _new_user.id.clone(),
            first_name: _new_user.first_name.clone(),
            last_name:  _new_user.last_name.clone(),
            email:      _new_user.email.clone(),
            perm:       _new_user.perm,
            image:      _new_user.image.clone(),
            uuid:       _new_user.get_uuid(),
        })
}