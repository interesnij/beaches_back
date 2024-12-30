use crate::schema;
use crate::schema::{
    users,
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

impl User {
    pub fn is_superuser(&self) -> bool {
        return self.perm > 59;
    }
    pub fn is_partner(&self) -> bool {
        return self.perm > 10 && self.perm < 60;
    }
    pub fn create_superuser(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(60))
                .execute(&_connection);
        }))
    }
    pub fn create_partner(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(10))
                .execute(&_connection);
        }))
    }
    pub fn delete_superuser(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(1))
                .execute(&_connection);
        }))
    }
    pub fn delete_partner(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(10))
                .execute(&_connection);
        }))
    }
    pub fn get_user_with_email(email: &String) -> Result<User, Error> {
        let _connection = establish_connection();
        return Ok(schema::users::table
            .filter(schema::users::email.eq(email))
            .first::<User>(&_connection)?);
    }
    pub fn create(form: Json<NewUserJson>) -> User {
        let _connection = establish_connection();
        let form_user = User {
            id:         uuid::Uuid::new_v4(),
            first_name: form.first_name.clone(),
            last_name:  form.last_name.clone(),
            email:      form.email.clone(),
            password:   crate::utils::hash_password(&form.password),
            perm:       1,
            level:      100,
            image:      None,
        };

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");
        
        if _new_user.id == 1 {
            diesel::update(&_new_user)
                .set(schema::users::perm.eq(60))
                .execute(&_connection)
                .expect("Error.");
        }
        return _new_user;
    }
    pub fn edit(id: String, form: EditUserJson) -> i16 {
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
                .filter(schema::users::perm.eq(60))
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
                .filter(schema::users::perm.between("10", "60"))
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
    pub fn get_users(&self) -> Json<Vec<UserJson>> {
        let _connection = establish_connection();
        if self.is_superuser() {
            return Json(schema::users::table
                .filter(schema::users::perm.gt(60))
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