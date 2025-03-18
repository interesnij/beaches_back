use crate::schema;
use crate::schema::{
    orders,
    feedbacks,
    logs,
    times,
};
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::errors::Error;
use actix_web::web::Json;
use crate::models::UserJson;


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="feedbacks"]
pub struct Feedback {
    pub id:       String,
    pub username: String,
    pub email:    String,
    pub message:  String,
}
#[derive(Deserialize)]
pub struct FeedbackJson {
    pub username: String,
    pub email:    String,
    pub message:  String,
}

impl Feedback {
    pub fn get_all() -> Json<Vec<Feedback>> {
        let _connection = establish_connection();
        return Json(schema::feedbacks::table
            .load::<Feedback>(&_connection)
            .expect("E"));
    }
    pub fn create(form: Json<FeedbackJson>) -> i16 {
        let _connection = establish_connection();
        let new_feedback = Feedback {
            id:       uuid::Uuid::new_v4().to_string(),
            username: form.username.clone(),
            email:    form.email.clone(),
            message:  form.message.clone()
        }; 
        let _new_feedback = diesel::insert_into(schema::feedbacks::table)
            .values(&new_feedback)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
}

#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="orders"]
pub struct Order {
    pub id:         String,
    pub title:      String,
    pub types:      i16, 
    pub place_id:   String,
    pub object_id:  String,
    pub created:    chrono::NaiveDateTime,
    pub user_id:    String,
    pub price:      i32,
    pub time_start: String,
    pub time_end:   String,
}
#[derive(Deserialize)]
pub struct OrderJson { 
    pub title:      String,
    pub place_id:   String,
    pub object_id:  String,
    pub price:      i32,
    pub time_start: String,
    pub time_end:   String, 
} 

impl Order {
    pub fn get_client(&self) -> UserJson {
        let _connection = establish_connection();
        return schema::users::table
            .filter(schema::users::id.eq(self.id.clone()))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::email,
                schema::users::level,
                schema::users::image,
            ))
            .first::<UserJson>(&_connection)
            .expect("E");
    }
    pub fn get_for_place(id: String) -> Json<Vec<Order>> {
        let _connection = establish_connection();
        return Json(schema::orders::table
            .filter(schema::orders::place_id.eq(id))
            .load::<Order>(&_connection)
            .expect("E"));
    } 
    pub fn create(user_id: String, form: Json<Vec<OrderJson>>) -> i16 {
        let _connection = establish_connection();

        for i in form.iter() {
            let time_start: String;
            let time_end: String;
            let format_start = chrono::NaiveDateTime::parse_from_str(&i.time_start, "%Y-%m-%d %H:%M:%S").unwrap();
            let format_end = chrono::NaiveDateTime::parse_from_str(&i.time_end, "%Y-%m-%d %H:%M:%S").unwrap();

            if schema::times::table
                .filter(schema::times::time.eq(format_start))
                .select(schema::times::id)
                .first::<String>(&_connection)
                .is_ok() {
                    time_start = i.time_start.clone();
            }
            else {
                let new = Time {
                    id:   uuid::Uuid::new_v4().to_string(),
                    time: format_start,
                }; 
                let _new_time = diesel::insert_into(schema::times::table)
                    .values(&new)
                    .execute(&_connection)
                    .expect("E.");
                time_start = i.time_start.clone();
            }

            if schema::times::table
                .filter(schema::times::time.eq(format_end))
                .select(schema::times::id)
                .first::<String>(&_connection)
                .is_ok() {
                    time_end = i.time_end.clone();
            }
            else {
                let new = Time {
                    id:   uuid::Uuid::new_v4().to_string(),
                    time: format_end,
                }; 
                let _new_time = diesel::insert_into(schema::times::table)
                    .values(&new)
                    .execute(&_connection)
                    .expect("E.");
                time_end = i.time_end.clone();
            }

            let new_order = Order {
                id:         uuid::Uuid::new_v4().to_string(),
                title:      i.title.clone(),
                types:      1,
                place_id:   i.place_id.clone(),
                object_id:  i.object_id.clone(),
                created:    chrono::Local::now().naive_utc(),
                user_id:    user_id.clone(),
                price:      i.price,
                time_start: time_start,
                time_end:   time_end, 
            }; 
            let _new_order = diesel::insert_into(schema::orders::table)
                .values(&new_order)
                .execute(&_connection)
                .expect("E.");
        }
        return 1;
    }


    pub fn delete(user_id: String, data: Json<crate::views::OrderIdsJson>) -> i16 {
        let _connection = establish_connection();
        diesel::delete (
            schema::orders::table
                .filter(schema::orders::user_id.eq(user_id))
                .filter(schema::orders::id.eq_any(data.ids.clone()))
        )
        .execute(&_connection)
        .expect("E");
        return 1;
    }

}

#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="logs"]
pub struct Log {
    pub id:       String,
    pub user_id:  String,
    pub text:     String,
    pub order_id: String,
    pub place_id: String,
    pub created:  chrono::NaiveDateTime,
}
#[derive(Deserialize)]
pub struct LogJson {
    pub user_id:  String,
    pub text:     String,
    pub order_id: String,
    pub place_id: String,
}

impl Log {
    pub fn get_all() -> Json<Vec<Log>> {
        let _connection = establish_connection();
        return Json(schema::logs::table
            .load::<Log>(&_connection)
            .expect("E"));
    }
    pub fn create(form: Json<LogJson>) -> i16 {
        let _connection = establish_connection();
        let new_log = Log {
            id:       uuid::Uuid::new_v4().to_string(),
            user_id:  form.user_id.clone(),
            text:     form.text.clone(),
            order_id: form.order_id.clone(),
            place_id: form.user_id.clone(),
            created:  chrono::Local::now().naive_utc(),
        }; 
        let _new_log = diesel::insert_into(schema::logs::table)
            .values(&new_log)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
}


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="times"]
pub struct Time {
    pub id:   String,
    pub time: chrono::NaiveDateTime,
}
#[derive(Deserialize)]
pub struct TimeJson {
    pub time: chrono::NaiveDateTime,
}

impl Time {
    pub fn get_all() -> Json<Vec<chrono::NaiveDateTime>> {
        let _connection = establish_connection();
        return Json(schema::times::table
            .order(schema::times::time.asc())
            .select(schema::times::time)
            .load::<chrono::NaiveDateTime>(&_connection)
            .expect("E"));
    }
    pub fn create(form: Json<TimeJson>) -> i16 {
        let _connection = establish_connection();
        if schema::times::table
            .filter(schema::times::time.eq(form.time.clone()))
            .select(schema::times::id)
            .first::<String>(&_connection)
            .is_ok() {
                return 0;
        }

        let new_time = Time {
            id:   uuid::Uuid::new_v4().to_string(),
            time: form.time.clone(),
        }; 
        let _new_time = diesel::insert_into(schema::times::table)
            .values(&new_time)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
}