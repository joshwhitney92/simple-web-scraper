pub struct HTTPResponse {
    pub html: String
}

pub struct HTTPClient {}

pub trait HTTPGetBlocking {
    fn get_blocking(&self, url: &str) -> Result<HTTPResponse, Box<dyn std::error::Error>>;
}

impl HTTPClient {
    pub fn new() -> Self {
        Self {}
    }
}

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

impl HTTPGetBlocking for HTTPClient {
    fn get_blocking(&self, url: &str) -> Result<HTTPResponse, Box<dyn std::error::Error>> {
        private_members::get_blocking(&url)
    }
}
