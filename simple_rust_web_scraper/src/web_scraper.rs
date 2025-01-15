
pub trait ScrapeStrategy<T> {
    fn scrape_it(&self, scraper: &WebScraper, url: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>;
}

pub trait Scrape {
    fn scrape<R, T: ScrapeStrategy<R>>(&self, strategy: T, url: &str) -> Result<Vec<R>, Box<dyn std::error::Error>>;
}

#[allow(unused)]
pub struct WebScraper {}

#[allow(unused)]
impl WebScraper {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_string_from_element_with_css_class<'a>(
        &self,
        root: &scraper::ElementRef,
        css_class: &'a str,
    ) -> Result<String, Box<dyn std::error::Error + 'a>> {
        let selector = scraper::Selector::parse(css_class)?;
        let info = root
            .select(&selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_owned())
            .ok_or(format!("Country {css_class} not found"))?;

        Ok(info)
    }
}

impl Scrape for WebScraper {
    fn scrape<R, T: ScrapeStrategy<R>>(&self, strategy: T, url: &str) -> Result<Vec<R>, Box<dyn std::error::Error>> {
        strategy.scrape_it(self, url)
    }
}




