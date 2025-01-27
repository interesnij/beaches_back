use crate::schema;
use crate::schema::{
    place_types,
    places,
    module_types,
    modules,
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

        if crate::place_types::table
            .filter(schema::place_types::title.eq(form.title.clone()))
            .select(schema::place_types::id)
            .first::<i32>(&_connection)
            .is_ok() {
                return 0;
        }

        let new_place_type = PlaceType {
            id:    uuid::Uuid::new_v4(),
            title: form.title.clone(),
        }; 
        let _place_type = diesel::insert_into(schema::place_types::table)
            .values(&place_type)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit(id: String, form: PlaceTypeJson) -> i16 {
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
        diesel::delete (
            orders
                .filter(schema::place_types::id.eq(id))
        )
        .execute(&_connection)
        .expect("E");
    }
}


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="module_types"]
pub struct ModuleType {
    pub id:    String,
    pub title: String,
}
#[derive(Deserialize)]
pub struct ModuleTypeJson {
    pub title: String,
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

        if crate::module_types::table
            .filter(schema::module_types::title.eq(form.title.clone()))
            .select(schema::module_types::id)
            .first::<i32>(&_connection)
            .is_ok() {
                return 0;
        }

        let new_place_type = ModuleType {
            id:    uuid::Uuid::new_v4(),
            title: form.title.clone(),
        }; 
        let _place_type = diesel::insert_into(schema::module_types::table)
            .values(&place_type)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit(id: String, form: ModuleTypeJson) -> i16 {
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
        diesel::delete (
            orders
                .filter(schema::module_types::id.eq(id))
        )
        .execute(&_connection)
        .expect("E");
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

}
#[derive(Deserialize)]
pub struct PlaceJson {
    pub title:   String,
    pub created: chrono::NaiveDateTime,
    pub user_id: String,
    pub type_id: String,
    pub image:   Option<String>,
}
#[derive(Deserialize)]
pub struct EditPlaceJson {
    pub title:   String,
    pub type_id: String,
    pub image:   Option<String>,
}

impl Place {
    pub fn get_all() -> Json<Vec<Place>> {
        let _connection = establish_connection();
        return Json(schema::places::table
            .load::<Place>(&_connection)
            .expect("E"));
    }
    pub fn create(form: Json<PlaceJson>) -> i16 {
        let _connection = establish_connection();
        
        let new_place = Place {
            id:      uuid::Uuid::new_v4(),
            title:   form.title.clone(),
            created: chrono::Local::now().naive_utc(),
            user_id: form.user_id.clone(),
            type_id: form.type_id.clone(),
            image:   form.image.clone(),
        }; 
        let _place = diesel::insert_into(schema::places::table)
            .values(&new_place)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit(id: String, form: PlaceJson) -> i16 {
        let _place = schema::places::table
            .filter(schema::places::id.eq(id))
            .first::<Place>(&_connection)
            .expect("E.");
        diesel::update(&_place)
            .set((
                schema::places::title.eq(&form.title.clone()),
                schema::places::type_id.eq(&form.type_id.clone()),
                schema::places::image.eq(&form.image.clone()),
            ))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn delete(id: String) -> i16 {
        diesel::delete (
            orders
                .filter(schema::places::id.eq(id))
        )
        .execute(&_connection)
        .expect("E");
    }
}


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="modules"]
pub struct Module {
    pub id:         String,
    pub title:      String,
    pub types:      i16,
    pub place_id:   String,
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
#[derive(Deserialize)]
pub struct ModuleJson {
    pub title:      String,
    pub place_id:   String,
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
#[derive(Deserialize)]
pub struct EditModuleJson {
    pub title:      String,
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
    pub fn create(form: Json<ModuleJson>) -> i16 {
        let _connection = establish_connection();

        if crate::module_types::table
            .filter(schema::module_types::id.eq(form.type_id.clone()))
            .select(schema::module_types::id)
            .first::<i32>(&_connection)
            .is_err() {
                return 0;
        }
        if crate::places::table
            .filter(schema::places::id.eq(form.place_id.clone()))
            .select(schema::places::id)
            .first::<i32>(&_connection)
            .is_err() {
                return 0;
        }
        
        let new_module = Module {
            id:         uuid::Uuid::new_v4(),
            title:      form.title.clone(),
            types:      1,
            created:    chrono::Local::now().naive_utc(),
            place_id:   form.place_id.clone(),
            type_id:    form.type_id.clone(),
            price:      form.price,
            width:      form.width,
            height:     form.height,
            left:       form.left,
            top:        form.top,
            angle:      form.angle,
            font_color: form.font_color.clone(),
            font_size:  form.font_size.clone(),
            back_color: form.back_color.clone(),
            image:      form.image.clone(),
        }; 
        let _module = diesel::insert_into(schema::modules::table)
            .values(&new_module)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }

    #[derive(Deserialize)]
        pub struct EditModuleJson {
        pub title:      String,
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
    pub fn edit(id: String, form: ModuleJson) -> i16 {
        let _module = schema::modules::table
            .filter(schema::modules::id.eq(id))
            .first::<Module>(&_connection)
            .expect("E.");
        diesel::update(&_place)
            .set((
                schema::modules::title.eq(&form.title.clone()),
                schema::modules::price.eq(&form.price),
                schema::modules::width.eq(&form.width),
                schema::modules::height.eq(&form.height),
                schema::modules::left.eq(&form.left),
                schema::modules::top.eq(&form.top),
                schema::modules::angle.eq(&form.angle),
                schema::modules::font_color.eq(&form.font_color.clone()),
                schema::modules::font_size.eq(&form.font_size.clone()),
                schema::modules::back_color.eq(&form.back_color.clone()),
                schema::modules::image.eq(&form.image.clone()),
            ))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn delete(id: String) -> i16 {
        diesel::delete (
            orders
                .filter(schema::modules::id.eq(id))
        )
        .execute(&_connection)
        .expect("E");
    }
}