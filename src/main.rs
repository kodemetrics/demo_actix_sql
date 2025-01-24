#![allow(
dead_code,
unused_crate_dependencies,
unused_variables,
unused_assignments,
unused,
non_snake_case,
non_camel_case_types,
warnings
)]

mod models;
mod utils;

use actix_web::web::Data;
use actix_web::{get,post,put,http, web, App, HttpRequest, HttpResponse, HttpServer,Responder};
use actix_cors::Cors;
use crate::models::error::APIError;
use crate::models::user::{UserRoles, User,Login,NewUser,UpdateUser,NewUpdateUser};
use crate::models::file_action::{FileAction, Movement};
use crate::models::file_tb::{FileRecord,GetFileRecord};
use crate::models::office::Office;
use crate::models::report::Report;
use serde_json::{json, Value};
use sqlx::{Pool,query,query_as, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::migrate::MigrateDatabase;

use crate::utils::email_utils::EmailService;
use std::fs::OpenOptions;
use log::LevelFilter;
use std::io::Write;
use env_logger::{Builder, Env};
use log::{info, error};

use actix_web::http::header::ContentType;
use futures::{stream, Stream};
use futures::SinkExt;
use std::time::Duration;
use tokio::time::sleep;
// use actix_web::web::Bytes;
// use bytes::Bytes;
// use actix_web::error::DispatchError::Body;
// use actix_web::web::JsonBody::Body;

pub struct AppState {}

fn init_logger() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("actix_errors.log")
        .unwrap();

    Builder::new()
        .filter(None, LevelFilter::Info) // Change log level as needed
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();
}

#[post("/api/login")]
async fn auth_user(user: web::Json<Login>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let new_user = user.into_inner();
    let result = new_user.clone();
    let query = r#" SELECT * from users where email = $1 and password = $2 "#;
    let result = sqlx::query_as::<_, User>(query)
       .bind(new_user.email)
       .bind(new_user.password)
       .fetch_one(db.get_ref())
       .await
       .unwrap();
   web::Json(result)
}

#[post("/api/users")]
async fn save_new_user(user: web::Json<NewUpdateUser>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let json_payload = user.clone();
    let new_user = user.into_inner();
    let result = new_user.clone();

    let file_number_exists = sqlx::query_as::<_, User>("SELECT * from users where email = $1")
       .bind(new_user.email)
       .fetch_optional(db.get_ref())
       .await;

       match file_number_exists {
        Ok(Some(existing_file)) => {
            utils::api_utils::updateUser(json_payload, db).await
        }
        Ok(None) => {
             utils::api_utils::saveUser(json_payload, db).await
        }
        Err(err) => {
            eprintln!("Error checking for file record: {:?}", err);
            HttpResponse::InternalServerError().json("Error occurred while accessing the database")
        }
    }
}

// #[post("/api/users")]
// async fn save_new_user(user: web::Json<NewUpdateUser>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
//     let json_payload = user.clone();
//     let new_user = user.into_inner();
//     let result = new_user.clone();
//
//     let response = sqlx::query(r#"INSERT INTO users (name,email,password,staff_id,office_id,role)VALUES (?,?,?,?,?,?)
//      "#)
//         .bind(new_user.name)
//         .bind(new_user.email)
//         .bind(new_user.password)
//         .bind(new_user.staff_id)
//         .bind(new_user.office_id)
//         .bind(new_user.role)
//         .execute(db.get_ref())
//         .await;
//         match response {
//             Ok(_) => HttpResponse::Created().json(result),
//             Err(e) => {
//                 println!("Database insert failed: {}", e);
//                 HttpResponse::InternalServerError().json(json!({"Error":format!("Database error: {}", e)}))
//             }
//         }
// }


#[get("/api/users")]
async fn fetch_users(db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let response = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db.get_ref())
        .await;
        match response {
            Ok(response) => web::Json(response),
            Err(_) => web::Json(vec![]),
        }
}

