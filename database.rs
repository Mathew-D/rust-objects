/*
Made by: Mathew Dusome
June 11 2025
Adds a database module for interacting with Supabase or any REST API database

In your mod.rs file located in the modules folder add the following to the end of the file:
    pub mod database;

Add the following to Cargo.toml under [dependencies]:
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    
    [target.'cfg(target_arch = "wasm32")'.dependencies]
    wasm-bindgen = "0.2"
    wasm-bindgen-futures = "0.4"
    js-sys = "0.3"
    web-sys = { version = "0.3", features = [
    "Window", "Request", "RequestInit", "RequestMode",
    "Headers", "Response"
    ] }

    [target.'cfg(not(target_arch = "wasm32"))'.dependencies]
    ureq = { version = "2.9", features = ["json"] }

Add with the other use statements:
    use crate::modules::database::{DatabaseClient, DatabaseTable, create_database_client};

SETUP INSTRUCTIONS:
1. Update the SUPABASE_URL and SUPABASE_API_KEY constants below with your project details
2. Customize the DatabaseTable struct to match your database table structure
3. Set up Row Level Security (RLS) policies in Supabase (see SQL SETUP section below)
4. Create a client using create_database_client() or create_supabase_client()

SQL SETUP - Run these commands in your Supabase SQL Editor:
    -- Enable Row Level Security on your table
    ALTER TABLE public.your_table ENABLE ROW LEVEL SECURITY;
    
    -- Allow anonymous users to insert records
    CREATE POLICY allow_anon_insert
      ON public.your_table
      FOR INSERT
      TO anon
      WITH CHECK (true);

    -- Allow anonymous users to select/read records
    CREATE POLICY allow_anon_select
      ON public.your_table
      FOR SELECT
      TO anon
      USING (true);

    -- Allow anonymous users to update records
    CREATE POLICY allow_anon_update
      ON public.your_table
      FOR UPDATE
      TO anon
      USING (true)
      WITH CHECK (true);

    -- Allow anonymous users to delete records
    CREATE POLICY allow_anon_delete
      ON public.your_table
      FOR DELETE
      TO anon
      USING (true);

    -- IMPORTANT: Replace 'your_table' with your actual table name!
    -- Example: If your table is called 'messages', replace 'your_table' with 'messages'

BASIC USAGE:
    // Create a database client
    let client = create_database_client();
    
    // Or create with custom credentials
    let client = create_supabase_client("your-url", "your-key");

FETCH EXAMPLES:
    // Fetch all records from a table
    let records: Vec<DatabaseTable> = client.fetch_table("your_table_name").await?;
    
    // Fetch with custom query parameters
    let filtered_records: Vec<DatabaseTable> = client
        .fetch_table_with_query("users", "select=id,name&age=gte.18&order=name")
        .await?;
    
    // Fetch specific fields only
    let names: Vec<DatabaseTable> = client
        .fetch_table_with_query("users", "select=id,name")
        .await?;

INSERT EXAMPLES:
    // Insert a single record
    let new_record = DatabaseTable {
        id: None,  // Will be auto-generated
        text: "Hello World".to_string(),
    };
    let inserted: Vec<DatabaseTable> = client
        .insert_record("messages", &new_record)
        .await?;
    
    // Insert multiple records
    let records = vec![
        DatabaseTable { id: None, text: "First message".to_string() },
        DatabaseTable { id: None, text: "Second message".to_string() },
    ];
    let inserted: Vec<DatabaseTable> = client
        .insert_records("messages", &records)
        .await?;

UPDATE EXAMPLES:
    // Update a specific record by ID
    let updated_record = DatabaseTable {
        id: Some(1),
        text: "Updated message".to_string(),
    };
    let result: Vec<DatabaseTable> = client
        .update_record_by_id("messages", 1, &updated_record)
        .await?;
    
    // Update multiple records with custom filter
    let updates = DatabaseTable {
        id: None,
        text: "Bulk update".to_string(),
    };
    let result: Vec<DatabaseTable> = client
        .update_records("messages", "author_id=eq.5", &updates)
        .await?;
    
    // Update with complex filters
    let result: Vec<DatabaseTable> = client
        .update_records("posts", "published=eq.false&author_id=eq.10", &updates)
        .await?;

DELETE EXAMPLES:
    // Delete a specific record by ID
    let deleted: Vec<DatabaseTable> = client
        .delete_record_by_id::<DatabaseTable>("messages", 1)
        .await?;
    
    // Delete multiple records with custom filter
    let deleted: Vec<DatabaseTable> = client
        .delete_records::<DatabaseTable>("messages", "author_id=eq.5")
        .await?;
    
    // Delete with complex filters
    let deleted: Vec<DatabaseTable> = client
        .delete_records::<DatabaseTable>("posts", "published=eq.false&created_at=lt.2024-01-01")
        .await?;

ADVANCED FILTERING EXAMPLES:
    // Equal to
    "id=eq.1"
    
    // Greater than, less than
    "age=gte.18&age=lte.65"
    
    // Text search
    "name=ilike.*john*"
    
    // Multiple conditions
    "published=eq.true&author_id=eq.5&created_at=gte.2024-01-01"
    
    // Ordering and limiting
    "select=*&order=created_at.desc&limit=10"

ERROR HANDLING:
    match client.fetch_table::<DatabaseTable>("messages").await {
        Ok(records) => {
            println!("Found {} records", records.len());
            for record in records {
                println!("ID: {:?}, Text: {}", record.id, record.text);
            }
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
        }
    }

CUSTOM STRUCT EXAMPLE:
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct User {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<i32>,
        pub name: String,
        pub email: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub avatar_url: Option<String>,
    }
    
    // Use with any of the methods above
    let users: Vec<User> = client.fetch_table("users").await?;
*/

