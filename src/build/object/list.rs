use reqwest::{header::HeaderValue, Method};

use crate::build::{
    builder::{BodyType, Builder},
    executor::Executor,
};

impl Builder {
    /// list all files within a bucket
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `body` - request body
    ///
    /// # Returns
    ///
    /// * `Executor` - The constructed `Executor` instance for executing the request.
    ///
    /// # Example
    /// ```
    /// use supabase_storage::{
    ///     Storage,
    ///     config::SupabaseConfig,
    ///     model::bucket::NewBucket,
    /// };
    /// use dotenv::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let config = SupabaseConfig::default();
    ///     let response = Storage::new_with_config(config)
    ///         .from()
    ///         .list_objects("thefux", r#"
    ///             {
    ///                 "prefix": "bitcoin.pdf",
    ///                 "limit": 100,
    ///                 "offset": 0,
    ///                 "sortBy": {
    ///                     "column": "name",
    ///                     "order": "asc",
    ///                 },
    ///             }"#)
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn list_objects(mut self, bucket_id: &str, body: &str) -> Executor {
        self.headers
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        self.method = Method::POST;
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push("list")
            .push(bucket_id);

        self.body = Some(BodyType::StringBody(body.to_string()));
        self.create_executor()
    }
}

#[cfg(test)]
mod test {
    use reqwest::{header::HeaderMap, Client};
    use url::{Host, Origin};

    use crate::build::builder::{BodyType, Builder};

    #[test]
    fn test_list_objects() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .list_objects("test_bucket", r#"{"test": "body"}"#);

        assert_eq!(
            executor.builder.headers.get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(executor.builder.url.path(), "/object/list/test_bucket");

        if let Some(typ) = executor.builder.body {
            match typ {
                BodyType::StringBody(val) => assert_eq!(val, r#"{"test": "body"}"#.to_string()),
                _ => panic!("nop"),
            }
        }

        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
    }
}
