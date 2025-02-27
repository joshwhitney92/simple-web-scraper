use clap::{Parser, ValueEnum};
use simple_rust_web_scraper::{
    models::{base::GetPrettyHTML, dice_job::DiceJobStrategy, general::GeneralStrategy},
    utils::{
        file_writer::{AppendToFile, CreateFile, FileWriter},
        http_client::HTTPClient,
    },
    web_scraper::{Scrape, WebScraper},
};

#[derive(Debug, Clone, ValueEnum)]
enum Strategy {
    Dice,
    General,
}

#[derive(Parser, Debug)]
struct Args {
    /// URL to scrape
    #[arg(short, long)]
    url: String,

    #[arg(short, long, value_enum)]
    strategy: Strategy,
}

/// Run the command parser to greet names
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Collect the arguments with clap
    let args = Args::parse();
    let url = args.url;

    // Dependencies for scraping
    let scraper = WebScraper::new();
    let http_client = HTTPClient::new();
    let file_path = String::from("output.html");

    // This will hold the scraped results
    let results: Vec<Box<dyn GetPrettyHTML>>;

    match args.strategy {
        Strategy::General => {
            results = scraper
                .scrape(GeneralStrategy, &http_client, &url)?
                .into_iter()
                .map(|x| Box::new(x) as Box<dyn GetPrettyHTML>)
                .collect();
        }
        Strategy::Dice => {
            results = scraper
                .scrape(DiceJobStrategy, &http_client, &url)?
                .into_iter()
                .map(|x| Box::new(x) as Box<dyn GetPrettyHTML>)
                .collect();
        }
    }

    if results.len() > 0 {
        FileWriter::create_file(&file_path)?;

        for res in results {
            FileWriter::append_text_to_file(&res.get_html(), &file_path)?;
        }
    }

    Ok(())
}