use serde::{Deserialize, Serialize};

// ============================================================================
// DATABASE SETUP SECTION - CUSTOMIZE FOR YOUR DATABASE
// ============================================================================

/// Configuration for your Supabase project
/// Update these constants with your actual Supabase project details
/// Example values shown below (replace with your own):
pub const SUPABASE_URL: &str = "https://xxxxxxxxxxxxxxxxx.supabase.co";
pub const SUPABASE_API_KEY: &str = "put_your_anon_key_here";

/// Generic database table struct - customize this for ANY table in your database
/// This single struct can be used for any table by adding/removing fields as needed
/// 
/// INSTRUCTIONS:
/// 1. Add your table's columns as fields below
/// 2. Use Option<T> for nullable database columns
/// 3. Add #[serde(skip_serializing_if = "Option::is_none")] for optional fields
/// 4. Remove fields you don't need, add fields you do need
/// 
/// EXAMPLES:
/// - For a messages table: keep id, text, add author, created_at
/// - For a users table: keep id, add name, email, avatar_url  
/// - For a posts table: keep id, add title, content, author_id, published
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseTable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    
    // TEXT FIELDS - rename/add/remove as needed for your table
    pub text: String,                    // Rename to: title, name, content, etc.
}


// ============================================================================
// DATABASE CLIENT IMPLEMENTATION - NO NEED TO MODIFY BELOW THIS LINE
// ============================================================================
/// Helper function to create a client with your Supabase configuration
/// This uses the constants defined above, or you can pass custom values
#[allow(unused)]
pub fn create_database_client() -> DatabaseClient {
    DatabaseClient::new(SUPABASE_URL.to_string(), SUPABASE_API_KEY.to_string())
}
#[allow(unused)]
/// Alternative helper to create client with custom credentials
pub fn create_supabase_client(project_url: &str, anon_key: &str) -> DatabaseClient {
    DatabaseClient::new(project_url.to_string(), anon_key.to_string())
}


pub struct DatabaseClient {
    base_url: String,
    api_key: String,
}

