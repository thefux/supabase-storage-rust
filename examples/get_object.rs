use dotenv::dotenv;
use supabase_storage::config::SupabaseConfig;
use supabase_storage::Storage;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = SupabaseConfig::default();
    let storage = Storage::new_with_config(config);

    let bucket_name = "thefux";
    let object = "btc.pdf";

    let response = storage
        .from()
        .get_object(bucket_name, object)
        .execute()
        .await
        .unwrap();

    println!("{:?}", response);
}
