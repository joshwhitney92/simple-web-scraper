use crate::{
    utils::{html_prettifier::{HtmlPrettifier, Prettify}, http_client::HTTPGetBlocking},
    web_scraper::{ElementToString, ScrapeStrategy},
};

use super::base::GetPrettyHTML;

#[derive(Debug, Clone, PartialEq)]
pub struct DiceJob {
    pub descrption: String,
}

pub struct DiceJobStrategy;

impl GetPrettyHTML for DiceJob {
    fn get_html(&self) -> String {
        HtmlPrettifier::prettify(&self.descrption)
    }
}

impl ScrapeStrategy<DiceJob> for DiceJobStrategy {
    fn scrape_it(
        &self,
        scraper: &crate::web_scraper::WebScraper,
        url: &str,
        http_client: &crate::utils::http_client::HTTPClient,
    ) -> Result<Vec<DiceJob>, Box<dyn std::error::Error>> {
        // Connect to the target page

        // NOTE: We have injected the http_client dependency
        let response = http_client.get_blocking(url)?;

        // extract the raw html and print it
        let html = response.html;

        // NOTE: Inject this dependency
        let document = scraper::Html::parse_document(&html);


        // Get only the job description
        let html_job_desc_selector = scraper::Selector::parse(r#"div[data-testid="jobDescriptionHtml"]"#)?;

        // Retrieve the iterator over the actual DOM elements with the selector.
        let html_job_desc_elements = document.select(&html_job_desc_selector);

        // iterate over the country html elements
        // and scrape them all.
        let mut jobs: Vec<DiceJob> = Vec::new();

        for html_job_detail_element in html_job_desc_elements {
            // We can use the `html_country_info_box_element` \
            // to select the child elements.
            let descrption = scraper.get_element_html(&html_job_detail_element)?;

            // let capital = scraper.parse_string_from_element_with_css_class(
            //     &html_country_info_box_element,
            //     ".country-capital",
            // )?;
            // let population = scraper.parse_string_from_element_with_css_class(
            //     &html_country_info_box_element,
            //     ".country-population",
            // )?;
            // let area = scraper.parse_string_from_element_with_css_class(
            //     &html_country_info_box_element,
            //     ".country-area",
            // )?;
            let dice_job = DiceJob { descrption };

            jobs.push(dice_job);
        }

        Ok(jobs)
    }
}
