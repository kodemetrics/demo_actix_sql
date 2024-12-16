CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL,
        role TEXT NOT NULL,
        staff_id TEXT NOT NULL,
        office_id INTEGER NOT NULL,
        -- status TEXT DEFAULT 'enabled',
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (office_id) REFERENCES office(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_role (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        users_id INTEGER NOT NULL,
        roles_id INTEGER NOT NULL,
        FOREIGN KEY (users_id) REFERENCES users(id) ON DELETE CASCADE,
        FOREIGN KEY (roles_id) REFERENCES roles(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS file_tb (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL DEFAULT 1,
        file_number TEXT NOT NULL UNIQUE,
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
        file_condition TEXT NOT NULL,
        number_of_pages INTEGER NOT NULL,
        location TEXT NOT NULL,
        application_date TEXT,
        coo_date TEXT,
        roo_date TEXT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS file_actions (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      user_id INTEGER,
      file_id INTEGER,
      from_office_id INTEGER,
      to_office_id INTEGER,
      remarks TEXT,
      status INTEGER,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (file_id) REFERENCES file_tb(id) ON DELETE CASCADE,
      FOREIGN KEY (from_office_id) REFERENCES office(id) ON DELETE CASCADE,
      FOREIGN KEY (to_office_id) REFERENCES office(id) ON DELETE CASCADE,
      FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
  );

INSERT INTO `office` (`id`, `name`) VALUES
(1, 'Archive'),
(2, 'Customer service'),
(3, 'File Tracking Office (FTO)'),
(4, 'Land services'),
(5, 'GIS'),
(6, 'R-of-O Production'),
(7, 'C-of-O Production'),
(8, 'Director General (DG)'),
(9, 'Surveyor General (SG)');

INSERT INTO users (name, email, password, role, staff_id, office_id)
VALUES
('John Doe', 'john.doe@example.com', 'password123', 'Archive', 'S12345', 1);

INSERT INTO users (name, email, password, role, staff_id, office_id)
VALUES
('Jane Smith', 'jane.smith@example.com', 'password456', 'Others', 'S67890', 2);

INSERT INTO users (name, email, password, role, staff_id, office_id)
VALUES
('Alice Johnson', 'alice.johnson@example.com', 'password789', 'Admin', 'S11223', 1);

INSERT INTO users (name, email, password, role, staff_id, office_id)
VALUES
('Bob Brown', 'bob.brown@example.com', 'password101', 'Others', 'S44556', 3);

INSERT INTO users (name, email, password, role, staff_id, office_id)
VALUES
('Charlie Davis', 'charlie.davis@example.com', 'password202', 'Archive', 'S77889', 2);

INSERT INTO users (name, email, password, role, staff_id, office_id)
VALUES
('Bridget Antai', 'bridgetantai@gmail.com', 'password202', 'Archive', 'S77889', 1);

INSERT INTO users (name, email, password, role, staff_id, office_id)
VALUES
('Bridget Antai', 'bridgetantai2024@gmail.com', 'password202', 'Admin', 'S77889', 9);


INSERT INTO `file_tb` (`id`,`user_id`, `file_number`, `owner_name`, `batch_number`, `rack_number`, `lga`, `land_application_exists`, `r_of_o_letter_exists`, `c_of_o_letter_exists`, `lan_number`, `phone_number`, `remark`, `file_condition`, `number_of_pages`, `location`, `created_at`, `updated_at`) VALUES
(1, 1, 'TSL001', 'John Doe', 101, 10, 'Lagos', 1, 1, 0, 'LAN001', '08012345678', 'First file in the system', 'new', 15, 'Archive', '2024-11-19 10:00:51', '2024-11-19 10:00:51'),
(2, 1, 'TSL002', 'Jane Smith', 102, 11, 'Abuja', 0, 1, 1, 'LAN002', '08098765432', 'Needs review', 'pending', 20, 'Archive', '2024-11-19 10:00:51', '2024-11-19 10:00:51'),
(3, 1, 'TSL003', 'Michael Johnson', 103, 12, 'Port Harcourt', 0, 0, 0, 'LAN003', '08123456789', 'File under process', 'approved', 10, 'Archive', '2024-11-19 10:00:51', '2024-11-19 10:00:51'),
(4, 1, 'TSL004', 'Emily Davis', 104, 13, 'Enugu', 1, 1, 1, 'LAN004', '07012345678', 'Urgent review needed', 'rejected', 25, 'Archive', '2024-11-19 10:00:51', '2024-11-19 10:00:51'),
(5, 1, 'TSL005', 'David Brown', 105, 14, 'Kaduna', 0, 0, 0, 'LAN005', '09012345678', 'Pending verification', 'pending', 12, 'Archive', '2024-11-19 10:00:51', '2024-11-19 10:00:51'),
(6, 1, 'TSL006', 'James', 12, 3, 'LGA', 0, 0, 0, '123', '123', 'ss', 'new', 12, 'Archive', '2024-11-23 15:44:43', '2024-11-23 15:44:43');

INSERT INTO `file_actions` (`id`, `user_id`, `file_id`, `from_office_id`, `to_office_id`, `remarks`, `created_at`) VALUES
(1, 1, 1, 1, 2, 'File transferred from Headquarters to Sales Department', '2024-11-19 10:00:51'),
(2, 2, 2, 2, 3, 'File moved from Sales Department to HR Department for review', '2024-11-19 10:00:51'),
(3, 3, 3, 3, 4, 'File approved and sent to Finance Department for processing', '2024-11-19 10:00:51'),
(4, 4, 4, 4, 5, 'File rejected and moved to IT Department for audit', '2024-11-19 10:00:51'),
(5, 5, 5, 1, 2, 'File moved from Headquarters to Sales Department for internal use', '2024-11-19 10:00:51'),
(6, 3, 1, 2, 3, NULL, '2024-11-21 11:11:53');
