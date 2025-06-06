/*
Made by: Mathew Dusome
June 6 2025

Adds database functionality for both web and native platforms using Supabase as the cloud database.
This module provides a simple and consistent API for storing and retrieving data across different platforms.

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
suparust = { version = "0.3.0", features = ["rustls"] }

# For web builds, add this section:
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["console"] }
```

The rustls feature works well for both web and native platforms, providing secure TLS connections.

⚠️ IMPORTANT: 
- To use Supabase, you must first create an account and project at https://supabase.com
- Tables must be created through the Supabase web dashboard
- Authentication settings are configured through the Supabase web dashboard

Basic example of how to use this module:

```rust
// 1. Create a connection to your Supabase database
let client = create_connection(
    "https://your-project.supabase.co",
    "your-api-key".to_string()
);

// 2. Insert data into the database
// First define a struct matching your table structure
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Person {
    id: Option<i64>,  // Optional as Supabase will generate this
    firstname: String,
    lastname: String,
    age: i32
}

// Create a new person
let new_person = Person {
    id: None,
    firstname: "John".to_string(),
    lastname: "Doe".to_string(),
    age: 30
};

// Insert the person
let inserted_id = insert_one(&client, "people", &new_person).await
    .expect("Failed to insert data");
println!("Inserted person with ID: {}", inserted_id);

// 3. Fetch data from the database
let people: Vec<Person> = select::<Person>(&client, "people", None).await
    .expect("Failed to get people");

// 4. Display the data
for person in people {
    println!("Name: {} {}, Age: {}", 
             person.firstname, person.lastname, person.age);
}

// 5. Update a record
let updated_person = Person {
    id: Some(1),
    firstname: "John".to_string(),
    lastname: "Doe".to_string(),
    age: 31  // Updated age
};
update_one(&client, "people", 1, &updated_person).await
    .expect("Failed to update person");

// 6. Delete a record
delete_one(&client, "people", 1).await
    .expect("Failed to delete person");

// 7. Filter data with conditions
let filtered_people: Vec<Person> = select_where::<Person>(
    &client, 
    "people", 
    |query| query.eq("lastname", "Doe").gte("age", 30)
).await.expect("Failed to query filtered people");
```
*/

use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt;

// Re-export suparust types we use
pub use suparust::{
    Supabase, 
    SessionChangeListener,
    AuthResponse,
    User,
    Session,
    storage::object::{Object, ListRequest, SortOrder, GetResponse}
};

// Web-specific imports for better error handling
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

