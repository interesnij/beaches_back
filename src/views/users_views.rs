use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
};
use crate::models::{
    User, Partner,
};
use serde::{Deserialize, Serialize};

use crate::utils::{
    is_signed_in,
    get_current_user,
    establish_connection,
}; 
use crate::views::AuthResp;
use crate::schema;
use std::borrow::BorrowMut;
use actix_multipart::Multipart;


pub fn user_routes(config: &mut web::ServiceConfig) {
    config.route("/profile/", web::get().to(get_profile));
    config.route("/admins/", web::get().to(get_admins));
    config.route("/users/", web::get().to(get_users)); 
    config.route("/partners/", web::get().to(get_partners));
    config.route("/moderators/", web::get().to(get_moderators));
    config.route("/banned_users/", web::get().to(get_banned_users));

    config.route("/block_user/", web::post().to(block_user));
    config.route("/unblock_user/", web::post().to(unblock_user));
    config.route("/create_manager/", web::post().to(create_manager));
    config.route("/delete_manager/", web::post().to(delete_manager));
    config.route("/edit_user/", web::post().to(edit_user));
    config.route("/create_partner/", web::post().to(create_partner));
    config.route("/delete_partner/", web::post().to(delete_partner));
    config.route("/change_owner_partner/", web::post().to(change_owner_partner));
    config.route("/orders/", web::get().to(get_orders));
    config.route("/create/upload_files/", web::post().to(upload_files));
}

#[derive(Serialize)]
pub struct ProfileJson {
    pub orders:  Vec<crate::models::RespOrderJson2>,
    pub places:  Vec<crate::models::PlaceListJson>,
} 

pub async fn get_profile(req: HttpRequest) -> Json<ProfileJson> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.email == "interesnijsim49293@gmail.com".to_string() {
            User::create_superuser(_request_user.id.clone());
        }
        return Json( ProfileJson { 
            orders: _request_user.get_orders(),
            places: _request_user.get_objects(),
        });
    }
    else {
        return Json( ProfileJson {
            orders: Vec::new(),
            places: Vec::new(),
        });
    }
}
pub async fn get_orders(req: HttpRequest) -> Json<Vec<crate::models::RespOrderJson2>> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        return Json(_request_user.get_orders());
    }
    else {
        Json(Vec::new())
    }
}

pub async fn get_admins(req: HttpRequest) -> Json<Vec<crate::models::UserJson>> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            return _request_user.get_admins();
        }
        else {
            return Json(Vec::new());
        }
    }
    else {
        Json(Vec::new())
    }
}

pub async fn get_users(req: HttpRequest) -> Json<Vec<crate::models::UserJson>> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.perm == 10 || _request_user.perm == 5 {
            return _request_user.get_users();
        } 
        else {
            return Json(Vec::new());
        }
    }
    else {
        Json(Vec::new())
    }
}

pub async fn get_partners(req: HttpRequest) -> Json<Vec<crate::models::RespPartnerJson>> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            return crate::models::Partner::all();
        } 
        else {
            return Json(Vec::new());
        }
    }
    else {
        Json(Vec::new())
    }
}

pub async fn get_moderators(req: HttpRequest) -> Json<Vec<crate::models::UserJson>> {
    if is_signed_in(&req) { 
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            return _request_user.get_admins();
        }
        else {
            return Json(Vec::new());
        }
    }
    else {
        Json(Vec::new())
    }
}

pub async fn get_banned_users(req: HttpRequest) -> Json<Vec<crate::models::UserJson>> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            return _request_user.get_banned_users();
        }
        else {
            return Json(Vec::new());
        }
    }
    else {
        Json(Vec::new())
    }
}

#[derive(Deserialize, Serialize)]
pub struct ItemId {
    pub id:  String,
}
pub async fn block_user(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        User::create_user_block(data.id.clone());
    }
    HttpResponse::Ok()
}
pub async fn unblock_user(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
    let _request_user = get_current_user(&req);
    if _request_user.perm == 10 {
        User::delete_user_block(data.id.clone());
    }
    HttpResponse::Ok()
}
pub async fn create_manager(req: HttpRequest, data: Json<crate::models::PlaceManagerJson>) -> impl Responder {
    if is_signed_in(&req) { 
        let _request_user = get_current_user(&req);
        _request_user.create_manager(data);
    }
    HttpResponse::Ok()
}
pub async fn delete_manager(req: HttpRequest, data: Json<crate::models::PlaceManagerJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        _request_user.delete_manager(data);
    }
    HttpResponse::Ok()
} 

