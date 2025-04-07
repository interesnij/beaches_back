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
    Region, Citie,
};
use serde::{Deserialize, Serialize};
use actix_multipart::{Field, Multipart};

use crate::utils::{
    is_signed_in,
    get_current_user,
    establish_connection,
    files_form,
}; 
use crate::views::{AuthResp2, AuthResp, ItemId};
use crate::schema;
use std::borrow::BorrowMut; 


pub fn places_routes(config: &mut web::ServiceConfig) {
    config.route("/places/{type_id}/", web::get().to(get_places));
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

    config.route("/regions/", web::post().to(regions));
    config.route("/create_region/", web::post().to(create_region));
    config.route("/edit_region/{id}/", web::post().to(edit_region));
    config.route("/delete_region/{id}/", web::post().to(delete_region));
    config.route("/create_city/", web::post().to(create_city));
    config.route("/edit_city/{id}/", web::post().to(edit_city));
    config.route("/delete_city/{id}/", web::post().to(delete_city));
} 
 
pub async fn get_places(type_id: web::Path<i16>) -> Json<Vec<Place>> {
    return Place::get_all(type_id);
}

pub async fn regions() -> Json<Vec<Region>> {
    return Json(Region::get_all());
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaceDataJson { 
    pub modules: Vec<Module>,
    pub orders:  Vec<RespOrderJson>,
    pub place:   Place,
}

pub async fn get_place(req: HttpRequest, id: web::Path<String>) -> Json<PlaceDataJson> {
    let place = Place::get_place(id.clone());
    return Json(PlaceDataJson {
        modules: place.get_modules(),
        orders:  place.get_orders(),
        place:   place, 
    });
}

pub async fn get_place_managers(req: HttpRequest, id: web::Path<String>) -> Json<Vec<UserJson>> {
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
            return Json(_place.get_orders());
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
        Place::create(
            data.title.clone(),
            data.user_id.clone(),
            data.type_id,
            data.cord.clone(),
        ); 
    }
    HttpResponse::Ok()
}
pub async fn edit_place(req: HttpRequest, data: Json<PlaceJson>, id: web::Path<String>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        Place::edit(
            id.to_string(),
            data.title.clone(),
            data.type_id,
            data.cord.clone(),
        ); 
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

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateRegionJson { 
    pub name: String,
    pub cord: Option<String>,
}

pub async fn create_region(req: HttpRequest, data: Json<CreateRegionJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            Region::create(data.name.clone(), data.cord.clone());
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_region(req: HttpRequest, data: Json<CreateRegionJson>, id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            Region::create(id, data.name.clone(), data.cord.clone());
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_region(req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            Region::delete(*id);
        }
    }
    HttpResponse::Ok()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCityJson {
    pub region_id: Option<i32>,
    pub name:      String,
    pub cord:      Option<String>,
}

pub async fn create_city(req: HttpRequest, data: Json<CreateCityJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            Citie::create(data.region_id, data.name.clone(), data.cord.clone());
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_city(req: HttpRequest, data: Json<CreateCityJson>, id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            Citie::edit(id, data.region_id, data.name.clone(), data.cord.clone());
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_city(req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            Citie::delete(*id);
        }
    }
    HttpResponse::Ok()
}