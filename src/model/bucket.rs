use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct NewBucket {
    pub name: String,
    pub id: Option<String>,
    pub public: Option<bool>,
    pub file_size_limit: Option<u32>,
    pub allowed_mime_types: Option<Vec<String>>,
}

impl NewBucket {
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: None,
            public: None,
            file_size_limit: None,
            allowed_mime_types: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BucketDetails {
    pub name: String,
    pub id: String,
    pub public: bool,
    pub file_size_limit: Option<u32>,
    pub allowed_mime_types: Option<Vec<String>>,
    pub owner: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize)]
pub struct BucketUpdate {
    pub public: bool,
    pub file_size_limit: u32,
    pub allowed_mime_types: Vec<String>,
}
