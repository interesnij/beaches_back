use actix_web::web;
use crate::views::{
    auth,
    //other_views,
    users_views,
    places_views,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(auth::auth_routes)
    //.configure(other_views::other_routes)
    .configure(users_views::users_routes)
    .configure(places_views::places_routes)
    ;
}
