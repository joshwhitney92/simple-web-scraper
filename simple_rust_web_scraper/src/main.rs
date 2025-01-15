pub mod web_scraper;
pub mod models {
    pub mod country;
}
pub mod utils {
    pub mod http_client;
}

use models::country::{Country, CountryStrategy};
use web_scraper::{Scrape, WebScraper};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.scrapethissite.com/pages/simple/";
    let scraper = WebScraper::new();
    let countries = scraper.scrape(CountryStrategy, url)?;
    let _ = write_to_csv(&countries)?;

    Ok(())
}


pub fn write_to_csv(countries: &Vec<Country>) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_path("countries.csv").map_err(|err| {
        format!(
            "Could not open file for writing!\nError: {}",
            err.to_string()
        )
    })?;

    // write the csv header
    writer.write_record(&["name", "capital", "population", "area"])?;

    // populate the file with each country
    for country in countries {
        writer.write_record(&[
            country.name.clone(),
            country.capital.clone(),
            country.population.clone(),
            country.area.clone(),
        ])?;
    }

    Ok(())
}
