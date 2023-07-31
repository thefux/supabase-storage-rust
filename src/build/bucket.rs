use reqwest::{header::HeaderValue, Method};

use crate::{
    build::{builder::Builder, executor::Executor},
    model::bucket::{BucketUpdate, NewBucket},
};

use super::builder::BodyType;

impl Builder {
    /// retrieve all buckets
    ///
    /// # Returns
    ///
    /// * `Executor` - The constructed `Executor` instance for executing the request.
    ///
    /// # Example
    /// ```
    /// use storage_rs::{
    ///     Storage,
    ///     config::SupabaseConfig,
    /// };
    /// use dotenv::dotenv;
    ///
    /// dotenv().ok();
    /// let config = SupabaseConfig::default();
    /// let storage = Storage::new_with_config(config)
    ///     .from()
    ///     .get_buckets()
    ///     .execute();
    /// ```
    pub fn get_buckets(mut self) -> Executor {
        self.url.path_segments_mut().unwrap().push("bucket");
        self.create_executor()
    }

    /// create a new bucket
    ///
    /// # Arguments
    ///
    /// * `body` - The request body as a string.
    ///
    /// # Returns
    ///
    /// * `Executor` - The constructed `Executor` instance for executing the request.
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
    /// dotenv().ok();
    /// let config = SupabaseConfig::default();
    /// let storage = Storage::new_with_config(config)
    ///     .from()
    ///     .create_bucket("thefux")
    ///     .execute();
    /// ```
    pub fn create_bucket(mut self, body: &str) -> Executor {
        self.method = Method::POST;
        self.url.path_segments_mut().unwrap().push("bucket");
        self.body = Some(BodyType::StringBody(body.into()));
        self.create_executor()
    }

    /// create a new bucket using a struct
    ///
    /// # Arguments
    ///
    /// * `body` - The `NewBucket` struct containing the request body.
    ///
    /// # Returns
    ///
    /// * `Executor` - The constructed `Executor` instance for executing the request.
    ///
    /// # Example
    /// ```
    /// use storage_rs::{
    ///     Storage,
    ///     config::SupabaseConfig,
    ///     model::bucket::NewBucket,
    /// };
    /// use dotenv::dotenv;
    ///
    /// dotenv().ok();
    /// let config = SupabaseConfig::default();
    /// let storage = Storage::new_with_config(config)
    ///     .from()
    ///     .create_bucket_from(NewBucket::new("thefux".to_string()))
    ///     .execute();
    /// ```
    pub fn create_bucket_from(mut self, body: NewBucket) -> Executor {
        self.method = Method::POST;
        self.url.path_segments_mut().unwrap().push("bucket");
        self.body = Some(BodyType::StringBody(
            serde_json::to_string(&body).unwrap_or_default(),
        ));
        self.create_executor()
    }

    /// empty the bucket
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - The identifier of the bucket to empty.
    ///
    /// # Returns
    ///
    /// * `Executor` - The constructed `Executor` instance for executing the request.
    ///
    /// # Example
    /// ```
    /// use storage_rs::{
    ///     Storage,
    ///     config::SupabaseConfig,
    /// };
    /// use dotenv::dotenv;
    ///
    /// dotenv().ok();
    /// let config = SupabaseConfig::default();
    /// let storage = Storage::new_with_config(config)
    ///     .from()
    ///     .empty_bucket("thefux")
    ///     .execute();
    /// ```
    pub fn empty_bucket(mut self, bucket_id: &str) -> Executor {
        self.method = Method::POST;
        self.url
            .path_segments_mut()
            .unwrap()
            .push("bucket")
            .push(bucket_id);

        self.create_executor()
    }

    /// get bucket details
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - The identifier of the bucket to empty.
    ///
    /// # Returns
    ///
    /// * `Executor` - The constructed `Executor` instance for executing the request.
    ///
    /// # Example
    /// ```
    /// use storage_rs::{
    ///     Storage,
    ///     config::SupabaseConfig,
    /// };
    /// use dotenv::dotenv;
    ///
    /// dotenv().ok();
    /// let config = SupabaseConfig::default();
    /// let storage = Storage::new_with_config(config)
    ///     .from()
    ///     .get_bucket_details("thefux")
    ///     .execute();
    /// ```
    pub fn get_bucket_details(mut self, bucket_id: &str) -> Executor {
        self.url
            .path_segments_mut()
            .unwrap()
            .push("bucket")
            .push(bucket_id);
        self.create_executor()
    }

    /// update the bucket
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - The identifier of the bucket to empty.
    ///
    /// # Returns
    ///
    /// * `Executor` - The constructed `Executor` instance for executing the request.
    ///
    /// # Example
    /// ```
    /// use storage_rs::{
    ///     Storage,
    ///     config::SupabaseConfig,
    /// };
    /// use dotenv::dotenv;
    ///
    /// dotenv().ok();
    /// let config = SupabaseConfig::default();
    /// let storage = Storage::new_with_config(config)
    ///     .from()
    ///     .update_bucket("thefux", r#"{ "public": true }"#)
    ///     .execute();
    /// ```
    pub fn update_bucket(mut self, bucket_id: &str, body: &str) -> Executor {
        self.headers
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        self.method = Method::PUT;
        self.url
            .path_segments_mut()
            .unwrap()
            .push("bucket")
            .push(bucket_id);
        self.body = Some(BodyType::StringBody(body.into()));
        self.create_executor()
    }

    /// update bucket using a struct
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - The identifier of the bucket to empty.
    ///
    /// # Returns
    ///
    /// * `Executor` - The constructed `Executor` instance for executing the request.
    ///
    /// # Example
    /// ```
    /// use storage_rs::{
    ///     Storage,
    ///     config::SupabaseConfig,
    ///     model::bucket::BucketUpdate,
    /// };
    /// use dotenv::dotenv;
    ///
    /// dotenv().ok();
    /// let config = SupabaseConfig::default();
    /// let storage = Storage::new_with_config(config)
    ///     .from()
    ///     .update_bucket_from("thefux", BucketUpdate {
    ///         public: false,
    ///         file_size_limit: Some(0),
    ///         allowed_mime_types: Some(vec!["application/pdf".to_string()]),
    ///     })
    ///     .execute();
    /// ```
    pub fn update_bucket_from(mut self, bucket_id: &str, body: BucketUpdate) -> Executor {
        self.method = Method::PUT;
        self.url
            .path_segments_mut()
            .unwrap()
            .push("bucket")
            .push(bucket_id);
        self.body = Some(BodyType::StringBody(
            serde_json::to_string(&body).unwrap_or_default(),
        ));
        self.create_executor()
    }

    /// delete a bucket
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - The identifier of the bucket to empty.
    ///
    /// # Returns
    ///
    /// * `Executor` - The constructed `Executor` instance for executing the request.
    ///
    /// # Example
    /// ```
    /// use storage_rs::{
    ///     Storage,
    ///     config::SupabaseConfig,
    /// };
    /// use dotenv::dotenv;
    ///
    /// dotenv().ok();
    /// let config = SupabaseConfig::default();
    /// let storage = Storage::new_with_config(config)
    ///     .from()
    ///     .delete_bucket("thefux")
    ///     .execute();
    /// ```
    pub fn delete_bucket(mut self, bucket_id: &str) -> Executor {
        self.method = Method::DELETE;
        self.url
            .path_segments_mut()
            .unwrap()
            .push("bucket")
            .push(bucket_id);
        self.create_executor()
    }
}
