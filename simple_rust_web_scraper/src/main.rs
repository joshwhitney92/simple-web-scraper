use simple_rust_web_scraper::{
    models::country::{Country, CountryStrategy},
    utils::http_client::HTTPClient,
    web_scraper::{Scrape, WebScraper},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.scrapethissite.com/pages/simple/";
    let scraper = WebScraper::new();
    let http_client = HTTPClient::new();
    let countries = scraper.scrape(CountryStrategy, &http_client, url)?;
    write_to_csv(&countries)?;

    Ok(())
}

pub fn write_to_csv(countries: &Vec<Country>) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_path("countries.csv")
        .map_err(|err| format!("Could not open file for writing!\nError: {}", err))?;

    // write the csv header
    writer.write_record(["name", "capital", "population", "area"])?;

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
