use chrono::{DateTime, Utc};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rusqlite::{params, Connection, Result};
use serde_derive::{Deserialize, Serialize};

//BASE functions
fn create_connection() -> Result<Connection> {
    let conn = Connection::open("D://data.db")?;
    Ok(conn)
}
fn curr_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    format!("{}", now.format("%Y-%m-%d %H:%M"))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RecordsResponse {
    pub rows: Vec<Child>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Child {
    pub child_id: i32,
    pub child_name: String,
    pub sex: String,
    pub dob: String,
    pub last_grade: String,
    pub school: String,
    pub community: String,
    pub photo_status: String,
    pub review_notes: String,
    pub review_datetime: String,
    pub user_id: i32,
}

pub fn push_children(children_rows: Vec<Child>) -> Result<usize> {
    let mut suc_count = 0;
    if children_rows.len() > 0 {
        let create_children_sql = "CREATE TABLE IF NOT EXISTS 'children' (
            'id' INTEGER PRIMARY KEY AUTOINCREMENT,
            'child_id' INTEGER UNIQUE NOT NULL,
            'child_name' TEXT NOT NULL,
            'sex' TEXT NOT NULL,
            'dob' TEXT NOT NULL,
            'last_grade' TEXT NOT NULL,
            'school' TEXT NOT NULL,
            'community' TEXT NOT NULL,
            'photo_status' TEXT NOT NULL,
            'rev_notes' TEXT,
            'rev_datetime' TEXT,
            'synced' INTEGER DEFAULT 0,
            'synced_dt' TEXT,
            'is_current' INTEGER DEFAULT 0)";

        let mut conn = create_connection().unwrap();
        println!("DB Connected!");
        conn.execute(create_children_sql, params![])?;
        let tx = conn.transaction()?;
        println!("Prepare Children Transaction...");

        for row in children_rows {
            //println!("Excuting for ({}, {})", child_id, &child_name);
            let sql_insert_children = "INSERT INTO children (child_id, child_name, sex, dob, last_grade, school, community, photo_status, rev_notes, rev_datetime) VALUES 
            (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)";
            match tx.execute(
                sql_insert_children,
                params![
                    row.child_id,
                    &row.child_name,
                    &row.sex,
                    &row.dob,
                    &row.last_grade,
                    &row.school,
                    &row.community,
                    &row.photo_status,
                    &row.review_notes,
                    &row.review_datetime
                ],
            ) {
                Ok(updated) => {
                    suc_count = suc_count + 1;
                    continue;
                }
                Err(err) => {
                    println!(
                        "Insert Children failed! child id:{}, user_id:{}, ",
                        row.child_id, err
                    );
                    break;
                }
            };
        }
        tx.commit();
    }
    Ok(suc_count)
}
fn get_children_res_unsynced() -> Result<Vec<Child>> {
    let conn = create_connection()?;
    let select_sql = "select child_id, child_name,sex, dob, last_grade, school, community, photo_status, rev_notes, rev_datetime from children where photo_status <> 'NOT_REV' and synced=0 LIMIT 10";

    let mut children: Vec<Child> = Vec::new();
    let mut stmt = conn.prepare(select_sql)?;
    let mut rows = stmt.query(params![])?;
    while let Some(row) = rows.next()? {
        let child = Child {
            child_id: row.get(0)?,
            child_name: row.get(1)?,
            sex: row.get(2)?,
            dob: row.get(3)?,
            last_grade: row.get(4)?,
            school: row.get(5)?,
            community: row.get(6)?,
            photo_status: row.get(7)?,
            review_notes: row.get(8)?,
            review_datetime: row.get(9)?,
            user_id: 0,
        };
        children.push(child);
    }
    Ok(children)
}

