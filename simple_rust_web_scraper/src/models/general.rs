use crate::{
    utils::{html_prettifier::{HtmlPrettifier, Prettify}, http_client::HTTPGetBlocking},
    web_scraper::ScrapeStrategy,
};

use super::base::GetPrettyHTML;

#[derive(Debug, Clone, PartialEq)]
pub struct General {
    pub html: String,
}

pub struct GeneralStrategy;

impl GetPrettyHTML for General {
    fn get_html(&self) -> String {
        HtmlPrettifier::prettify(&self.html)
    }
}

#[allow(unused_variables)]
impl ScrapeStrategy<General> for GeneralStrategy {
    fn scrape_it(
        &self,
        scraper: &crate::web_scraper::WebScraper,
        url: &str,
        http_client: &crate::utils::http_client::HTTPClient,
    ) -> Result<Vec<General>, Box<dyn std::error::Error>> {
        // Connect to the target page

        // NOTE: We have injected the http_client dependency
        let response = http_client.get_blocking(url)?;

        // extract the raw html and print it
        let html = response.html;

        // NOTE: Inject this dependency
        // let document = scraper::Html::parse_document(&html);
        //
        let results = vec![General { html }];

        Ok(results)
    }
}
