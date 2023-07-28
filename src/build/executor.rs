use crate::model::errors;
use reqwest::{header::HeaderMap, Client, Error, Method, RequestBuilder, Response, StatusCode};
use serde::Deserialize;

pub struct Executor {
    url: String,
    method: Method,
    client: Client,
    headers: HeaderMap,
    body: Option<String>,
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
    pub fn new<T>(method: Method, url: T, client: Client, headers: HeaderMap) -> Self
    where
        T: Into<String>,
    {
        Self {
            url: url.into(),
            method,
            client,
            headers,
            body: None,
        }
    }

    pub fn build(self) -> RequestBuilder {
        self.client
            .request(self.method, self.url.to_string())
            .headers(self.headers)
            .body(self.body.unwrap_or_default())
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
    /// use storage_rs::{
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
        self.build().send().await
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
    /// use storage_rs::{
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
        let response = self.build().send().await.unwrap();
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
