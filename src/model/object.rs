use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Response {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct SingedUrlToUpload {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct MoveCopyObject {
    #[serde(rename = "bucketId")]
    pub bucket_id: String,
    #[serde(rename = "sourceKey")]
    pub source_key: String,
    #[serde(rename = "destinationKey")]
    pub destination_key: String,
}