#[get("/api/users/{user_email}")]
async fn fetch_user_by_email(user_email: web::Path<String>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let email = user_email.into_inner();
    let query = r#" SELECT * from users where email = $1"#;
    let result = sqlx::query_as::<_, User>(query)
       .bind(email)
       .fetch_one(db.get_ref())
       .await
       .unwrap();
     web::Json(result)
}

#[put("/api/users")]
async fn update_user(data: web::Json<UpdateUser>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let user = data.into_inner();
    let sql = r#"UPDATE users SET name = $1,email = $2,password = $3,role = $4 WHERE id = $5"#;
    let result = query(sql)
       .bind(user.name)
       .bind(user.email)
       .bind(user.password)
       .bind(user.role)
       .bind(user.id)
       .execute(db.get_ref()) // Execute the query on the database
       .await;
   match result {
        Ok(_) => HttpResponse::Ok().json("User updated successfully."),
        Err(e) => {
            eprintln!("Error updating user: {}", e);
            HttpResponse::InternalServerError().json("Failed to update user.")
        }
    }
}

#[get("/api/report")]
async fn getReport(db: web::Data<Pool<Sqlite>>) -> impl Responder {

    let file_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM file_tb")
           .fetch_one(db.get_ref())
           .await
           .unwrap();

   let unit_count: Vec<Office> = sqlx::query_as(r#"
            SELECT COUNT(*) as unit_count,to_office_id as id,name FROM file_actions AS f
            JOIN office AS o ON f.to_office_id = o.id
            group by to_office_id"#)
          .fetch_all(db.get_ref())
          .await
          .unwrap();
     let result = json!({"file_count":file_count.0, "unit_count":unit_count});

     // HttpResponse::Ok().json("Report was generated successfully")
     HttpResponse::Ok().json(result)
}

#[post("/api/report")]
async fn getReportByDate (data: web::Json<Report>,db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let instance = data.into_inner();

    let file_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM file_tb WHERE created_at BETWEEN $1 AND $2")
           .bind(&instance.from) // Bind the from date to the query
           .bind(&instance.to)  // Bind the to date to the query
           .fetch_one(db.get_ref())
           .await
           .unwrap();

   let unit_count: Vec<Office> = sqlx::query_as(r#"
             SELECT COUNT(*) as unit_count,to_office_id as id,name,f.created_at FROM file_actions AS f
             JOIN office AS o ON f.to_office_id = o.id
             WHERE f.created_at BETWEEN $1 AND $2
             group by to_office_id"#)
            .bind(&instance.from) // Bind the from date to the query
            .bind(&instance.to)  // Bind the to date to the query
            .fetch_all(db.get_ref())
            .await
            .unwrap();
     let result = json!({"file_count":file_count.0, "unit_count":unit_count});
     HttpResponse::Ok().json(result)
}

#[post("/api/files")]
async fn save_new_file(file: web::Json<FileRecord>, db: web::Data<Pool<Sqlite>>) -> impl Responder {

    let json_payload = file.clone();
    let file_data = file.into_inner(); // This gives you the actual FileRecord
    let file_number = &file_data.file_number;

    let file_number_exists = sqlx::query_as::<_, GetFileRecord>("SELECT * from file_tb where file_number = $1")
       .bind(file_number)
       .fetch_optional(db.get_ref())
       .await;

       match file_number_exists {
        Ok(Some(existing_file)) => {
            utils::api_utils::updateFile(json_payload, db).await
        }
        Ok(None) => {
             utils::api_utils::saveFile(json_payload, db).await
        }
        Err(err) => {
            // Handle database error, if any
            eprintln!("Error checking for file record: {:?}", err);
            // Return an error response
            HttpResponse::InternalServerError().json("Error occurred while accessing the database")
        }
    }
}

#[get("/api/file/{file_no}")]
async fn fetchFileByfileNumber(file_no: web::Path<String>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let fileno = file_no.into_inner();
    let query = r#" SELECT * from file_tb where file_number = $1"#;
    let result = sqlx::query_as::<_, GetFileRecord>(query)
       .bind(fileno)
       .fetch_one(db.get_ref())
       .await
       .unwrap();
     web::Json(result)
}

#[put("/api/file")]
async fn update_file(data: web::Json<GetFileRecord>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let file_data = data.into_inner();
    let sql = r#"UPDATE file_tb SET user_id = $1,file_number = $2,owner_name = $3,lga = $4,
    batch_number = $5,rack_number = $6,land_application_exists = $7,c_of_o_letter_exists = $8, r_of_o_letter_exists = $9,
    lan_number = $10,phone_number = $11,remark = $12, file_condition = $13,number_of_pages = $14,location = $15,
    application_date = $16,roo_date = $17,coo_date = $18 WHERE id = $19"#;
    let result = query(sql)
        .bind(file_data.user_id)
        .bind(file_data.file_number)
        .bind(file_data.owner_name)
        .bind(file_data.lga)
        .bind(file_data.batch_number)
        .bind(file_data.rack_number)
        .bind(file_data.land_application_exists)
        .bind(file_data.c_of_o_letter_exists)
        .bind(file_data.r_of_o_letter_exists)
        .bind(file_data.lan_number)
        .bind(file_data.phone_number)
        .bind(file_data.remark)
        .bind(file_data.file_condition)
        .bind(file_data.number_of_pages)
        .bind(file_data.location)
        .bind(file_data.application_date)
        .bind(file_data.roo_date)
        .bind(file_data.coo_date)
        .bind(file_data.id)
       .execute(db.get_ref()) // Execute the query on the database
       .await;
       match result {
        Ok(_) => HttpResponse::Ok().json("File updated successfully."),
        Err(e) => {
            eprintln!("Error updating user: {}", e);
            HttpResponse::InternalServerError().json("Failed to update File.")
        }
    }
}



#[get("/api/files")]
async fn fetch_files(db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let response = sqlx::query_as::<_, FileRecord>("SELECT * FROM file_tb")
        .fetch_all(db.get_ref())
        .await;

    match response {
        Ok(response) => {
            let output = utils::api_utils::transformFileRecord(response);
            // web::Json(response)
            web::Json(output)
        },
        Err(_) => web::Json(vec![]),
    }
}

#[post("/api/transaction")]
async fn save_file_movements(action: web::Json<FileAction>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let new_movement = action.into_inner();
    let result = new_movement.clone();
    let response = sqlx::query(r#"INSERT INTO file_actions (user_id,file_id,from_office_id,to_office_id,status,remarks)VALUES (?,?,?,?,?,?)
     "#)
        .bind(new_movement.user_id)
        .bind(new_movement.file_id)
        .bind(new_movement.from_office_id)
        .bind(new_movement.to_office_id)
        .bind(0) // new_user.status
        .bind(new_movement.remarks)
        .execute(db.get_ref())
        .await;
        match response {
            Ok(_) => HttpResponse::Created().json(result),
            Err(e) => {
                println!("Database insert failed: {}", e);
                HttpResponse::InternalServerError().json(json!({"Error":format!("Database error: {}", e)}))
            }
        }
}

#[get("/api/transaction/{id}")]
async fn fetch_movements(path: web::Path<String>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let file_no = path.into_inner();
    let query = r#"
        SELECT
            f.id AS file_id,
            f.file_number,
            f.owner_name,
            f.batch_number,
            f.rack_number,
            fm.remarks,
            oa.name AS previous_location,
            oa.id AS previous_location_id,
            ob.name AS current_location,
            ob.id AS current_location_id,
            fm.created_at
        FROM file_actions AS fm
        JOIN file_tb AS f ON fm.file_id = f.id
        JOIN office AS oa ON fm.from_office_id = oa.id
        JOIN office AS ob ON fm.to_office_id = ob.id
        WHERE f.file_number = $1
        ORDER BY fm.created_at DESC
    "#;
    let result = sqlx::query_as::<_, Movement>(query)
       .bind(file_no)
       .fetch_all(db.get_ref())
       .await
       .unwrap();
   let output = utils::api_utils::transformMovement(result);
   // web::Json(result)
   web::Json(output)
}

#[get("/api/location/{id}")]
async fn fetch_locations(path: web::Path<String>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
    let file_no = path.into_inner();
    let query = r#"
        SELECT
            f.id AS file_id,
            f.file_number,
            f.owner_name,
            f.batch_number,
            f.rack_number,
            fm.remarks,
            oa.name AS previous_location,
            oa.id AS previous_location_id,
            ob.name AS current_location,
            ob.id AS current_location_id,
            fm.created_at
        FROM file_actions AS fm
        JOIN file_tb AS f ON fm.file_id = f.id
        JOIN office AS oa ON fm.from_office_id = oa.id
        JOIN office AS ob ON fm.to_office_id = ob.id
        WHERE f.file_number = $1
        ORDER BY fm.created_at DESC
        LIMIT 1
    "#;

    let result = sqlx::query_as::<_, Movement>(query)
       .bind(file_no)
       .fetch_all(db.get_ref())
       .await
       .unwrap();
    let output = utils::api_utils::transformMovement(result);
    // web::Json(result)
    web::Json(output)
}


#[post("/api/v1/auth/send-email")]
async fn send_email(data: web::Json<EmailService>) -> impl Responder {
    utils::email_utils::send_email(data.to.clone(), data.from.clone(), data.reply.clone(), data.subject.clone(), data.body.to_string());
    HttpResponse::Ok().body("Email sent!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    // init_logger();
    env_logger::init();


    //let database_url = "sqlite://test.db"; // Path to your SQLite database file
    let database_url = "sqlite:test.db"; // Path to your SQLite database file

    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        println!("Creating database {}", database_url);
        match Sqlite::create_database(database_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

      // Configure the pool options
     let pool = SqlitePoolOptions::new()
         .max_connections(5)  // Set the maximum number of connections in the pool
         .connect(database_url)
         .await
         .expect("Error connecting to the database");

    // let pool = Pool::<Sqlite>::connect(database_url)
    //     .await
    //     .expect("Error connecting to the database");

    // Await the create_tables function if it is async
    // utils::seed_utils::create_tables(pool.clone()).await.unwrap();
    // if let Err(e) = utils::seed_utils::create_tables(pool.clone()).await {
    //     eprintln!("Error creating tables: {}", e);
    // }

   //utils::seed_utils::seed_office_table(pool.clone()).await;

   let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
   let migrations = std::path::Path::new(&crate_dir).join("./migrations");

   let migration_results = sqlx::migrate::Migrator::new(migrations)
       .await
       .unwrap()
       .run(&pool)
       .await;

   match migration_results {
       Ok(_) => println!("Migration success"),
       Err(error) => {
           panic!("error: {}", error);
       }
   }

    let port = 8000;
    println!("🚀 Server started successfully on port: {}", port);
    HttpServer::new(move || {

        let cors = Cors::default()
        .allow_any_origin().send_wildcard()
        // .allowed_origin("http://localhost:1420")
        .allowed_methods(vec!["GET", "POST","PUT","DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);

        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(cors)
            // .route("/events", web::get().to(streamSSE))
            .service(send_email)
            .service(auth_user)
            .service(fetch_users)
            .service(fetch_user_by_email)
            .service(update_user)


            .service(save_new_user)
            .service(save_new_file)
            .service(save_file_movements)

            .service(fetch_files)
            .service(fetchFileByfileNumber)
            .service(fetch_movements)
            .service(fetch_locations)
            .service(update_file)
            .service(getReport)
            .service(getReportByDate)



    })
        // .workers(8)
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
