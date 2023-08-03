use dotenv::dotenv;
use storage_rs::{
    config::SupabaseConfig,
    model::{
        bucket::BucketDetails,
        object::{Response, SingedUrlToUpload},
        options::{Format, Resize},
    },
    Storage,
};
use tokio::{
    fs::{copy, File},
    io::AsyncWriteExt,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let file = File::open("test.pdf").await.unwrap();
    let config = SupabaseConfig::default();
    let storage = Storage::new_with_config(config)
        .from()
        // .get_buckets()
        // .get_bucket_details("thefux")
        // .execute_from::<BucketDetails>()
        // .get_object("thefux", "test.png")
        // .delete_object("thefux", "hello.pdf")
        // .execute_from::<Response>()
        // .delete_objects("test", r#" { "prefixes": [ "bitcoin.pdf", "test.pdf" ] }"#)
        // .generate_pre_signed_url("thefux", "hello.pdf")
        // .execute_from::<SingedUrlToUpload>()
        // .get_object_with_pre_assigned_url("thefux", "hellp.pdf", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1cmwiOiJ0aGVmdXgvaGVsbG8ucGRmIiwiaWF0IjoxNjkwODMwODAyLCJleHAiOjE2OTA4MzgwMDJ9.YNyuBmbxsPG4Dxwh4tfz2_8Yo1MRI4coNQfnhymHXQ4")
        // .generate_signed_url("test", "hello.pdf", r#"{"expiresIn": 3600}"#)
        // .generate_signed_urls("test", "hello.pdf", r#"{"expiresIn": 3600, "paths": ["hello.pdf", "test.pdf"]}"#)
        // .update_object("thefux", "test123.pdf", "test.pdf")
        // .upload_object("thefux", "test123.pdf", "test.pdf")
        // .list_objects("test", r#"{
        //         "prefix": "",
        //         "limit": 100,
        //         "offset": 0
        //     }"#)
        // .move_object("thefux", "bitcoin.pdf", "test/bitcoin.pdf")
        // .delete_objects("thefux", r#"{ "prefixes": ["file_name.pdf", "test123.pdf"]}"#)
        // .get_public_object("thefux", "test/bitcoin.pdf")
        // .get_public_object_info("thefux", "test/bitcoin.pdf")
        // .update_bucket("thefux", r#"{ "public": true }"#)
        // .get_public_url("thefux", "test/bitcoin.pdf")
        // .get_public_object_with_transform("thefux", "test.png", storage_rs::model::options::Transform {
        //     format: Some(Format::Avif),
        //     height: Some(400),
        //     quality: Some(1),
        //     resize: Some(Resize::Cover),
        //     width: Some(200),
        // })
        // .create_signed_upload_url("thefux", "bitcoin.pdf")
        // .upload_to_signed_url_async(
        //     "thefux",
        //     "bitcoin.pdf",
        //     "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1cmwiOiJ0aGVmdXgvYml0Y29pbi5wZGYiLCJpYXQiOjE2OTEwMDkzNDAsImV4cCI6MTY5MTAxNjU0MH0.owm3P8VfzCmho4VmiVnrbvSymeYL3NpOhNR7MwxOBvE",
        //     "test.pdf",
        //     storage_rs::model::options::FileOptions { cache_control: None, content_type: None, upsert: Some(true) }).await
        .get_public_object_info("thefux", "test/bitcoin.pdf")
        .execute()
        .await
        .unwrap();
    // .text()
    // .await
    // .unwrap();

    // let mut file = File::create("test.pdf").await.unwrap();
    // let mut bytes = storage.bytes().await.unwrap();
    // file.write_all(&mut bytes).await.unwrap();
    println!("{:?}", storage);
}
