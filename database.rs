/*
Made by: Mathew Dusome
Jul 31 2026
Turso (libSQL) database module for Rust

April 2: Dray52 Added fetch by id with examples
================================
INITIAL SETUP:
================================
1. Add to mod.rs: pub mod database;
2. Sign up: https://turso.tech
3. Create DB: turso db create my-db
4. Get URL: turso db show my-db
5. Get token: turso db tokens create my-db

6. Add dependencies to Cargo.toml. Add these 2 lines to [dependencies]:
    In the termal, run:
        cargo add serde@1.0 --features derive
        cargo add serde_json@1.0
        cargo add ureq@2.9 --features json --target 'cfg(not(target_arch = "wasm32"))'

    Or manually add to Cargo.toml in the [dependencies] section:
        
        serde = { version = "1.0", features = ["derive"] }
        serde_json = "1.0"
      
      and add this to the [target.'cfg(not(target_arch = "wasm32"))'.dependencies] section: (create it if it doesn't exist)
        ureq = { version = "2.9", features = ["json"] }
   
7.  Follow the instructions db-directions.md to set up your Cloudflare Worker backend. This is required for the database module to work in both native and WASM builds.

8. Add use statement:
    use crate::utils::database::{create_database_client, DatabaseTable};
9. Add to mod.rs:
    pub mod database;


================================
CUSTOMIZE YOUR DATABASE SCHEMA:
================================
1. Modify the DatabaseTable struct below
   - Add/remove fields to match your table columns
   - Use appropriate Rust types: i32 for INTEGER, String for TEXT, bool for BOOLEAN, f64 for REAL
   - Keep id: i32 (0 for INSERT means auto-generate, populated with actual ID for SELECT)
   - Use serde attributes for custom naming if needed

2. Create your table in Turso (via CLI or SQL):
   
   Using Turso CLI:
     turso db shell my-db
     CREATE TABLE my_table (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       column1 TEXT NOT NULL,
       column2 INTEGER,
       ...
     );

 
3. Column type mapping:
   - INTEGER → i32, i64
   - TEXT → String
   - REAL → f64
   - BOOLEAN → bool
   - NUMERIC → f64 or String

================================
USAGE EXAMPLES:
================================

// NOTE: The table used in these examples is called 'messages'.
    let client = create_database_client();

// Fetch all records (for display)
    let mut records: Vec<DatabaseTable> = Vec::new();
    let fetched_results = client.fetch_table("messages").await;
    if let Ok(result) = fetched_results {
        records = result;
       // To update a ListView with these records:
        // update_listview(&mut list_view, &records);
        }
    } else {
       println!("Error fetching records from database: {} ",fetched_results.err().unwrap());
    }

     if let Ok(Some(record)) = client.fetch_record_by_id::<DatabaseTable>("message", id).await {
                println!("Successfully fetched record from database.");
      else if let Ok(None) = client.fetch_record_by_id::<DatabaseTable>("message", id).await {
                println!("No record found with id {}", id);
      } else if let Err(err) = client.fetch_record_by_id::<DatabaseTable>("message", id).await {
                println!("Error fetching record from database: {}", err);
      }

// Insert a record (from user text input)
    let new_record = DatabaseTable { id: 0, text: "User entered text".to_string() };
    let insert_results =  client.insert_record("messages", &new_record).await;
    if let Ok(id) = insert_results {
        // Inserted, id contains the new record's id
    } else {
        println!("Error inserting records from database: {} ",insert_results.err().unwrap());
    }


// Update a record by id (Can only do one column at a time with this method)
    if let Ok(updated_count) = client.update_record_by_id("messages", 5, "text", "New text").await {
        // updated_count is the number of records updated
    } else {
        // Handle error
    }

// Update a record by struct (update all non-id fields)
    let updated_record = DatabaseTable { id: 5, text: "Updated text".to_string() };
    if let Ok(updated_count) = client.update_record_by_struct("messages", &updated_record).await {
        // updated_count is the number of records updated
    } else {
        // Handle error
    }

// Delete a record by id (from user id input)
    if let Ok(deleted_count) = client.delete_record_by_id("messages", 5).await {
        // deleted_count is the number of records deleted
    } else {
        // Handle error
    }


// Displaying records in a ListView:
//Where 'list_view' is your ListView instance and 'records' is the Vec<DatabaseTable> fetched from the database.
//Change the items.push! line to customize how each record is displayed in the list.
   
   fn update_listview(list_view: &mut ListView, messages: &Vec<DatabaseTable>) {
    list_view.clear();
    let mut items: Vec<String> = Vec::new();
    for (i, msg) in messages.iter().enumerate() {
        items.push(format!("  {}: ID={}, Text={}", i + 1, msg.id, msg.text));
    }
    list_view.add_items(&items);
}
*/

