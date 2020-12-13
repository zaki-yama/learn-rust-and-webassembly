use reqwest::blocking::ClientBuilder;
use std::time::Duration;
use url::Url;
use web_crawler_book::crawler::Crawler;
use web_crawler_book::LinkExtractor;

fn main() -> eyre::Result<()> {
    env_logger::init();

    let url = std::env::args()
        .nth(1)
        .unwrap_or(String::from("https://www.rust-lang.org"));
    let url = Url::parse(&url)?;
    let client = ClientBuilder::new().build()?;
    let extractor = LinkExtractor::from_client(client);

    let crawler = Crawler::new(&extractor, url);

    let wait = Duration::from_millis(100);

    for url in crawler.take(10) {
        println!("{}", url);
        std::thread::sleep(wait.clone());
    }

    Ok(())
}