impl DatabaseClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self { base_url, api_key }
    }

    /// Fetch data from a table and return as a vector of the specified struct type
    /// Results are automatically ordered by ID for consistent ordering
    #[allow(unused)]
    pub async fn fetch_table<T>(&self, table: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/rest/v1/{}?select=*&order=id", self.base_url, table);
        let json_data = self.fetch_json(&url).await?;
        
        let parsed: Vec<T> = serde_json::from_str(&json_data)?;
        Ok(parsed)
    }

    /// Fetch data with custom query parameters
    #[allow(unused)]
    pub async fn fetch_table_with_query<T>(&self, table: &str, query: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/rest/v1/{}?{}", self.base_url, table, query);
        let json_data = self.fetch_json(&url).await?;
        
        let parsed: Vec<T> = serde_json::from_str(&json_data)?;
        Ok(parsed)
    }

    /// Generic method to fetch raw JSON data
    #[allow(unused)]
    pub async fn fetch_json(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        #[cfg(target_arch = "wasm32")]
        {
            self.fetch_json_web(url).await
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.fetch_json_native(url).await
        }
    }

    /// Web version using WASM bindings
    #[allow(unused)]
    #[cfg(target_arch = "wasm32")]
    async fn fetch_json_web(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        use wasm_bindgen_futures::JsFuture;
        use wasm_bindgen::JsCast;
        use web_sys::{Request, RequestInit, RequestMode, Headers, Response, window};

        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let headers = Headers::new().map_err(|_| "Failed to create headers")?;
        headers.append("apikey", &self.api_key).map_err(|_| "Failed to add apikey header")?;
        headers.append("Authorization", &format!("Bearer {}", self.api_key)).map_err(|_| "Failed to add Authorization header")?;
        headers.append("Content-Type", "application/json").map_err(|_| "Failed to add Content-Type header")?;
        opts.set_headers(&headers);

        let req = Request::new_with_str_and_init(url, &opts).map_err(|_| "Failed to create request")?;
        let win = window().ok_or("Failed to get window")?;
        let resp_value = JsFuture::from(win.fetch_with_request(&req)).await.map_err(|_| "Fetch failed")?;
        let resp: Response = resp_value.dyn_into().map_err(|_| "Failed to cast response")?;
        
        if !resp.ok() {
            return Err(format!("HTTP error: {}", resp.status()).into());
        }
        
        let text_value = JsFuture::from(resp.text().map_err(|_| "Failed to get text")?).await.map_err(|_| "Failed to read response text")?;
        text_value.as_string().ok_or("Failed to convert response to string".into())
    }

    /// Native version using ureq
    #[allow(unused)]
    #[cfg(not(target_arch = "wasm32"))]
    async fn fetch_json_native(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = ureq::get(url)
            .set("apikey", &self.api_key)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .call()?;

        let json_string = response.into_string()?;
        Ok(json_string)
    }

    /// Insert a record into a table
    #[allow(unused)]
    pub async fn insert_record<T>(&self, table: &str, record: &T) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let url = format!("{}/rest/v1/{}", self.base_url, table);
        let json_data = serde_json::to_string(record)?;
        let response_json = self.post_json(&url, &json_data).await?;
        
        // Parse the response to get the inserted record(s)
        let inserted_records: Vec<T> = serde_json::from_str(&response_json)?;
        Ok(inserted_records)
    }

    /// Insert multiple records into a table
    #[allow(unused)]
    pub async fn insert_records<T>(&self, table: &str, records: &[T]) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let url = format!("{}/rest/v1/{}", self.base_url, table);
        let json_data = serde_json::to_string(records)?;
        let response_json = self.post_json(&url, &json_data).await?;
        
        // Parse the response to get the inserted record(s)
        let inserted_records: Vec<T> = serde_json::from_str(&response_json)?;
        Ok(inserted_records)
    }

    /// Update records in a table based on a filter condition
    /// Example: update_records("users", "id=eq.1", &updated_user).await?;
    /// Example: update_records("posts", "author_id=eq.5&published=eq.false", &updates).await?;
    #[allow(unused)]
    pub async fn update_records<T>(&self, table: &str, filter: &str, record: &T) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        let url = format!("{}/rest/v1/{}?{}", self.base_url, table, filter);
        let json_data = serde_json::to_string(record)?;
        let response_json = self.patch_json(&url, &json_data).await?;
        
        // Parse the response to get the updated record(s)
        let updated_records: Vec<T> = serde_json::from_str(&response_json)?;
        Ok(updated_records)
    }

    /// Update a single record by ID
    /// This is a convenience method for the common case of updating by ID
    #[allow(unused)]
    pub async fn update_record_by_id<T>(&self, table: &str, id: i32, record: &T) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        self.update_records(table, &format!("id=eq.{}", id), record).await
    }

    /// Delete records from a table based on a filter condition
    /// Example: delete_records("users", "id=eq.1").await?;
    /// Example: delete_records("posts", "author_id=eq.5&published=eq.false").await?;
    #[allow(unused)]
    pub async fn delete_records<T>(&self, table: &str, filter: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/rest/v1/{}?{}", self.base_url, table, filter);
        let response_json = self.delete_json(&url).await?;
        
        // Parse the response to get the deleted record(s)
        let deleted_records: Vec<T> = serde_json::from_str(&response_json)?;
        Ok(deleted_records)
    }

    /// Delete a single record by ID
    /// This is a convenience method for the common case of deleting by ID
    #[allow(unused)]
    pub async fn delete_record_by_id<T>(&self, table: &str, id: i32) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.delete_records(table, &format!("id=eq.{}", id)).await
    }

    /// Generic method to post JSON data
    pub async fn post_json(&self, url: &str, json_data: &str) -> Result<String, Box<dyn std::error::Error>> {
        #[cfg(target_arch = "wasm32")]
        {
            self.post_json_web(url, json_data).await
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.post_json_native(url, json_data).await
        }
    }

    /// Generic method to patch JSON data (for updates)
    pub async fn patch_json(&self, url: &str, json_data: &str) -> Result<String, Box<dyn std::error::Error>> {
        #[cfg(target_arch = "wasm32")]
        {
            self.patch_json_web(url, json_data).await
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.patch_json_native(url, json_data).await
        }
    }

    /// Web version using WASM bindings for POST requests
    #[allow(unused)]
    #[cfg(target_arch = "wasm32")]
    async fn post_json_web(&self, url: &str, json_body: &str) -> Result<String, Box<dyn std::error::Error>> {
        use wasm_bindgen_futures::JsFuture;
        use wasm_bindgen::JsCast;
        use web_sys::{Request, RequestInit, RequestMode, Headers, Response, window};

        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(RequestMode::Cors);
        opts.set_body(&wasm_bindgen::JsValue::from_str(json_body));

        let headers = Headers::new().map_err(|_| "Failed to create headers")?;
        headers.append("apikey", &self.api_key).map_err(|_| "Failed to add apikey header")?;
        headers.append("Authorization", &format!("Bearer {}", self.api_key)).map_err(|_| "Failed to add Authorization header")?;
        headers.append("Content-Type", "application/json").map_err(|_| "Failed to add Content-Type header")?;
        headers.append("Prefer", "return=representation").map_err(|_| "Failed to add Prefer header")?;
        opts.set_headers(&headers);

        let req = Request::new_with_str_and_init(url, &opts).map_err(|_| "Failed to create request")?;
        let win = window().ok_or("Failed to get window")?;
        let resp_value = JsFuture::from(win.fetch_with_request(&req)).await.map_err(|_| "POST failed")?;
        let resp: Response = resp_value.dyn_into().map_err(|_| "Failed to cast response")?;
        
        if !resp.ok() {
            return Err(format!("HTTP error: {}", resp.status()).into());
        }
        
        let text_value = JsFuture::from(resp.text().map_err(|_| "Failed to get text")?).await.map_err(|_| "Failed to read response text")?;
        text_value.as_string().ok_or("Failed to convert response to string".into())
    }

    /// Native version using ureq for POST requests
    #[allow(unused)]
    #[cfg(not(target_arch = "wasm32"))]
    async fn post_json_native(&self, url: &str, json_body: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = ureq::post(url)
            .set("apikey", &self.api_key)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .set("Prefer", "return=representation")
            .send_string(json_body);

        match response {
            Ok(resp) => {
                let json_string = resp.into_string()?;
                Ok(json_string)
            }
            Err(ureq::Error::Status(code, response)) => {
                let error_body = response.into_string().unwrap_or_else(|_| "Could not read error body".to_string());
                Err(format!("HTTP {} error: {}", code, error_body).into())
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }

    /// Web version using WASM bindings for PATCH requests
    #[allow(unused)]
    #[cfg(target_arch = "wasm32")]
    async fn patch_json_web(&self, url: &str, json_body: &str) -> Result<String, Box<dyn std::error::Error>> {
        use wasm_bindgen_futures::JsFuture;
        use wasm_bindgen::JsCast;
        use web_sys::{Request, RequestInit, RequestMode, Headers, Response, window};

        let opts = RequestInit::new();
        opts.set_method("PATCH");
        opts.set_mode(RequestMode::Cors);
        opts.set_body(&wasm_bindgen::JsValue::from_str(json_body));

        let headers = Headers::new().map_err(|_| "Failed to create headers")?;
        headers.append("apikey", &self.api_key).map_err(|_| "Failed to add apikey header")?;
        headers.append("Authorization", &format!("Bearer {}", self.api_key)).map_err(|_| "Failed to add Authorization header")?;
        headers.append("Content-Type", "application/json").map_err(|_| "Failed to add Content-Type header")?;
        headers.append("Prefer", "return=representation").map_err(|_| "Failed to add Prefer header")?;
        opts.set_headers(&headers);

        let req = Request::new_with_str_and_init(url, &opts).map_err(|_| "Failed to create request")?;
        let win = window().ok_or("Failed to get window")?;
        let resp_value = JsFuture::from(win.fetch_with_request(&req)).await.map_err(|_| "PATCH failed")?;
        let resp: Response = resp_value.dyn_into().map_err(|_| "Failed to cast response")?;
        
        if !resp.ok() {
            return Err(format!("HTTP error: {}", resp.status()).into());
        }
        
        let text_value = JsFuture::from(resp.text().map_err(|_| "Failed to get text")?).await.map_err(|_| "Failed to read response text")?;
        text_value.as_string().ok_or("Failed to convert response to string".into())
    }

    /// Native version using ureq for PATCH requests
    #[allow(unused)]
    #[cfg(not(target_arch = "wasm32"))]
    async fn patch_json_native(&self, url: &str, json_body: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = ureq::patch(url)
            .set("apikey", &self.api_key)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .set("Prefer", "return=representation")
            .send_string(json_body);

        match response {
            Ok(resp) => {
                let json_string = resp.into_string()?;
                Ok(json_string)
            }
            Err(ureq::Error::Status(code, response)) => {
                let error_body = response.into_string().unwrap_or_else(|_| "Could not read error body".to_string());
                Err(format!("HTTP {} error: {}", code, error_body).into())
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }

    /// Generic method to delete JSON data
    pub async fn delete_json(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        #[cfg(target_arch = "wasm32")]
        {
            self.delete_json_web(url).await
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.delete_json_native(url).await
        }
    }

    /// Web version using WASM bindings for DELETE requests
    #[allow(unused)]
    #[cfg(target_arch = "wasm32")]
    async fn delete_json_web(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        use wasm_bindgen_futures::JsFuture;
        use wasm_bindgen::JsCast;
        use web_sys::{Request, RequestInit, RequestMode, Headers, Response, window};

        let opts = RequestInit::new();
        opts.set_method("DELETE");
        opts.set_mode(RequestMode::Cors);

        let headers = Headers::new().map_err(|_| "Failed to create headers")?;
        headers.append("apikey", &self.api_key).map_err(|_| "Failed to add apikey header")?;
        headers.append("Authorization", &format!("Bearer {}", self.api_key)).map_err(|_| "Failed to add Authorization header")?;
        headers.append("Content-Type", "application/json").map_err(|_| "Failed to add Content-Type header")?;
        opts.set_headers(&headers);

        let req = Request::new_with_str_and_init(url, &opts).map_err(|_| "Failed to create request")?;
        let win = window().ok_or("Failed to get window")?;
        let resp_value = JsFuture::from(win.fetch_with_request(&req)).await.map_err(|_| "DELETE failed")?;
        let resp: Response = resp_value.dyn_into().map_err(|_| "Failed to cast response")?;
        
        if !resp.ok() {
            return Err(format!("HTTP error: {}", resp.status()).into());
        }
        
        let text_value = JsFuture::from(resp.text().map_err(|_| "Failed to get text")?).await.map_err(|_| "Failed to read response text")?;
        text_value.as_string().ok_or("Failed to convert response to string".into())
    }

    /// Native version using ureq for DELETE requests
    #[allow(unused)]
    #[cfg(not(target_arch = "wasm32"))]
    async fn delete_json_native(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = ureq::delete(url)
            .set("apikey", &self.api_key)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .set("Prefer", "return=representation")
            .call();

        match response {
            Ok(resp) => {
                let json_string = resp.into_string()?;
                Ok(json_string)
            }
            Err(ureq::Error::Status(code, response)) => {
                let error_body = response.into_string().unwrap_or_else(|_| "Could not read error body".to_string());
                Err(format!("HTTP {} error: {}", code, error_body).into())
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }
}