use actix_web::web::Data;
use actix_web::{web,HttpRequest, HttpResponse, HttpServer,Responder};
use crate::models::file_tb::{FileRecord,GetFileRecord};
use crate::models::file_action::{FileAction, Movement};
use serde_json::{json, Value};
use sqlx::{Pool,query,query_as, Sqlite};


pub async fn saveFile(data: FileRecord, db: web::Data<Pool<Sqlite>>) -> HttpResponse  {
    let file_data = data;
    let result = file_data.clone();
    let response = sqlx::query(r#"
    INSERT INTO file_tb(user_id,file_number, owner_name,lga, batch_number, rack_number,
     land_application_exists, c_of_o_letter_exists, r_of_o_letter_exists,
     lan_number,phone_number,remark,file_condition,number_of_pages,location,application_date,roo_date,coo_date)
     VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
     "#)
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
        .execute(db.get_ref())
        .await;

    let action = sqlx::query_as::<_, GetFileRecord>("SELECT * from file_tb where file_number = $1")
       .bind(&result.file_number)
       .fetch_one(db.get_ref())
       .await
       .unwrap();

    let new_file_action_row = sqlx::query(r#"INSERT INTO file_actions (file_id, user_id,from_office_id, to_office_id, remarks)
    VALUES (?,?,?,?,?)"#)
    .bind(&action.id)
    .bind(&action.user_id)
    .bind(1)
    .bind(1)
    .bind("")
    .execute(db.get_ref())
    .await;

    println!("new_file_action_row result: {:?}", new_file_action_row);
    println!("Query result: {:?}", action.file_number);

    match response {
        Ok(_) => HttpResponse::Created().json(result),
        Err(e) => {
            println!("Database insert failed: {}", e);
            HttpResponse::InternalServerError().json(json!({"Error":format!("Database error: {}", e)}))
        }
    }

    // HttpResponse::Created().json("")
}

pub async fn updateFile(data: FileRecord, db: web::Data<Pool<Sqlite>>) -> HttpResponse{
    let file_data = data;
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
      // HttpResponse::Ok().json("")
}

pub  fn transformFileRecord(data: Vec<FileRecord>) -> Vec<FileRecord>  {
    let output: Vec<FileRecord> = data.into_iter().map(|mut record| {
    // Insert space between 'TSL' and the number
    if let Some(pos) = record.file_number.find(|c: char| c.is_digit(10)) {
         record.file_number.insert(pos, ' ');
     }
     record
   }).collect();
   output
}

pub  fn transformMovement(data: Vec<Movement>) -> Vec<Movement>  {
    let output: Vec<Movement> = data.into_iter().map(|mut record| {
    // Insert space between 'TSL' and the number
    if let Some(pos) = record.file_number.find(|c: char| c.is_digit(10)) {
         record.file_number.insert(pos, ' ');
     }
     record
   }).collect();
   output
}



// pub async fn saveFile(data: FileRecord, db: web::Data<Pool<Sqlite>>) -> i32 {
//    let result: i32 = 1;
//    if(result == 0){
//        return  result;
//    }
//    result
// }
//
// pub async fn updateFile(data: FileRecord, db: web::Data<Pool<Sqlite>>) -> i32 {
//     let result: i32 = 1;
//     if(result == 0){
//         return  result;
//     }
//     result
// }

// pub async fn saveFile(data: web::Json<FileRecord>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
//     let file_data = data.into_inner();
//     let result = file_data.clone();
//     let response = sqlx::query(r#"
//     INSERT INTO file_tb(user_id,file_number, owner_name,lga, batch_number, rack_number,
//      land_application_exists, c_of_o_letter_exists, r_of_o_letter_exists,
//      lan_number,phone_number,remark,file_condition,number_of_pages,location,application_date,roo_date,coo_date)
//      VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
//      "#)
//         .bind(file_data.user_id)
//         .bind(file_data.file_number)
//         .bind(file_data.owner_name)
//         .bind(file_data.lga)
//         .bind(file_data.batch_number)
//         .bind(file_data.rack_number)
//         .bind(file_data.land_application_exists)
//         .bind(file_data.c_of_o_letter_exists)
//         .bind(file_data.r_of_o_letter_exists)
//         .bind(file_data.lan_number)
//         .bind(file_data.phone_number)
//         .bind(file_data.remark)
//         .bind(file_data.file_condition)
//         .bind(file_data.number_of_pages)
//         .bind(file_data.location)
//         .bind(file_data.application_date)
//         .bind(file_data.roo_date)
//         .bind(file_data.coo_date)
//         .execute(db.get_ref())
//         .await;
//
//     let action = sqlx::query_as::<_, GetFileRecord>("SELECT * from file_tb where file_number = $1")
//        .bind(&result.file_number)
//        .fetch_one(db.get_ref())
//        .await
//        .unwrap();
//
//     let new_file_action_row = sqlx::query(r#"INSERT INTO file_actions (file_id, user_id,from_office_id, to_office_id, remarks)
//     VALUES (?,?,?,?,?)"#)
//     .bind(&action.id)
//     .bind(&action.user_id)
//     .bind(1)
//     .bind(1)
//     .bind("")
//     .execute(db.get_ref())
//     .await;
//
//     println!("new_file_action_row result: {:?}", new_file_action_row);
//     println!("Query result: {:?}", action.file_number);
//
//     match response {
//         Ok(_) => HttpResponse::Created().json(result),
//         Err(e) => {
//             println!("Database insert failed: {}", e);
//             //HttpResponse::InternalServerError().body(format!("Database error: {}", e))
//             HttpResponse::InternalServerError().json(json!({"Error":format!("Database error: {}", e)}))
//         }
//     }
//
// }
//
// pub async fn updateFile(data: web::Json<FileRecord>, db: web::Data<Pool<Sqlite>>) -> impl Responder {
//     let file_data = data.into_inner();
//     let sql = r#"UPDATE file_tb SET user_id = $1,file_number = $2,owner_name = $3,lga = $4,
//     batch_number = $5,rack_number = $6,land_application_exists = $7,c_of_o_letter_exists = $8, r_of_o_letter_exists = $9,
//     lan_number = $10,phone_number = $11,remark = $12, file_condition = $13,number_of_pages = $14,location = $15,
//     application_date = $16,roo_date = $17,coo_date = $18 WHERE id = $19"#;
//     let result = query(sql)
//         .bind(file_data.user_id)
//         .bind(file_data.file_number)
//         .bind(file_data.owner_name)
//         .bind(file_data.lga)
//         .bind(file_data.batch_number)
//         .bind(file_data.rack_number)
//         .bind(file_data.land_application_exists)
//         .bind(file_data.c_of_o_letter_exists)
//         .bind(file_data.r_of_o_letter_exists)
//         .bind(file_data.lan_number)
//         .bind(file_data.phone_number)
//         .bind(file_data.remark)
//         .bind(file_data.file_condition)
//         .bind(file_data.number_of_pages)
//         .bind(file_data.location)
//         .bind(file_data.application_date)
//         .bind(file_data.roo_date)
//         .bind(file_data.coo_date)
//         .bind(file_data.id)
//        .execute(db.get_ref()) // Execute the query on the database
//       .await;
//        match result {
//         Ok(_) => HttpResponse::Ok().json("File updated successfully."),
//         Err(e) => {
//             eprintln!("Error updating user: {}", e);
//             HttpResponse::InternalServerError().json("Failed to update File.")
//         }
//     }
//
// }
