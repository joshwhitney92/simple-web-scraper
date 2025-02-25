pub struct HTTPResponse {
    pub html: String
}

#[derive(Debug, Default)]
pub struct HTTPClient {}

pub trait HTTPGetBlocking {
    fn get_blocking(&self, url: &str) -> Result<HTTPResponse, Box<dyn std::error::Error>>;
}

impl HTTPClient {
    pub fn new() -> Self {
        Self {}
    }
}

/// Private module to encapsulate reqwest call.
mod private_members {
    use reqwest::blocking::Response;
    use super::HTTPResponse;

    pub fn get_blocking(url: &str) -> Result<HTTPResponse, Box<dyn std::error::Error>> {
        let response: Response = reqwest::blocking::get(url)?;
        Ok(HTTPResponse { 
            html: response.text()?
        })
    }
}

/// The client will simply implement the HTTPGetBlocking trait.
/// This allows us to mock the get request in our unit tests.
impl HTTPGetBlocking for HTTPClient {
    fn get_blocking(&self, url: &str) -> Result<HTTPResponse, Box<dyn std::error::Error>> {
        private_members::get_blocking(url)
    }
}

