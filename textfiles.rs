/*
Made by: Mathew Dusome
April 30 2025
Updated: December 17 2025
To import you need:
Adds TextFile functionality for cross-platform file operations

For web support (WebAssembly) only:
    add into Cargo.toml the following:
        [target.'cfg(target_arch = "wasm32")'.dependencies]
        gloo-storage = "0.3"
        wasm-bindgen = "0.2"


        
Used for Everything:

    In your mod.rs file located in the modules folder add the following to the end of the file
        pub mod textfiles;
        
    Add with the other use statements
        use crate::modules::textfiles::TextFile;

 

Simple examples:

1. Save different data to separate files:

    // Save string data (player names)
    let names = vec!["Alice", "Bob", "Charlie"];
    let result = TextFile::save_strings("player_names.txt", names).await;
    if let Err(e) = result {
        println!("Error saving names: {}", e);
    }
    
    // Save integer data (scores)
    let scores = vec![100, 85, 92];
    let result = TextFile::save_numbers("high_scores.txt", scores).await;
    if let Err(e) = result {
        println!("Error saving scores: {}", e);
    }
    

2. Load different data from separate files:

    // Load player names
    let result = TextFile::load_strings("player_names.txt").await;
    Without error checking:
    let names: Vec<String> = result.unwrap_or_default();

    With error checking:
    if let Ok(names) = result {
        for name in names {
            println!("Player: {}", name);
        }
    } else if let Err(e) = result {
        println!("Error loading names: {}", e);
    }
    
    // Load high scores
    let result = TextFile::load_numbers::<i32>("high_scores.txt").await;
    if let Ok(scores) = result {
        for score in scores {
            println!("Score: {}", score);
        }
    } else if let Err(e) = result {
        println!("Error loading scores: {}", e);
    }
    

3. Load game configuration from an asset file:

    let result = TextFile::load_asset("assets/config.txt").await;
    if let Ok(content) = result {
        for line in content.lines() {
            println!("Config: {}", line);
        }
    } else if let Err(e) = result {
        println!("Error loading config: {}", e);
    }

Platform notes:
- On desktop: Saves files with the exact filename you provide (include .txt extension)
- On web: Uses browser's localStorage via quad-storage with the same API
- Asset loading works on both platforms, but web requires files in the assets directory
*/

//use macroquad::prelude::*;

// Only import storage helpers when targeting WebAssembly
#[cfg(target_arch = "wasm32")]
use gloo_storage::{errors::StorageError, LocalStorage, Storage};

/// TextFile is a utility module for reading and writing text files
/// that works across all platforms, including web.
pub struct TextFile;

impl TextFile {
    /// Saves a vector of strings to a file or local storage (for web)
    pub async fn save(name: &str, data: Vec<String>) -> Result<(), String> {
        let joined = data.join("\n");
        
        #[cfg(target_arch = "wasm32")]
        {
            // debug!("Web: saving data to key '{}': {}", name, &joined);
            LocalStorage::set(name, &joined)
                .map(|_| { /* debug!("Web: saved key '{}'", name) */ })
                .map_err(|e| {
                    let error = format!("Failed to save to storage key '{}': {:?}", name, e);
                    error!("Web: {}", error);
                    error
                })
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Use the exact filename provided (no auto extension)
            let filename = name;
            
            std::fs::write(&filename, joined)
                .map_err(|e| format!("Failed to write to file {}: {}", filename, e))
        }
    }

    /// Loads a vector of strings from a file or local storage (for web)
    pub async fn load(name: &str) -> Result<Vec<String>, String> {
        #[cfg(target_arch = "wasm32")]
        {
            // debug!("Web: loading data from key '{}'", name);
            match LocalStorage::get::<String>(name) {
                Ok(content) => {
                    // debug!("Web: loaded data from key '{}': {}", name, content);
                    Ok(content.lines().map(|s| s.to_string()).collect())
                }
                Err(StorageError::KeyNotFound(_)) => {
                    // debug!("Web: No data found for key '{}'", name);
                    Ok(Vec::new())
                }
                Err(e) => {
                    let error = format!("Failed to load from storage key '{}': {:?}", name, e);
                    error!("Web: {}", error);
                    Err(error)
                }
            }
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Use the exact filename provided (no auto extension)
            let filename = name;
            
            match std::fs::read_to_string(&filename) {
                Ok(content) => Ok(content.lines().map(|s| s.to_string()).collect()),
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        Ok(Vec::new()) // Return empty vector if file doesn't exist yet
                    } else {
                        Err(format!("Failed to read file {}: {}", filename, e))
                    }
                }
            }
        }
    }

    /// Saves a vector of strings to a file or local storage (for web)
    /// Convenience method that takes Vec<&str> directly
    pub async fn save_strings<T: AsRef<str>>(name: &str, data: Vec<T>) -> Result<(), String> {
        let string_data: Vec<String> = data.into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();
        Self::save(name, string_data).await
    }

    /// Saves a vector of numbers to a file or local storage (for web)
    /// Handles any type that can be converted to a string
    #[allow(dead_code)]
    pub async fn save_numbers<T: ToString>(name: &str, data: Vec<T>) -> Result<(), String> {
        let string_data: Vec<String> = data.into_iter()
            .map(|n| n.to_string())
            .collect();
        Self::save(name, string_data).await
    }

    /// Loads a vector of strings from a file or local storage (for web)
    /// Alias for load() for consistent naming with save_strings()
    pub async fn load_strings(name: &str) -> Result<Vec<String>, String> {
        Self::load(name).await
    }

    /// Loads a vector of numbers from a file or local storage (for web)
    /// Handles any type that can be parsed from a string
    #[allow(dead_code)]
    pub async fn load_numbers<T>(name: &str) -> Result<Vec<T>, String> 
    where
        T: std::str::FromStr,
    {
        let strings = Self::load(name).await?;
        
        let mut numbers = Vec::with_capacity(strings.len());
        for s in strings {
            match s.parse::<T>() {
                Ok(n) => numbers.push(n),
                Err(_) => return Err(format!("Failed to parse '{}' as number", s))
            }
        }
        
        Ok(numbers)
    }
/*
    /// Loads an asset file (read-only data)
    #[allow(dead_code)]
    pub async fn load_asset(path: &str) -> Result<String, String> {
        match load_string(path).await {
            Ok(content) => Ok(content),
            Err(e) => Err(format!("Failed to load asset '{}': {:?}", path, e))
        }
    }
    */
}
