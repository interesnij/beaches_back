use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
};
use serde::{Deserialize, Serialize};
use crate::models::{Order, OrderJson};

use crate::utils::{
    is_signed_in,
    get_current_user,
    establish_connection,
};


pub fn other_routes(config: &mut web::ServiceConfig) {
    config.route("/create_order/", web::get().to(create_order));
    config.route("/delete_order/", web::get().to(delete_order));
} 

pub async fn create_order(req: HttpRequest, data: Json<Vec<OrderJson>>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        Order::create(
            _request_user.id.clone(),
            data
        ); 
    }
    HttpResponse::Ok()
}

#[derive(Deserialize)]
pub struct OrderIdsJson { 
    pub ids: Vec<String>,
}
pub async fn delete_order(req: HttpRequest, data: Json<Vec<OrderIdsJson>>) -> impl Responder {
    if is_signed_in(&req) {
        let _request_user = get_current_user(&req);
        Order::delete(
            _request_user.id.clone(),
            data
        ); 
    }
    HttpResponse::Ok()
}