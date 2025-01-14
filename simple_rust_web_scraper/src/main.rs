mod web_scraper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let countries = web_scraper::scrape_countries()?;
    let _ = web_scraper::write_to_csv(&countries)?;

    Ok(())
}
