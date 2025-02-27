use crate::{
    utils::{
        html_prettifier::{HtmlPrettifier, Prettify},
        http_client::HTTPGetBlocking,
    },
    web_scraper::{ElementToString, ScrapeStrategy},
};

use super::base::GetPrettyHTML;

#[derive(Debug, Clone, PartialEq)]
pub struct DiceJob {
    pub descrption: String,
    pub company_name: Option<String>,
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
        // NOTE: We have injected the http_client dependency
        let response = http_client.get_blocking(url)?;

        // extract the raw html and print it
        let html = response.html;

        // NOTE: Inject this dependency
        let document = scraper::Html::parse_document(&html);

        // Get only the job description
        let html_job_desc_selector =
            scraper::Selector::parse(r#"div[data-testid="jobDescriptionHtml"]"#)?;
        let html_company_name_selector =
            scraper::Selector::parse(r#"a[data-cy="companyNameLink"]"#)?;

        // Retrieve the iterator over the actual DOM elements with the selector.
        let html_job_desc_elements = document.select(&html_job_desc_selector);
        let company_name = document.select(&html_company_name_selector).next();

        // iterate over the country html elements
        // and scrape them all.
        let mut jobs: Vec<DiceJob> = Vec::new();

        for html_job_detail_element in html_job_desc_elements {
            // We can use the `html_country_info_box_element` \
            // to select the child elements.
            let descrption = scraper.get_element_html(&html_job_detail_element)?;

            let dice_job: DiceJob = match company_name {
                Some(inner) => DiceJob {
                    descrption,
                    company_name: Some(
                        inner
                            .text()
                            .next()
                            .unwrap_or("No_Name")
                            .replace(" ", "_")
                            .to_string(),
                    ),
                },
                _ => DiceJob {
                    descrption,
                    company_name: Some(String::from("No_Name")),
                },
            };

            jobs.push(dice_job);
        }

        Ok(jobs)
    }
}
