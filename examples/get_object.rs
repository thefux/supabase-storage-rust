use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use supabase_storage::config::SupabaseConfig;
use supabase_storage::Storage;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = SupabaseConfig::default();
    let storage = Storage::new_with_config(config.clone());

    let mut headers = HeaderMap::new();
    if let Some(api_key) = config.clone().supabase_api_key {
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", api_key)).expect("header value is invalid"),
        );
        headers.insert(
            "apiKey",
            HeaderValue::from_str(&format!("{}", api_key)).expect("header value is invalid"),
        );
    }

    let bucket_name = "thefux";
    let object = "btc.pdf";

    let mut response = storage
        .from()
        .get_object(bucket_name, object)
        .execute()
        .await
        .unwrap();

    println!("{:?}", response);
    response = storage
        .from()
        .get_object(bucket_name, object)
        .execute()
        .await
        .unwrap();

    println!("{:?}", response);
}
