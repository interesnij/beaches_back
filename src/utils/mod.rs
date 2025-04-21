mod auth;
mod reqwest;

pub use self::{
    auth::*,
};
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use actix_web::{
    HttpRequest,
    web,
};
use crate::schema;
use serde::{Deserialize, Serialize};
use crate::diesel::{
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
    PgConnection,
};
use crate::models::User;
use std::fs::File;
use std::io::{Write,BufWriter};
use std::fs::create_dir_all;


#[derive(Deserialize)]
pub struct FileForm {
    pub name: String,
    pub size:  i32,
}

#[derive(Debug, Clone)]
pub struct UploadedFiles {
    pub name: String, 
    pub path: String,
}
impl UploadedFiles {
    fn new(filename: String) -> UploadedFiles {
        use chrono::Datelike;

        let now = chrono::Local::now().naive_utc();
        let format_folder = format!(
            "/beaches_front/media/{}/{}/{}/",
            now.year().to_string(),
            now.month().to_string(),
            now.day().to_string(),
        );
        let format_path = format_folder.clone() + &filename.to_string();
        create_dir_all(format_folder).unwrap();

        UploadedFiles {
            name: filename.to_string(),
            path: format_path.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileForm2 {
    pub id: i32,
    pub files: Vec<String>,
}
pub async fn files_form(payload: &mut Multipart) -> FileForm2 {
    let mut form: FileForm2 = FileForm2 {
        id:    0,
        files: Vec::new(),
    }; 

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "files[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap(); 
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                };
                form.files.push(file.path.clone().replace("/beaches_front",""));
            }
        }
    }
    form
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ImageForm {
    pub image: String,
}
pub async fn image_form(payload: &mut Multipart) -> ImageForm {
    let mut form: ImageForm = ImageForm {
        image: "".to_string(),
    }; 

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap(); 
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                };
                
                form.image = file.path.clone().replace("/beaches_front","");
            }
        }
    }
    form
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModuleTypeForm {
    pub place_id:    String,
    pub title:       String,
    pub description: Option<String>,
    pub types:       String,
    pub image:       Option<String>,
}

pub async fn module_type_form(payload: &mut Multipart) -> ModuleTypeForm {
    let mut form: ModuleTypeForm = ModuleTypeForm {
        place_id:    "".to_string(),
        title:       "".to_string(),
        description: "".to_string(),
        types:       "".to_string(),
        image:       None,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("Failed to open hello.txt"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("Failed to open hello.txt");
                }
                form.image = file.path.clone().replace("/beaches_front","");
            }
        }
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string
                    } else if field.name() == "description" {
                        form.description = data_string
                    } else if field.name() == "place_id" {
                        form.place_id = data_string
                    }
                    else if field.name() == "types" {
                        form.types = data_string
                    }
                }
            }
        }
    }
    form
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EventForm {
    pub place_id:    String,
    pub title:       String,
    pub description: String,
    pub price:       i32,
    pub time_start:  String,
    pub time_end:    String,
    pub image:       Option<String>,
}

pub async fn event_form(payload: &mut Multipart) -> EventForm {
    let mut form: EventForm = EventForm {
        place_id:    "".to_string(),
        title:       "".to_string(),
        description: "".to_string(),
        price:       0,
        time_start:  "".to_string(),
        time_end:    "".to_string(),
        image:       None,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("Failed to open hello.txt"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("Failed to open hello.txt");
                }
                form.image = file.path.clone().replace("/beaches_front","");
            }
        }
        else if name == "price" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = std::str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.price = _int;
                }
            }
        }
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string
                    } else if field.name() == "description" {
                        form.description = data_string
                    } else if field.name() == "place_id" {
                        form.place_id = data_string
                    }
                    else if field.name() == "time_start" {
                        form.time_start = data_string
                    }
                    else if field.name() == "time_end" {
                        form.time_end = data_string
                    }
                }
            }
        }
    }
    form
}

pub async fn save_file(data: String) -> String {
    let file_data: FileForm = serde_json::from_str(&data).unwrap();
    let path = "/beaches_front/media/".to_owned() + &file_data.name;
    let mut f = File::create(&path).expect("Unable to create file");

    //println!("metadata: {:?}", f.metadata());
    //let len = f.try_clone().expect("REASON").metadata().expect(" no metadata").len();
    //while (file_data.size - 8) >= len.try_into().unwrap() {
        //println!("len: {:?}", f.metadata().expect(" no metadata").len());
        //f.write_all(data.as_bytes()).map(|_| f).expect("Unable to write data");
    //}
    //let bin = data.as_bytes();
        //let _data = chunk.unwrap();
    //f.write_all(&bin).map(|_| f).expect("Unable to write data");
    
    //println!("f: {:?}", f);
    //f.set_len(file_data.size.try_into().unwrap());
    //println!("metadata: {:?}", f.metadata());
    return "".to_string();
}

#[derive(Deserialize, Serialize)]
pub struct NewUserForm {
    pub first_name: String,
    pub last_name:  String,
    pub email:      String,
    pub password:   String,
}

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn get_secret<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("secret")?.to_str().ok();
}

pub fn is_signed_in(req: &HttpRequest) -> bool {
  get_secret(&req).is_some()
}

pub fn get_current_user(req: &HttpRequest) -> User {

    let secret = get_secret(&req).unwrap();
    let uuid = hex::decode(secret).expect("failed decode");
    let _connection = establish_connection();
    return schema::users::table
        .filter(schema::users::uuid.eq(uuid))
        .first::<User>(&_connection)
        .expect("Error.");
} 

pub fn get_limit (
    limit: Option<i64>,
    default_limit: i64
) -> i64 {
    let _limit: i64;
    if limit.is_some() {
        let l_unwrap = limit.unwrap();
        if l_unwrap > 100 {
            _limit = default_limit;
        }
        else {
            _limit = l_unwrap;
        }
    }
    else {
        _limit = default_limit;
    }
    _limit
}

pub fn get_page(req: &HttpRequest) -> i32 {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub page: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let page: i32;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
    }
    else {
        page = 1;
    }
    page
}

pub fn get_id(req: &HttpRequest) -> i32 {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub id: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let id: i32;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.id.is_some() {
            id = params.id.unwrap();
        }
        else {
            id = 0;
        }
    }
    else {
        id = 0;
    }
    id
}


pub fn get_user(id: String) -> User { 
    let _connection = establish_connection();
    return schema::users::table
        .filter(schema::users::id.eq(id))
        .first::<User>(&_connection)
        .expect("Error.");
}