fn get_children_res_psfilter(photo_status: &str) -> Result<Vec<Child>> {
    let conn = create_connection()?;
    let select_sql = "select child_id, child_name,sex, dob, last_grade, school, community, photo_status, rev_notes, rev_datetime from children where photo_status = ?1 order by school asc, last_grade asc, child_id asc";

    let mut children: Vec<Child> = Vec::new();
    let mut stmt = conn.prepare(select_sql)?;
    let mut rows = stmt.query(params![photo_status])?;
    while let Some(row) = rows.next()? {
        let child = Child {
            child_id: row.get(0)?,
            child_name: row.get(1)?,
            sex: row.get(2)?,
            dob: row.get(3)?,
            last_grade: row.get(4)?,
            school: row.get(5)?,
            community: row.get(6)?,
            photo_status: row.get(7)?,
            review_notes: row.get(8)?,
            review_datetime: row.get(9)?,
            user_id: 0,
        };
        children.push(child);
    }
    Ok(children)
}
#[tauri::command]
pub fn decrypt(txt: &str) -> String {
    let mc = new_magic_crypt!("spons23", 256);
    if let Ok(dec_txt) = mc.decrypt_base64_to_string(txt) {
        return dec_txt;
    }
    String::from(txt)
}
#[tauri::command]
pub fn get_children_unsynced() -> Vec<Child> {
    let def_children: Vec<Child> = Vec::new();
    if let Ok(children) = get_children_res_unsynced() {
        return children;
    }
    def_children
}
#[tauri::command]
pub fn get_children_psfilter(photo_status: &str) -> Vec<Child> {
    let def_children: Vec<Child> = Vec::new();
    if let Ok(children) = get_children_res_psfilter(photo_status) {
        return children;
    }
    def_children
}
fn get_child_ids_res_psfilter(photo_status: &str) -> Result<Vec<i32>> {
    let conn = create_connection()?;
    let select_sql = "select child_id from children where photo_status = ?1";

    let mut ids: Vec<i32> = Vec::new();
    let mut stmt = conn.prepare(select_sql)?;
    let mut rows = stmt.query(params![photo_status])?;
    while let Some(row) = rows.next()? {
        ids.push(row.get(0)?)
    }
    Ok(ids)
}
fn get_child_ids_res() -> Result<Vec<i32>> {
    let conn = create_connection()?;
    let select_sql = "select child_id from children";

    let mut ids: Vec<i32> = Vec::new();
    let mut stmt = conn.prepare(select_sql)?;
    let mut rows = stmt.query(params![])?;
    while let Some(row) = rows.next()? {
        ids.push(row.get(0)?)
    }
    Ok(ids)
}
#[tauri::command]
pub fn get_child_ids() -> Vec<i32> {
    let def_vec: Vec<i32> = Vec::new();
    if let Ok(ids) = get_child_ids_res() {
        return ids;
    }
    def_vec
}
#[tauri::command]
pub fn get_child_count() -> usize {
    get_child_ids().len()
}
#[tauri::command]
pub fn get_child_count_psfilter(photo_status: &str) -> usize {
    if let Ok(ids) = get_child_ids_res_psfilter(photo_status) {
        return ids.len();
    }
    0
}

fn get_child_res(child_id: i32) -> Result<Child> {
    let mut child_row: Child = Child {
        child_id: 0,
        child_name: String::from(""),
        sex: String::from(""),
        dob: String::from(""),
        last_grade: String::from(""),
        school: String::from(""),
        community: String::from(""),
        photo_status: String::from(""),
        review_notes: String::from(""),
        review_datetime: String::from(""),
        user_id: 0,
    };
    if let Ok(conn) = create_connection() {
        let child_sql = "SELECT * FROM CHILDREN WHERE child_id = ?1";
        let mut stmt = conn.prepare(child_sql)?;
        let mut rows = stmt.query(params![child_id])?;

        while let Some(row) = rows.next()? {
            child_row = Child {
                child_id: row.get(1)?,
                child_name: row.get(2)?,
                sex: row.get(3)?,
                dob: row.get(4)?,
                last_grade: row.get(5)?,
                school: row.get(6)?,
                community: row.get(7)?,
                photo_status: row.get(8)?,
                review_notes: row.get(9)?,
                review_datetime: row.get(10)?,
                user_id: 0,
            }
        }
    }
    Ok(child_row)
}

