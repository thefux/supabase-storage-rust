use std::sync::{Arc, Mutex};

use super::executor::Executor;
use reqwest::{
    header::{HeaderMap, HeaderValue, IntoHeaderName},
    Body, Client, Error, Method, RequestBuilder, Response,
};

use url::Url;

#[derive(Debug)]
pub enum BodyType {
    StringBody(String),
    ReqwestBody(Body),
}

pub struct Builder {
    pub url: Url,
    pub headers: Arc<Mutex<HeaderMap>>,
    pub client: Arc<Mutex<Client>>,
    pub method: Method,
    pub body: Option<BodyType>,
}

impl Builder {
    /// Creates a new `Builder` instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL for the request.
    /// * `headers` - The `HeaderMap` containing the headers for the request.
    /// * `client` - The `Client` to use for making the request.
    ///
    /// # Example
    /// ```
    /// use supabase_storage::build::builder::Builder;
    /// use reqwest::header::{HeaderMap, HeaderValue};
    /// use reqwest::Client;
    /// use url::Url;
    ///
    /// let url = Url::parse("http://localhost").unwrap();
    /// let builder = Builder::new(url, HeaderMap::new(), Client::new());
    /// ```
    pub fn new(url: Url, headers: Arc<Mutex<HeaderMap>>, client: Arc<Mutex<Client>>) -> Self {
        Self {
            url,
            headers,
            client,
            method: Method::GET,
            body: None,
        }
    }

    /// Constructs and returns a `RequestBuilder` instance based on the current `Builder` configuration.
    ///
    /// # Returns
    ///
    /// * `RequestBuilder` - The constructed `RequestBuilder` instance.
    // pub fn build(self) -> RequestBuilder {
    //     self.client
    //         .request(self.method, self.url)
    //         .headers(self.headers)
    //         .body(self.body.unwrap_or_default())
    // }
    pub fn build(self) -> RequestBuilder {
        // let headers = self.headers.lock().unwrap();
        let headers = Arc::try_unwrap(self.headers).unwrap();
        let mut request = self
            .client
            .lock()
            .unwrap()
            .request(self.method, self.url.to_string())
            .headers(headers.into_inner().unwrap());

        if let Some(body) = self.body {
            match body {
                BodyType::StringBody(body_string) => request = request.body(body_string),
                BodyType::ReqwestBody(reqwest_body) => request = request.body(reqwest_body),
            }
        }

        request
    }

    /// Adds a new header to the request.
    ///
    /// # Arguments
    ///
    /// * `key` - The header name, implementors of `IntoHeaderName` are accepted.
    /// * `value` - The header value as a string.
    ///
    /// # Returns
    ///
    /// * `Self` - The updated `Builder` instance with the new header added.
    ///
    /// # Example
    ///
    /// ```
    /// use supabase_storage::build::builder::Builder;
    /// use reqwest::header::{HeaderMap, HeaderValue};
    /// use reqwest::Client;
    /// use url::Url;
    ///
    /// let url = Url::parse("http://localhost").unwrap();
    ///
    /// let _ = Builder::new(url, HeaderMap::new(), Client::new())
    ///     .header("Authorization", HeaderValue::from_static("Bearer <token>"));
    /// ```
    pub fn header(self, key: impl IntoHeaderName, value: HeaderValue) -> Self {
        self.headers.lock().unwrap().insert(key, value);
        self
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
    /// use supabase_storage::build::builder::Builder;
    /// use reqwest::header::{HeaderMap, HeaderValue};
    /// use reqwest::Client;
    /// use url::Url;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let url = Url::parse("http://localhost").unwrap();
    ///     let mut headers = HeaderMap::new();
    ///     headers.insert("Authorization", HeaderValue::from_static("Bearer YOUR_ACCESS_TOKEN"));
    ///
    ///     let builder = Builder::new(url, headers, Client::new())
    ///         .header("Authorization", HeaderValue::from_static("Bearer <token>"));
    ///
    ///     // Execute the request and handle the response
    ///     let response = builder.run().await;
    ///     match response {
    ///         Ok(response) => {
    ///             let body = response.text().await.unwrap();
    ///             println!("Response body: {:?}", body);
    ///         }
    ///         Err(error) => {
    ///             eprintln!("Error occurred: {:?}", error);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn run(self) -> Result<Response, Error> {
        self.build().send().await
    }

    /// Creates a new `Executor` instance based on the current `Builder` configuration.
    ///
    /// # Returns
    ///
    /// * `Executor` - The created `Executor` instance.
    pub fn create_executor(self) -> Executor {
        Executor::new(self)
    }
}

#[cfg(test)]
mod test {
    use reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    };
    use std::sync::{Arc, Mutex};
    use url::Url;

    use super::Builder;

    #[test]
    fn test_create_builder() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_static("Bearer test"));
        let url = Url::parse("http://localhost").unwrap();
        let builder = Builder::new(
            url,
            Arc::new(Mutex::new(headers)),
            Arc::new(Mutex::new(Client::new())),
        );
        assert_eq!(builder.url.scheme(), "http");
        assert_eq!(builder.headers.lock().unwrap().len(), 1);
    }

    #[test]
    fn test_add_header() {
        let url = Url::parse("http://localhost").unwrap();
        let builder = Builder::new(
            url,
            Arc::new(Mutex::new(HeaderMap::new())),
            Arc::new(Mutex::new(Client::new())),
        )
        .header("Authorization", HeaderValue::from_static("Bearer test"));
        assert_eq!(builder.headers.lock().unwrap().len(), 1);
    }
}