impl From<suparust::Error> for DbError {
    fn from(error: suparust::Error) -> Self {
        DbError {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for DbError {
    fn from(error: serde_json::Error) -> Self {
        DbError {
            message: format!("JSON error: {}", error),
        }
    }
}

// Define the Result type for our database operations
pub type Result<T> = std::result::Result<T, DbError>;

/// Creates a connection to the Supabase database
///
/// # Arguments
///
/// * `base_url` - The Supabase base URL (e.g., "https://your-project.supabase.co")
/// * `api_key` - The Supabase API key for authentication
///
/// # Returns
///
/// * `Supabase` - The Supabase client
pub fn create_connection(base_url: &str, api_key: String) -> Supabase {
    Supabase::new(
        base_url,
        &api_key,
        None, // No session initially
        SessionChangeListener::Ignore // Ignore session changes (you can change this for auth)
    )
}

/// Inserts a record into a table
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `table` - Table name
/// * `data` - The data to insert, must match table structure
///
/// # Returns
///
/// * `Result<i64>` - Row ID or error
pub async fn insert_one<T: Serialize>(client: &Supabase, table: &str, data: &T) -> Result<i64> {
    let response = client
        .from(table)
        .await?
        .insert(serde_json::to_string(data)?)
        .execute()
        .await?;
    
    let json: serde_json::Value = response.json().await?;
    
    // Extract the ID
    if let Some(rows) = json.as_array() {
        if let Some(first_row) = rows.first() {
            if let Some(id) = first_row.get("id").and_then(|v| v.as_i64()) {
                return Ok(id);
            }
        }
    }
    
    Err(DbError {
        message: "Failed to get ID from inserted row".to_string(),
    })
}

/// Inserts multiple records into a table
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `table` - Table name
/// * `data` - The data array to insert, elements must match table structure
///
/// # Returns
///
/// * `Result<Vec<i64>>` - Row IDs or error
pub async fn insert_many<T: Serialize>(client: &Supabase, table: &str, data: &[T]) -> Result<Vec<i64>> {
    let json_data = serde_json::to_string(data)?;
    
    let response = client
        .from(table)
        .await?
        .insert(json_data)
        .execute()
        .await?;
    
    let json: serde_json::Value = response.json().await?;
    
    // Extract the IDs
    if let Some(rows) = json.as_array() {
        let mut ids = Vec::new();
        
        for row in rows {
            if let Some(id) = row.get("id").and_then(|v| v.as_i64()) {
                ids.push(id);
            }
        }
        
        return Ok(ids);
    }
    
    Err(DbError {
        message: "Failed to get IDs from inserted rows".to_string(),
    })
}

/// Selects data from a table
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `table` - Table name
/// * `select_columns` - Optional columns to select (use None for all columns)
///
/// # Returns
///
/// * `Result<Vec<T>>` - Result rows or error
pub async fn select<T: for<'a> Deserialize<'a>>(
    client: &Supabase, 
    table: &str,
    select_columns: Option<&str>
) -> Result<Vec<T>> {
    let query_builder = client.from(table).await?;
    
    let query = match select_columns {
        Some(columns) => query_builder.select(columns),
        None => query_builder.select("*"),
    };
    
    let response = query.execute().await?;
    let result: Vec<T> = response.json().await?;
    
    Ok(result)
}

/// Selects data from a table with specific conditions
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `table` - Table name
/// * `query_builder` - Function to build the query with conditions
///
/// # Returns
///
/// * `Result<Vec<T>>` - Result rows or error
pub async fn select_where<T, F>(
    client: &Supabase, 
    table: &str,
    query_builder: F
) -> Result<Vec<T>> 
where 
    T: for<'a> Deserialize<'a>,
    F: FnOnce(&mut postgrest::Builder) -> &mut postgrest::Builder
{
    let mut builder = client.from(table).await?;
    
    // Apply the custom filtering
    query_builder(&mut builder);
    
    let response = builder.select("*").execute().await?;
    let result: Vec<T> = response.json().await?;
    
    Ok(result)
}

/// Gets a single record by ID
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `table` - Table name
/// * `id` - The ID of the record to get
///
/// # Returns
///
/// * `Result<T>` - The record or error
pub async fn get_by_id<T: for<'a> Deserialize<'a>>(
    client: &Supabase, 
    table: &str,
    id: i64
) -> Result<T> {
    let response = client
        .from(table)
        .await?
        .eq("id", id.to_string())
        .select("*")
        .execute()
        .await?;
    
    let mut rows: Vec<T> = response.json().await?;
    
    if rows.is_empty() {
        return Err(DbError {
            message: format!("No record found with id {}", id),
        });
    }
    
    Ok(rows.remove(0))
}

/// Updates a record by ID
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `table` - Table name
/// * `id` - The ID of the record to update
/// * `data` - The data to update with
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub async fn update_one<T: Serialize>(
    client: &Supabase,
    table: &str,
    id: i64,
    data: &T,
) -> Result<()> {
    let json_data = serde_json::to_string(data)?;
    
    let response = client
        .from(table)
        .await?
        .eq("id", id.to_string())
        .update(json_data)
        .execute()
        .await?;
    
    // Check if the response was successful
    if response.status().is_success() {
        Ok(())
    } else {
        Err(DbError {
            message: format!("Failed to update record: {}", response.status()),
        })
    }
}

/// Deletes a record by ID
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `table` - Table name
/// * `id` - The ID of the record to delete
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub async fn delete_one(
    client: &Supabase,
    table: &str,
    id: i64,
) -> Result<()> {
    let response = client
        .from(table)
        .await?
        .eq("id", id.to_string())
        .delete()
        .execute()
        .await?;
    
    // Check if the response was successful
    if response.status().is_success() {
        Ok(())
    } else {
        Err(DbError {
            message: format!("Failed to delete record: {}", response.status()),
        })
    }
}

/// Deletes records matching specific conditions
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `table` - Table name
/// * `query_builder` - Function to build the query with conditions
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub async fn delete_where<F>(
    client: &Supabase,
    table: &str,
    query_builder: F,
) -> Result<()>
where
    F: FnOnce(&mut postgrest::Builder) -> &mut postgrest::Builder
{
    let mut builder = client.from(table).await?;
    
    // Apply the custom filtering
    query_builder(&mut builder);
    
    let response = builder.delete().execute().await?;
    
    // Check if the response was successful
    if response.status().is_success() {
        Ok(())
    } else {
        Err(DbError {
            message: format!("Failed to delete records: {}", response.status()),
        })
    }
}

// ---------- Authentication Functions ----------

/// Registers a new user with email and password
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `email` - User's email
/// * `password` - User's password
///
/// # Returns
///
/// * `Result<AuthResponse>` - Authentication response or error
pub async fn register_user(
    client: &Supabase,
    email: &str,
    password: &str,
) -> Result<AuthResponse> {
    let response = client.signup_with_email(email, password).await?;
    Ok(response)
}

/// Logs in a user with email and password
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `email` - User's email
/// * `password` - User's password
///
/// # Returns
///
/// * `Result<Session>` - Session data or error
pub async fn login_user(
    client: &Supabase,
    email: &str,
    password: &str,
) -> Result<Session> {
    let response = client.login_with_email(email, password).await?;
    Ok(response)
}

/// Logs out the current user
///
/// # Arguments
///
/// * `client` - The Supabase client
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub async fn logout_user(client: &Supabase) -> Result<()> {
    client.logout().await?;
    Ok(())
}

/// Gets the current user if logged in
///
/// # Arguments
///
/// * `client` - The Supabase client
///
/// # Returns
///
/// * `Result<Option<User>>` - User data or error
pub async fn get_current_user(client: &Supabase) -> Result<Option<User>> {
    if let Some(session) = client.get_session().await? {
        Ok(Some(session.user))
    } else {
        Ok(None)
    }
}

// ---------- Storage Functions ----------

/// Uploads a file to Supabase Storage
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `bucket` - The storage bucket name
/// * `path` - The file path in storage
/// * `data` - The file data as bytes
/// * `content_type` - Optional content type (MIME type)
///
/// # Returns
///
/// * `Result<String>` - File URL or error
pub async fn upload_file(
    client: &Supabase,
    bucket: &str,
    path: &str,
    data: &[u8],
    content_type: Option<&str>,
) -> Result<String> {
    let storage = client.storage().await?;
    
    let key = storage
        .object()
        .upload(bucket, path, data, content_type)
        .await?;
    
    Ok(key)
}

/// Gets a file from Supabase Storage
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `bucket` - The storage bucket name
/// * `path` - The file path in storage
///
/// # Returns
///
/// * `Result<GetResponse>` - The file data and metadata or error
pub async fn get_file(
    client: &Supabase,
    bucket: &str,
    path: &str,
) -> Result<GetResponse> {
    let storage = client.storage().await?;
    
    let file = storage
        .object()
        .get_one(bucket, path)
        .await?;
    
    Ok(file)
}

/// Lists files in a Supabase Storage bucket
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `bucket` - The storage bucket name
/// * `path` - Optional directory path to list
///
/// # Returns
///
/// * `Result<Vec<Object>>` - List of objects or error
pub async fn list_files(
    client: &Supabase,
    bucket: &str,
    path: &str,
) -> Result<Vec<Object>> {
    let storage = client.storage().await?;
    
    let request = ListRequest::new(path.to_string());
    
    let objects = storage
        .object()
        .list(bucket, request)
        .await?;
    
    Ok(objects)
}

/// Deletes a file from Supabase Storage
///
/// # Arguments
///
/// * `client` - The Supabase client
/// * `bucket` - The storage bucket name
/// * `path` - The file path in storage
///
/// # Returns
///
/// * `Result<()>` - Success or error
pub async fn delete_file(
    client: &Supabase,
    bucket: &str,
    path: &str,
) -> Result<()> {
    let storage = client.storage().await?;
    
    storage
        .object()
        .delete(bucket, path)
        .await?;
    
    Ok(())
}
