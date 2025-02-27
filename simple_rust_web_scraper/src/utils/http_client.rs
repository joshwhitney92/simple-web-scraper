use private_members::BlockingConcreteHTTPClient;

/*
 * Struct definitions
 */

/// Wraps a string representing a html response.
pub struct HTTPResponse {
    pub html: String,
}

#[derive(Debug, Default)]
/// Http client provider, *not* asyncronous.
pub struct HTTPClient {
    blocking_client: BlockingConcreteHTTPClient,
}

/*
 * Trait Definitions
 */

/// Provides function to make a blocking *(non async)* http request.
pub trait HTTPGetBlocking {
    /// Make non-async http request, given a `url`.
    fn get_blocking(&self, url: &str) -> Result<HTTPResponse, Box<dyn std::error::Error>>;
}

/*
 * Implementations
 */

impl HTTPClient {
    /// Sets up the blocking http client instance.
    pub fn init() -> Self {
        Self {
            blocking_client: BlockingConcreteHTTPClient::init(),
        }
    }
}

/// The client will simply implement the HTTPGetBlocking trait.
/// This allows us to mock the get request in our unit tests.
impl HTTPGetBlocking for HTTPClient {
    fn get_blocking(&self, url: &str) -> Result<HTTPResponse, Box<dyn std::error::Error>> {
        private_members::get_blocking(self, url)
    }
}

/*
 * Private Members
 */

/// Private module to encapsulate concrete http client implementations.
/// Only usable within the parent module.
mod private_members {

    /*
     * This concrete implementation uses the Reqwest crate.
     * It manages a http client and exposes trait methods
     * to perform common http requests.
     *
     * NOTE: Keep external dependencies (like `reqwest`)
     * encapsulated in this module.
     */

    use super::{HTTPClient, HTTPResponse};
    use reqwest::{cookie::Jar, redirect::Policy};
    use std::{rc::Rc, sync::Arc};

    #[derive(Debug, Default)]
    pub struct BlockingConcreteHTTPClient {
        inner: Rc<reqwest::blocking::Client>,
    }

    impl BlockingConcreteHTTPClient {
        pub fn init() -> Self {
            let cookie_jar = Arc::new(Jar::default());

            let client = reqwest::blocking::ClientBuilder::new()
            .redirect(Policy::default())
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .cookie_store(true)
            .cookie_provider(cookie_jar)
            .build()
            .unwrap();

            Self {
                inner: Rc::new(client),
            }
        }
    }

    pub fn get_blocking(
        client: &HTTPClient,
        url: &str,
    ) -> Result<HTTPResponse, Box<dyn std::error::Error>> {
        let response = client.blocking_client.inner.as_ref().get(url).send()?;
        Ok(HTTPResponse {
            html: response.text()?,
        })
    }
}
