pub mod auth;
//pub mod other_views;
pub mod users_views;
pub mod places_views;

pub use self::{
    auth::*,
    //other_views::*,
    users_views::*,
    places_views::*,
};
