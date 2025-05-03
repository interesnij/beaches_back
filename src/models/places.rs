use crate::schema;
use crate::schema::{
    place_types,
    places,
    module_types,
    modules,
    place_managers,
    events,
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
use crate::models::{Order, Time, UserJson};


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
    pub id:          String,
    pub place_id:    String,
    pub title:       String,
    pub description: String,
    pub types:       String,
    pub image:       Option<String>,
    pub price:       i32,
} 
#[derive(Deserialize)]
pub struct ModuleTypeJson {
    pub place_id:    String,
    pub title:       String,
    pub description: String,
    pub types:       String,
    pub image:       Option<String>,
    pub price:       i32,
}
#[derive(Deserialize)]
pub struct ModuleEditTypeJson {
    pub title:       String,
    pub description: String,
    pub types:       String,
    pub image:       Option<String>,
    pub price:       i32,
}

impl ModuleType {
    pub fn get(id: String) -> ModuleType {
        let _connection = establish_connection();
        return schema::module_types::table
            .filter(schema::module_types::id.eq(id))
            .first::<ModuleType>(&_connection)
            .expect("E");
    }
    pub fn get_all_for_place(place_id: String) -> Json<Vec<ModuleType>> {
        let _connection = establish_connection();
        return Json(schema::module_types::table
            .filter(schema::module_types::place_id.eq(&place_id))
            .load::<ModuleType>(&_connection)
            .expect("E"));
    }
    pub fn change_image(id: String, image: Option<String>) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(schema::module_types::table.filter(schema::module_types::id.eq(id)))
                .set(schema::module_types::image.eq(image))
                .execute(&_connection);
        }))
    }
    pub fn create (
        place_id:    String,
        title:       String,
        description: String,
        types:       String,
        price:       String,
    ) -> String {
        let _connection = establish_connection();

        if schema::module_types::table
            .filter(schema::module_types::place_id.eq(&place_id))
            .filter(schema::module_types::title.eq(&title))
            .select(schema::module_types::id)
            .first::<String>(&_connection)
            .is_ok() {
                return "".to_string();
        }

        let uuid = uuid::Uuid::new_v4().to_string();
        let _price: i32 = price.parse().unwrap();

        let new_place_type = ModuleType {
            id:          uuid.clone(),
            place_id:    place_id,
            title:       title,
            description: description,
            types:       types,
            image:       None,
            price:       _price,
        }; 
        let _place_type = diesel::insert_into(schema::module_types::table)
            .values(&new_place_type)
            .execute(&_connection)
            .expect("E.");
        return uuid;
    }
    pub fn edit (
        id:          String, 
        title:       String,
        description: String,
        types:       String,
        price:       String,
    ) -> String {
        let _connection = establish_connection();
        let _type = schema::module_types::table
            .filter(schema::module_types::id.eq(id))
            .first::<ModuleType>(&_connection)
            .expect("E.");
        let _price: i32 = price.parse().unwrap();
        diesel::update(&_type)
                .set((
                    schema::module_types::title.eq(title),
                    schema::module_types::description.eq(description),
                    schema::module_types::types.eq(types),
                    schema::module_types::price.eq(_price),
                ))
                .execute(&_connection)
                .expect("E");
        return _type.id;
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
    pub id:         String,
    pub title:      String, 
    pub types:      i16,
    pub created:    chrono::NaiveDateTime,
    pub user_id:    String,
    pub city_id:    i32,
    pub type_id:    i16,
    pub image:      Option<String>,
    pub background: Option<String>,
    pub cord:       Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceJson {
    pub title:   String,
    pub user_id: String,
    pub city_id: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RespOrderJson {
    pub title:      String,
    pub place_id:   String,
    pub object_id:  String,
    pub user:       UserJson,
    pub price:      i32,
    pub time_start: String,
    pub time_end:   String,
    pub created:    String,
}

impl Place {
    pub fn get_orders(&self) -> Vec<RespOrderJson> {
        let _connection = establish_connection();
        let list = schema::orders::table
            .filter(schema::orders::place_id.eq(self.id.clone()))
            .order(schema::orders::created.desc())
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
                created:    i.created.format("%Y-%m-%d %H:%M:%S").to_string(), 
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
    
    pub fn get_all(type_id: i16) -> Json<Vec<Place>> {
        let _connection = establish_connection();
        return Json(schema::places::table
            .filter(schema::places::type_id.eq(type_id))
            .filter(schema::places::types.eq(1))
            .load::<Place>(&_connection)
            .expect("E"));
    }
    pub fn all() -> Json<Vec<Place>> {
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
    pub fn create(
        title:   String,
        user_id: String,
        city_id: String,
        type_id: String, 
        cord:    Option<String>
    ) -> i16 {
        let _connection = establish_connection();
        let _city_id: i32 = city_id.parse().unwrap();
        let _type_id: i16 = type_id.parse().unwrap();
        let new_place = Place {
            id:         uuid::Uuid::new_v4().to_string(),
            title:      title,
            types:      1,
            created:    chrono::Local::now().naive_utc() + chrono::Duration::hours(3),
            user_id:    user_id,
            city_id:    _city_id,
            type_id:    _type_id,
            image:      None,
            background: None, 
            cord:       cord,
        };
        let _place = diesel::insert_into(schema::places::table)
            .values(&new_place)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
    pub fn edit (
        id:      String, 
        title:   String,
        type_id: String,
        cord:    Option<String>
    ) -> i16 { 
        let _connection = establish_connection();
        let _type_id: i16 = type_id.parse().unwrap();
        let _place = schema::places::table
            .filter(schema::places::id.eq(id))
            .first::<Place>(&_connection)
            .expect("E."); 
        diesel::update(&_place) 
            .set((
                schema::places::title.eq(title),
                schema::places::type_id.eq(_type_id),
                schema::places::cord.eq(cord),
            ))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn change_avatar(place_id: String, image: Option<String>) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(places::table.filter(places::id.eq(place_id)))
                .set(schema::places::image.eq(image))
                .execute(&_connection);
        }))
    }
    pub fn change_background(place_id: String, background: Option<String>) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(places::table.filter(places::id.eq(place_id)))
                .set(schema::places::background.eq(background))
                .execute(&_connection);
        }))
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
    pub label:      String,
    pub types:      i16,
    pub place_id:   String,
    pub type_id:    String,
    pub price:      i32,
    pub z_index:    i32,
    pub _width:     i16,
    pub _height:    i16,
    pub _left:      f64,
    pub _top:       f64,
    pub _angle:     f64,
    pub font_color: String,
    pub font_size:  String,
    pub back_color: String,
    pub image:      Option<String>,
    pub event_id:   Option<String>,

}
#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleJson { 
    pub id:         String,
    pub title:      String,
    pub label:      String,
    pub type_id:    String,
    pub price:      i32,
    pub z_index:    i32,
    pub width:      i16,
    pub height:     i16,
    pub left:       f64,
    pub top:        f64,
    pub angle:      f64,
    pub font_color: String,
    pub font_size:  String,
    pub back_color: String,
    pub image:      Option<String>,
    pub event_id:   Option<String>,
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
            //.filter(schema::modules::types.eq(1))
            .select(schema::modules::id)
            .load::<String>(&_connection)
            .expect("E");
        
        let mut new_modules_ids = Vec::new();

        for i in data.modules.iter() {
            new_modules_ids.push(&i.id);
            if modules_ids.contains(&i.id) {
                println!("update case");
                let _module = schema::modules::table
                    .filter(schema::modules::id.eq(&i.id))
                    .first::<Module>(&_connection)
                    .expect("E");
                diesel::update(&_module) 
                .set((
                    schema::modules::title.eq(&i.title),
                    schema::modules::label.eq(&i.label),
                    schema::modules::type_id.eq(&i.type_id),
                    schema::modules::price.eq(&i.price),
                    schema::modules::z_index.eq(&i.z_index),
                    schema::modules::_width.eq(&i.width),
                    schema::modules::_height.eq(&i.height),
                    schema::modules::_left.eq(&i.left),
                    schema::modules::_top.eq(&i.top),
                    schema::modules::_angle.eq(&i.angle),
                    schema::modules::font_color.eq(&i.font_color),
                    schema::modules::font_size.eq(&i.font_size),
                    schema::modules::back_color.eq(&i.back_color),
                    schema::modules::image.eq(&i.image),
                    schema::modules::event_id.eq(&i.event_id),
                ))
                .execute(&_connection)
                .expect("E");
            }
            else {
                println!("create case");
                let new_module = Module {
                    id:         i.id.clone(),
                    title:      i.title.clone(),
                    label:      i.label.clone(),
                    types:      1, 
                    place_id:   place_id.clone(),
                    type_id:    i.type_id.clone(),
                    price:      i.price,
                    z_index:    i.z_index,
                    _width:     i.width,
                    _height:    i.height,
                    _left:      i.left,
                    _top:       i.top,
                    _angle:     i.angle,
                    font_color: i.font_color.clone(),
                    font_size:  i.font_size.clone(),
                    back_color: i.back_color.clone(),
                    image:      i.image.clone(),
                    event_id:   i.event_id.clone(),
                };  
                let _module = diesel::insert_into(schema::modules::table)
                    .values(&new_module)
                    .execute(&_connection)
                    .expect("E.");
            }

            //modules_ids.retain(|x| *x != i.id);
        }

        diesel::delete (
            schema::modules::table
                .filter(schema::modules::place_id.eq(&data.place_id))
                .filter(schema::modules::id.ne_any(new_modules_ids))
        )
        .execute(&_connection)
        .expect("E");
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


