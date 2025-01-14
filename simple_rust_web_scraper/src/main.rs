#[allow(unused)]
struct Country {
    pub name: String,
    pub capital: String,
    pub population: String,
    pub area: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the target page
    let response = reqwest::blocking::get("https://www.scrapethissite.com/pages/simple/")?;

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
        let name = parse_info_from_root_as_string_by_css_class(
            &html_country_info_box_element,
            ".country-name",
        )?;
        let capital = parse_info_from_root_as_string_by_css_class(
            &html_country_info_box_element,
            ".country-capital",
        )?;
        let population = parse_info_from_root_as_string_by_css_class(
            &html_country_info_box_element,
            ".country-population",
        )?;
        let area = parse_info_from_root_as_string_by_css_class(
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

    for country in countries {
        println!("Country Name: {}", country.name);
        println!("Country Capital: {}", country.capital);
        println!("Country Population: {}", country.population);
        println!("Country Area: {}", country.area);
        println!("");
    }

    Ok(())
}

fn parse_info_from_root_as_string_by_css_class<'a>(
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
