use crate::schema;
use crate::schema::{
    place_types,
    places,
    module_types,
    modules,
    place_managers,
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
use crate::models::Order;
use crate::models::UserJson;


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="place_types"]
pub struct PlaceType {
    pub id:    String,
    pub title: String,
}
#[derive(Deserialize)]
pub struct PlaceTypeJson {
    pub title: String,
}

impl PlaceType {
    pub fn get_all() -> Json<Vec<PlaceType>> {
        let _connection = establish_connection();
        return Json(schema::place_types::table
            .load::<PlaceType>(&_connection)
            .expect("E"));
    }
    pub fn create(form: Json<PlaceTypeJson>) -> i16 {
        let _connection = establish_connection();

        if schema::place_types::table
            .filter(schema::place_types::title.eq(form.title.clone()))
            .select(schema::place_types::id)
            .first::<String>(&_connection)
            .is_ok() {
                return 0;
        }

        let new_place_type = PlaceType {
            id:    uuid::Uuid::new_v4().to_string(),
            title: form.title.clone(),
        }; 
        let _place_type = diesel::insert_into(schema::place_types::table)
            .values(&new_place_type)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit(id: String, form: Json<PlaceTypeJson>) -> i16 {
        let _connection = establish_connection();
        let _type = schema::place_types::table
            .filter(schema::place_types::id.eq(id))
            .first::<PlaceType>(&_connection)
            .expect("E.");
        diesel::update(&_type)
                .set(schema::place_types::title.eq(&form.title.clone()))
                .execute(&_connection)
                .expect("E");
        return 1;
    }
    pub fn delete(id: String) -> i16 {
        let _connection = establish_connection();
        diesel::delete (
            schema::place_types::table
                .filter(schema::place_types::id.eq(&id))
        )
        .execute(&_connection)
        .expect("E");
        return 1;
    }
}


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="module_types"]
pub struct ModuleType {
    pub id:    String,
    pub title: String,
    pub types: i16,
    pub image: Option<String>,
} 
#[derive(Deserialize)]
pub struct ModuleTypeJson {
    pub title: String,
    pub image: Option<String>,
}

impl ModuleType {
    pub fn get_all() -> Json<Vec<ModuleType>> {
        let _connection = establish_connection();
        return Json(schema::module_types::table
            .load::<ModuleType>(&_connection)
            .expect("E"));
    }
    pub fn create(form: Json<ModuleTypeJson>) -> i16 {
        let _connection = establish_connection();

        if schema::module_types::table
            .filter(schema::module_types::title.eq(form.title.clone()))
            .select(schema::module_types::id)
            .first::<String>(&_connection)
            .is_ok() {
                return 0;
        }

        let new_place_type = ModuleType {
            id:    uuid::Uuid::new_v4().to_string(),
            title: form.title.clone(),
            types: 1,
            image: form.image.clone(),
        }; 
        let _place_type = diesel::insert_into(schema::module_types::table)
            .values(&new_place_type)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit(id: String, form: ModuleTypeJson) -> i16 {
        let _connection = establish_connection();
        let _type = schema::module_types::table
            .filter(schema::module_types::id.eq(id))
            .first::<ModuleType>(&_connection)
            .expect("E.");
        diesel::update(&_type)
                .set(schema::module_types::title.eq(&form.title.clone()))
                .execute(&_connection)
                .expect("E");
        return 1;
    }
    pub fn delete(id: String) -> i16 {
        let _connection = establish_connection();
        diesel::delete (
            schema::module_types::table
                .filter(schema::module_types::id.eq(&id))
        )
        .execute(&_connection)
        .expect("E");
        return 1;
    }
}


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="places"]
pub struct Place {
    pub id:      String,
    pub title:   String, 
    pub types:   i16,
    pub created: chrono::NaiveDateTime,
    pub user_id: String,
    pub type_id: String,
    pub image:   Option<String>,
    pub cord:    Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceJson {
    pub title:   String,
    pub user_id: String,
    pub type_id: String,
    pub image:   Option<String>,
    pub cord:    Option<String>,
}
#[derive(Deserialize)]
pub struct EditPlaceJson {
    pub title:   String,
    pub type_id: String,
    pub image:   Option<String>,
    pub cord:    Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RespOrderJson {
    pub title:      String,
    pub place_id:   String,
    pub object_id:  String,
    pub user:       UserJson,
    pub price:      i32,
    pub time_start: String,
    pub time_end:   String,
}

impl Place {
    pub fn get_orders(&self) -> Vec<RespOrderJson> {
        let _connection = establish_connection();
        let list = schema::orders::table
            .filter(schema::orders::place_id.eq(self.id.clone()))
            .load::<Order>(&_connection)
            .expect("E");
        let mut stack = Vec::new();
        for i in list {
            stack.push(RespOrderJson{
                title:      i.title.clone(),
                place_id:   i.place_id.clone(),
                object_id:  i.object_id.clone(),
                user:       i.get_client(),
                price:      i.price,
                time_start: i.time_start.clone(),
                time_end:   i.time_end.clone(),
            });
        }
        return stack;
    }
    pub fn get_modules(&self) -> Vec<Module> {
        let _connection = establish_connection();
        return schema::modules::table
            .filter(schema::modules::place_id.eq(self.id.clone()))
            .filter(schema::modules::types.eq(1))
            .load::<Module>(&_connection)
            .expect("E");
    }
    
