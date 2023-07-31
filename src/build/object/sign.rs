use reqwest::{header::HeaderValue, Method};

use crate::build::{
    builder::{BodyType, Builder},
    executor::Executor,
};

impl Builder {
    /// generate presigned url to retrieve an object
    ///
    /// # Arguments
    ///
    /// * `bucket_name` - bucket name
    /// * `object` - object name
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
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let config = SupabaseConfig::default();
    ///     let response = Storage::new_with_config(config)
    ///         .from()
    ///         .create_signed_url("thefux", "bitcoin.pdf", r#"
    ///             {
    ///                 "expiresIn": 3600,
    ///                 "transform": {
    ///                     "height": 0,
    ///                     "width": 0,
    ///                     "resize": "cover",
    ///                     "format": "origin",
    ///                     "quality": 100
    ///                 }
    ///             }"#)
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn create_signed_url(mut self, bucket_name: &str, object: &str, body: &str) -> Executor {
        self.headers
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        self.method = Method::POST;
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push("sign")
            .push(bucket_name)
            .push(object);

        self.body = Some(BodyType::StringBody(body.to_string()));
        self.create_executor()
    }

    /// generate presigned urls to retrieve objects
    ///
    /// # Arguments
    ///
    /// * `bucket_name` - bucket name
    /// * `object` - object name
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
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let config = SupabaseConfig::default();
    ///     let response = Storage::new_with_config(config)
    ///         .from()
    ///         .create_signed_urls("thefux", r#"{"expiresIn": 3600, "paths": ["hello.pdf", "test.pdf"]}"#)
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn create_signed_urls(mut self, bucket_name: &str, body: &str) -> Executor {
        self.headers
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        self.method = Method::POST;
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push("sign")
            .push(bucket_name);

        self.body = Some(BodyType::StringBody(body.to_string()));
        self.create_executor()
    }

    /// get object via pre-signed url
    ///
    /// # Arguments
    ///
    /// * `bucket_name` - bucket name
    /// * `object` - object name
    /// * `token` - sign token
    /// * `file` - file object
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
    /// use tokio::fs::File;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let config = SupabaseConfig::default();
    ///     let response = Storage::new_with_config(config)
    ///         .from()
    ///         .get_object_with_pre_assigned_url("thefux", "btc.pdf", "<token>")
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn get_object_with_pre_assigned_url(
        mut self,
        bucket_name: &str,
        object: &str,
        token: &str,
    ) -> Executor {
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push("sign")
            .push(bucket_name)
            .push(object);

        self.url.query_pairs_mut().append_pair("token", token);

        self.create_executor()
    }
}

#[cfg(test)]
mod test {
    use reqwest::{header::HeaderMap, Client, Method};
    use url::{Host, Origin};

    use crate::build::builder::{BodyType, Builder};

    #[test]
    fn test_get_object_with_signed_url() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .get_object_with_pre_assigned_url("thefux", "btc.pdf", "token");

        assert_eq!(executor.builder.method, Method::GET);
        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
        assert_eq!(executor.builder.url.path(), "/object/sign/thefux/btc.pdf");
        assert_eq!(executor.builder.url.query(), Some("token=token"));
    }

    #[test]
    fn test_create_signed_url() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .create_signed_url(
            "thefux",
            "btc.pdf",
            r#"
                    {
                        "expiresIn": 3600,
                        "transform": {
                            "height": 0,
                            "width": 0,
                            "resize": "cover",
                            "format": "origin",
                            "quality": 100
                        }
                    }"#,
        );

        if let Some(typ) = executor.builder.body {
            match typ {
                BodyType::StringBody(val) => assert_eq!(
                    val,
                    r#"
                    {
                        "expiresIn": 3600,
                        "transform": {
                            "height": 0,
                            "width": 0,
                            "resize": "cover",
                            "format": "origin",
                            "quality": 100
                        }
                    }"#
                ),
                _ => panic!("nop"),
            }
        }
        assert_eq!(executor.builder.method, Method::POST);
        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
        assert_eq!(executor.builder.url.path(), "/object/sign/thefux/btc.pdf");
    }

    #[test]
    fn test_create_signed_urls() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .create_signed_urls("thefux", r#"{"paths":["btc.pdf","test.pdf"]}"#);

        if let Some(typ) = executor.builder.body {
            match typ {
                BodyType::StringBody(val) => assert_eq!(val, r#"{"paths":["btc.pdf","test.pdf"]}"#),
                _ => panic!("nop"),
            }
        }
        assert_eq!(executor.builder.method, Method::POST);
        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
        assert_eq!(executor.builder.url.path(), "/object/sign/thefux");
    }
}
