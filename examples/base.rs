use dotenv::dotenv;
use storage_rs::{config::SupabaseConfig, model::bucket::BucketDetails, Storage};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = SupabaseConfig::default();
    let storage = Storage::new_with_config(config)
        .from()
        // .get_buckets()
        .get_bucket_details("thefux")
        .execute_from::<BucketDetails>()
        .await
        .unwrap();
    // .text()
    // .await
    // .unwrap();
    println!("{:?}", storage);
}
