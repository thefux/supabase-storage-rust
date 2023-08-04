pub mod list;
pub mod move_copy;
pub mod public;
pub mod render;
pub mod sign;
pub mod upload;

pub use super::object::list::*;
pub use super::object::move_copy::*;
pub use super::object::public::*;
pub use super::object::render::*;
pub use super::object::sign::*;
pub use super::object::upload::*;

use reqwest::{header::HeaderValue, Body, Method};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::build::builder::BodyType;

use super::{builder::Builder, executor::Executor};

impl Builder {
    fn delete_object_intern(mut self) -> Executor {
        self.method = Method::DELETE;
        self.create_executor()
    }

    /// delete an object, could be any kind of data stored in the given storage
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
    ///         .delete_object("thefux", "file_name.pdf")
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn delete_object(mut self, bucket_id: &str, object: &str) -> Executor {
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push(bucket_id)
            .push(object);
        self.delete_object_intern()
    }

    /// delete multiple objects
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `body` - json object with list of perfixes
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
    ///         .delete_objects("thefux", r#"{ "prefixes" : [ "file_name.pdf" ]}"#)
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn delete_objects(mut self, bucket_id: &str, body: &str) -> Executor {
        self.headers
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push(bucket_id);
        self.body = Some(BodyType::StringBody(body.to_string()));
        self.delete_object_intern()
    }

    /// get an object from the storage
    ///
    /// # Arguments
    ///
    /// * `bucket_name` - bucket name
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
    ///         .delete_object("thefux", "file_name.pdf")
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn get_object(mut self, bucket_name: &str, object: &str) -> Executor {
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push(bucket_name)
            .push(object);
        self.create_executor()
    }

    async fn shared_upload(mut self, bucket_name: &str, object: &str, file_path: &str) -> Executor {
        let mime = mime_guess::from_path(object)
            .first_or_octet_stream()
            .to_string();
        self.headers
            .insert("Content-Type", HeaderValue::from_str(&mime).unwrap());

        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push(bucket_name)
            .push(object);

        let file = File::open(file_path).await.unwrap();
        let stream = FramedRead::new(file, BytesCodec::new());
        self.body = Some(BodyType::ReqwestBody(Body::wrap_stream(stream)));

        self.create_executor()
    }

    /// update an object
    ///
    /// # Arguments
    ///
    /// * `bucket_name` - bucket name
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
    ///         .update_object_async("thefux", "file_name.pdf", "out/test.pdf")
    ///         .await
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn update_object_async(
        mut self,
        bucket_name: &str,
        object: &str,
        file_path: &str,
    ) -> Executor {
        self.method = Method::PUT;
        self.shared_upload(bucket_name, object, file_path).await
    }

    /// upload an object
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
    ///         .upload_object("thefux", "file_name.pdf", "out/test.pdf")
    ///         .await
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn upload_object(
        mut self,
        bucket_name: &str,
        object: &str,
        file_path: &str,
    ) -> Executor {
        self.method = Method::POST;
        self.shared_upload(bucket_name, object, file_path).await
    }

    /// download object
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
    ///         .download_object("thefux")
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn download_object(mut self, bucket_id: &str) -> Executor {
        self.headers
            .insert("Content-Type", HeaderValue::from_static("application/json"));
        self.method = Method::POST;
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push(bucket_id);
        self.create_executor()
    }
}

#[cfg(test)]
mod test {
    use reqwest::{header::HeaderMap, Client};
    use url::{Host, Origin};

    use crate::build::builder::Builder;

    #[test]
    fn test_download_object() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .download_object("test_bucket");

        assert_eq!(
            executor.builder.headers.get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(executor.builder.url.path(), "/object/test_bucket");
        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
    }
}
