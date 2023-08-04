[![Crates.io](https://img.shields.io/crates/v/supabase-storage.svg)](https://crates.io/crates/supabase-storage)
[![Workflow Status](https://github.com/thefux/supabase-storage-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/thefux/supabase-storage-rust/actions/workflows/rust.yml/badge.svg?branch=main)


# Supabase Storage Rust Client

This is a Rust client library for interacting with Supabase Storage,
allowing you to perform various operations such as uploading, downloading,
and managing files in your Supabase Storage bucket.

## Getting Started

To use the Supabase Storage Rust client, you'll need to first create a `Storage` instance with your Supabase configuration.
Make sure to set the required environment variables before using the client library. You can use `dotenv` to load the environment variables from a `.env` file.

The SupabaseConfig assumes the presence of the environment variables SUPABASE_URL_STORAGE and SUPABASE_API_KEY, ensuring that both the authorization header and the base URL are appropriately configured.

```rust
use supabase_storage::Storage;
use supabase_storage::config::SupabaseConfig;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = SupabaseConfig::default();
    let storage = Storage::new_with_config(config);

    // Now you can use the `storage` instance to interact with Supabase Storage.
}
```

Utilizing the SupabaseConfig struct isn't obligatory. Alternatively, you can manually load the storage URL and API key values.

```rust
use reqwest::header::HeaderValue;
use supabase_storage::Storage;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let url = "<base storage url>";
    let api_key = "<api key>";
    let storage = Storage::new(url);

    let bucket_name = "thefux";

    let response = storage
        .from()
        .header("Authorization", HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap())
        .get_bucket_details(bucket_name)
        .execute()
        .await
        .unwrap();

    println!("{:?}", response);

    // Handle the response as needed.
}
```


## get a File

To get a file from Supabase Storage, you can use the `get_object` method of the `Storage` instance.

```rust
use supabase_storage::Storage;
use supabase_storage::config::SupabaseConfig;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = SupabaseConfig::default();
    let storage = Storage::new_with_config(config);

    let bucket_name = "thefux";
    let object = "test/bitcoin.pdf";

    let response = storage
        .from()
        .get_object(bucket_name, object)
        .execute()
        .await
        .unwrap();

    // Handle the response as needed.
}
```

## Updating an Object

You can also update an object in the bucket using the `update_object` method.
The update method is only async for now.

```rust
use supabase_storage::Storage;
use supabase_storage::config::SupabaseConfig;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = SupabaseConfig::default();
    let storage = Storage::new_with_config(config);

    let bucket_name = "thefux";
    let object = "btc.pdf";
    let file_path = "/user/test.pdf";

    let response = storage
        .from()
        .update_object_async(bucket_name, object, file_path)
        .await
        .execute()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:?}", response);

    // Handle the response as needed.
}
```

For those who prefer working with objects, rather than using the execute function, the execute_from function can be utilized, allowing for the subsequent parsing of the response.

```rust
use dotenv::dotenv;
use supabase_storage::Storage;
use supabase_storage::config::SupabaseConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub message: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = SupabaseConfig::default();
    let storage = Storage::new_with_config(config);

    let bucket_name = "thefux";
    let object = "btc.pdf";

    let response = storage
        .from()
        .delete_object(bucket_name, object)
        .execute_from::<Response>()
        .await
        .unwrap();

    println!("{:?}", response);

    // Handle the response as needed.
}
```

## Contributing
Calling all brilliant minds and passionate developers! 🚀
Feel free to join and make a difference in the project!

## License

This library is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

Feel free to add more examples and documentation to this readme based on your specific use cases and requirements.
