use reqwest::{header::HeaderValue, Body, Method};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    build::{
        builder::{BodyType, Builder},
        executor::Executor,
    },
    model::options::FileOptions,
};

impl Builder {
    fn url(&mut self, bucket_id: &str, object: &str) {
        self.url
            .path_segments_mut()
            .unwrap()
            .push("object")
            .push("upload")
            .push("sign")
            .push(bucket_id)
            .push(object);
    }

    /// generate pre-signed url to upload an object
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
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
    ///         .create_signed_upload_url("thefux", "bitcoin.pdf")
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn create_signed_upload_url(mut self, bucket_id: &str, object: &str) -> Executor {
        self.method = Method::POST;
        self.url(bucket_id, object);
        self.create_executor()
    }

    /// upload object via pre-signed url
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `object` - object name
    /// * `token` - sign token
    /// * `file_path` - file path
    /// * `file_options` - file options
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
    ///     model::options::FileOptions,
    /// };
    /// use dotenv::dotenv;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().ok();
    ///     let config = SupabaseConfig::default();
    ///     let response = Storage::new_with_config(config)
    ///         .from()
    ///         .upload_to_signed_url_async("thefux", "btc.pdf", "<token>", "out/test.pdf", FileOptions {
    ///             cache_control: None,
    ///             content_type: Some("application/pdf".to_string()),
    ///             upsert: Some(true),
    ///         })
    ///         .await
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn upload_to_signed_url_async(
        mut self,
        bucket_id: &str,
        object: &str,
        token: &str,
        file_path: &str,
        file_options: FileOptions,
    ) -> Executor {
        self.method = Method::PUT;
        self.url(bucket_id, object);

        if let Some(cache_content) = file_options.cache_control {
            self.headers.insert(
                "cache-control",
                HeaderValue::from_str(&format!("max-age={}", cache_content)).unwrap(),
            );
        }

        if let Some(content_type) = file_options.content_type {
            self.headers.insert(
                "content-type",
                HeaderValue::from_str(&content_type).unwrap(),
            );
        }

        self.url.query_pairs_mut().append_pair("token", token);

        let file = File::open(file_path).await.unwrap();
        let stream = FramedRead::new(file, BytesCodec::new());
        self.body = Some(BodyType::ReqwestBody(Body::wrap_stream(stream)));

        self.create_executor()
    }

    /// upload object via pre-signed url with auto detecting content-type
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
    /// * `object` - object name
    /// * `token` - sign token
    /// * `file_path` - file path
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
    ///         .upload_to_signed_url_no_options_async("thefux", "btc.pdf", "<token>", "out/test.pdf")
    ///         .await
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn upload_to_signed_url_no_options_async(
        mut self,
        bucket_id: &str,
        object: &str,
        token: &str,
        file_path: &str,
    ) -> Executor {
        let mime = mime_guess::from_path(object)
            .first_or_octet_stream()
            .to_string();
        self.headers
            .insert("Content-Type", HeaderValue::from_str(&mime).unwrap());

        self.method = Method::PUT;
        self.url(bucket_id, object);

        self.url.query_pairs_mut().append_pair("token", token);

        let file = File::open(file_path).await.unwrap();
        let stream = FramedRead::new(file, BytesCodec::new());
        self.body = Some(BodyType::ReqwestBody(Body::wrap_stream(stream)));

        self.create_executor()
    }

    /// upload object via pre-signed url
    ///
    /// # Arguments
    ///
    /// * `bucket_id` - bucket id
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
    ///     model::options::FileOptions,
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
    ///         .upload_from_file_with_pre_assigned_url("thefux", "btc.pdf", "<token>",
    ///             File::open("out/test.pdf").await.unwrap(),
    ///             FileOptions {
    ///                 cache_control: None,
    ///                 content_type: None,
    ///                 upsert: None,
    ///             })
    ///         .execute()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn upload_from_file_with_pre_assigned_url(
        mut self,
        bucket_id: &str,
        object: &str,
        token: &str,
        file: File,
        file_options: FileOptions,
    ) -> Executor {
        if let Some(cache_content) = file_options.cache_control {
            self.headers.insert(
                "cache-control",
                HeaderValue::from_str(&format!("max-age={}", cache_content)).unwrap(),
            );
        }

        if let Some(content_type) = file_options.content_type {
            self.headers.insert(
                "content-type",
                HeaderValue::from_str(&content_type).unwrap(),
            );
        } else {
            let mime = mime_guess::from_path(object)
                .first_or_octet_stream()
                .to_string();
            self.headers
                .insert("Content-Type", HeaderValue::from_str(&mime).unwrap());
        }

        if let Some(upsert) = file_options.upsert {
            self.headers.insert(
                "x-upsert",
                HeaderValue::from_str(&upsert.to_string()).unwrap(),
            );
        }

        self.method = Method::PUT;
        self.url(bucket_id, object);

        self.url.query_pairs_mut().append_pair("token", token);

        let stream = FramedRead::new(file, BytesCodec::new());
        self.body = Some(BodyType::ReqwestBody(Body::wrap_stream(stream)));

        self.create_executor()
    }
}

#[cfg(test)]
mod test {
    use reqwest::{header::HeaderMap, Client, Method};
    use url::{Host, Origin};

    use super::*;

    #[test]
    fn test_create_signed_upload_url() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .create_signed_upload_url("thefux", "bitcoin.pdf");

        assert_eq!(executor.builder.method, Method::POST);
        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
        assert_eq!(
            executor.builder.url.path(),
            "/object/upload/sign/thefux/bitcoin.pdf"
        );
    }

    #[tokio::test]
    async fn test_upload_to_signed_url_async() {
        let executor = Builder::new(
            url::Url::parse("http://localhost").unwrap(),
            HeaderMap::new(),
            Client::new(),
        )
        .upload_to_signed_url_async(
            "thefux",
            "btc.pdf",
            "token",
            "out/test.pdf",
            FileOptions {
                cache_control: None,
                content_type: Some("application/pdf".to_string()),
                upsert: Some(true),
            },
        )
        .await;

        assert_eq!(executor.builder.method, Method::PUT);
        assert_eq!(
            executor.builder.url.origin(),
            Origin::Tuple("http".into(), Host::Domain("localhost".into()), 80)
        );
        assert_eq!(
            executor.builder.url.path(),
            "/object/upload/sign/thefux/btc.pdf"
        );
        assert_eq!(executor.builder.url.query(), Some("token=token"));
    }
}
