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
    config.route("/admins/", web::get().to(get_admins));
    config.route("/users/", web::get().to(get_users)); 
    config.route("/partners/", web::get().to(get_partners));
    config.route("/moderators/", web::get().to(get_moderators));
    config.route("/banned_users/", web::get().to(get_banned_users));

    config.route("/block_user/", web::post().to(block_user));
    config.route("/unblock_user/", web::post().to(unblock_user));
    config.route("/create_manager/", web::post().to(create_manager));
    config.route("/delete_manager/", web::post().to(delete_manager));
    //config.route("/edit/", web::post().to(edit_user));
    config.route("/create_partner/", web::post().to(create_partner));
    config.route("/delete_partner/", web::post().to(delete_partner));
    config.route("/change_owner_partner/", web::post().to(change_owner_partner));
    config.route("/orders/", web::get().to(get_orders));
    config.route("/change_avatar/", web::post().to(change_avatar));
}


pub async fn get_orders(req: HttpRequest) -> Json<Vec<crate::models::RespOrderJson2>> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        return _request_user.get_orders();
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

//pub async fn edit_user(req: HttpRequest, data: Json<EditUserJson>) -> impl Responder {
//    if is_signed_in(&req) {
//        let _request_user = get_current_user(&req);
//        User::edit(data);
//    }
//    HttpResponse::Ok()
//}


pub async fn change_owner_partner(req: HttpRequest, data: Json<crate::models::EditOwnerPartnerJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.perm == 10 { 
            User::edit_owner_partner(data);
        }
    }
    HttpResponse::Ok()
}


pub async fn change_avatar(mut payload: Multipart, req: HttpRequest) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        let form = crate::utils::image_form(payload.borrow_mut()).await;
        User::change_avatar(_request_user.id, Some(form.image.clone()));
    } 
    HttpResponse::Ok()
}