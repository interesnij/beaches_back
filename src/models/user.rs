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

#[derive(Deserialize)]
pub struct UserJson {
    pub id:         String,
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub level:      i16,
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
    pub time_start: String,
    pub time_end:   String,
}
#[derive(Serialize)]
pub struct PlaceListJson {
    pub title: String,
    pub image: Option<String>,
    pub cord:  String,
}

#[derive(Serialize)]
pub struct RespOrderJson {
    pub order:  OrderListJson,
    pub place:  PlaceListJson,
}

impl User {
    pub fn is_superuser(&self) -> bool {
        return self.perm == 10;
    }
    pub fn is_partner(&self) -> bool {
        return self.perm == 4;
    }
    pub fn get_orders(&self) -> Json<Vec<RespOrderJson>> {
        let _connection = establish_connection();
        let list = schema::orders::table
            .filter(schema::orders::user_id.eq(self.id.clone()))
            .load::<Order>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for i in list {
            let _place = crate::models::Place::get_place(i.place_id.clone());
            let _place_item = PlaceListJson {
                title: i.title.clone(),
                image: i.image.clone(),
                cord:  i.cord.clone(),
            };
            let _order_item = OrderListJson {
                object_id:  i.title.clone(),
                price:      i.price,
                time_start: i.time_start,
                time_end:   i.time_end,
            }

            stack.push(RespOrderJson {
                order:  _order_item,
                place:  _place_item,
            });
        }
        return stack;
    }
    pub fn create_manager(&self, form: Json<crate::models::PlaceManagerJson>) -> i16 {
        let _connection = establish_connection();
        let place_id = &form.place_id;
        let user_id = &form.user_id;
        let _place = crate::models::Place::get_place(place_id);
        if &self.id != user_id {
            return 0;
        }
        
        let new_place_manager = PlaceManager {
            id:       uuid::Uuid::new_v4().to_string(),
            user_id:  form.user_id.clone(),
            place_id: form.place_id.clone(),
        }; 
        let _place_manager = diesel::insert_into(schema::place_managers::table)
            .values(&new_place_manager)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn delete_manager(&self, form: Json<crate::models::PlaceManagerJson>) -> i16 {
        let _connection = establish_connection();
        let place_id = &form.place_id;
        let user_id = &form.user_id;
        let _place = crate::models::Place::get_place(place_id);
        if &self.id != user_id {
            return 0;
        }
        diesel::delete (
            schema::place_managers::table
                .filter(schema::place_managers::user_id.eq(user_id))
                .filter(schema::place_managers::place_id.eq(place_id))
            )
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
    pub fn create_superuser(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(10))
                .execute(&_connection);
        }))
    }
    pub fn edit_owner_partner(form: crate::models::EditOwnerPartnerJson) -> i16 {
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
    pub fn delete_superuser(user_id: i32) -> Result<(), Error> {
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

    pub fn create_user_block(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(21))
                .execute(&_connection);
        }))
    }
    pub fn delete_user_block(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(1))
                .execute(&_connection);
        }))
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
        
        if _new_user.id == 1 {
            diesel::update(&_new_user)
                .set(schema::users::perm.eq(10))
                .execute(&_connection)
                .expect("Error.");
        }
        return _new_user;
    }
    pub fn edit(id: String, form: Json<EditUserJson>) -> i16 {
        let _user = schema::users::table
            .filter(schema::users::id.eq(id))
            .first::<User>(&_connection)
            .expect("E.");
        diesel::update(&_user)
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
            return Vec::new();
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
            return Vec::new();
        }
    }
    pub fn get_partner_objects(&self) -> Json<Vec<crate::models::Place>> {
        let _connection = establish_connection();
        if self.perm == 10 {
            return crate::models::Place::get_all();
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
                    schema::users::level,
                    schema::users::image,
                ))
                .load::<UserJson>(&_connection)
                .expect("E"));
        }
        else {
            return Vec::new();
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
            return Vec::new();
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
    pub id:    i32,
    pub email: String,
}


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
    pub fn get_owner(&self, id: String) -> UserJson {
        let _connection = establish_connection();
        return Json(schema::users::table
            .filter(schema::users::id.eq(self.user_id))
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
    pub fn get(id: String) -> Json<RespPartnerJson> {
        let _connection = establish_connection();
        let item = schema::partners::table
            .filter(schema::partners::id.eq(id))
            .first::<Partner>(&_connection)
            .expect("E"); 
        
        return Json(RespPartnerJson {
            title:   item.title.clone(),
            inn:     item.inn.clone(),
            types:   item.types,
            created: item.created,
            user:    Partner::get_owner(i.user_id.clone()),
        });
    }
    pub fn all() -> Vec<RespPartnerJson> {
        let _connection = establish_connection();
        let item = schema::partners::table
            .filter(schema::partners::types.eq(1))
            .load::<Partner>(&_connection)
            .expect("E"); 
        
        return Json(RespPartnerJson {
            title:   item.title.clone(),
            inn:     item.inn.clone(),
            types:   item.types,
            created: item.created,
            user:    item.get_owner(item.user_id.clone()),
        });
    }
    pub fn create(form: Json<PartnerJson>) -> i16 {
        let _connection = establish_connection();
        
        let new = Partner {
            id:      uuid::Uuid::new_v4().to_string(),
            title:   form.title.clone(),
            inn:     form.inn.clone(),
            types:   1,
            created: chrono::Local::now().naive_utc(),
            user_id: form.user_id.clone(),
        }; 
        let _partner = diesel::insert_into(schema::partners::table)
            .values(&new)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }

    pub fn edit(id: String, form: EditPartnerJson) -> i16 {
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
    pub fn delete(id: String) -> i16 {
        diesel::delete (
            orders
                .filter(schema::partners::id.eq(&id))
        )
        .execute(&_connection)
        .expect("E");
        return 1;
    }
}