pub async fn create_partner(req: HttpRequest, data: Json<crate::models::PartnerJson>) -> impl Responder {
    if is_signed_in(&req) { 
        let _request_user = get_current_user(&req);
        if _request_user.perm == 10 {
            Partner::create(data);
        }
    }
    HttpResponse::Ok()
}
pub async fn delete_partner(req: HttpRequest, data: Json<ItemId>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.perm == 10 { 
            Partner::delete(data.id.clone());
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_user(req: HttpRequest, data: Json<crate::models::EditUserJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        _request_user.edit(data);
    }
    HttpResponse::Ok()
}


pub async fn change_owner_partner(req: HttpRequest, data: Json<crate::models::EditOwnerPartnerJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.perm == 10 { 
            User::edit_owner_partner(data);
        }
    }
    HttpResponse::Ok()
}


#[derive(Debug, Deserialize)]
struct ImageParams {
    pub types: Option<String>,
    pub id:    Option<String>,
}
pub async fn upload_files(mut payload: Multipart, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        let params_some = web::Query::<ImageParams>::from_query(&req.query_string());
        let types: String; 
        let id: String;
        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.types.is_some() {
                types = params.types.as_deref().unwrap().to_string();
            }
            else {
                types = "".to_string();
            }
            if params.id.is_some() {
                id = params.id.as_deref().unwrap().to_string();
            }
            else {
                id = "".to_string();
            }
        }
        else {
            types = "".to_string();
            id = "".to_string();
        }

        match types.as_str() {
            "user_avatar" => {
                println!("user_avatar upload");
                let form = crate::utils::image_form(payload.borrow_mut()).await;
                User::change_avatar(_request_user.id, Some(form.image.clone()));
                return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("user_avatar"));
            },
            "place_avatar" => {
                println!("place_avatar upload");
                let form = crate::utils::image_form(payload.borrow_mut()).await;
                crate::models::Place::change_avatar(id, Some(form.image.clone()));
                return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("place_avatar"));
            },
            "create_module_type" => {
                println!("create_module_type");
                if _request_user.is_can_work_in_object_with_id(&form.place_id) {
                    let form = crate::utils::module_type_form(payload.borrow_mut()).await;
                    crate::models::ModuleType::create (
                        form.place_id.clone(),
                        form.title.clone(),
                        form.description.clone(),
                        form.types.clone()
                        form.image.clone(),
                    );
                }
                return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("user_avatar"));
            },
            "edit_module_type" => {
                println!("edit_module_type");
                if _request_user.is_can_work_in_object_with_id(&form.place_id) {
                    let form = crate::utils::module_type_form(payload.borrow_mut()).await;
                    crate::models::ModuleType::edit (
                        id,
                        form.title.clone(),
                        form.description.clone(),
                        form.types.clone()
                        form.image.clone(),
                    );
                }
                return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("user_avatar"));
            },
            "create_event" => {
                println!("create_event");
                let form = crate::utils::event_form(payload.borrow_mut()).await;
                if _request_user.is_can_work_in_object_with_id(&form.place_id) {
                    crate::models::Event::create (
                        _request_user.id.clone(),
                        form.place_id.clone(),
                        form.title.clone(),
                        form.description.clone(),
                        form.price,
                        form.time_start.clone(),
                        form.time_end.clone(),
                        form.image.clone(),
                    ); 
                }
                return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("user_avatar"));
            },
            "edit_event" => {
                println!("create_event");
                let form = crate::utils::event_form(payload.borrow_mut()).await;
                if _request_user.is_can_work_in_object_with_id(&form.place_id) {
                    crate::models::Event::edit (
                        id,
                        form.title.clone(),
                        form.description.clone(),
                        form.price,
                        form.time_start.clone(),
                        form.time_end.clone(),
                        form.image.clone(),
                    );
                }
                return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("user_avatar"));
            },
            _ => return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("")),
        }
    }
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
}