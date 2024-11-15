# Team Exercise Oct 31, 2024

## What is this?

The Searchcraft engineering team often does small coding exercises together to compare solutions that we may come up with for a problem. These exercises are scoped to be completed in an hour or less.

## Exercise Parameters

- Create an in-memory cache in Rust that stores key-value pairs and automatically evicts entries after a given expiration time. The cache should allow insertion, retrieval, and manual invalidation of entries.
- Methods: `insert`, `get`, `invalidate`
- `get` method should automatically invalidate expired entries before retrieval

```rust
let mut cache = Cache::new();
cache.insert("user_xyz", "session_value", Duration::new(30, 0)); // TTL 30 seconds

if let Some(token) = cache.get("user_xyz") {
    println!("Token: {}", token);
} else {
    println!("Token expired or not found");
}

cache.invalidate("user_xyz");
```

## How to run this project

```bash
cargo run -- insert -k mykey -v myvalue -t 60
cargo run -- get -k mykey
cargo run -- invalidate -k mykey
```