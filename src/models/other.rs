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
use crate::models::Time;


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
            id:       uuid::Uuid::new_v4(),
            username: form.username.clone(),
            email:    form.email.clone(),
            message:  form.message
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
    pub user_id:    String,
    pub price:      i32,
    pub time_start: String,
    pub time_end:   String,
}

impl Order {
    pub fn get_for_place(id: String) -> Json<Vec<Order>> {
        let _connection = establish_connection();
        return Json(schema::orders::table
            .filter(schema::orders::place_id.eq(id))
            .load::<Feedback>(&_connection)
            .expect("E"));
    }
    pub fn create(form: Json<OrderJson>) -> i16 {
        let _connection = establish_connection();

        let time_start: String;
        let time_end: String;

        if crate::times::table
            .filter(schema::times::time.eq(form.time_start.clone()))
            .select(schema::times::id)
            .first::<i32>(&_connection)
            .is_ok() {
                time_start = crate::times::table
                    .filter(schema::times::time.eq(form.time_start.clone()))
                    .select(schema::times::time)
                    .first::<String>(&_connection)
                    .expect();
        }
        else {
            let new = Time {
                id:   uuid::Uuid::new_v4(),
                time: form.time_start.clone(),
            }; 
            let _new_time = diesel::insert_into(schema::times::table)
                .values(&new)
                .execute(&_connection)
                .expect("E.");
            time_start = form.time_start.clone();
        }

        if crate::times::table
            .filter(schema::times::time.eq(form.time_end.clone()))
            .select(schema::times::id)
            .first::<i32>(&_connection)
            .is_ok() {
                time_end = crate::times::table
                    .filter(schema::times::time.eq(form.time_end.clone()))
                    .select(schema::times::time)
                    .first::<String>(&_connection)
                    .expect();
        }
        else {
            let new = Time {
                id:   uuid::Uuid::new_v4(),
                time: form.time_end.clone(),
            }; 
            let _new_time = diesel::insert_into(schema::times::table)
                .values(&new)
                .execute(&_connection)
                .expect("E.");
            time_start = form.time_end.clone();
        }

        let new_order = Order {
            id:         uuid::Uuid::new_v4(),
            title:      form.title.clone(),
            types:      1,
            place_id:   form.place_id.clone(),
            object_id:  form.object_id.clone(),
            created:    chrono::Local::now().naive_utc(),
            user_id:    form.user_id.clone(),
            price:      form.price,
            time_start: time_start,
            time_end:   time_end,
        }; 
        let _new_order = diesel::insert_into(schema::orders::table)
            .values(&new_order)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }

    pub fn delete(id: String) -> i16 {
        diesel::delete (
            orders
                .filter(schema::orders::id.eq(id))
        )
        .execute(&_connection)
        .expect("E");
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
            id:       uuid::Uuid::new_v4(),
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
    pub time: String,
}

impl Time {
    pub fn get_all() -> Json<Vec<String>> {
        let _connection = establish_connection();
        return Json(schema::times::table
            .order(schema::times::time.asc())
            .select(schema::times::time)
            .load::<String>(&_connection)
            .expect("E"));
    }
    pub fn create(form: Json<TimeJson>) -> i16 {
        let _connection = establish_connection();
        if crate::times::table
            .filter(schema::times::time.eq(form.time.clone()))
            .select(schema::times::id)
            .first::<i32>(&_connection)
            .is_ok() {
                return 0;
        }

        let new_time = Time {
            id:   uuid::Uuid::new_v4(),
            time: form.time.clone(),
        }; 
        let _new_time = diesel::insert_into(schema::times::table)
            .values(&new_time)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
}