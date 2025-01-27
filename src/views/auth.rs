use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
    dev::ConnectionInfo,
    web::Json,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    is_signed_in,
    verify,
};
use futures::StreamExt;
use crate::models::{User, SessionUser};
use actix_session::Session;
use crate::errors::AuthError;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.route("/signup/", web::post().to(process_signup));
    config.route("/login/", web::post().to(login));
    config.route("/logout/", web::get().to(logout));
}

pub async fn logout(session: Session) -> Result<HttpResponse, AuthError> {
    session.clear();
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("ok"))
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
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthResp {
    pub id:         i32,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub perm:       i16,
    pub level:      i16,
    pub image:      Option<String>,
}

fn find_user(data: Json<LoginUser2>) -> Result<SessionUser, AuthError> {
    let user_some = User::get_user_with_email(&data.email); 
    if user_some.is_ok() {
        let _user = user_some.expect("Error.");
        if let Ok(matching) = verify(&_user.password, &data.password) {
            if matching {
                let f_user = SessionUser {
                    id:    _user.id,
                    email: _user.email,
                };
                return Ok(f_user.into());
            }
        }
    }
    Err(AuthError::NotFound(String::from("User not found")))
}

fn handle_sign_in (
    data: Json<LoginUser2>,
    session: &Session,
    req: &HttpRequest
) -> Result<HttpResponse, AuthError> {
    use crate::utils::{is_json_request, set_current_user};

    let result = find_user(data);
    let is_json = is_json_request(req);

    match result {
        Ok(user) => {
            set_current_user(&session, &user);
            if is_json {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        },
        Err(err) => {
            if is_json {
                Ok(HttpResponse::Unauthorized().json(err.to_string()))
            } else {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
            }
        },
    }
}


pub async fn login(req: HttpRequest, session: Session, data: Json<LoginUser2>) -> Json<AuthResp> {
    if is_signed_in(&session) {
        return Json(AuthResp {
            id:         0,
            first_name: "".to_string(),
            last_name:  "".to_string(),
            email:      "".to_string(),
            perm:       0,
            level:      0,
            image:      None,
        });
    }
    else {
        let user_some = User::get_user_with_email(&data.email); 
        if user_some.is_ok() {
            let _new_user = user_some.expect("E.");
            handle_sign_in(data, &session, &req);
            return Json(AuthResp {
                id:         _new_user.id,
                first_name: _new_user.first_name.clone(),
                last_name:  _new_user.last_name.clone(),
                email:      _new_user.email.clone(),
                perm:       _new_user.perm,
                level:      _new_user.level,
                image:      _new_user.image,
            });
        }
        else {
            return Json(AuthResp {
                id:         0,
                first_name: "".to_string(),
                last_name:  "".to_string(),
                email:      "".to_string(),
                perm:       0,
                level:      0,
                image:      None,
            });
        }
    }
}

pub async fn process_signup(session: Session, data: Json<NewUserJson>) -> Json<AuthResp> {
    if is_signed_in(&session) {
        return Json(AuthResp {
            id:         0,
            first_name: "".to_string(),
            last_name:  "".to_string(),
            email:      "".to_string(),
            perm:       0,
            level:      0,
            image:      None,
        });
    }
    else { 
        let _new_user = User::create(data);

        let _session_user = SessionUser {
            id:    _new_user.id,
            email: _new_user.email.clone(),
        };

        crate::utils::set_current_user(&session, &_session_user);
        return Json(AuthResp {
            id:         _new_user.id,
            first_name: _new_user.first_name.clone(),
            last_name:  _new_user.last_name.clone(),
            email:      _new_user.email.clone(),
            perm:       _new_user.perm,
            level:      _new_user.level,
            image:      _new_user.image,
        })
    }
}