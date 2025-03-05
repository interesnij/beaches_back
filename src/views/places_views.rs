use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
};
use crate::models::{
    User, Place, PlaceJson, UserJson, ModuleJson, 
    RespOrderJson, CreateModuleJson, Module,
};
use serde::{Deserialize, Serialize};

use crate::utils::{
    is_signed_in,
    get_current_user,
    establish_connection,
}; 
use crate::views::{AuthResp2, AuthResp, ItemId};
use crate::schema;


pub fn places_routes(config: &mut web::ServiceConfig) {
    config.route("/places/", web::get().to(get_places));
    config.route("/place/{id}/", web::get().to(get_place));
    config.route("/place/{id}/managers/", web::get().to(get_place_managers));
    config.route("/place/{id}/orders/", web::get().to(get_place_orders));
    config.route("/suggest_places/", web::get().to(get_suggest_places));
    config.route("/closed_places/", web::get().to(get_closed_places));

    config.route("/create_place/", web::post().to(create_place));
    config.route("/edit_place/{id}/", web::post().to(edit_place));
    config.route("/create_modules/", web::post().to(create_modules));
    //config.route("/close_place/{id}/", web::post().to(close_place));
    //config.route("/hide_place/{id}/", web::post().to(hide_place));
    //config.route("/publish_place/{id}/", web::post().to(publish_place));
}  

pub async fn get_places(req: HttpRequest) -> Json<Vec<Place>> {
    return Place::get_all();
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaceDataJson { 
    pub modules: Vec<ModuleJson>,
    pub orders:  Vec<RespOrderJson>,
    pub place:   PlaceJson,
}

pub async fn get_place(req: HttpRequest, id: web::Path<String>) -> Json<Place> {
    let place = Place::get_place(id.clone());
    return PlaceDataJson {
        modules: place.get_modules();
        orders:  place.get_orders();
        place:   place;
    };
}

pub async fn get_place_managers(req: HttpRequest, id: web::Path<String>) -> Json<Vec<PlaceDataJson>> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        let _place = Place::get(id.clone());
        if _place.user_id.clone() == _request_user.id {
            return _place.get_managers();
        } 
        else {
            return Json(Vec::new());
        }
    }
    else {
        Json(Vec::new())
    }
}
pub async fn get_place_orders(req: HttpRequest, id: web::Path<String>) -> Json<Vec<crate::models::RespOrderJson>> {
    if is_signed_in(&req) { 
        let _request_user = get_current_user(&req);
        let _place = Place::get_place(id.clone());
        if _place.user_id.clone() == _request_user.id {
            return Place::get_orders(_place.id);
        }
        else {
            return Json(Vec::new());
        }
    }
    else {
        Json(Vec::new())
    }
}

pub async fn get_suggest_places(req: HttpRequest) -> Json<Vec<Place>> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            return Place::get_suggest();
        }
        else {
            return Json(Vec::new());
        }
    }
    else {
        Json(Vec::new())
    }
}
pub async fn get_closed_places(req: HttpRequest) -> Json<Vec<Place>> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            return Place::get_closed();
        }
        else {
            return Json(Vec::new());
        }
    }
    else {
        Json(Vec::new())
    }
}

pub async fn create_place(req: HttpRequest, data: Json<PlaceJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);

        Place::create(data); 
    }
    HttpResponse::Ok()
}
pub async fn edit_place(req: HttpRequest, data: Json<PlaceJson>, id: web::Path<String>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.perm == 4 {

        }
        Place::edit(id.to_string(), data);
    }
    HttpResponse::Ok()
}

pub async fn create_modules(req: HttpRequest, data: Json<CreateModuleJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        Module::create(data); 
    }
    HttpResponse::Ok()
}
