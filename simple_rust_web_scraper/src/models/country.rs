use crate::web_scraper::{ScrapeStrategy, WebScraper};

#[allow(unused)]
pub struct Country {
    pub name: String,
    pub capital: String,
    pub population: String,
    pub area: String,
}

pub struct CountryStrategy;
impl ScrapeStrategy<Country> for CountryStrategy {
    fn scrape_it(&self, scraper: &WebScraper, url: &str) -> Result<Vec<Country>, Box<dyn std::error::Error>> {
        // Connect to the target page
        let response = reqwest::blocking::get(url)?;

        // extract the raw html and print it
        let html = response.text()?;
        let document = scraper::Html::parse_document(&html);

        // Get the top level Country container selector.
        let html_country_info_box_selector = scraper::Selector::parse(".country")?;

        // Retrieve the iterator over the actual DOM elements with the selector.
        let html_country_info_box_elements = document.select(&html_country_info_box_selector);

        // iterate over the country html elements
        // and scrape them all.
        let mut countries: Vec<Country> = Vec::new();

        for html_country_info_box_element in html_country_info_box_elements {
            // We can use the `html_country_info_box_element` \
            // to select the child elements.
            let name = scraper.parse_string_from_element_with_css_class(
                &html_country_info_box_element,
                ".country-name",
            )?;
            let capital = scraper.parse_string_from_element_with_css_class(
                &html_country_info_box_element,
                ".country-capital",
            )?;
            let population = scraper.parse_string_from_element_with_css_class(
                &html_country_info_box_element,
                ".country-population",
            )?;
            let area = scraper.parse_string_from_element_with_css_class(
                &html_country_info_box_element,
                ".country-area",
            )?;

            let country = Country {
                name,
                capital,
                population,
                area,
            };

            countries.push(country);
        }

        /* Print the country info */

        for country in countries.iter() {
            println!("Country Name: {}", country.name);
            println!("Country Capital: {}", country.capital);
            println!("Country Population: {}", country.population);
            println!("Country Area: {}", country.area);
            println!("");
        }

        Ok(countries)

    }
}