use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use ureq;
#[cfg(target_arch = "wasm32")]
use macroquad::prelude::next_frame;

// Helper function for serde to skip serializing id when it's 0
fn is_zero(num: &i32) -> bool {
    *num == 0
}

// URL of your Cloudflare Worker backend
pub const WORKER_URL: &str = "https://db-worker.mathew-dusome.workers.dev";


// ============================================================================
// CUSTOMIZE THIS STRUCT FOR YOUR DATABASE SCHEMA
// ============================================================================

/// Your database record struct - used for both INSERT and SELECT operations
/// When inserting: set id to 0 (it will be auto-generated by the database)
/// When fetching: id will be populated with the actual ID
/// 
/// Modify this struct to match your table columns:
/// #[derive(Debug, Deserialize, Serialize, Clone)]
/// pub struct DatabaseTable {
///     #[serde(default, skip_serializing_if = "is_zero")]
///     pub id: i32,
///     pub email: String,
///     pub age: i32,
///     pub active: bool,
///     pub score: f64,
/// }
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseTable {
    #[serde(default, skip_serializing_if = "is_zero")]
    pub id: i32,
    pub text: String,
    // Example: Add more fields like this:
    // pub email: String,
    // pub age: i32,
    // pub active: bool,
    // pub score: f64,
}

// ============================================================================
// CONVENIENCE FUNCTIONS
// ============================================================================

#[allow(unused)]
pub fn create_database_client() -> DatabaseClient {
    DatabaseClient::new(WORKER_URL.to_string())
}


/// Create a table with custom name and schema
/// The table name and columns are fully customizable
/// Update this function if you want to change the table structure
/// 
/// Example for different schemas:
/// ```
/// // For users table:
/// CREATE TABLE users (
///   id INTEGER PRIMARY KEY AUTOINCREMENT,
///   email TEXT NOT NULL UNIQUE,
///   age INTEGER,
///   active BOOLEAN DEFAULT 1,
///   score REAL
/// )
/// 
/// // For products table:
/// CREATE TABLE products (
///   id INTEGER PRIMARY KEY AUTOINCREMENT,
///   name TEXT NOT NULL,
///   price REAL,
///   in_stock BOOLEAN
/// )
/// ```

pub struct DatabaseClient {
    worker_url: String,
}

impl DatabaseClient {
        #[allow(unused)]    
        pub async fn insert_record<T: Serialize>(&self, table: &str, record: &T) -> Result<i64, Box<dyn std::error::Error>> {
            let payload = serde_json::json!({
                "action": "insert",
                "table": table,
                "record": record
            });
            let resp = self.send_request(&payload).await?;
            Ok(resp["id"].as_i64().unwrap_or(0))
        }

        #[allow(unused)]
        pub async fn update_record_by_struct<T: Serialize>(&self, table: &str, record: &T) -> Result<i64, Box<dyn std::error::Error>> {
            let payload = serde_json::json!({
                "action": "update",
                "table": table,
                "record": record
            });
            let resp = self.send_request(&payload).await?;
            Ok(resp["updated"].as_i64().unwrap_or(0))
        }

