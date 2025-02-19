use actix_web::{
    HttpRequest,
    web,
    web::Json,
};

use crate::models::{Order, Feedback, Log, Time};
use serde::{Deserialize, Serialize};

use crate::utils::{
    get_request_user, UserResp,
};
use crate::errors::Error;


pub fn other_routes(config: &mut web::ServiceConfig) {
    config.route("/search", web::get().to(empty_search_page));
    config.route("/search", web::get().to(search_page));
    config.route("/search_blogs", web::get().to(search_blogs_page));
    config.route("/search_services", web::get().to(search_services_page));
    config.route("/search_stores", web::get().to(search_stores_page));
    config.route("/search_wikis", web::get().to(search_wikis_page));
    config.route("/search_works", web::get().to(search_works_page));
    config.route("/search_help", web::get().to(search_helps_page));
}