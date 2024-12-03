use sqlx::{Pool, Sqlite};

pub async fn create_tables(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>>{
    // SQL statements to create all tables and relationships
    println!("Hello world");
    let create_tables = vec![
        // Create the 'office' table
        r#"
        CREATE TABLE IF NOT EXISTS office (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        "#,

        // Create the 'roles' table
        r#"
        CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        "#,

        // Create the 'users' table
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL,
            staff_id TEXT NOT NULL,
            office_id INTEGER NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (office_id) REFERENCES office(id) ON DELETE CASCADE
        );
        "#,

        // Create the 'user_role' table
        r#"
        CREATE TABLE IF NOT EXISTS user_role (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            users_id INTEGER NOT NULL,
            roles_id INTEGER NOT NULL,
            FOREIGN KEY (users_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (roles_id) REFERENCES roles(id) ON DELETE CASCADE
        );
        "#,

        // Create the 'file_tb' table
        r#"
        CREATE TABLE IF NOT EXISTS file_tb (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_number TEXT NOT NULL,
            owner_name TEXT NOT NULL,
            batch_number INTEGER NOT NULL,
            rack_number INTEGER NOT NULL,
            lga TEXT NOT NULL,
            land_application_exists INTEGER NOT NULL DEFAULT 0,
            r_of_o_letter_exists INTEGER NOT NULL DEFAULT 0,
            c_of_o_letter_exists INTEGER NOT NULL DEFAULT 0,
            lan_number TEXT NOT NULL,
            phone_number TEXT NOT NULL,
            remark TEXT,
            file_condition TEXT NOT NULL CHECK(file_condition IN ('new', 'pending', 'approved', 'rejected')),
            number_of_pages INTEGER NOT NULL,
            location TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#,

        // Create the 'file_actions' table
        r#"
        CREATE TABLE IF NOT EXISTS file_actions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER,
            file_id INTEGER,
            from_office_id INTEGER,
            to_office_id INTEGER,
            remarks TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (file_id) REFERENCES file_tb(id) ON DELETE CASCADE,
            FOREIGN KEY (from_office_id) REFERENCES office(id) ON DELETE CASCADE,
            FOREIGN KEY (to_office_id) REFERENCES office(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );
        "#,
    ];

    // Execute all the create table statements
    for sql in create_tables {
        let result=  sqlx::query(sql).execute(&pool).await.unwrap();
        println!("creating tables: {:?}", result);
    }
    Ok(())
}


pub async fn seed_office_table(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>>{
     let users = vec!["Archive","Customer service","File Tracking Office (FTO)","Land Services","GIS","R-of-O Production","C-of-O Production","Director General (DG)","Surveyor General (SG)"];
     for (index, (office)) in users.iter().enumerate() {
        let result =  sqlx::query("INSERT INTO office (name) VALUES ($1)")
            .bind(office)
            .execute(&pool)
            .await?;
         println!("creating tables: {:?}", result);
     }
    Ok(())
}

pub async fn seed_file_action_table(pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error>>{
    let file_tb = vec![
      (1, "FN001".to_string(), "John Doe".to_string(), 101, 10, "Lagos".to_string(), 1, 1, 0, "LAN001".to_string(), "08012345678".to_string(), "First file in the system".to_string(), "new".to_string(), 15, "Lagos HQ".to_string(), "2024-11-19 10:00:51".to_string(), "2024-11-19 10:00:51".to_string()),
      (2, "FN002".to_string(), "Jane Smith".to_string(), 102, 11, "Abuja".to_string(), 0, 1, 1, "LAN002".to_string(), "08098765432".to_string(), "Needs review".to_string(), "pending".to_string(), 20, "Abuja Office".to_string(), "2024-11-19 10:00:51".to_string(), "2024-11-19 10:00:51".to_string()),
      (3, "FN003".to_string(), "Michael Johnson".to_string(), 103, 12, "Port Harcourt".to_string(), 0, 0, 0, "LAN003".to_string(), "08123456789".to_string(), "File under process".to_string(), "approved".to_string(), 10, "Port Harcourt".to_string(), "2024-11-19 10:00:51".to_string(), "2024-11-19 10:00:51".to_string()),
      (4, "FN004".to_string(), "Emily Davis".to_string(), 104, 13, "Enugu".to_string(), 1, 1, 1, "LAN004".to_string(), "07012345678".to_string(), "Urgent review needed".to_string(), "rejected".to_string(), 25, "Enugu Office".to_string(), "2024-11-19 10:00:51".to_string(), "2024-11-19 10:00:51".to_string()),
      (5, "FN005".to_string(), "David Brown".to_string(), 105, 14, "Kaduna".to_string(), 0, 0, 0, "LAN005".to_string(), "09012345678".to_string(), "Pending verification".to_string(), "pending".to_string(), 12, "Kaduna Branch".to_string(), "2024-11-19 10:00:51".to_string(), "2024-11-19 10:00:51".to_string()),
      (11, "FN006".to_string(), "James".to_string(), 12, 3, "LGA".to_string(), 0, 0, 0, "123".to_string(), "123".to_string(), "ss".to_string(), "new".to_string(), 12, "location".to_string(), "2024-11-23 15:44:43".to_string(), "2024-11-23 15:44:43".to_string())
     ];

    let file_actions = vec![
       (1, 1, 1, 1, 2, Some("File transferred from Headquarters to Sales Department".to_string()), "2024-11-19 10:00:51".to_string()),
       (2, 2, 2, 2, 3, Some("File moved from Sales Department to HR Department for review".to_string()), "2024-11-19 10:00:51".to_string()),
       (3, 3, 3, 3, 4, Some("File approved and sent to Finance Department for processing".to_string()), "2024-11-19 10:00:51".to_string()),
       (4, 4, 4, 4, 5, Some("File rejected and moved to IT Department for audit".to_string()), "2024-11-19 10:00:51".to_string()),
       (5, 5, 5, 1, 2, Some("File moved from Headquarters to Sales Department for internal use".to_string()), "2024-11-19 10:00:51".to_string()),
       (6, 3, 1, 2, 3, None, "2024-11-21 11:11:53".to_string())
   ];

   let users = vec![
       (1, "John Doe".to_string(), "SD123".to_string(), 1, "2024-11-19 10:00:51".to_string()),
       (2, "Jane Smith".to_string(), "HR456".to_string(), 2, "2024-11-19 10:00:51".to_string()),
       (3, "Michael Johnson".to_string(), "FN789".to_string(), 3, "2024-11-19 10:00:51".to_string()),
       (4, "Emily Davis".to_string(), "IT101".to_string(), 4, "2024-11-19 10:00:51".to_string()),
       (5, "David Brown".to_string(), "SD112".to_string(), 1, "2024-11-19 10:00:51".to_string())
   ];



   for (index, (id,file_id,user_id,from_office_id,to_office_id,remark,date)) in file_actions.iter().enumerate() {
      let result =  sqlx::query("INSERT INTO office (name) VALUES ($1,$2,$3,$4,$5)")
          .bind(file_id)
          .bind(user_id)
          .bind(from_office_id)
          .bind(to_office_id)
          .bind(remark)
          .execute(&pool)
          .await?;
       println!("creating tables: {:?}", result);
   }

  Ok(())
}