        #[allow(unused)]
        pub async fn update_record_by_id(&self, table: &str, id: i64, column: &str, value: &serde_json::Value) -> Result<i64, Box<dyn std::error::Error>> {
            let payload = serde_json::json!({
                "action": "update_by_column",
                "table": table,
                "id": id,
                "column": column,
                "value": value
            });
            let resp = self.send_request(&payload).await?;
            Ok(resp["updated"].as_i64().unwrap_or(0))
        }

        #[allow(unused)]
        pub async fn delete_record_by_id(&self, table: &str, id: i64) -> Result<i64, Box<dyn std::error::Error>> {
            let payload = serde_json::json!({
                "action": "delete",
                "table": table,
                "id": id
            });
            let resp = self.send_request(&payload).await?;
            Ok(resp["deleted"].as_i64().unwrap_or(0))
        }
        
        #[allow(unused)]
        async fn send_request(&self, payload: &serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
            let body = payload.to_string();
            #[cfg(not(target_arch = "wasm32"))]
            {
                let url = &self.worker_url;
                let response = ureq::post(url)
                    .set("Content-Type", "application/json")
                    .send_string(&body);
                let text = match response {
                    Ok(resp) => resp.into_string()?,
                    Err(ureq::Error::Status(code, resp)) => {
                        let err_body = resp.into_string().unwrap_or_else(|_| "Could not read error body".to_string());
                        return Err(format!("HTTP {} error: {}", code, err_body).into());
                    }
                    Err(e) => return Err(e.into()),
                };
                let json: serde_json::Value = serde_json::from_str(&text)?;
                Ok(json)
            }
            #[cfg(target_arch = "wasm32")]
            {
                extern "C" {
                    fn mq_db_query(ptr: *const u8, len: usize, url_ptr: *const u8, url_len: usize);
                    fn mq_db_query_result_len() -> usize;
                    fn mq_db_query_fill_result(ptr: *mut u8);
                    fn mq_db_query_clear_result();
                }
                let url_bytes = self.worker_url.as_bytes();
                let json_bytes = body.as_bytes();
                // Call JS: mq_db_query(ptr, len, url_ptr, url_len)
                unsafe {
                    mq_db_query(
                        json_bytes.as_ptr(),
                        json_bytes.len(),
                        url_bytes.as_ptr(),
                        url_bytes.len(),
                    );
                }
                let mut tries = 0;
                let max_tries = 100;
                let mut result_len = 0;
                while tries < max_tries {
                    result_len = unsafe { mq_db_query_result_len() };
                    if result_len > 0 {
                        break;
                    }
                    tries += 1;
                    next_frame().await;
                }
                if result_len == 0 {
                    return Err("No result from JS db_query (timeout or JS error)".into());
                }
                let mut buf = vec![0u8; result_len];
                unsafe {
                    mq_db_query_fill_result(buf.as_mut_ptr());
                    mq_db_query_clear_result();
                }
                let text = String::from_utf8(buf).map_err(|e| format!("UTF-8 error: {}", e))?;
                let json: serde_json::Value = serde_json::from_str(&text)?;
                Ok(json)
            }
        }
    pub fn new(worker_url: String) -> Self {
        Self { worker_url }
    }


   
    #[allow(unused)]
    pub async fn fetch_table<T: for<'de> Deserialize<'de>>(&self, table: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "action": "fetch",
            "table": table
        });
        let resp = self.send_request(&payload).await?;
        let records = resp["records"].as_array().cloned().unwrap_or_default();
        let mut result = Vec::new();
        for record in records {
            result.push(serde_json::from_value(record)?);
        }
        Ok(result)
    }
    #[allow(unused)]
    pub async fn fetch_record_by_id<T: for<'de> Deserialize<'de>>(&self, table: &str, id: i64) -> Result<Option<T>, Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "action": "fetch_by_id",
            "table": table,
            "id": id
        });
        let resp = self.send_request(&payload).await?;
        let record = resp.get("record").cloned();
        match record {
            Some(val) if !val.is_null() => Ok(Some(serde_json::from_value(val)?)),
            _ => Ok(None)
        }
    }



    // All direct SQL and WASM FFI helpers removed; only HTTP/Worker logic remains
}
