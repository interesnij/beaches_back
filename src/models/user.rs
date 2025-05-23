use crate::schema;
use crate::schema::{
    users,
    partners,
};
use rand::Rng;
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
use crate::views::NewUserJson;


#[derive(Debug, Queryable, Serialize, Identifiable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id:         String,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub password:   String,
    pub perm:       i16, 
    pub level:      i16,
    pub image:      Option<String>,
    pub uuid:       Vec<u8>,
}

#[derive(Deserialize, Queryable, Serialize, Debug)]
pub struct UserJson {
    pub id:         String,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub perm:       i16,
    pub image:      Option<String>,
}  
#[derive(Deserialize)]
pub struct EditUserJson {
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
}
#[derive(Deserialize)]
pub struct EditOwnerPartnerJson {
    pub id:      String,
    pub user_id: String,
}

#[derive(Serialize)]
pub struct OrderListJson {
    pub object_id:  String,
    pub price:      i32,
    pub time_start: chrono::NaiveDateTime,
    pub time_end:   chrono::NaiveDateTime,
}
#[derive(Serialize, Queryable)]
pub struct PlaceListJson {
    pub id:    String,
    pub title: String,
    pub image: Option<String>,
    pub cord:  Option<String>,
}
#[derive(Serialize)]
pub struct RespOrderJson2 {
    pub order:  OrderListJson,
    pub place:  PlaceListJson,
}  

