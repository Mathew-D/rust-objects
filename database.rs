/*
Made by: Mathew Dusome
May 23 2025

Adds database functionality for both web and native platforms using Supabase as the cloud database.
This module provides a simple and consistent API for storing and retrieving game data, user profiles,
settings, and other persistent information across different platforms.

In your mod.rs file located in the modules folder add the following to the end of the file:
    pub mod database;
    
Then add the following with the use commands:
use crate::modules::database::*;  // This imports all functions directly

To use this module, add these dependencies to your Cargo.toml file:

```toml
[dependencies]
# Add these to your existing dependencies:
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }

# For web builds, add this section:
# These dependencies ensure proper functionality in WebAssembly environments
[target.'cfg(target_arch = "wasm32")'.dependencies]
reqwest = { version = "0.11", features = ["json", "wasm-client"] }
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["console"] }
```

This configuration enables seamless database connectivity across both:
- Native platforms (Windows, macOS, Linux)
- Web platforms (via WebAssembly)

The Supabase REST API works consistently in both environments with no code changes needed.

Basic example of how to use this in your code:

// 1. Create a connection to your Supabase database
let client = create_connection(
    "https://your-project.supabase.co/rest/v1",
    "your-api-key".to_string()
);

// 2. Create a table for storing game scores (usually done once at app startup)
// The game_scores table tracks player performance with fields for:
// - player: The username of the player
// - score: The numeric score achieved in the game
// - level: The game level where the score was achieved
// - date: When the score was recorded
create_table(&client, "game_scores", &[
    "player TEXT", 
    "score INTEGER",
    "level INTEGER", 
    "date TEXT"
]).await
    .expect("Failed to create table");

// 3. Insert data into the database
let columns = &["player", "score", "level", "date"];
let values = &["Player1", "500", "5", "2025-05-23"];
insert_db(&client, "game_scores", columns, values).await
    .expect("Failed to insert data");

// 4. Fetch data from the database
let scores = select_db(&client, "game_scores", None).await
    .expect("Failed to get scores");

// 5. Display the data
for score in scores {
    let player = score.data.get("player")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    
    let points = score.data.get("score")
        .and_then(|v| v.as_str())
        .unwrap_or("0");
    
    let level = score.data.get("level")
        .and_then(|v| v.as_str())
        .unwrap_or("1");
        
    let date = score.data.get("date")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");
    
    println!("Player: {}, Score: {}, Level: {}, Date: {}", 
             player, points, level, date);
}

// 6. Update a record
update_db(&client, "game_scores", 1, &["score=600"]).await
    .expect("Failed to update score");

// 7. Delete a record
delete_db(&client, "game_scores", 1).await
    .expect("Failed to delete score");

// ---- Authentication Example ----

// 8. Create a users table for authentication
create_table(&client, "users", &[
    "username TEXT",
    "password TEXT", // In production, store hashed passwords only!
    "email TEXT",
    "last_login TEXT"
]).await
    .expect("Failed to create users table");

// 9. Insert a new user
let user_columns = &["username", "password", "email", "last_login"];
let user_values = &["player_one", "hashed_password_here", "player@example.com", "2025-05-23"];
insert_db(&client, "users", user_columns, user_values).await
    .expect("Failed to insert user");

// 10. Authenticate a user (simple example - use proper authentication in production)
// Note: This example works identically on both web and native platforms
let username = "player_one";
let password = "user_entered_password"; // This would come from user input
let conditions = &[&format!("username={}", username)];

let users = select_db(&client, "users", Some(conditions)).await
    .expect("Failed to query users");

if let Some(user) = users.first() {
    let stored_password = user.data.get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    if verify_password(password, stored_password) {
        println!("Authentication successful!");
    } else {
        println!("Authentication failed: incorrect password");
    }
} else {
    println!("Authentication failed: user not found");
}

// Helper function to verify password (implement proper hashing in production)
fn verify_password(input: &str, stored: &str) -> bool {
    // In a real application, you would use a proper password hashing library
    // like bcrypt, argon2, or pbkdf2 to compare hashed passwords
    // 
    // Note: For web projects, be sure to include the appropriate wasm-compatible 
    // hashing libraries in your Cargo.toml. Most modern crypto libraries now support wasm32 targets.
    input == stored
}
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;

// Web-specific imports for better error handling
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Web compatibility notes:
// This module has been designed to work seamlessly on both native and web platforms.
// - For web platforms, all async operations use wasm-bindgen-futures internally
// - Network requests automatically use the appropriate transport mechanism per platform
// - The Supabase REST API works identically in both environments
//
// When building for web, be aware of:
// 1. CORS policies: Your Supabase project needs proper CORS configuration
// 2. API key security: Never expose your service_role key in web apps, use anon key instead
// 3. Authentication: Consider using Supabase Auth directly for web apps instead of the simplified example

// Custom error type for database operations
#[derive(Debug)]
pub struct DbError {
    pub message: String,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database error: {}", self.message)
    }
}

impl StdError for DbError {}

// Define the Result type for our database operations
pub type Result<T> = std::result::Result<T, DbError>;

// The Database client represents the connection to our Supabase database
#[derive(Clone)]
pub struct DbClient {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
}

// Model for a database row
#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub id: Option<i64>,
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl DbClient {
    /// Creates a new database client for Supabase
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the Supabase REST API (e.g., "https://your-project.supabase.co/rest/v1")
    /// * `api_key` - The Supabase API key for authentication
    ///
    /// # Returns
    ///
    /// * `DbClient` - The database client
    pub fn new(base_url: &str, api_key: String) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Makes an authenticated request to the Supabase REST API
    pub(crate) async fn request<T: Serialize + ?Sized>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut req_builder = self.client.request(method, &url);
        
        // Add Supabase authentication headers
        req_builder = req_builder
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key));
        
        // Add the request body if provided
        if let Some(data) = body {
            req_builder = req_builder.json(data);
        }
        
        req_builder
            .send()
            .await
            .map_err(|e| DbError { message: e.to_string() })
    }
    
    /// Get the path for a table operation in Supabase
    fn get_table_path(&self, table: &str) -> String {
        format!("/{}", table)
    }
    
    /// Get the path for a row operation in Supabase
    fn get_row_path(&self, table: &str, row_id: Option<i64>) -> String {
        if let Some(id) = row_id {
            format!("/{}?id=eq.{}", table, id)
        } else {
            format!("/{}", table)
        }
    }
    
    /// Format the request body for creating tables in Supabase
    pub(crate) fn format_create_table_body(&self, table: &str, schema: &HashMap<String, String>) -> serde_json::Value {
        // Supabase would typically use SQL for creating tables
        // This is simplified for the API approach
        serde_json::json!({
            "name": table,
            "columns": schema
        })
    }
    
    /// Format the data for insert in Supabase
    pub(crate) fn format_insert_body(&self, _table: &str, data: &HashMap<String, String>) -> serde_json::Value {
        // Supabase directly accepts the data object
        serde_json::json!(data)
    }
    
    /// Format the query parameters for select in Supabase
    pub(crate) fn format_select_params(&self, table: &str, conditions: Option<&[&str]>) -> (String, Option<serde_json::Value>) {
        if conditions.is_none() || conditions.unwrap().is_empty() {
            (self.get_table_path(table), None)
        } else {
            let conditions = conditions.unwrap();
            let mut query = self.get_table_path(table);
            let filters: Vec<String> = conditions.iter()
                .map(|c| {
                    let parts: Vec<&str> = c.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        format!("{}=eq.{}", parts[0].trim(), parts[1].trim().trim_matches('\'').trim_matches('"'))
                    } else {
                        String::new()
                    }
                })
                .filter(|s| !s.is_empty())
                .collect();
            
            if !filters.is_empty() {
                query.push('?');
                query.push_str(&filters.join("&"));
            }
            
            (query, None)
        }
    }
    
    /// Format the update body for Supabase
    pub(crate) fn format_update_body(&self, table: &str, row_id: i64, updates: &HashMap<String, String>) -> (String, serde_json::Value) {
        (
            format!("/{}?id=eq.{}", table, row_id),
            serde_json::json!(updates)
        )
    }
    
    /// Format the delete parameters for Supabase
    pub(crate) fn format_delete_params(&self, table: &str, row_id: i64) -> (String, Option<serde_json::Value>) {
        (format!("/{}?id=eq.{}", table, row_id), None)
    }
    
    /// Parse the response from Supabase
    pub(crate) async fn parse_select_response(&self, response: reqwest::Response) -> Result<Vec<Row>> {
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| DbError { message: e.to_string() })?;
        
        // Supabase returns an array of objects
        if let Some(rows) = json.as_array() {
            let mut result = Vec::new();
            
            for row_value in rows {
                if let Some(row_obj) = row_value.as_object() {
                    let mut row = Row {
                        id: row_obj.get("id").and_then(|v| v.as_i64()),
                        data: HashMap::new(),
                    };
                    
                    for (key, value) in row_obj {
                        if key != "id" {
                            row.data.insert(key.clone(), value.clone());
                        }
                    }
                    
                    result.push(row);
                }
            }
            
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }
}

/// Creates a connection to the Supabase database
///
/// # Arguments
///
/// * `db_url` - The Supabase REST API URL (e.g., "https://your-project.supabase.co/rest/v1")
/// * `api_key` - The Supabase API key for authentication
///
/// # Returns
///
/// * `DbClient` - The database client
pub fn create_connection(db_url: &str, api_key: String) -> DbClient {
    DbClient::new(db_url, api_key)
}

/// Creates a table in the Supabase database
///
/// # Arguments
///
/// * `client` - The database client
/// * `table` - Table name
/// * `columns` - Column definitions
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub async fn create_table(client: &DbClient, table: &str, columns: &[&str]) -> Result<()> {
    // Create a schema object from column definitions
    let mut schema = HashMap::new();
    
    for col_def in columns {
        let parts: Vec<&str> = col_def.split_whitespace().collect();
        if parts.len() >= 2 {
            let col_name = parts[0];
            let col_type = parts[1].to_lowercase();
            
            let field_type = match col_type.as_str() {
                "text" => "string",
                "integer" => "number",
                "real" => "number",
                "blob" => "binary",
                _ => "string", // Default to string for unknown types
            };
            
            schema.insert(col_name.to_string(), field_type.to_string());
        }
    }
    
    // Supabase uses RPC for schema changes
    let path = "/rpc/create_table";
    
    // Format the request body for Supabase
    let body = client.format_create_table_body(table, &schema);
    
    // Send request
    let response = client
        .request(reqwest::Method::POST, path, Some(&body))
        .await?;
    
    // Check response status
    if response.status().is_success() {
        Ok(())
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(DbError {
            message: format!("Failed to create table: {} - {}", response.status(), error_text),
        })
    }
}

/// Inserts data into a table
///
/// # Arguments
///
/// * `client` - The database client
/// * `table` - Table name
/// * `columns` - Column names
/// * `data` - Data values (must match columns in number)
///
/// # Returns
///
/// * `Result<i64>` - Row ID or error
pub async fn insert_db(
    client: &DbClient, 
    table: &str, 
    columns: &[&str], 
    data: &[&str]
) -> Result<i64> {
    if columns.len() != data.len() {
        return Err(DbError {
            message: "Columns and data length mismatch".to_string(),
        });
    }
    
    // Build the data object
    let mut row_data = HashMap::new();
    for (i, col) in columns.iter().enumerate() {
        row_data.insert(col.to_string(), data[i].to_string());
    }
    
    // Format the request body based on provider
    let body = client.format_insert_body(table, &row_data);
    
    // Get the path based on provider
    let path = match client.provider {
        DbProvider::Supabase => client.get_table_path(table),
        DbProvider::Firebase => client.get_row_path(table, None),
        DbProvider::MongoDB => "/action/insertOne",
        DbProvider::Neon | DbProvider::Generic => client.get_row_path(table, None),
    };
    
    // Send request
    let response = client
        .request(reqwest::Method::POST, &path, Some(&body))
        .await?;
    
    // Check response status
    if response.status().is_success() {
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| DbError { message: e.to_string() })?;
        
        // Extract ID based on provider format
        let id = match client.provider {
            DbProvider::Supabase => json.get("id").and_then(|v| v.as_i64()),
            DbProvider::Firebase => json.get("name").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()),
            DbProvider::MongoDB => json.get("insertedId").and_then(|v| v.as_i64()),
            DbProvider::Neon | DbProvider::Generic => json.get("id").and_then(|v| v.as_i64()),
        };
        
        Ok(id.unwrap_or(0))
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(DbError {
            message: format!("Failed to insert data: {} - {}", response.status(), error_text),
        })
    }
}

/// Selects data from a table
///
/// # Arguments
///
/// * `client` - The database client
/// * `table` - Table name
/// * `conditions` - Optional conditions for WHERE clause (format: "column=value")
///
/// # Returns
///
/// * `Result<Vec<Row>>` - Result rows or error
pub async fn select_db(
    client: &DbClient, 
    table: &str, 
    conditions: Option<&[&str]>
) -> Result<Vec<Row>> {
    // Format query parameters based on provider
    let (path, body_opt) = client.format_select_params(table, conditions);
    
    // Create the request based on whether we need a body or not
    let response = if let Some(body) = body_opt {
        // Some providers like MongoDB require a POST with a body for queries
        client
            .request(reqwest::Method::POST, &path, Some(&body))
            .await?
    } else {
        // Others use GET with query parameters
        client
            .request(reqwest::Method::GET, &path, Option::<&()>::None)
            .await?
    };
    
    // Check response status
    if response.status().is_success() {
        // Parse the response based on provider format
        client.parse_select_response(response).await
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(DbError {
            message: format!("Failed to select data: {} - {}", response.status(), error_text),
        })
    }
}

/// Updates data in a table
///
/// # Arguments
///
/// * `client` - The database client
/// * `table` - Table name
/// * `row_id` - ID of the row to update
/// * `updates` - Column and value pairs (format: ["column=value", ...])
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub async fn update_db(
    client: &DbClient,
    table: &str,
    row_id: i64,
    updates: &[&str],
) -> Result<()> {
    // Build the update data
    let mut update_data = HashMap::new();
    
    for update in updates {
        let parts: Vec<&str> = update.splitn(2, '=').collect();
        if parts.len() == 2 {
            let col = parts[0].trim();
            let val = parts[1].trim().trim_matches('\'').trim_matches('"');
            update_data.insert(col.to_string(), val.to_string());
        }
    }
    
    // Format the path and body based on provider
    let (path, body) = client.format_update_body(table, row_id, &update_data);
    
    // Determine the HTTP method based on provider
    let method = match client.provider {
        DbProvider::Supabase => reqwest::Method::PATCH,
        DbProvider::Firebase => reqwest::Method::PUT,  // Firebase prefers PUT for updates
        DbProvider::MongoDB => reqwest::Method::POST,  // MongoDB uses POST for updateOne
        DbProvider::Neon | DbProvider::Generic => reqwest::Method::PATCH,
    };
    
    // Send request
    let response = client
        .request(method, &path, Some(&body))
        .await?;
    
    // Check response status
    if response.status().is_success() {
        Ok(())
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(DbError {
            message: format!("Failed to update data: {} - {}", response.status(), error_text),
        })
    }
}

/// Deletes data from a table
///
/// # Arguments
///
/// * `client` - The database client
/// * `table` - Table name
/// * `row_id` - ID of the row to delete
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub async fn delete_db(client: &DbClient, table: &str, row_id: i64) -> Result<()> {
    // Format the delete parameters based on provider
    let (path, body_opt) = client.format_delete_params(table, row_id);
    
    // Determine the HTTP method based on provider
    let method = match client.provider {
        DbProvider::Supabase => reqwest::Method::DELETE,
        DbProvider::Firebase => reqwest::Method::DELETE,
        DbProvider::MongoDB => reqwest::Method::POST,  // MongoDB uses POST for deleteOne
        DbProvider::Neon | DbProvider::Generic => reqwest::Method::DELETE,
    };
    
    // Send request with or without body as needed
    let response = if let Some(body) = body_opt {
        client.request(method, &path, Some(&body)).await?
    } else {
        client.request(method, &path, Option::<&()>::None).await?
    };
    
    // Check response status
    if response.status().is_success() {
        Ok(())
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(DbError {
            message: format!("Failed to delete data: {} - {}", response.status(), error_text),
        })
    }
}

/// Deletes data from a table based on conditions
///
/// # Arguments
///
/// * `client` - The database client
/// * `table` - Table name
/// * `conditions` - Conditions for the delete (format: ["column=value", ...])
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub async fn delete_where(
    client: &DbClient,
    table: &str,
    conditions: &[&str],
) -> Result<()> {
    // For Supabase, we can do this in one request
    let mut path = client.get_table_path(table);
    
    let filters: Vec<String> = conditions.iter()
        .map(|c| {
            let parts: Vec<&str> = c.splitn(2, '=').collect();
            if parts.len() == 2 {
                format!("{}=eq.{}", parts[0].trim(), parts[1].trim().trim_matches('\'').trim_matches('"'))
            } else {
                String::new()
            }
        })
        .filter(|s| !s.is_empty())
        .collect();
    
    if !filters.is_empty() {
        path.push('?');
        path.push_str(&filters.join("&"));
        
        let response = client
            .request(reqwest::Method::DELETE, &path, Option::<&()>::None)
            .await?;
        
        if response.status().is_success() {
            return Ok(());
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(DbError {
                message: format!("Failed to delete data: {} - {}", response.status(), error_text),
            });
        }
    }
    
    // If no conditions were provided, we'll select all rows and delete them individually
    let rows = select_db(client, table, Some(conditions)).await?;
    
    // Delete each row by ID
    for row in rows {
        if let Some(id) = row.id {
            delete_db(client, table, id).await?;
        }
    }
    
    Ok(())
}