use std::collections::HashMap;
use std::fs;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use anyhow::Result;

/// A key-value cache with automatic expiration
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    value: T,
    expiry: u64,
}

/// An in-memory cache that automatically evicts entries after their TTL expires
///
/// # Example
///
/// ```
/// use std::time::Duration;
/// use memory_cache::Cache;
/// let mut cache = Cache::new();
///
/// // Store a value with 30 second TTL
/// cache.insert("api_key", "secret123", Duration::from_secs(30));
///
/// // Retrieve the value
/// assert_eq!(cache.get("api_key"), Some("secret123"));
///
/// // Manually invalidate
/// cache.invalidate("api_key");
/// assert_eq!(cache.get("api_key"), None);
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Cache<T> {
    entries: HashMap<String, CacheEntry<T>>,
}

impl<T: Clone> Cache<T> {
    /// Creates a new empty cache
    ///
    /// # Example
    ///
    /// ```
    /// use memory_cache::Cache;
    /// let cache: Cache<String> = Cache::new();
    /// ```
    pub fn new() -> Self {
        Cache {
            entries: HashMap::new(),
        }
    }

    /// Inserts a value into the cache with a specified TTL
    ///
    /// # Example
    ///
    /// ```
    /// use std::time::Duration;
    /// use memory_cache::Cache;
    /// let mut cache = Cache::new();
    /// cache.insert("session", "token123", Duration::from_secs(60));
    /// ```
    pub fn insert(&mut self, key: &str, value: T, ttl: Duration) {
        // Calculate the absolute expiry timestamp
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.entries.insert(
            key.to_string(),
            CacheEntry {
                value,
                expiry: now + ttl.as_secs()
            }
        );
    }

    /// Retrieves a value from the cache, returning None if expired or not found
    ///
    /// # Example
    ///
    /// ```
    /// use std::time::Duration;
    /// use memory_cache::Cache;
    /// let mut cache = Cache::new();
    /// cache.insert("user", "alice", Duration::from_secs(30));
    ///
    /// if let Some(user) = cache.get("user") {
    ///     println!("Found user: {}", user);
    /// }
    /// ```
    pub fn get(&mut self, key: &str) -> Option<T> {
        if let Some(entry) = self.entries.get(key) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if now < entry.expiry {
                return Some(entry.value.clone());
            }
            self.invalidate(key);
        }
        None
    }

    /// Manually removes an entry from the cache
    ///
    /// # Example
    ///
    /// ```
    /// use std::time::Duration;
    /// use memory_cache::Cache;
    /// let mut cache = Cache::new();
    /// cache.insert("temp", "data", Duration::from_secs(60));
    /// cache.invalidate("temp");
    /// assert_eq!(cache.get("temp"), None);
    /// ```
    pub fn invalidate(&mut self, key: &str) {
        self.entries.remove(key);
    }
}

const CACHE_FILE: &str = "cache_state.json";

pub fn load_cache() -> Result<Cache<String>> {
    match fs::read_to_string(CACHE_FILE) {
        Ok(contents) => {
            let cache: Cache<String> = serde_json::from_str(&contents)?;
            Ok(cache)
        }
        Err(_) => {
            let cache = Cache::new();
            save_cache(&cache)?;
            Ok(cache)
        }
    }
}

pub fn save_cache(cache: &Cache<String>) -> Result<()> {
    let serialized = serde_json::to_string(cache)?;
    fs::write(CACHE_FILE, serialized)?;
    Ok(())
}