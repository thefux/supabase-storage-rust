use reqwest::header::HeaderValue;
use reqwest::Method;

use crate::build::builder::BodyType;
use crate::build::executor::Executor;
use crate::model::object::MoveCopyObject;
use crate::Builder;

enum Action {
    Move,
    Copy,
}

impl From<Action> for &str {
    fn from(value: Action) -> Self {
        match value {
            Action::Move => "move",
            Action::Copy => "copy",
        }
    }
}

impl Builder {
    pub(crate) fn action_intern(mut self, move_obj: MoveCopyObject, action: &str) -> Executor {
        self.headers
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        self.method = Method::POST;
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push(action);

        self.body = Some(BodyType::StringBody(
            serde_json::to_string(&move_obj).unwrap(),
        ));

        self.create_executor()
    }

    fn action_intern_from(self, move_obj: MoveCopyObject, action: &str) -> Executor {
        self.action_intern(move_obj, action)
    }

    fn action_intern_explicit(
        self,
        bucket_id: &str,
        from: &str,
        to: &str,
        action: &str,
    ) -> Executor {
        let move_body = MoveCopyObject {
            bucket_id: bucket_id.to_string(),
            source_key: from.to_string(),
            destination_key: to.to_string(),
        };

        self.action_intern(move_body, action)
    }

    /// move an object
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `from` - object soruce
    /// * `to` - object destination
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
    ///         .move_object("thefux", "from", "to")
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn move_object(self, bucket_id: &str, from: &str, to: &str) -> Executor {
        self.action_intern_explicit(bucket_id, from, to, Action::Move.into())
    }

    /// move an object
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `from` - object soruce
    /// * `to` - object destination
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
    ///     model::object::MoveCopyObject,
    /// };
    /// use dotenv::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let config = SupabaseConfig::default();
    ///     let move_obj = MoveCopyObject {
    ///         bucket_id: "thefux".to_string(),
    ///         source_key: "from".to_string(),
    ///         destination_key: "to".to_string(),
    ///     };
    ///     let response = Storage::new_with_config(config)
    ///         .from()
    ///         .move_object_from(move_obj)
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn move_object_from(self, obj: MoveCopyObject) -> Executor {
        self.action_intern(obj, Action::Move.into())
    }

    /// copy an object
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `from` - object soruce
    /// * `to` - object destination
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
    ///         .copy_object("thefux", "from", "to")
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn copy_object(self, bucket_id: &str, from: &str, to: &str) -> Executor {
        self.action_intern_explicit(bucket_id, from, to, Action::Copy.into())
    }

    /// copy an object
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `from` - object soruce
    /// * `to` - object destination
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
    ///     model::object::MoveCopyObject,
    /// };
    /// use dotenv::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let config = SupabaseConfig::default();
    ///     let move_obj = MoveCopyObject {
    ///         bucket_id: "thefux".to_string(),
    ///         source_key: "from".to_string(),
    ///         destination_key: "to".to_string(),
    ///     };
    ///     let response = Storage::new_with_config(config)
    ///         .from()
    ///         .copy_object_from(move_obj)
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn copy_object_from(self, obj: MoveCopyObject) -> Executor {
        self.action_intern_from(obj, Action::Copy.into())
    }
}

#[cfg(test)]
mod test {
    use reqwest::{header::HeaderMap, Client, Method};
    use url::{Host, Origin};

    use crate::{
        build::builder::{BodyType, Builder},
        model::object::MoveCopyObject,
    };

    #[test]
    fn test_copy_object() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .copy_object("thefux", "from", "to");

        assert_eq!(
            executor.builder.headers.get("Content-Type").unwrap(),
            "application/json"
        );

        if let Some(typ) = executor.builder.body {
            match typ {
                BodyType::StringBody(val) => assert_eq!(
                    val,
                    r#"{"bucketId":"thefux","sourceKey":"from","destinationKey":"to"}"#.to_string()
                ),
                _ => panic!("nop"),
            }
        }

        assert_eq!(executor.builder.method, Method::POST);
        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
        assert_eq!(executor.builder.url.path(), "/object/copy");
    }

    #[test]
    fn test_move_object() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .move_object("thefux", "from", "to");

        assert_eq!(executor.builder.url.path(), "/object/move");
    }

    #[test]
    fn test_move_object_from() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .move_object_from(MoveCopyObject {
            bucket_id: "thefux".to_string(),
            source_key: "from".to_string(),
            destination_key: "to".to_string(),
        });

        if let Some(typ) = executor.builder.body {
            match typ {
                BodyType::StringBody(val) => assert_eq!(
                    val,
                    r#"{"bucketId":"thefux","sourceKey":"from","destinationKey":"to"}"#.to_string()
                ),
                _ => panic!("nop"),
            }
        }
        assert_eq!(executor.builder.url.path(), "/object/move");
    }

    #[test]
    fn test_copy_object_from() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .copy_object_from(MoveCopyObject {
            bucket_id: "thefux".to_string(),
            source_key: "from".to_string(),
            destination_key: "to".to_string(),
        });

        assert_eq!(executor.builder.url.path(), "/object/copy");
    }
}