#[derive(Debug, Queryable, Deserialize, Serialize, Identifiable, Insertable)]
#[table_name="events"]
pub struct Event {
    pub id:          String,
    pub user_id:     String,
    pub place_id:    String,
    pub title:       String,
    pub description: String,
    pub types:       i16, 
    pub created:     chrono::NaiveDateTime,
    pub price:       i32,
    pub time_start:  String,
    pub time_end:    String,
    pub image:       Option<String>,
} 

impl Event {
    pub fn change_image(id: String, image: Option<String>) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(schema::events::table.filter(schema::events::id.eq(id)))
                .set(schema::events::image.eq(image))
                .execute(&_connection);
        }))
    }
    pub fn get(id: String) -> Event {
        let _connection = establish_connection();
        return schema::events::table
            .filter(schema::events::id.eq(id))
            .first::<Event>(&_connection)
            .expect("E");
    }
    pub fn get_all_for_place(id: String) -> Json<Vec<Event>> {
        let _connection = establish_connection();
        return Json(schema::events::table
            .filter(schema::events::place_id.eq(id))
            .load::<Event>(&_connection)
            .expect("E"));
    }
    pub fn create ( 
        user_id:     String,
        place_id:    String,
        title:       String,
        description: String,
        price:       String,
        time_start:  String,
        time_end:    String,
    ) -> String {
        let _connection = establish_connection();
 
        let format_start = chrono::NaiveDateTime::parse_from_str(&time_start, "%Y-%m-%d %H:%M:%S").unwrap();
        let format_end = chrono::NaiveDateTime::parse_from_str(&time_end, "%Y-%m-%d %H:%M:%S").unwrap();
        let new_time_start: String;
        let new_time_end: String;

        if schema::times::table
            .filter(schema::times::time.eq(format_start))
            .select(schema::times::id)
            .first::<String>(&_connection)
            .is_ok() {
                new_time_start = time_start.clone();
        }
        else {
            let new = Time {
                id:   uuid::Uuid::new_v4().to_string(),
                time: format_start + chrono::Duration::hours(3),
            }; 
            let _new_time = diesel::insert_into(schema::times::table)
                .values(&new)
                .execute(&_connection)
                .expect("E.");
            new_time_start = time_start.clone();
        }

        if schema::times::table
            .filter(schema::times::time.eq(format_end))
            .select(schema::times::id)
            .first::<String>(&_connection)
            .is_ok() {
                new_time_end = time_end.clone();
        }
        else {
            let new = Time {
                id:   uuid::Uuid::new_v4().to_string(),
                time: format_end + chrono::Duration::hours(3),
            }; 
            let _new_time = diesel::insert_into(schema::times::table)
                .values(&new)
                .execute(&_connection)
                .expect("E.");
            new_time_end = time_end.clone();
        }
        let uuid = uuid::Uuid::new_v4().to_string();
        let _price: i32 = price.parse().unwrap();

        let new_event = Event {
            id:          uuid.clone(),
            user_id:     user_id,
            place_id:    place_id,
            title:       title,
            description: description,
            types:       1,
            created:     chrono::Local::now().naive_utc(),
            price:       _price,
            time_start:  new_time_start,
            time_end:    new_time_end,
            image:       None,
        }; 
        let _new_event = diesel::insert_into(schema::events::table)
            .values(&new_event)
            .execute(&_connection)
            .expect("E.");
        return uuid;
    }

    pub fn edit (
        id:          String,
        title:       String,
        description: String,
        price:       String,
        time_start:  String,
        time_end:    String,
    ) -> String { 
        let _connection = establish_connection();
        let _price: i32 = price.parse().unwrap();

        let _event = schema::events::table
            .filter(schema::events::id.eq(id))
            .first::<Event>(&_connection)
            .expect("E.");

        diesel::update(&_event) 
            .set((
                schema::events::title.eq(title),
                schema::events::description.eq(description),
                schema::events::price.eq(_price),
                schema::events::time_start.eq(time_start),
                schema::events::time_end.eq(time_end),
            ))
            .execute(&_connection)
            .expect("E");
        return _event.id;
    }

    pub fn delete(id: String) -> i16 {
        let _connection = establish_connection();
        diesel::delete (
            schema::orders::table
                .filter(schema::orders::id.eq(id))
        )
        .execute(&_connection)
        .expect("E");
        return 1;
    }

}

