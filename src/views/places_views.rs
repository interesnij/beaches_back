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
    Region, NewRegion, Citie, NewCitie,
    ModuleType, Event,

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
    config.route("/place/{id}/events/", web::get().to(get_place_events));
    config.route("/place/{id}/module_types/", web::get().to(get_place_module_types));
    config.route("/suggest_places/", web::get().to(get_suggest_places));
    config.route("/closed_places/", web::get().to(get_closed_places));

    config.route("/create_place/", web::post().to(create_place));
    config.route("/edit_place/{id}/", web::post().to(edit_place)); 
    config.route("/create_modules/", web::post().to(create_modules)); 
    //config.route("/close_place/{id}/", web::post().to(close_place));
    //config.route("/hide_place/{id}/", web::post().to(hide_place));
    //config.route("/publish_place/{id}/", web::post().to(publish_place));

    config.route("/create_module_type/", web::post().to(create_module_type));
    config.route("/edit_module_type/{id}/", web::post().to(edit_module_type));
    config.route("/create_event/", web::post().to(create_event));
    config.route("/edit_event/{id}/", web::post().to(edit_event));

    config.route("/regions/", web::get().to(regions));
    config.route("/cities/", web::get().to(cities)); 
    config.route("/region/{id}/", web::get().to(get_region));
    config.route("/create_region/", web::post().to(create_region));
    config.route("/edit_region/{id}/", web::post().to(edit_region));
    config.route("/delete_region/{id}/", web::post().to(delete_region));
    config.route("/city/{id}/", web::get().to(get_city));
    config.route("/module_type/{id}/", web::get().to(get_module_type));
    config.route("/event/{id}/", web::get().to(get_event)); 
    config.route("/city/{id}/", web::get().to(get_city));
    config.route("/create_city/", web::post().to(create_city));
    config.route("/edit_city/{id}/", web::post().to(edit_city));
    config.route("/delete_city/{id}/", web::post().to(delete_city));

    config.route("/delete_module_type/{id}/", web::post().to(delete_module_type));
    config.route("/delete_event/{id}/", web::post().to(delete_event));
}
 
pub async fn get_places(type_id: web::Path<i16>) -> Json<Vec<Place>> {
    return Place::get_all(*type_id);
}

pub async fn regions() -> Json<Vec<Region>> {
    return Json(Region::get_all());
}
pub async fn cities() -> Json<Vec<Citie>> {
    return Json(Citie::get_all());
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
pub async fn get_region(req: HttpRequest, id: web::Path<i32>) -> Json<Region> {
    let _region = Region::get(*id);
    return Json(_region);
}
pub async fn get_city(req: HttpRequest, id: web::Path<i32>) -> Json<Citie> {
    let _city = Citie::get(*id);
    return Json(_city);
}
pub async fn get_module_type(req: HttpRequest, id: web::Path<String>) -> Json<ModuleType> {
    let _module_type = ModuleType::get(id.clone());
    return Json(_module_type);
}
pub async fn get_event(req: HttpRequest, id: web::Path<String>) -> Json<Event> {
    let _event = Event::get(id.clone());
    return Json(_event);
}

pub async fn get_place_managers(req: HttpRequest, id: web::Path<String>) -> Json<Vec<UserJson>> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        let _place = Place::get(id.clone());
        if _request_user.is_can_work_in_object_with_id(&id) {
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
        if _request_user.is_can_work_in_object_with_id(&id) {
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
pub async fn get_place_module_types(req: HttpRequest, id: web::Path<String>) -> Json<Vec<crate::models::ModuleType>> {
    if is_signed_in(&req) { 
        let _request_user = get_current_user(&req);
        if _request_user.is_can_work_in_object_with_id(&id) {
            return crate::models::ModuleType::get_all_for_place(id.to_string());
        }
        else {
            return Json(Vec::new());
        }
    }
    else {
        Json(Vec::new())
    }
}
pub async fn get_place_events(req: HttpRequest, id: web::Path<String>) -> Json<Vec<crate::models::Event>> {
    if is_signed_in(&req) { 
        let _request_user = get_current_user(&req);
        if _request_user.is_can_work_in_object_with_id(&id) {
            return crate::models::Event::get_all_for_place(id.to_string());
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
            data.city_id.clone(),
            data.type_id.clone(),
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
            data.type_id.clone(),
            data.cord.clone(),
        ); 
    }
    HttpResponse::Ok()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateModuleType {
    pub place_id:    String,
    pub title:       String,
    pub description: String,
    pub types:       String,
    pub price:       String,
} 
pub async fn create_module_type(req: HttpRequest, data: Json<CreateModuleType>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        let uuid = crate::models::ModuleType::create(
            data.place_id.clone(),
            data.title.clone(),
            data.description.clone(),
            data.types.clone(),
            data.price,
        ); 
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(uuid));
    }
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("error!"))
}
#[derive(Deserialize, Serialize, Debug)]
pub struct EditModuleType {
    pub title:       String,
    pub description: String,
    pub types:       String,
    pub price:       String,
} 
pub async fn edit_module_type(req: HttpRequest, data: Json<EditModuleType>, id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        let uuid = crate::models::ModuleType::edit(
            id.to_string(),
            data.title.clone(),
            data.description.clone(),
            data.types.clone(),
            data.price,
        ); 
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(uuid));
    }
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("error!"))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateEvent {
    pub place_id:    String,
    pub title:       String,
    pub description: String,
    pub price:       String,
    pub time_start:  String,
    pub time_end:    String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct EditEvent {
    pub title:       String,
    pub description: String,
    pub price:       String,
    pub time_start:  String,
    pub time_end:    String,
} 
pub async fn create_event(req: HttpRequest, data: Json<CreateEvent>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        let uuid = crate::models::Event::create(
            _request_user.id,
            data.place_id.clone(),
            data.title.clone(), 
            data.description.clone(),
            data.price.clone(),
            data.time_start.clone(),
            data.time_end.clone(),
        ); 
        println!("uuid: {:?}", uuid);
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(uuid));
    }
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("error!"))
}
pub async fn edit_event(req: HttpRequest, data: Json<EditEvent>, id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        let uuid = crate::models::Event::edit(
            id.to_string(), 
            data.title.clone(),
            data.description.clone(),
            data.price.clone(),
            data.time_start.clone(),
            data.time_end.clone(),
        );
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(uuid));
    }
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("error!"))
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
            Region::edit(*id, data.name.clone(), data.cord.clone());
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
    pub region_id: Option<String>,
    pub name:      String,
    pub cord:      Option<String>,
}

pub async fn create_city(req: HttpRequest, data: Json<CreateCityJson>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            Citie::create(data.region_id.clone(), data.name.clone(), data.cord.clone());
        }
    }
    HttpResponse::Ok()
}

pub async fn edit_city(req: HttpRequest, data: Json<CreateCityJson>, id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        if _request_user.is_superuser() {
            Citie::edit(*id, data.region_id.clone(), data.name.clone(), data.cord.clone());
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

pub async fn delete_module_type(req: HttpRequest, id: web::Path<String>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        let place_id = id.clone();
        if _request_user.is_can_work_in_object_with_id(&place_id) {
            ModuleType::delete(place_id);
        }
    }
    HttpResponse::Ok()
}
pub async fn delete_event(req: HttpRequest, id: web::Path<String>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        let place_id = id.clone();
        if _request_user.is_can_work_in_object_with_id(&place_id) {
            crate::models::Event::delete(place_id);
        }
    }
    HttpResponse::Ok()
}