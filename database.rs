/*
Made by: Mathew Dusome
April 27 2025

Adds database functionality for both web and native platforms using a cloud-hosted database
In the mod objects section add:
    pub mod database;
    
Then add the following with the use commands:
use crate::objects::database::*;  // This imports all functions directly

To use this module, add these dependencies to your Cargo.toml file:

```toml
[dependencies]
# Add these to your existing dependencies:
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }

# For web builds, add this section:
[target.'cfg(target_arch = "wasm32")'.dependencies]
reqwest = { version = "0.11", features = ["json", "wasm-client"] }
wasm-bindgen-futures = "0.4"
```

Basic example of how to use this in your code:

// 1. Create a connection to your cloud database 
//    Change the provider type based on which service you're using:
//    - DbProvider::Supabase
//    - DbProvider::Firebase
//    - DbProvider::MongoDB
//    - DbProvider::Neon
//    - DbProvider::Generic (for other REST APIs)
let client = create_connection(
    DbProvider::Supabase,
    "https://your-project.supabase.co/rest/v1",
    Some("your-api-key".to_string())
);

// 2. Create a table (usually done once at the start)
create_table(&client, "game_scores", &["player TEXT", "score INTEGER"]).await
    .expect("Failed to create table");

// 3. Insert data into the database
let columns = &["player", "score"];
let values = &["Player1", "500"];
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
    
    println!("{}: {}", player, points);
}

// 6. Update a record
update_db(&client, "game_scores", 1, &["score=600"]).await
    .expect("Failed to update score");

// 7. Delete a record
delete_db(&client, "game_scores", 1).await
    .expect("Failed to delete score");
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;

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

/// Supported database providers
#[derive(Debug, Clone, Copy)]
pub enum DbProvider {
    /// Supabase REST API
    Supabase,
    /// Firebase Realtime Database
    Firebase,
    /// MongoDB Atlas Data API
    MongoDB,
    /// Neon Postgres
    Neon,
    /// Generic REST API that follows our format
    Generic,
}

// The Database client represents the connection to our cloud database
#[derive(Clone)]
pub struct DbClient {
    provider: DbProvider,
    base_url: String,
    api_key: Option<String>,
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
    /// Creates a new database client
    ///
    /// # Arguments
    ///
    /// * `provider` - The database provider to use
    /// * `base_url` - The base URL for the database service
    /// * `api_key` - Optional API key for authentication
    ///
    /// # Returns
    ///
    /// * `DbClient` - The database client
    pub fn new(provider: DbProvider, base_url: &str, api_key: Option<String>) -> Self {
        Self {
            provider,
            base_url: base_url.to_string(),
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Makes an authenticated request to the database API
    pub(crate) async fn request<T: Serialize + ?Sized>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<reqwest::Response> {
        let url = format!("{}{}", self.base_url, path);
        let mut req_builder = self.client.request(method, &url);
        
        // Add appropriate authentication based on provider
        match self.provider {
            DbProvider::Supabase => {
                if let Some(key) = &self.api_key {
                    req_builder = req_builder
                        .header("apikey", key)
                        .header("Authorization", format!("Bearer {}", key));
                }
            },
            DbProvider::Firebase => {
                // Firebase typically uses query parameters for auth
                if let Some(key) = &self.api_key {
                    let url = if url.contains('?') {
                        format!("{}&auth={}", url, key)
                    } else {
                        format!("{}?auth={}", url, key)
                    };
                    req_builder = self.client.request(method, &url);
                }
            },
            DbProvider::MongoDB => {
                if let Some(key) = &self.api_key {
                    req_builder = req_builder.header("api-key", key);
                }
            },
            DbProvider::Neon | DbProvider::Generic => {
                if let Some(key) = &self.api_key {
                    req_builder = req_builder.header("Authorization", format!("Bearer {}", key));
                }
            },
        }
        
        // Add the request body if provided
        if let Some(data) = body {
            req_builder = req_builder.json(data);
        }
        
        req_builder
            .send()
            .await
            .map_err(|e| DbError { message: e.to_string() })
    }
    
    /// Get the correct path for a table operation based on provider
    fn get_table_path(&self, table: &str) -> String {
        match self.provider {
            DbProvider::Supabase => format!("/{}", table),
            DbProvider::Firebase => format!("/{}.json", table),
            DbProvider::MongoDB => "/action/find".to_string(), // MongoDB uses action endpoints
            DbProvider::Neon => format!("/tables/{}", table),  // Would use SQL in a real implementation
            DbProvider::Generic => format!("/tables/{}", table),
        }
    }
    
    /// Get the correct path for a row operation based on provider
    fn get_row_path(&self, table: &str, row_id: Option<i64>) -> String {
        match self.provider {
            DbProvider::Supabase => {
                if let Some(id) = row_id {
                    format!("/{}?id=eq.{}", table, id)
                } else {
                    format!("/{}", table)
                }
            },
            DbProvider::Firebase => {
                if let Some(id) = row_id {
                    format!("/{}/{}.json", table, id)
                } else {
                    format!("/{}.json", table)
                }
            },
            DbProvider::MongoDB => "/action/updateOne".to_string(),  // For MongoDB
            DbProvider::Neon => {
                if let Some(id) = row_id {
                    format!("/tables/{}/rows/{}", table, id)
                } else {
                    format!("/tables/{}/rows", table)
                }
            },
            DbProvider::Generic => {
                if let Some(id) = row_id {
                    format!("/tables/{}/rows/{}", table, id)
                } else {
                    format!("/tables/{}/rows", table)
                }
            },
        }
    }
    
    /// Format the request body for creating tables based on provider
    pub(crate) fn format_create_table_body(&self, table: &str, schema: &HashMap<String, String>) -> serde_json::Value {
        match self.provider {
            DbProvider::Supabase => {
                // Supabase would typically use SQL for creating tables
                // This is simplified for the API approach
                serde_json::json!({
                    "name": table,
                    "columns": schema
                })
            },
            DbProvider::Firebase => {
                // Firebase doesn't require explicit schema creation
                serde_json::json!({})
            },
            DbProvider::MongoDB => {
                // MongoDB Atlas has a createCollection operation
                serde_json::json!({
                    "collection": table
                })
            },
            DbProvider::Neon | DbProvider::Generic => {
                serde_json::json!({
                    "table": table,
                    "schema": schema
                })
            },
        }
    }
    
    /// Format the data for insert based on provider
    pub(crate) fn format_insert_body(&self, table: &str, data: &HashMap<String, String>) -> serde_json::Value {
        match self.provider {
            DbProvider::Supabase => {
                // Supabase directly accepts the data object
                serde_json::json!(data)
            },
            DbProvider::Firebase => {
                // Firebase directly accepts the data
                serde_json::json!(data)
            },
            DbProvider::MongoDB => {
                // MongoDB requires a specific format
                serde_json::json!({
                    "collection": table,
                    "document": data
                })
            },
            DbProvider::Neon | DbProvider::Generic => {
                serde_json::json!({ "data": data })
            },
        }
    }
    
    /// Format the query parameters for select based on provider
    pub(crate) fn format_select_params(&self, table: &str, conditions: Option<&[&str]>) -> (String, Option<serde_json::Value>) {
        if conditions.is_none() || conditions.unwrap().is_empty() {
            match self.provider {
                DbProvider::Supabase => (self.get_table_path(table), None),
                DbProvider::Firebase => (self.get_row_path(table, None), None),
                DbProvider::MongoDB => {
                    (
                        "/action/find".to_string(),
                        Some(serde_json::json!({
                            "collection": table,
                            "filter": {}
                        }))
                    )
                },
                DbProvider::Neon | DbProvider::Generic => (self.get_row_path(table, None), None),
            }
        } else {
            let conditions = conditions.unwrap();
            match self.provider {
                DbProvider::Supabase => {
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
                },
                DbProvider::Firebase => {
                    // Firebase uses query parameters for filtering
                    // For complex queries, you'd use orderBy and other params
                    (self.get_row_path(table, None), None)
                },
                DbProvider::MongoDB => {
                    // Build MongoDB filter
                    let mut filter = serde_json::Map::new();
                    for condition in conditions {
                        let parts: Vec<&str> = condition.splitn(2, '=').collect();
                        if parts.len() == 2 {
                            let key = parts[0].trim();
                            let value = parts[1].trim().trim_matches('\'').trim_matches('"');
                            filter.insert(key.to_string(), serde_json::Value::String(value.to_string()));
                        }
                    }
                    
                    (
                        "/action/find".to_string(),
                        Some(serde_json::json!({
                            "collection": table,
                            "filter": filter
                        }))
                    )
                },
                DbProvider::Neon | DbProvider::Generic => {
                    let mut query_params = Vec::new();
                    
                    for cond in conditions {
                        let parts: Vec<&str> = cond.splitn(2, '=').collect();
                        if parts.len() == 2 {
                            let col = parts[0].trim();
                            let val = parts[1].trim().trim_matches('\'').trim_matches('"');
                            query_params.push(format!("filter[{}]={}", col, val));
                        }
                    }
                    
                    let path = self.get_row_path(table, None);
                    let query_string = if !query_params.is_empty() {
                        format!("{}?{}", path, query_params.join("&"))
                    } else {
                        path
                    };
                    
                    (query_string, None)
                },
            }
        }
    }
    
    /// Format the update body based on provider
    pub(crate) fn format_update_body(&self, table: &str, row_id: i64, updates: &HashMap<String, String>) -> (String, serde_json::Value) {
        match self.provider {
            DbProvider::Supabase => {
                (
                    format!("/{}?id=eq.{}", table, row_id),
                    serde_json::json!(updates)
                )
            },
            DbProvider::Firebase => {
                (
                    self.get_row_path(table, Some(row_id)),
                    serde_json::json!(updates)
                )
            },
            DbProvider::MongoDB => {
                (
                    "/action/updateOne".to_string(),
                    serde_json::json!({
                        "collection": table,
                        "filter": { "_id": row_id },
                        "update": { "$set": updates }
                    })
                )
            },
            DbProvider::Neon | DbProvider::Generic => {
                (
                    self.get_row_path(table, Some(row_id)),
                    serde_json::json!({ "data": updates })
                )
            },
        }
    }
    
    /// Format the delete parameters based on provider
    pub(crate) fn format_delete_params(&self, table: &str, row_id: i64) -> (String, Option<serde_json::Value>) {
        match self.provider {
            DbProvider::Supabase => {
                (format!("/{}?id=eq.{}", table, row_id), None)
            },
            DbProvider::Firebase => {
                (self.get_row_path(table, Some(row_id)), None)
            },
            DbProvider::MongoDB => {
                (
                    "/action/deleteOne".to_string(),
                    Some(serde_json::json!({
                        "collection": table,
                        "filter": { "_id": row_id }
                    }))
                )
            },
            DbProvider::Neon | DbProvider::Generic => {
                (self.get_row_path(table, Some(row_id)), None)
            },
        }
    }
    
    /// Parse the response based on provider
    pub(crate) async fn parse_select_response(&self, response: reqwest::Response) -> Result<Vec<Row>> {
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| DbError { message: e.to_string() })?;
        
        match self.provider {
            DbProvider::Supabase => {
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
            },
            DbProvider::Firebase => {
                // Firebase returns an object where keys are IDs and values are the data
                if let Some(obj) = json.as_object() {
                    let mut result = Vec::new();
                    
                    for (key, value) in obj {
                        if let Some(data_obj) = value.as_object() {
                            let mut row = Row {
                                id: key.parse::<i64>().ok(),
                                data: HashMap::new(),
                            };
                            
                            for (k, v) in data_obj {
                                row.data.insert(k.clone(), v.clone());
                            }
                            
                            result.push(row);
                        }
                    }
                    
                    Ok(result)
                } else {
                    Ok(Vec::new())
                }
            },
            DbProvider::MongoDB => {
                // MongoDB Data API returns documents in a specific format
                if let Some(documents) = json.get("documents").and_then(|d| d.as_array()) {
                    let mut result = Vec::new();
                    
                    for doc in documents {
                        if let Some(doc_obj) = doc.as_object() {
                            let mut row = Row {
                                id: doc_obj.get("_id").and_then(|v| v.as_i64()),
                                data: HashMap::new(),
                            };
                            
                            for (key, value) in doc_obj {
                                if key != "_id" {
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
            },
            DbProvider::Neon | DbProvider::Generic => {
                // For the generic format we assume a "rows" array
                if let Some(rows) = json.get("rows").and_then(|r| r.as_array()) {
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
            },
        }
    }
}

/// Creates a connection to the database
///
/// # Arguments
///
/// * `provider` - The database provider to use
/// * `db_url` - The database service URL
/// * `api_key` - Optional API key for authentication
///
/// # Returns
///
/// * `DbClient` - The database client
pub fn create_connection(provider: DbProvider, db_url: &str, api_key: Option<String>) -> DbClient {
    DbClient::new(provider, db_url, api_key)
}

/// Creates a table in the database
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
    
    // Get the correct path and format the body for this provider
    let path = match client.provider {
        DbProvider::Supabase => "/rpc/create_table", // Supabase often uses RPC for schema changes
        DbProvider::Firebase => "", // Firebase doesn't need explicit creation
        DbProvider::MongoDB => "/action/createCollection",
        DbProvider::Neon => "/tables",
        DbProvider::Generic => "/tables",
    };
    
    // Skip for Firebase as it doesn't require explicit table creation
    if client.provider == DbProvider::Firebase {
        return Ok(());
    }
    
    // Format the request body based on provider
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
    // For MongoDB, we can do this in one request
    if client.provider == DbProvider::MongoDB {
        // Build MongoDB filter
        let mut filter = serde_json::Map::new();
        for condition in conditions {
            let parts: Vec<&str> = condition.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim().trim_matches('\'').trim_matches('"');
                filter.insert(key.to_string(), serde_json::Value::String(value.to_string()));
            }
        }
        
        let body = serde_json::json!({
            "collection": table,
            "filter": filter
        });
        
        let response = client
            .request(reqwest::Method::POST, "/action/deleteMany", Some(&body))
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
    
    // For Supabase, we can also do it in one request
    if client.provider == DbProvider::Supabase {
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
    }
    
    // For other providers, we first select the matching rows to get their IDs, then delete each one
    let rows = select_db(client, table, Some(conditions)).await?;
    
    // Delete each row by ID
    for row in rows {
        if let Some(id) = row.id {
            delete_db(client, table, id).await?;
        }
    }
    
    Ok(())
}