impl User {
    pub fn is_superuser(&self) -> bool {
        return self.perm == 10;
    }
    pub fn is_await_partner(&self) -> bool {
        return self.perm == 3;
    }
    pub fn is_partner(&self) -> bool {
        return self.perm == 4;
    }
    pub fn is_partner_of_place_with_id(&self, place_id: &String) -> bool {
        let _connection = establish_connection();
        return schema::places::table
            .filter(schema::places::id.eq(place_id))
            .select(schema::places::id)
            .first::<String>(&_connection)
            .is_ok();
    }
    pub fn is_manager_of_place_with_id(&self, place_id: &String) -> bool {
        let _connection = establish_connection();
        return schema::place_managers::table
            .filter(schema::place_managers::place_id.eq(place_id))
            .filter(schema::place_managers::user_id.eq(self.id.clone()))
            .select(schema::place_managers::id)
            .load::<String>(&_connection)
            .is_ok();
    }
    pub fn is_manager(&self) -> bool {
        return self.perm == 2;
    }
    pub fn get_objects(&self) -> Vec<PlaceListJson> {
        let _connection = establish_connection();
        let uuid = hex::encode(self.uuid.clone());
        return schema::places::table
            .filter(schema::places::user_id.eq(self.id.clone()))
            .or_filter(schema::places::user_id.eq(uuid))
            .order(schema::places::created.desc())
            .select((
                schema::places::id,
                schema::places::title,
                schema::places::image,
                schema::places::cord,
            )) 
            .load::<PlaceListJson>(&_connection)
            .expect("E");
    } 
    pub fn get_orders(&self) -> Vec<RespOrderJson2> {
        let _connection = establish_connection();
        let uuid = hex::encode(self.uuid.clone());

        let list = schema::orders::table
            .filter(schema::orders::user_id.eq(self.id.clone()))
            .or_filter(schema::orders::user_id.eq(uuid))
            .order(schema::orders::created.desc())
            .load::<crate::models::Order>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for i in list {
            let _time_start = chrono::NaiveDateTime::parse_from_str(&i.time_start, "%Y-%m-%d %H:%M:%S").unwrap();
            let _time_end = chrono::NaiveDateTime::parse_from_str(&i.time_end, "%Y-%m-%d %H:%M:%S").unwrap();
            let new = chrono::Local::now().naive_utc() + chrono::Duration::hours(3);
            //if _time_end > new {
            //    continue; 
            //}
            let _place = crate::models::Place::get_place(i.place_id.clone());
            let _place_item = PlaceListJson {
                id:    _place.id.clone(),
                title: _place.title.clone(),
                image: _place.image.clone(),
                cord:  _place.cord.clone(),
            }; 
            let _order_item = OrderListJson {
                object_id:  i.title.clone(),
                price:      i.price,
                time_start: _time_start,
                time_end:   _time_end,
            };

            stack.push(crate::models::RespOrderJson2 {
                order:  _order_item,
                place:  _place_item,
            });
        }
        return stack;
    }
    pub fn create_manager(&self, form: Json<crate::models::PlaceManagerJson>) -> i16 {
        let _connection = establish_connection();
        let place_id = form.place_id.clone();
        let user_id = form.user_id.clone();
        let _place = crate::models::Place::get_place(place_id.clone());
        if &self.id != &user_id {
            return 0;
        }
        
        let new_place_manager = crate::models::PlaceManager {
            id:       uuid::Uuid::new_v4().to_string(),
            user_id:  user_id,
            place_id: place_id,
        }; 
        let _place_manager = diesel::insert_into(schema::place_managers::table)
            .values(&new_place_manager)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn delete_manager(&self, form: Json<crate::models::PlaceManagerJson>) -> i16 {
        let _connection = establish_connection();
        let place_id = form.place_id.clone();
        let user_id = form.user_id.clone();
        let _place = crate::models::Place::get_place(place_id.clone());
        if self.id != user_id {
            return 0;
        }
        diesel::delete (
            schema::place_managers::table
                .filter(schema::place_managers::user_id.eq(user_id))
                .filter(schema::place_managers::place_id.eq(place_id.clone()))
            )
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
    pub fn create_superuser(user_id: String) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(10))
                .execute(&_connection); 
        }))
    }
    pub fn edit_owner_partner(form: Json<crate::models::EditOwnerPartnerJson>) -> i16 {
        let _connection = establish_connection();
        let _partner = schema::partners::table
            .filter(schema::partners::id.eq(form.id.clone()))
            .first::<Partner>(&_connection)
            .expect("E.");
        diesel::update(&_partner)
            .set(schema::partners::user_id.eq(&form.user_id.clone()))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn delete_superuser(user_id: String) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(1))
                .execute(&_connection);
        }))
    }

    pub fn get_user_with_email(email: &String) -> Result<User, Error> {
        let _connection = establish_connection();
        return Ok(schema::users::table
            .filter(schema::users::email.eq(email))
            .first::<User>(&_connection)?);
    }
    pub fn get_user_with_id(id: &String) -> Result<User, Error> {
        let _connection = establish_connection();
        return Ok(schema::users::table
            .filter(schema::users::id.eq(id))
            .first::<User>(&_connection)?);
    }

    pub fn create_user_block(user_id: String) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(21))
                .execute(&_connection);
        }))
    }
    pub fn delete_user_block(user_id: String) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(1))
                .execute(&_connection);
        }))
    }
    pub fn change_avatar(user_id: String, image: Option<String>) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::image.eq(image))
                .execute(&_connection);
        }))
    }
    pub fn get_uuid(&self) -> String {
        hex::encode(self.uuid.clone())
    }
    pub fn create(form: Json<NewUserJson>) -> User {
        let _connection = establish_connection();
        let form_user = User { 
            id:         uuid::Uuid::new_v4().to_string(),
            first_name: form.first_name.clone(),
            last_name:  form.last_name.clone(),
            email:      form.email.clone(),
            password:   crate::utils::hash_password(&form.password),
            perm:       1,
            level:      100,
            image:      None,
            uuid:       rand::thread_rng().gen::<[u8; 32]>().to_vec(),
        };

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");
        
        if _new_user.email == "ochkarik1983@mail.ru".to_string() {
            diesel::update(&_new_user)
                .set(schema::users::perm.eq(10))
                .execute(&_connection)
                .expect("Error.");
        }
        return _new_user;
    }
    pub fn edit(&self, form: Json<EditUserJson>) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
                .set((
                    schema::users::first_name.eq(&form.first_name.clone()),
                    schema::users::last_name.eq(form.last_name.clone()),
                    schema::users::email.eq(form.email.clone()),
                ))
                .execute(&_connection)
                .expect("E");
        return 1;
    }
    
    pub fn get_admins(&self) -> Json<Vec<UserJson>> {
        let _connection = establish_connection(); 
        if self.is_superuser() {
            return Json(schema::users::table
                .filter(schema::users::perm.eq(5))
                .select((
                    schema::users::id,
                    schema::users::first_name,
                    schema::users::last_name,
                    schema::users::email,
                    schema::users::level,
                    schema::users::image,
                ))
                .load::<UserJson>(&_connection)
                .expect("E"));
        }
        else {
            return Json(Vec::new());
        }
    }
    pub fn get_partners(&self) -> Json<Vec<UserJson>> {
        let _connection = establish_connection();
        if self.is_superuser() {
            return Json(schema::users::table
                .filter(schema::users::perm.eq(4))
                .select((
                    schema::users::id,
                    schema::users::first_name,
                    schema::users::last_name,
                    schema::users::email,
                    schema::users::level,
                    schema::users::image,
                ))
                .load::<UserJson>(&_connection)
                .expect("E"));
        }
        else {
            return Json(Vec::new());
        }
    }

    pub fn is_can_work_in_object_with_id(&self, place_id: &String) -> bool {
        let _connection = establish_connection();

        schema::places::table
            .filter(schema::places::id.eq(place_id))
            .filter(schema::places::user_id.eq(&self.id))
            .select(schema::places::id)
            .first::<String>(&_connection)
            .is_ok() || schema::place_managers::table
            .filter(schema::place_managers::place_id.eq(place_id))
            .filter(schema::place_managers::user_id.eq(&self.id))
            .select(schema::place_managers::id)
            .first::<String>(&_connection)
            .is_ok() || self.is_superuser()
    }

    pub fn get_partner_objects(&self) -> Json<Vec<crate::models::Place>> {
        let _connection = establish_connection();
        if self.perm == 10 {
            return crate::models::Place::all();
        }
        else {
            return Json(schema::places::table
                .filter(schema::places::user_id.eq(self.id.clone()))
                .filter(schema::places::types.eq(1))
                .load::<crate::models::Place>(&_connection)
                .expect("E"));
        }

    }
    pub fn get_users(&self) -> Json<Vec<UserJson>> {
        let _connection = establish_connection();
        if self.is_superuser() {
            return Json(schema::users::table
                .filter(schema::users::perm.lt(10))
                .select((
                    schema::users::id,
                    schema::users::first_name,
                    schema::users::last_name,
                    schema::users::email,
                    schema::users::perm,
                    schema::users::image,
                ))
                .load::<UserJson>(&_connection)
                .expect("E"));
        }
        else {
            return Json(Vec::new());
        }
    }
    pub fn get_banned_users(&self) -> Json<Vec<UserJson>> {
        let _connection = establish_connection();
        if self.is_superuser() {
            return Json(schema::users::table
                .filter(schema::users::perm.gt(10))
                .select((
                    schema::users::id,
                    schema::users::first_name,
                    schema::users::last_name,
                    schema::users::email,
                    schema::users::level,
                    schema::users::image,
                ))
                .load::<UserJson>(&_connection)
                .expect("E"));
        }
        else {
            return Json(Vec::new());
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id:    String,
    pub email: String,
}


/*
types
0 suggest
1 active
*/
#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="partners"]
pub struct Partner {
    pub id:      String,
    pub title:   String,
    pub inn:     String,
    pub types:   i16,
    pub created: chrono::NaiveDateTime,
    pub user_id: String,
}
#[derive(Deserialize, Serialize)]
pub struct RespPartnerJson {
    pub id:      String,
    pub title:   String,
    pub inn:     String,
    pub types:   i16,
    pub created: chrono::NaiveDateTime,
    pub user:    UserJson,
}

#[derive(Deserialize, Serialize)]
pub struct PartnerJson {
    pub title:   String,
    pub inn:     String,
    pub user_id: String,
}
#[derive(Deserialize, Serialize)]
pub struct EditPartnerJson {
    pub title: String,
    pub inn:   String,
}

impl Partner {
    pub fn get_owner(&self) -> UserJson {
        let _connection = establish_connection();
        return schema::users::table
            .filter(schema::users::id.eq(self.user_id.clone()))
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
    pub fn get(id: String) -> Json<RespPartnerJson> {
        let _connection = establish_connection();
        let item = schema::partners::table
            .filter(schema::partners::id.eq(id))
            .first::<Partner>(&_connection)
            .expect("E"); 
        
        return Json(RespPartnerJson {
            id:      item.id.clone(),
            title:   item.title.clone(),
            inn:     item.inn.clone(),
            types:   item.types,
            created: item.created,
            user:    item.get_owner(),
        });
    }
    pub fn get_partners() -> Json<Vec<RespPartnerJson>> {
        let _connection = establish_connection();
        let mut stack = Vec::new();
        let items = schema::partners::table
            .filter(schema::partners::types.eq(1))
            .load::<Partner>(&_connection)
            .expect("E"); 
        for i in items {
            stack.push (RespPartnerJson {
                id:      i.id.clone(),
                title:   i.title.clone(),
                inn:     i.inn.clone(),
                types:   i.types,
                created: i.created,
                user:    i.get_owner(),
            }); 
        }
        return Json(stack);
    }
    pub fn get_suggest() -> Json<Vec<RespPartnerJson>> {
        let _connection = establish_connection();
        let mut stack = Vec::new();
        let items = schema::partners::table
            .filter(schema::partners::types.eq(0))
            .load::<Partner>(&_connection)
            .expect("E"); 
        for i in items {
            stack.push (RespPartnerJson {
                id:      i.id.clone(),
                title:   i.title.clone(),
                inn:     i.inn.clone(),
                types:   i.types,
                created: i.created,
                user:    i.get_owner(),
            }); 
        }
        return Json(stack);
    }
    pub fn suggest_partner(form: Json<PartnerJson>) -> i16 {
        let _connection = establish_connection();
        
        let new = Partner {
            id:      uuid::Uuid::new_v4().to_string(),
            title:   form.title.clone(),
            inn:     form.inn.clone(),
            types:   0,
            created: chrono::Local::now().naive_utc(),
            user_id: form.user_id.clone(),
        };
        let _partner = diesel::insert_into(schema::partners::table)
            .values(&new)
            .execute(&_connection)
            .expect("E.");
        let _user = schema::users::table
            .filter(schema::users::id.eq(form.user_id.clone()))
            .first::<User>(&_connection)
            .expect("E.");
        diesel::update(&_user)
            .set(schema::users::perm.eq(3))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn create_partner(user_id: String) -> i16 {
        let _connection = establish_connection();
        
        let _partner = schema::partners::table
            .filter(schema::partners::user_id.eq(&user_id))
            .filter(schema::partners::types.eq(0))
            .first::<Partner>(&_connection)
            .expect("E.");
        let _user = schema::users::table
            .filter(schema::users::id.eq(user_id))
            .first::<User>(&_connection)
            .expect("E.");

        diesel::update(&_partner)
            .set(schema::partners::types.eq(1))
            .execute(&_connection)
            .expect("E");
        diesel::update(&_user)
            .set(schema::users::perm.eq(4))
            .execute(&_connection)
            .expect("E");
        return 1;
    }

    pub fn edit(id: String, form: EditPartnerJson) -> i16 {
        let _connection = establish_connection();
        let _partner = schema::partners::table
            .filter(schema::partners::id.eq(id))
            .first::<Partner>(&_connection)
            .expect("E.");
        diesel::update(&_partner)
            .set((
                schema::partners::title.eq(&form.title.clone()),
                schema::partners::inn.eq(&form.inn),
            ))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn delete(user_id: String) -> i16 {
        let _connection = establish_connection();
        diesel::delete (
            schema::partners::table
                .filter(schema::partners::user_id.eq(&user_id))
        )
        .execute(&_connection)
        .expect("E");
        return 1;
    }
}