#[tauri::command]
pub fn get_child(child_id: i32) -> Child {
    println!("Get child requested: id {}", child_id);
    if let Ok(child_row) = get_child_res(child_id) {
        println!("Got child {}: {:?}", child_id, child_row);
        return child_row;
    }
    Child {
        child_id: 0,
        child_name: String::from(""),
        sex: String::from(""),
        dob: String::from(""),
        last_grade: String::from(""),
        school: String::from(""),
        community: String::from(""),
        photo_status: String::from(""),
        review_notes: String::from(""),
        review_datetime: String::from(""),
        user_id: 0,
    }
}
fn get_top_id() -> i32 {
    if let Ok(conn) = create_connection() {
        let select_sql =
            "select child_id from children WHERE photo_status='NOT_REV' order by school asc, last_grade asc, child_id asc";

        if let Ok(mut stmt) = conn.prepare(select_sql) {
            if let Ok(mut rows) = stmt.query(params![]) {
                while let Ok(Some(row)) = rows.next() {
                    if let Ok(id) = row.get(0) {
                        return id;
                    }
                }
            }
        }
    }
    0
}

#[tauri::command]
pub fn get_selected_id() -> i32 {
    if let Ok(conn) = create_connection() {
        let select_sql = "select child_id from children where is_current='1'";

        if let Ok(mut stmt) = conn.prepare(select_sql) {
            if let Ok(mut rows) = stmt.query(params![]) {
                while let Ok(Some(row)) = rows.next() {
                    if let Ok(id) = row.get(0) {
                        return id;
                    }
                }
            }
        }
    }
    get_top_id()
}
#[tauri::command]
pub fn count_synced() -> usize {
    if let Ok(conn) = create_connection() {
        let select_sql = "SELECT COUNT(child_id) as row_count FROM CHILDREN WHERE synced=1";

        if let Ok(mut stmt) = conn.prepare(select_sql) {
            if let Ok(mut rows) = stmt.query(params![]) {
                while let Ok(Some(row)) = rows.next() {
                    if let Ok(c) = row.get(0) {
                        return c;
                    }
                }
            }
        }
    }
    0
}

#[tauri::command]
pub fn update_accept(child_id: i32) -> usize {
    if let Ok(mut conn) = create_connection() {
        if let Ok(tx) = conn.transaction() {
            let photo_status = "REV_ACC";
            let update_date = curr_date();
            let update_sql = "Update CHILDREN SET photo_status = ?1, rev_datetime=?2, is_current=0, synced=0, synced_dt=NULL WHERE child_id=?3";
            match tx.execute(update_sql, params![photo_status, update_date, child_id]) {
                Ok(update_res) => {
                    tx.commit();
                    return update_res;
                }
                Err(err) => println!("Update FAILED, Err: {:?}", err),
            }
        }
    }
    return 0;
}
#[tauri::command]
pub fn update_synced(child_id: i32, dt_synced: String) -> usize {
    if let Ok(mut conn) = create_connection() {
        if let Ok(tx) = conn.transaction() {
            let photo_status = "REV_ACC";
            let update_date = curr_date();
            let update_sql = "Update CHILDREN SET synced = 1, synced_dt=?1 WHERE child_id=?2";
            match tx.execute(update_sql, params![dt_synced, child_id]) {
                Ok(update_res) => {
                    tx.commit();
                    return update_res;
                }
                Err(err) => println!("Update FAILED, Err: {:?}", err),
            }
        }
    }
    0
}
#[tauri::command]
pub fn update_reject(child_id: i32, rev_notes: String) -> usize {
    if let Ok(mut conn) = create_connection() {
        if let Ok(tx) = conn.transaction() {
            let update_date = curr_date();
            let photo_status = "REV_REJECT";
            let update_sql = "Update CHILDREN SET photo_status = ?1,rev_notes=?2, rev_datetime=?3, is_current=0, synced=0, synced_dt=NULL WHERE child_id=?4";
            match tx.execute(
                update_sql,
                params![photo_status, rev_notes, update_date, child_id],
            ) {
                Ok(update_res) => {
                    tx.commit();
                    return update_res;
                }
                Err(err) => println!("Update FAILED, Err: {:?}", err),
            }
        }
    }
    0
}
