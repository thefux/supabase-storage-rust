use crate::build::{builder::Builder, executor::Executor};

impl Builder {
    /// get public object from the storage
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `object` - a wildcard
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
    ///         .get_public_object("thefux", "file_name.pdf")
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn get_public_object(mut self, bucket_id: &str, object: &str) -> Executor {
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push("public")
            .push(bucket_id)
            .push(object);
        self.create_executor()
    }

    /// get public object info
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `object` - a wildcard
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
    ///         .get_public_object_info("thefux", "file_name.pdf")
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn get_public_object_info(mut self, bucket_id: &str, object: &str) -> Executor {
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push("info")
            .push("public")
            .push(bucket_id)
            .push(object);
        self.create_executor()
    }
}

#[cfg(test)]
mod test {
    use reqwest::{header::HeaderMap, Client, Method};
    use url::{Host, Origin};

    use super::*;

    #[test]
    fn test_get_public_object() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .get_public_object("thefux", "test.pdf");

        assert_eq!(executor.builder.method, Method::GET);
        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
        assert_eq!(
            executor.builder.url.path(),
            "/object/public/thefux/test.pdf"
        );
    }

    #[test]
    fn test_get_public_object_info() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .get_public_object_info("thefux", "test.pdf");

        assert_eq!(executor.builder.method, Method::GET);
        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
        assert_eq!(
            executor.builder.url.path(),
            "/object/info/public/thefux/test.pdf"
        );
    }
}