    pub fn get_all() -> Json<Vec<Place>> {
        let _connection = establish_connection();
        return Json(schema::places::table
            .filter(schema::places::types.eq(1))
            .load::<Place>(&_connection)
            .expect("E"));
    }
    pub fn get(id: String) -> Json<Place> { 
        let _connection = establish_connection();
        return Json(schema::places::table
            .filter(schema::places::id.eq(id))
            .filter(schema::places::types.eq(1))
            .first::<Place>(&_connection)
            .expect("E"));
    }
    pub fn get_place(id: String) -> Place {
        let _connection = establish_connection();
        return schema::places::table
            .filter(schema::places::id.eq(id))
            .filter(schema::places::types.eq(1))
            .first::<Place>(&_connection)
            .expect("E");
    }
    pub fn get_suggest() -> Json<Vec<Place>> {
        let _connection = establish_connection();
        return Json(schema::places::table
            .filter(schema::places::types.eq(0))
            .load::<Place>(&_connection)
            .expect("E"));
    }
    pub fn get_edited() -> Json<Vec<Place>> {
        let _connection = establish_connection();
        return Json(schema::places::table
            .filter(schema::places::types.eq(2))
            .load::<Place>(&_connection)
            .expect("E"));
    }
    pub fn get_closed() -> Json<Vec<Place>> {
        let _connection = establish_connection();
        return Json(schema::places::table
            .filter(schema::places::types.eq(3))
            .load::<Place>(&_connection)
            .expect("E"));
    }
    pub fn create(form: Json<PlaceJson>) -> i16 {
        let _connection = establish_connection();
        
        let new_place = Place {
            id:      uuid::Uuid::new_v4().to_string(),
            title:   form.title.clone(),
            types:   1,
            created: chrono::Local::now().naive_utc(),
            user_id: form.user_id.clone(),
            type_id: form.type_id.clone(),
            image:   form.image.clone(), 
            cord:   form.cord.clone(),
        };
        let _place = diesel::insert_into(schema::places::table)
            .values(&new_place)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit(id: String, form: Json<PlaceJson>) -> i16 { 
        let _connection = establish_connection();
        let _place = schema::places::table
            .filter(schema::places::id.eq(id))
            .first::<Place>(&_connection)
            .expect("E."); 
        diesel::update(&_place) 
            .set((
                schema::places::title.eq(&form.title),
                schema::places::type_id.eq(&form.type_id),
                schema::places::image.eq(&form.image),
                schema::places::cord.eq(&form.cord),
            ))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn delete(id: String) -> i16 {
        let _connection = establish_connection();
        diesel::delete (
            schema::places::table
                .filter(schema::places::id.eq(&id))
        )
        .execute(&_connection)
        .expect("E");
        return 1;
    }
    pub fn get_managers(&self) -> Json<Vec<crate::models::UserJson>> {
        let _connection = establish_connection();
        let users_ids = schema::place_managers::table
            .filter(schema::place_managers::place_id.eq(self.id.clone()))
            .select(schema::place_managers::user_id)
            .load::<String>(&_connection)
            .expect("E");
        return Json(schema::users::table
            .filter(schema::users::id.eq_any(users_ids))
            //.filter(schema::users::perm.eq(2))
            .select((
                schema::users::id,
                schema::users::first_name,
                schema::users::last_name,
                schema::users::email,
                schema::users::level,
                schema::users::image,
            ))
            .load::<crate::models::UserJson>(&_connection)
            .expect("E"));
    }
}

/// 
#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="place_managers"]
pub struct PlaceManager {
    pub id:       String,
    pub user_id:  String,
    pub place_id: String,
}

#[derive(Deserialize)]
pub struct PlaceManagerJson {
    pub user_id:  String,
    pub place_id: String,
}

impl PlaceManager {
    pub fn create(form: Json<PlaceManagerJson>) -> i16 {
        let _connection = establish_connection();
        
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
    pub fn delete(id: String) -> i16 {
        let _connection = establish_connection();
        diesel::delete (
            schema::place_managers::table
                .filter(schema::place_managers::id.eq(&id))
        )
        .execute(&_connection)
        .expect("E");
        return 1;
    }
}
///

#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="modules"]
pub struct Module {
    pub id:         String,
    pub title:      String,
    pub types:      i16,
    pub place_id:   String,
    pub type_id:    String,
    pub price:      i32,
    pub _width:     i16,
    pub _height:    i16,
    pub _left:      f64,
    pub _top:       f64,
    pub _angle:     f64,
    pub font_color: String,
    pub font_size:  String,
    pub back_color: String,
    pub image:      Option<String>,

}
#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleJson { 
    pub id:         String,
    pub title:      String,
    pub type_id:    String,
    pub price:      i32,
    pub width:      i16,
    pub height:     i16,
    pub left:       f64,
    pub top:        f64,
    pub angle:      f64,
    pub font_color: String,
    pub font_size:  String,
    pub back_color: String,
    pub image:      Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateModuleJson {
    place_id: String,
    modules:  Vec<ModuleJson>,
} 

impl Module {
    pub fn get_all_for_place(place_id: String) -> Json<Vec<Module>> {
        let _connection = establish_connection();
        return Json(schema::modules::table
            .filter(schema::modules::place_id.eq(place_id))
            .load::<Module>(&_connection)
            .expect("E"));
    }
    pub fn get_all_for_place_with_type(place_id: String, type_id: String) -> Json<Vec<Module>> {
        let _connection = establish_connection();
        return Json(schema::modules::table
            .filter(schema::modules::place_id.eq(place_id))
            .filter(schema::modules::type_id.eq(type_id))
            .load::<Module>(&_connection)
            .expect("E"));
    }
    pub fn create(data: Json<CreateModuleJson>) -> i16 {
        let place_id = data.place_id.clone();
        let _connection = establish_connection();

        let mut modules_ids = schema::modules::table
            .filter(schema::modules::place_id.eq(&data.place_id))
            .filter(schema::modules::types.eq(1))
            .select(schema::modules::id)
            .load::<String>(&_connection)
            .expect("E");

        for i in data.modules.iter() {
            if i.id.clone() in modules_ids {
                let _module = schema::modules::table
                    .filter(schema::modules::place_id.eq(&i.id))
                    .first::<Module>(&_connection)
                    .expect("E");
                diesel::update(&_module) 
                .set((
                    schema::modules::title.eq(&i.title),
                    schema::modules::type_id.eq(&i.type_id),
                    schema::modules::price.eq(&i.price),
                    schema::modules::_width.eq(&i._width),
                    schema::modules::_height.eq(&i._height),
                    schema::modules::_left.eq(&i._left),
                    schema::modules::_top.eq(&i._top),
                    schema::modules::_angle.eq(&i._angle),
                    schema::modules::font_color.eq(&i.font_color),
                    schema::modules::font_size.eq(&i.font_size),
                    schema::modules::back_color.eq(&i.back_color),
                    schema::modules::_width.eq(&i._width),
                    schema::modules::_height.eq(&i._height),
                    schema::modules::_left.eq(&i._left),
                    schema::modules::_top.eq(&i._top),
                    schema::modules::image.eq(&i.image),
                ))
                .execute(&_connection)
                .expect("E");
            }
            else {
                let new_module = Module {
                    id:         i.id.clone(),
                    title:      i.title.clone(),
                    types:      1, 
                    place_id:   place_id.clone(),
                    type_id:    i.type_id.clone(),
                    price:      i.price,
                    _width:     i.width,
                    _height:    i.height,
                    _left:      i.left,
                    _top:       i.top,
                    _angle:     i.angle,
                    font_color: i.font_color.clone(),
                    font_size:  i.font_size.clone(),
                    back_color: i.back_color.clone(),
                    image:      i.image.clone(),
                };  
                let _module = diesel::insert_into(schema::modules::table)
                    .values(&new_module)
                    .execute(&_connection)
                    .expect("E.");
            }

            modules_ids.retain(|&x| x != i.id.clone());
        }
        for i in modules_ids.iter() {
            let _module = schema::modules::table
                .filter(schema::modules::place_id.eq(i))
                .first::<Module>(&_connection)
                .expect("E");

            diesel::update(&_module) 
                .set(schema::modules::types.eq(2))
                .execute(&_connection)
                .expect("E");
        }
        return 1;
    }

    pub fn delete(id: String) -> i16 {
        let _connection = establish_connection();
        diesel::delete (
            schema::modules::table
                .filter(schema::modules::id.eq(&id))
        )
        .execute(&_connection)
        .expect("E");
        return 1;
    }
}
