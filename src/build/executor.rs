use crate::model::errors;
use reqwest::{Error, Response, StatusCode};
use serde::Deserialize;

use super::builder::Builder;

pub struct Executor {
    pub builder: Builder,
}

impl Executor {
    /// Creates a new `Executor` instance with the provided HTTP method, URL, client, and headers.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method for the request.
    /// * `url` - The URL for the request.
    /// * `client` - The `Client` to use for making the request.
    /// * `headers` - The `HeaderMap` containing the headers for the request.
    pub fn new(builder: Builder) -> Self {
        Self { builder }
    }

    /// Executes the constructed HTTP request and returns the response as a `Result`.
    ///
    /// # Returns
    ///
    /// * `Result<Response, Error>` - The result of the executed request.
    ///
    /// # Example
    ///
    /// ```
    /// use supabase_storage::{
    ///     Storage,
    ///     config::SupabaseConfig,
    /// };
    /// use dotenv::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let config = SupabaseConfig::default();
    ///     let storage = Storage::new_with_config(config);
    ///     let response = storage
    ///         .from()
    ///         .get_bucket_details("thefux")
    ///         .execute()
    ///         .await
    ///         .unwrap()
    ///         .text()
    ///         .await
    ///         .unwrap();
    ///
    ///     // Now 'response' contains the reponse as text.
    ///     println!("{:?}", response);
    /// }
    /// ```
    pub async fn execute(self) -> Result<Response, Error> {
        self.builder.build().send().await
    }

    /// Executes the constructed HTTP request and deserializes the response body into a generic struct.
    ///
    /// # Returns
    ///
    /// * `Result<T, errors::Error>` - The result of deserializing the response body into the provided generic struct.
    ///
    /// # Example
    ///
    /// ```
    /// use supabase_storage::{
    ///     Storage,
    ///     config::SupabaseConfig,
    ///     model::bucket::BucketDetails,
    /// };
    /// use dotenv::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let config = SupabaseConfig::default();
    ///     let storage = Storage::new_with_config(config);
    ///     let response = storage
    ///         .from()
    ///         .get_bucket_details("thefux")
    ///         .execute_from::<BucketDetails>()
    ///         .await
    ///         .unwrap();
    ///
    ///     // Now 'response' contains the deserialized 'BucketDetails' based on the response.
    ///     println!("{:?}", response);
    /// }
    /// ```
    pub async fn execute_from<T>(self) -> Result<T, errors::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self.builder.build().send().await.unwrap();
        let status = response.status();

        let text = response.text().await.unwrap();

        if status == StatusCode::OK {
            let result: T = serde_json::from_str(&text).unwrap();
            Ok(result)
        } else {
            let error: errors::Error = serde_json::from_str(&text).unwrap();
            Err(error)
        }
    }
}
