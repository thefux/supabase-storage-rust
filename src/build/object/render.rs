use crate::{
    build::{builder::Builder, executor::Executor},
    model::options::Transform,
};

impl Builder {
    /// get public object from the storage
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `object` - object name/path
    /// * `transform` - tranformation options to transform before serving it to client
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
    ///     model::options::{Transform, Format, Resize}
    /// };
    /// use dotenv::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let config = SupabaseConfig::default();
    ///     let response = Storage::new_with_config(config)
    ///         .from()
    ///         .get_object_with_transform("thefux", "test.png", Transform {
    ///             format: Some(Format::Origin),
    ///             height: Some(0),
    ///             quality: Some(0),
    ///             resize: Some(Resize::Cover),
    ///             width: Some(0),
    ///         })
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn get_object_with_transform(
        mut self,
        bucket_id: &str,
        object: &str,
        transform: Transform,
    ) -> Executor {
        self.url
            .path_segments_mut()
            .unwrap()
            .push("render")
            .push("image")
            .push("authenticated")
            .push(bucket_id)
            .push(object);

        self.url
            .set_query(Some(&serde_qs::to_string(&transform).unwrap()));

        self.create_executor()
    }
}

#[cfg(test)]
mod test {
    use reqwest::{header::HeaderMap, Client, Method};
    use url::{Host, Origin};

    use crate::{
        build::builder::BodyType,
        model::options::{Format, Resize},
    };

    use super::*;

    #[test]
    fn test_get_object_with_transform() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .get_object_with_transform(
            "thefux",
            "test.png",
            Transform {
                format: Some(Format::Origin),
                height: Some(0),
                quality: Some(0),
                resize: Some(Resize::Cover),
                width: Some(0),
            },
        );

        if let Some(typ) = executor.builder.body {
            match typ {
                BodyType::StringBody(val) => assert_eq!(
                    val,
                    r#"
                {
                    "format":"origin",
                    "height":0,
                    "quality":"cover",
                    "resize":0,
                    "width":0,
                }"#
                ),
                _ => panic!("nop"),
            }
        }

        assert_eq!(executor.builder.method, Method::GET);
        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
        assert_eq!(
            executor.builder.url.path(),
            "/render/image/authenticated/thefux/test.png"
        );
    }
}
